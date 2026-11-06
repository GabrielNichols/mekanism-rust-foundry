[CmdletBinding()]
param(
    [Parameter(ValueFromPipeline)][string]$SnapshotJson,
    [string]$ObservedSnapshot
)
begin {
    $ErrorActionPreference = 'Stop'
    $lines = [Collections.Generic.List[string]]::new()
}
process {
    if ($SnapshotJson) { $lines.Add($SnapshotJson) }
}
end {
    if ($ObservedSnapshot -and $lines.Count) { throw 'Supply a snapshot file or pipeline JSON' }
    $raw = if ($ObservedSnapshot) { Get-Content -Raw -LiteralPath $ObservedSnapshot } else { $lines -join [Environment]::NewLine }
    $snapshot = $raw | ConvertFrom-Json -AsHashtable
    $review = Get-Content -Raw (Join-Path $PSScriptRoot '../docs/planning/triage-review.json') | ConvertFrom-Json -AsHashtable
    if ($snapshot.schema -ne 1 -or $review.schema -ne 1 -or $snapshot.repository -cne $review.repository -or
        $snapshot.project -ne $review.project -or $snapshot.issues -isnot [array] -or $snapshot.known_labels -isnot [array]) {
        throw 'Unexpected triage snapshot identity or shape'
    }

    $issueUrl = "https://github.com/$($review.repository)/issues"
    $mirror = 'status:ready-to-claim'
    $issues = @{}
    $changes = @()
    $notes = @()
    foreach ($issue in $snapshot.issues) {
        if ($issue.number -isnot [long] -or $issue.number -le 0 -or $issues.ContainsKey($issue.number) -or
            $issue.state -cnotin 'open', 'closed' -or $issue.assigned -isnot [bool] -or $issue.labels -isnot [array]) {
            throw 'Malformed or duplicate issue in triage snapshot'
        }
        $issues[$issue.number] = $issue
        $shouldMirror = $issue.state -ceq 'open' -and !$issue.assigned -and $issue.status -ceq 'Ready'
        if ($shouldMirror -ne ($mirror -cin $issue.labels)) {
            $notes += @{number = $issue.number; url = "$issueUrl/$($issue.number)"; reason = 'Review discovery mirror drift'}
        }
    }

    function Get-LabelKey($labels) {
        ConvertTo-Json -Compress -InputObject @($labels | Sort-Object -CaseSensitive)
    }

    foreach ($row in ($review.items | Sort-Object number)) {
        $item = $issues[$row.number]
        $expectedLabels = Get-LabelKey $row.expected_labels
        $desiredLabels = Get-LabelKey ($row.expected_labels + $row.add_labels)
        $observedLabels = Get-LabelKey $item.labels
        $reason = $null
        if (!$item -or $item.task_id -cne $row.task_id -or $item.state -cne 'open' -or $item.assigned -or
            $item.milestone -cne $row.milestone) {
            $reason = 'Issue identity, state, assignment or milestone differs from the review baseline'
        } elseif ($item.status -ceq $row.status -and $observedLabels -ceq $desiredLabels) {
            continue
        } elseif ($item.status -cne $row.expected_status -or $observedLabels -cne $expectedLabels) {
            $reason = 'Project status or labels differ from the review baseline'
        } elseif ($row.add_labels | Where-Object { $_ -cnotin $snapshot.known_labels }) {
            $reason = 'A proposed label is absent from the repository catalog'
        } elseif ($row.status -ceq 'Blocked' -and (!$row.blockers -or ($row.blockers | Where-Object {
            $_.issue -eq $row.number -or $issues[$_.issue].state -cne 'open' -or !$_.missing
        }))) {
            $reason = 'Recheck blocker links and missing evidence before proposing Blocked'
        }
        if ($reason) {
            $notes += @{number = $row.number; url = "$issueUrl/$($row.number)"; reason = $reason}
            continue
        }

        $changes += [ordered]@{
            number = $row.number
            url = "$issueUrl/$($row.number)"
            status = if ($item.status -cne $row.status) { $row.status }
            add_labels = @($row.add_labels | Where-Object { $_ -cnotin $item.labels })
            next_action = $row.next_action
            blockers = @(foreach ($blocker in $row.blockers) {
                [ordered]@{url = "$issueUrl/$($blocker.issue)"; missing = $blocker.missing}
            })
        }
    }

    [ordered]@{
        schema = 1
        repository = $review.repository
        project = $review.project
        changes = $changes
        review = @($notes | Sort-Object -Property { $_.number }, { $_.reason })
    } | ConvertTo-Json -Depth 8
}
