[CmdletBinding(DefaultParameterSetName='Live')]
param(
    [Parameter(Mandatory,ParameterSetName='Offline')][string]$Observed,
    [Parameter(ParameterSetName='Live')][string]$ApplyPreview,
    [string]$PreviewPath,
    [string[]]$Ids
)
$ErrorActionPreference='Stop'
$repo='GabrielNichols/mekanism-rust-foundry'
$repoId=1447583916
$root=Split-Path -Parent $PSScriptRoot
$mapPath=Join-Path $root 'docs/planning/github-roadmap.json'
$map=Get-Content -Raw -LiteralPath $mapPath|ConvertFrom-Json
$manifestHash=(Get-FileHash -LiteralPath $mapPath).Hash
$selected=@($map.roadmap|Where-Object {-not $Ids -or $_.id -cin $Ids})
if($Ids|Where-Object {$_ -cnotin $selected.id}){throw 'Unknown or non-roadmap ID in milestone selection'}

function Invoke-GitHub($arguments){
    $result=gh api @arguments
    if($LASTEXITCODE -ne 0){throw "GitHub milestone operation failed: $($arguments[0])"}
    $result
}
function Read-Observed {
    if($Observed){return (Get-Content -Raw -LiteralPath $Observed|ConvertFrom-Json)}
    $milestones=@(Invoke-GitHub @('--paginate',"repos/$repo/milestones?state=all&per_page=100")|ConvertFrom-Json)
    $pages=Invoke-GitHub @('--paginate','--slurp',"repos/$repo/issues?state=all&per_page=100")|ConvertFrom-Json -NoEnumerate
    $issues=@(foreach($page in $pages){foreach($row in $page){if(-not $row.pull_request){$row}}})
    @{milestones=$milestones;issues=$issues}
}
function New-Plan($observedState){
    $byIssue=@{}
    foreach($issue in $observedState.issues){$byIssue[$issue.number]=$issue}
    $byTitle=@{}
    foreach($milestone in $observedState.milestones){
        if($byTitle.ContainsKey($milestone.title)){throw "Duplicate milestone $($milestone.title)"}
        $byTitle[$milestone.title]=$milestone
    }
    $milestones=@(foreach($phase in $selected.phase|Sort-Object -Unique){
        $live=$byTitle[$phase]
        [ordered]@{phase=$phase;action=if($live){'preserve'}else{'create'};number=$live.number}
    })
    $issues=@(foreach($row in $selected){
        $issue=$byIssue[$row.issue]
        $live=$byTitle[$row.phase]
        $action=if(-not $issue -or $issue.body -notmatch "(?m)^<!-- meklab-task: $($row.id) -->\s*$"){'conflict'}
            elseif($issue.milestone -and $issue.milestone.title -cne $row.phase){'preserve_manual'}
            elseif($issue.milestone){'preserve'}
            elseif($live.state -ceq 'closed'){'conflict'}
            else{'assign'}
        [ordered]@{id=$row.id;issue=$row.issue;phase=$row.phase;current=$issue.milestone.title;action=$action}
    })
    [ordered]@{schema=1;repository=$repo;manifest_hash=$manifestHash;milestones=$milestones;issues=$issues}
}
$plan=New-Plan (Read-Observed)
if($ApplyPreview){
    $reviewed=Get-Content -Raw -LiteralPath $ApplyPreview|ConvertFrom-Json -AsHashtable
    if([long](Invoke-GitHub @("repos/$repo",'--jq','.id')) -ne $repoId -or
       $reviewed.repository -cne $repo -or $reviewed.manifest_hash -cne $manifestHash -or
       ($reviewed.issues.id -join ',') -cne ($plan.issues.id -join ',')){throw 'Reviewed milestone plan identity changed'}
    if($plan.issues|Where-Object action -eq 'conflict'){throw 'Resolve milestone conflicts before applying'}
    $numbers=@{}
    foreach($row in $plan.milestones){
        if($row.action -eq 'create' -and -not ($reviewed.milestones|Where-Object {$_.phase -ceq $row.phase -and $_.action -ceq 'create'})){
            throw "Unreviewed milestone creation $($row.phase)"
        }
        if($row.number){$numbers[$row.phase]=$row.number}
    }
    foreach($row in $plan.issues|Where-Object action -eq 'assign'){
        if(-not ($reviewed.issues|Where-Object {$_.id -ceq $row.id -and $_.issue -eq $row.issue -and $_.phase -ceq $row.phase -and $_.action -ceq 'assign'})){
            throw "Unreviewed issue assignment $($row.id)"
        }
    }
    foreach($row in $plan.milestones|Where-Object action -eq 'create'){
        $phase=$row.phase
        $criteria="https://github.com/$repo/blob/main/docs/planning/MILESTONES.md#$($phase.ToLowerInvariant())"
        $numbers[$phase]=[int](Invoke-GitHub @('--method','POST',"repos/$repo/milestones",'-f',"title=$phase",'-f',"description=Exit criteria and evidence: $criteria",'--jq','.number'))
        Write-Host "Created milestone $phase"
        Start-Sleep -Milliseconds 1250
    }
    foreach($row in $plan.issues|Where-Object action -eq 'assign'){
        $current=Invoke-GitHub @("repos/$repo/issues/$($row.issue)")|ConvertFrom-Json
        if($current.body -notmatch "(?m)^<!-- meklab-task: $($row.id) -->\s*$"){throw "Issue identity changed $($row.id)"}
        if($current.milestone){Write-Host "Preserved live milestone on #$($row.issue)"; continue}
        $number=$numbers[$row.phase]
        Invoke-GitHub @('--method','PATCH',"repos/$repo/issues/$($row.issue)",'-F',"milestone=$number",'--jq','.milestone.title')|Out-Null
        Write-Host "Assigned #$($row.issue) to $($row.phase)"
        Start-Sleep -Milliseconds 1250
    }
    $plan=New-Plan (Read-Observed)
}
$json=$plan|ConvertTo-Json -Depth 8
if($PreviewPath){[IO.File]::WriteAllText([IO.Path]::GetFullPath($PreviewPath),$json+[Environment]::NewLine)}
$json
