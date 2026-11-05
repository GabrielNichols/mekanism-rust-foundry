$ErrorActionPreference='Stop'
$fixture=New-TemporaryFile
$sync=Join-Path $PSScriptRoot 'sync-roadmap-milestones.ps1'
function Get-Plan($state){
    $state|ConvertTo-Json -Depth 8|Set-Content -LiteralPath $fixture.FullName
    & $sync -Observed $fixture.FullName -Ids 'RF-SCOPE-01','RF-SCOPE-02','RF-MODEL-01'|ConvertFrom-Json
}
try {
    $m00=@{title='M00';number=1;state='open'}
    $manual=@{title='Maintainer milestone';number=20;state='open'}
    $first=@{number=9;body='<!-- meklab-task: RF-SCOPE-01 -->';milestone=$null}
    $second=@{number=10;body='<!-- meklab-task: RF-SCOPE-02 -->';milestone=$manual}
    $third=@{number=17;body='<!-- meklab-task: RF-MODEL-01 -->';milestone=$null}
    $state=@{milestones=@($m00,$manual);issues=@($first,$second,$third)}
    $plan=Get-Plan $state
    if(($plan.milestones.action -join ',') -cne 'preserve,create' -or ($plan.issues.action -join ',') -cne 'assign,preserve_manual,assign'){throw 'Unexpected milestone diff'}
    $m01=@{title='M01';number=2;state='open'}
    $state.milestones+=@($m01)
    $first.milestone=$m00
    $third.milestone=$m01
    $plan=Get-Plan $state
    if($plan.issues|Where-Object action -in 'assign','conflict'){throw 'Repeat preview proposed a new write'}
    $third.body='<!-- meklab-task: RF-MODEL-02 -->'
    if((Get-Plan $state).issues[-1].action -cne 'conflict'){throw 'Wrong issue marker accepted'}
    Write-Host 'Verified milestone creation, assignments, repeat no-op, manual preservation and stable marker conflict.'
} finally {
    Remove-Item -LiteralPath $fixture.FullName
}
