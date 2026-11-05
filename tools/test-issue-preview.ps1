$ErrorActionPreference='Stop'
$preview=Join-Path $PSScriptRoot 'preview-issues.ps1'
$temp=Join-Path ([IO.Path]::GetTempPath()) ('meklab-issue-preview-'+[Guid]::NewGuid().ToString('N')+'.json')
function Preview($rows){
    [IO.File]::WriteAllText($temp,(ConvertTo-Json -InputObject $rows -Depth 8))
    & $preview -Ids W1-CAT,W1-PROC,W1-CAT -ObservedIssues $temp | ConvertFrom-Json
}
try {
    $first=Preview @()
    if($first.tasks.Count -ne 2 -or $first.tasks[0].action -ne 'prepare_create'){throw 'First batch must deduplicate requested IDs'}
    $existing=@(@{number=9;state='closed';title='Maintainer rewrote the title';body=('Edited body'+[Environment]::NewLine+'<!-- meklab-task: W1-CAT -->')})
    $second=Preview $existing
    if($second.tasks[0].action -ne 'preserve_existing' -or $second.tasks[0].issue_number -ne 9 -or $null -ne $second.tasks[0].body){throw 'Existing maintainer text and closed issue must be preserved'}
    if($second.tasks[1].action -ne 'prepare_create'){throw 'An unrelated task must still be prepared'}
    $collision=Preview ($existing+@(@{number=10;state='open';title='[W1-CAT] Manual issue';body=''}))
    if($collision.tasks[0].action -ne 'conflict' -or $collision.tasks[0].issue_numbers.Count -ne 2){throw 'Conflicting matches must block a create proposal'}
    'Issue preview passed: batch selection, repeat deduplication, preservation, conflict.'
} finally {
    Remove-Item -LiteralPath $temp -ErrorAction SilentlyContinue
}
