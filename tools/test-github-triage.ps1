$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$plan = Join-Path $PSScriptRoot 'preview-github-triage.ps1'
$review = Get-Content -Raw (Join-Path $root 'docs/planning/triage-review.json') | ConvertFrom-Json -AsHashtable
$snapshot = @{
    schema = 1
    repository = $review.repository
    project = $review.project
    known_labels = @('area:energy', 'area:chemicals', 'area:transport', 'area:multiblocks',
        'area:lifecycle', 'type:scenario', 'workflow:needs-triage', 'status:ready-to-claim')
    issues = @(
        foreach ($row in $review.items) {
            @{
                number = $row.number; task_id = $row.task_id; state = 'open'; assigned = $false
                labels = @($row.expected_labels); status = $row.expected_status; milestone = $row.milestone
            }
        }
        @{number = 1; task_id = 'W1-CAT'; state = 'open'; assigned = $false; status = 'Ready'; labels = @('status:ready-to-claim', 'manual')}
        @{number = 6; task_id = 'W1-ENG'; state = 'open'; assigned = $false; status = 'Ready'; labels = @('status:ready-to-claim')}
        @{number = 9; task_id = 'RF-SCOPE-01'; state = 'open'; assigned = $false; status = 'Todo'; labels = @('status:blocked')}
        @{number = 10; task_id = 'RF-SCOPE-02'; state = 'open'; assigned = $true; status = 'Ready'; labels = @('status:ready-to-claim', 'manual')}
        @{number = 15; task_id = 'RF-SCOPE-07'; state = 'open'; assigned = $false; status = 'Todo'; labels = @('status:blocked')}
        @{number = 145; task_id = 'RF-AUDIT-09'; state = 'open'; assigned = $false; status = 'Todo'; labels = @('status:blocked')}
    )
}
function Preview {
    $snapshot | ConvertTo-Json -Depth 10 | & $plan | ConvertFrom-Json -AsHashtable
}
function Assert($condition, $message) {
    if (-not $condition) { throw $message }
}

$result = Preview
Assert (($result.changes.number -join ',') -ceq '3,4,5,7,146') 'Expected the bounded first-wave and G9 proposals'
$heat = $result.changes[0]
Assert ($heat.status -ceq 'Ready' -and $heat.add_labels -ccontains 'status:ready-to-claim') 'Draft work should become discoverable'
Assert ($heat.blockers.url -contains 'https://github.com/GabrielNichols/mekanism-rust-foundry/issues/1') 'Draft must keep its runtime prerequisite link'
$future = $result.changes[-1]
Assert ($future.status -ceq 'Blocked' -and $future.blockers.url -contains 'https://github.com/GabrielNichols/mekanism-rust-foundry/issues/145') 'G9 needs a concrete blocker'
Assert ($result.changes[3].status -eq $null) 'Umbrella review must preserve Todo'
Assert ($result.review.number -contains 10) 'Flag discovery drift for review'
Assert ($result.review.number -notcontains 9) 'A historical label is not an operational change'

foreach ($change in $result.changes) {
    $item = $snapshot.issues | Where-Object number -eq $change.number
    $item.labels = @($item.labels + $change.add_labels)
    if ($change.status) { $item.status = $change.status }
}
$second = Preview
Assert ($second.changes.Count -eq 0) 'Reconciliation must converge after applying its exact proposals'
Assert (($second.review.number -join ',') -ceq '10') 'Converged proposals must not become conflicts'

$manual = $snapshot.issues | Where-Object number -eq 3
$manual.labels = @('type:tooling')
$manual.status = 'In Progress'
$manual.assigned = $true
$manual.milestone = 'M08'
$partial = $snapshot.issues | Where-Object number -eq 4
$partial.labels = @('area:transport')
$partial.status = 'Todo'
$closed = $snapshot.issues | Where-Object number -eq 5
$closed.state = 'closed'
$missing = $snapshot.issues | Where-Object number -eq 7
$missing.status = $null
$futureItem = $snapshot.issues | Where-Object number -eq 146
$futureItem.status = 'Todo'
($snapshot.issues | Where-Object number -eq 145).state = 'closed'
$drift = Preview
Assert ($drift.changes.Count -eq 0) 'Manual edits, partial application, a missing Project row, and stale dependencies must stop proposals'
Assert (($drift.review.number | Sort-Object -Unique) -join ',' -ceq '3,4,5,7,10,146') 'Explain drift without overriding current state'
Write-Host 'Verified bounded triage proposals, dependency links, manual drift guards, and idempotence.'
