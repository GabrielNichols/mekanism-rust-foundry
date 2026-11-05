[CmdletBinding(DefaultParameterSetName='Remote')]
param(
    [Parameter(Mandatory,ParameterSetName='Remote')][ValidatePattern('^[A-Za-z0-9-]+/[A-Za-z0-9_.-]+$')][string]$Repository,
    [Parameter(Mandatory,ParameterSetName='Offline')][string]$ObservedIssues,
    [string[]]$Ids,
    [string]$PreviewPath,
    [Parameter(ParameterSetName='Remote')][string]$ApplyPreview,
    [Parameter(ParameterSetName='Remote')][long]$ExpectedRepositoryId
)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$roadmapPath=Join-Path $root 'docs/planning/roadmap.json'
$queuePath=Join-Path $root 'docs/planning/issue-queue.json'
$roadmap=Get-Content -Raw -LiteralPath $roadmapPath|ConvertFrom-Json -AsHashtable
$queue=Get-Content -Raw -LiteralPath $queuePath|ConvertFrom-Json -AsHashtable
$manifestHash=(Get-FileHash -LiteralPath $roadmapPath).Hash+(Get-FileHash -LiteralPath $queuePath).Hash
$byId=[ordered]@{}
function Add-Task($task){
    if($byId.Contains($task.id)){throw "Duplicate task $($task.id)"}
    $byId[$task.id]=$task
}
foreach($row in $queue.tasks){Add-Task ([ordered]@{id=$row.id;title=$row.title;existing_only=$true})}
Add-Task ([ordered]@{id='START-HERE';title='Start here — choose a small first contribution';labels=@('status:ready-to-claim','good first issue')
    body=@'
Start with the README quickstart and one small issue from the first wave.

- #1 covers one runtime registry and recipe pair.
- #2 covers a blocked factory lane fixture.
- #6 covers a deterministic missing-worker test.

Ask to claim the issue, run the dedicated Java smoke case in an isolated checkout, and attach sanitized evidence with the pull request template. The Parity Lab records the pinned Java reference. Gameplay porting follows scope qualification.

The umbrella is #7. The roadmap and work packages are the specification. Issues track ownership and progress.
'@})
foreach($row in $roadmap.workitems){
    if($row.id -cnotmatch '^RF-[A-Z]+-[0-9]+$'){throw "Invalid roadmap ID $($row.id)"}
    $state=if($row.depends_on.Count -or $row.unresolved_runtime_requirements){'blocked'}else{'planned'}
    $deps=if($row.depends_on.Count){$row.depends_on -join ', '}else{'None declared'}
    $topology=$row.topology_refs -join ', '
    $evidence=@($row.evidence_required|ForEach-Object {"- $_"}) -join [Environment]::NewLine
    $sections=@(
        '## Scope',"$($row.title). Gate $($row.target_gate), milestone $($row.legacy_phase), family $($row.stream).",
        '## Readiness',"$state. Advance only after the linked prerequisites and bindings are reviewed.",
        '## Dependencies',$deps,
        '## Acceptance',$row.functional_acceptance,
        '## Suggested first PR',$row.minimal_green_deliverable,
        '## Validation',"Topology: $topology. $($row.red_contract) Run the affected contracts and a fresh reference when this slice requires Java. Preserve the expected result.",
        '## Negative control',$row.negative_control,
        '## Required evidence',$evidence,
        '## Qualification boundary',$row.release_slice_rule
    )
    Add-Task ([ordered]@{id=$row.id;title=$row.title;labels=@('phase:'+$row.legacy_phase,'family:'+$row.stream.ToLowerInvariant(),'status:'+$state)
        body=($sections -join ([Environment]::NewLine+[Environment]::NewLine))})
}
if($byId.Count -ne ($roadmap.workitems.Count+$queue.tasks.Count+1)){throw 'Roadmap coverage mismatch'}
function Read-Observed {
    if($ObservedIssues){
        $rows=Get-Content -Raw -LiteralPath $ObservedIssues|ConvertFrom-Json -AsHashtable -NoEnumerate
        if($rows -isnot [array]){throw 'Observed snapshot must be an array'}
        return ,$rows
    }
    $raw=gh api --paginate --slurp "repos/$Repository/issues?state=all&per_page=100"
    if($LASTEXITCODE -ne 0){throw 'Issue API read failed'}
    return ,@(foreach($page in ($raw|ConvertFrom-Json -AsHashtable -NoEnumerate)){
        foreach($issue in $page){if(-not $issue.Contains('pull_request')){$issue}}
    })
}
function Get-Plan($observed,[string[]]$selection){
    $markers=@{}
    $titles=@{}
    foreach($issue in $observed){
        if([int]$issue.number -le 0){throw 'Observed issue is missing its number'}
        foreach($match in [regex]::Matches([string]$issue.body,'(?m)^<!-- meklab-task: ([A-Z0-9-]+) -->\s*$')){
            $id=$match.Groups[1].Value
            if(-not $markers.ContainsKey($id)){$markers[$id]=@()}
            $markers[$id]+= $issue
        }
        $match=[regex]::Match([string]$issue.title,'^\[([A-Z0-9-]+)\]')
        if($match.Success){
            $id=$match.Groups[1].Value
            if(-not $titles.ContainsKey($id)){$titles[$id]=@()}
            $titles[$id]+= $issue
        }
    }
    $seen=[System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
    foreach($id in $selection){
        if(-not $byId.Contains($id)){throw "Unknown task ID $id"}
        if(-not $seen.Add($id)){continue}
        $task=$byId[$id]
        $marked=@($markers[$id]|Where-Object {$_})
        $candidates=@(($marked+@($titles[$id]))|Where-Object {$_}|Sort-Object -Property number -Unique)
        if($candidates.Count -eq 1 -and $marked.Count -eq 1){
            [ordered]@{id=$task.id;action='preserve_existing';number=$candidates[0].number;state=$candidates[0].state}
        } elseif($candidates.Count -or $task.existing_only){
            [ordered]@{id=$task.id;action='conflict';numbers=@($candidates.number)}
        } else {
            $url=if($Repository){"https://github.com/$Repository"}else{'https://github.com/GabrielNichols/mekanism-rust-foundry'}
            $body="<!-- meklab-task: $($task.id) -->"+[Environment]::NewLine+[Environment]::NewLine+$task.body
            $body += [Environment]::NewLine+[Environment]::NewLine+"[Roadmap]($url/blob/main/docs/planning/ROADMAP.md) · [Contributor guide]($url/blob/main/CONTRIBUTING.md)"
            [ordered]@{id=$task.id;action='prepare_create';title="[$($task.id)] $($task.title)";body=$body;labels=$task.labels}
        }
    }
}
$observed=Read-Observed
$selection=if($Ids){$Ids}else{@($byId.Keys)}
$tasks=@(Get-Plan $observed $selection)
if($ApplyPreview){
    $approved=Get-Content -Raw -LiteralPath $ApplyPreview|ConvertFrom-Json -AsHashtable
    $repoId=gh api "repos/$Repository" --jq '.id'
    if($LASTEXITCODE -ne 0 -or -not $ExpectedRepositoryId -or [long]$repoId -ne $ExpectedRepositoryId -or
       $approved.repository -cne $Repository -or $approved.manifest_hash -cne $manifestHash){
        throw 'Repository or reviewed preview identity mismatch'
    }
    if(($tasks.id -join ',') -cne ($approved.tasks.id -join ',')){throw 'Reviewed task selection changed'}
    if(@($tasks|Where-Object action -eq 'conflict').Count){throw 'Resolve issue conflicts before applying'}
    $ready=@($tasks|Where-Object action -eq 'prepare_create')
    foreach($task in $ready){
        $prior=@($approved.tasks|Where-Object id -eq $task.id)
        if($prior.Count -ne 1 -or $prior[0].action -ne 'prepare_create' -or
            $prior[0].title -cne $task.title -or $prior[0].body -cne $task.body -or
            ($prior[0].labels -join ',') -cne ($task.labels -join ',')){throw "Unreviewed create $($task.id)"}
    }
    if($ready.Count){
        $known=gh label list --repo $Repository --limit 1000 --json name --jq '.[].name'
        if($LASTEXITCODE -ne 0){throw 'Label API read failed'}
        foreach($label in @($ready|ForEach-Object labels|Sort-Object -Unique)){
            if($label -in $known){continue}
            $color=switch -Wildcard ($label){'phase:*'{'bfdadc'} 'family:*'{'c5def5'} 'status:blocked'{'b60205'} default{'e4e669'}}
            gh label create $label --repo $Repository --color $color | Out-Null
            if($LASTEXITCODE -ne 0){throw "Label create failed for $label"}
            Start-Sleep -Milliseconds 1150
        }
        foreach($task in $ready){
            $args=@('--repo',$Repository,'--title',$task.title,'--body',$task.body)
            foreach($label in $task.labels){$args+=@('--label',$label)}
            $url=gh issue create @args
            if($LASTEXITCODE -ne 0 -or $url -notmatch '/issues/[0-9]+$'){throw "Issue creation failed for $($task.id). Re-preview before continuing."}
            Write-Host "$($task.id) $url"
            Start-Sleep -Milliseconds 1250
        }
        # GitHub can lag immediately after a batch. Retry only the read.
        for($attempt=1;$attempt -le 4;$attempt++){
            $tasks=@(Get-Plan (Read-Observed) $selection)
            if(-not @($tasks|Where-Object action -ne 'preserve_existing').Count){break}
            if($attempt -lt 4){Start-Sleep -Seconds (2*$attempt)}
        }
        if(@($tasks|Where-Object action -ne 'preserve_existing').Count){throw 'Post-apply issue mapping incomplete'}
    }
}
$plan=[ordered]@{schema=1;repository=$Repository;manifest_hash=$manifestHash;tasks=$tasks}
$json=$plan|ConvertTo-Json -Depth 16
if($PreviewPath){[IO.File]::WriteAllText([IO.Path]::GetFullPath($PreviewPath),$json+[Environment]::NewLine)}
$json
