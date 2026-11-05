$ErrorActionPreference='Stop'
$planner=Join-Path $PSScriptRoot 'plan-github-labels.ps1'
$fixture=New-TemporaryFile
function Get-Transition($item){
    $item|ConvertTo-Json -Depth 6|Set-Content -LiteralPath $fixture.FullName
    & $planner -Transition $fixture.FullName|ConvertFrom-Json
}
try {
    $issue=@{surface='issue';event='ready';labels=@('area:processing','workflow:needs-triage','status:planned');open=$true;assigned=$false;project_status='Todo'}
    $step=Get-Transition $issue
    if(($step.add -join ',') -cne 'status:ready-to-claim' -or ($step.remove -join ',') -cne 'workflow:needs-triage' -or $step.project_status -cne 'Ready'){
        throw 'Readiness did not remove triage and set the expected issue status'
    }
    $issue.labels=@('area:processing','status:planned','status:ready-to-claim')
    $issue.project_status='Ready'
    $step=Get-Transition $issue
    if($step.add.Count -or $step.remove.Count -or $step.project_status){throw 'Repeated readiness produced a write'}
    $issue.event='claimed'
    $issue.assigned=$true
    $step=Get-Transition $issue
    if(($step.remove -join ',') -cne 'status:ready-to-claim' -or $step.project_status){throw 'Claim did not remove only the discovery label'}
    $discussion=@{surface='discussion';labels=@('area:processing','type:scenario','workflow:needs-triage')}
    $newIssue=@{surface='issue';event='linked';labels=@();sources=@($discussion)}
    $step=Get-Transition $newIssue
    if(($step.add -join ',') -cne 'area:processing,type:scenario'){throw 'Discussion promoted workflow state or lost durable labels'}
    $linked=@{surface='issue';labels=@('area:processing','type:scenario','family:pilot','phase:M08','status:ready-to-claim','workflow:needs-evidence','approved')}
    $pr=@{surface='pull_request';event='linked';labels=@('type:scenario');sources=@($linked);project_status='Draft'}
    $step=Get-Transition $pr
    if(($step.add -join ',') -cne 'area:processing,family:pilot,phase:M08' -or $step.project_status){throw 'PR linked state copied workflow or lost classification'}
    $pr.labels+=@($step.add)
    $repeat=Get-Transition $pr
    if($repeat.add.Count -or $repeat.remove.Count){throw 'Repeating PR propagation produced a write'}
    $pr.sources+=@(@{surface='issue';labels=@('type:docs','phase:M00','area:runtime')})
    $step=Get-Transition $pr
    if($step.conflicts.Count -ne 2 -or $step.add.Count -or $step.remove.Count){throw 'Multiple linked issues did not fail closed'}
    $pr.event='evidence-accepted'
    $pr.labels+=@('workflow:needs-evidence','maintenance-custom')
    $step=Get-Transition $pr
    if(($step.remove -join ',') -cne 'workflow:needs-evidence'){throw 'Evidence acceptance removed unrelated metadata'}

    '[{"name":"area:processing","color":"ffffff","description":"Maintained separately"}]'|Set-Content -LiteralPath $fixture.FullName
    $preview=& $planner -ObservedLabels $fixture.FullName|ConvertFrom-Json
    if(($preview.labels|Where-Object name -eq 'area:processing').action -cne 'preserve_different' -or
       ($preview.labels|Where-Object name -eq 'type:scenario').action -cne 'create'){throw 'Catalog would overwrite maintained metadata or miss a label'}
    $catalog=Get-Content -Raw (Join-Path $PSScriptRoot '../docs/planning/github-labels.json')|ConvertFrom-Json
    ConvertTo-Json -InputObject $catalog.labels -Depth 6|Set-Content -LiteralPath $fixture.FullName
    $preview=& $planner -ObservedLabels $fixture.FullName|ConvertFrom-Json
    if($preview.labels|Where-Object action -ne 'preserve'){throw 'Matching catalog did not produce an empty write set'}
    Write-Host 'Verified taxonomy previews, idempotence, cross-surface propagation, conflicts, and label removal.'
} finally {
    Remove-Item -LiteralPath $fixture.FullName
}
