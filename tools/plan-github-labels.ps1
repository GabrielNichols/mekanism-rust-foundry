[CmdletBinding(DefaultParameterSetName='Catalog')]
param(
    [Parameter(ParameterSetName='Catalog')][string]$ObservedLabels,
    [Parameter(ParameterSetName='Catalog')][string]$PreviewPath,
    [Parameter(ParameterSetName='Catalog')][string]$ApplyPreview,
    [Parameter(Mandatory,ParameterSetName='Transition')][string]$Transition
)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$catalogPath=Join-Path $root 'docs/planning/github-labels.json'
$catalog=Get-Content -Raw -LiteralPath $catalogPath|ConvertFrom-Json
$definitions=@{}
$surfaces='discussion','issue','pull_request'
foreach($row in $catalog.labels){
    if($row.name -cnotmatch '^[a-z-]+:[A-Za-z0-9-]+$' -or
       $row.color -cnotmatch '^[0-9a-f]{6}$' -or $row.description.Length -gt 100 -or
       -not $row.surfaces -or ($row.surfaces|Where-Object {$_ -cnotin $surfaces}) -or
       $row.authority -cne 'maintainer' -or $definitions.ContainsKey($row.name)){
        throw "Invalid or duplicate label definition $($row.name)"
    }
    $definitions[$row.name]=$row
}

if($Transition){
    $item=Get-Content -Raw -LiteralPath $Transition|ConvertFrom-Json
    if($item.surface -cnotin $surfaces){throw 'Unsupported transition surface'}
    $add=@()
    $remove=@()
    $conflicts=@()
    $status=$null
    switch -CaseSensitive ($item.event){
        'submitted' {$add=@('workflow:needs-triage')}
        'triaged' {$remove=@('workflow:needs-triage')}
        'ready' {
            if($item.surface -cne 'issue' -or $item.assigned -ne $false -or $item.open -ne $true){
                throw 'Ready requires an open unassigned issue'
            }
            $add=@('status:ready-to-claim')
            $remove=@('workflow:needs-triage')
            $status='Ready'
        }
        'claimed' {
            if($item.surface -cne 'issue' -or $item.assigned -ne $true){throw 'Claim requires an assigned issue'}
            $remove=@('status:ready-to-claim')
        }
        'work-started' {
            if($item.surface -cne 'issue' -or $item.assigned -ne $true){throw 'Starting work requires an assigned issue'}
            $remove=@('status:ready-to-claim')
            $status='In Progress'
        }
        'blocked' {
            if($item.surface -cne 'issue'){throw 'Blocked status applies to an issue'}
            $remove=@('status:ready-to-claim')
            $status='Blocked'
        }
        'evidence-requested' {$add=@('workflow:needs-evidence')}
        'evidence-accepted' {$remove=@('workflow:needs-evidence')}
        'linked' {
            $eligible=@(
                foreach($source in $item.sources){
                    if($source.surface -cnotin $surfaces){throw 'Unsupported source surface'}
                    foreach($label in $source.labels){
                        $definition=$definitions[$label]
                        if($definition.policy -in 'durable','family','phase' -and
                           $source.surface -cin $definition.surfaces -and $item.surface -cin $definition.surfaces){
                            $definition.name
                        }
                    }
                }
            )|Sort-Object -Unique
            foreach($prefix in 'type:*','phase:*'){
                $options=@((@($eligible)+@($item.labels))|Where-Object {$_ -like $prefix}|Sort-Object -Unique)
                if($options.Count -gt 1){$conflicts+=($options -join ', ')}
            }
            $add=@($eligible|Where-Object {$_})
        }
        default {throw "Unsupported transition event $($item.event)"}
    }
    foreach($name in ($add+$remove)){
        if($item.surface -cnotin $definitions[$name].surfaces){throw "Label $name does not apply to $($item.surface)"}
    }
    if($conflicts.Count){
        $add=@()
        $remove=@()
    }
    [ordered]@{
        event=$item.event;surface=$item.surface;conflicts=$conflicts
        add=@($add|Where-Object {$_ -cnotin $item.labels})
        remove=@($remove|Where-Object {$_ -cin $item.labels})
        project_status=if($status -cne $item.project_status){$status}
    }|ConvertTo-Json -Depth 6
    return
}

function Read-Labels {
    if($ObservedLabels){
        $json=Get-Content -Raw -LiteralPath $ObservedLabels
        return ,@($json|ConvertFrom-Json)
    }
    $json=gh api --paginate --slurp "repos/$($catalog.repository)/labels?per_page=100"
    if($LASTEXITCODE -ne 0){throw 'GitHub label read failed'}
    return ,@(foreach($page in ($json|ConvertFrom-Json -NoEnumerate)){foreach($label in $page){$label}})
}
function Get-CatalogPlan {
    $existing=@{}
    foreach($label in (Read-Labels)){$existing[$label.name]=$label}
    foreach($row in $catalog.labels){
        $live=$existing[$row.name]
        $action=if(-not $live){'create'}elseif($row.color -cne $live.color -or
                                               $row.description -cne $live.description){'preserve_different'}else{'preserve'}
        [ordered]@{action=$action;name=$row.name;color=$row.color;description=$row.description}
    }
}
$hash=(Get-FileHash -LiteralPath $catalogPath).Hash
$plan=@(Get-CatalogPlan)
if($ApplyPreview){
    if($ObservedLabels){throw 'Apply requires a fresh live label read'}
    $approved=Get-Content -Raw -LiteralPath $ApplyPreview|ConvertFrom-Json -AsHashtable
    $id=gh api "repos/$($catalog.repository)" --jq '.id'
    if($LASTEXITCODE -ne 0 -or [long]$id -ne $catalog.repository_id -or
       $approved.catalog_hash -cne $hash -or $approved.repository -cne $catalog.repository){
        throw 'Repository or catalog preview identity mismatch'
    }
    $reviewed=@{}
    foreach($row in $approved.labels){$reviewed[$row.name]=$row}
    $pending=@($plan|Where-Object action -eq 'create')
    foreach($row in $pending){
        $prior=$reviewed[$row.name]
        if($prior.action -cne 'create' -or $prior.color -cne $row.color -or $prior.description -cne $row.description){
            throw "Unreviewed create $($row.name)"
        }
    }
    foreach($row in $pending){
        gh label create $row.name --repo $catalog.repository --color $row.color --description $row.description|Out-Null
        if($LASTEXITCODE -ne 0){throw "Could not create $($row.name). Re-preview before retrying."}
        Start-Sleep -Milliseconds 1150
    }
    $plan=@(Get-CatalogPlan)
}
$json=[ordered]@{repository=$catalog.repository;catalog_hash=$hash;labels=$plan}|ConvertTo-Json -Depth 8
if($PreviewPath){[IO.File]::WriteAllText([IO.Path]::GetFullPath($PreviewPath),$json+[Environment]::NewLine)}
$json
