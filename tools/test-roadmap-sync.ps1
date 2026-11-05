$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$roadmap=Get-Content -Raw (Join-Path $root 'docs/planning/roadmap.json')|ConvertFrom-Json
$mapping=Get-Content -Raw (Join-Path $root 'docs/planning/github-roadmap.json')|ConvertFrom-Json
$onboarding=Get-Content -Raw (Join-Path $root 'docs/planning/issue-queue.json')|ConvertFrom-Json
$fixture=New-TemporaryFile
try {
    if(($mapping.roadmap.id -join ',') -cne ($roadmap.workitems.id -join ',') -or
       ($mapping.roadmap.phase -join ',') -cne ($roadmap.workitems.legacy_phase -join ',') -or
       ($mapping.roadmap.family -join ',') -cne ($roadmap.workitems.stream -join ',')){
        throw 'GitHub mapping disagrees with canonical roadmap'
    }
    if(($mapping.onboarding.id -join ',') -cne (($onboarding.tasks.id+@('START-HERE')) -join ',')){
        throw 'GitHub mapping disagrees with onboarding queue'
    }
    $entries=$mapping.onboarding+$mapping.roadmap
    if(($entries.issue|Sort-Object -Unique).Count -ne $entries.Count){throw 'Duplicate issue number'}
    $sample=$mapping.roadmap[0].id
    $fixtureRows=@(
        @{number=101;title='Maintainer edit';body="<!-- meklab-task: $sample -->";state='closed'}
        @{number=102;title="[$($mapping.roadmap[1].id)] Title without stable marker";body='';state='open'}
    )
    ConvertTo-Json -InputObject $fixtureRows | Set-Content -LiteralPath $fixture.FullName
    $sync=Join-Path $PSScriptRoot 'sync-roadmap.ps1'
    $result=& $sync -ObservedIssues $fixture.FullName -Ids @($sample,$sample,$mapping.roadmap[1].id,$mapping.roadmap[2].id,$onboarding.tasks[0].id)|ConvertFrom-Json
    if(($result.tasks.action -join ',') -cne 'preserve_existing,conflict,prepare_create,conflict' -or
       $result.tasks[0].number -ne 101 -or $result.tasks[0].state -cne 'closed'){
        throw 'Issue planner lost stable identity, existing state, or conflict detection'
    }
    Write-Host "Verified issue mapping for $($entries.Count) tasks and offline reconciliation controls."
} finally {
    Remove-Item -LiteralPath $fixture.FullName
}
