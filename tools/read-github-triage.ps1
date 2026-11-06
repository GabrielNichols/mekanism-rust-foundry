# Read public issue metadata and the public Project without changing GitHub.
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$repo = 'GabrielNichols/mekanism-rust-foundry'
$repoId = 1447583916
$project = 'https://github.com/users/GabrielNichols/projects/2'
$projectId = 27891684

function Read-GitHubPages([string]$endpoint) {
    $raw = gh api --paginate --slurp $endpoint
    foreach ($page in ($raw | ConvertFrom-Json -AsHashtable -NoEnumerate)) {
        $page
    }
}

function Read-ProjectData([string]$uri, [string[]]$ids) {
    $html = (Invoke-WebRequest -Uri $uri -TimeoutSec 30).Content
    foreach ($id in $ids) {
        $pattern = '<script type="application/json" id="' + $id + '">(.*?)</script>'
        $match = [regex]::Match($html, $pattern, 'Singleline')
        if (!$match.Success) { throw "Public Project did not expose $id" }
        ConvertFrom-Json -InputObject $match.Groups[1].Value -AsHashtable -NoEnumerate
    }
}

$board, $identity = Read-ProjectData "$project/views/3" @('memex-paginated-items-data', 'memex-data')
if (!$identity.public -or $identity.id -ne $projectId -or $board.groups.pageInfo.hasNextPage -or
    $board.totalCount.isApproximate -or ($board.groups.nodes | Where-Object { $_.totalCount.isApproximate })) {
    throw 'Unexpected public Project or incomplete board counts'
}
$counts = [ordered]@{}
foreach ($group in $board.groups.nodes) { $counts[$group.groupValue] = $group.totalCount.value }
$statusByNumber = @{}
$countsFromRows = @{}
$seen = [Collections.Generic.HashSet[long]]::new()
$milestoneFilters = @('milestone:M00,M01,M02,M03,M04,M05,M06,M07',
    'milestone:M08,M09,M10,M11,M12,M13,M14', 'no:milestone')
foreach ($filter in $milestoneFilters) {
    $uri = "$project/views/1?filterQuery=" + [uri]::EscapeDataString($filter)
    $data, $columns = Read-ProjectData $uri @('memex-paginated-items-data', 'memex-columns-data')
    if ($data.groups.pageInfo.hasNextPage -or $data.totalCount.isApproximate -or
        ($data.groupedItems | Where-Object { $_.pageInfo.hasNextPage })) {
        throw "Incomplete Phase view for $filter"
    }
    $options = @{}
    foreach ($option in ($columns | Where-Object id -ceq 'Status').settings.options) {
        $options[$option.id] = $option.name
    }
    $rows = @($data.groupedItems | ForEach-Object { $_.nodes })
    if ($rows.Count -ne $data.totalCount.value) { throw "Incomplete item rows for $filter" }
    foreach ($row in $rows) {
        if ($row.contentRepositoryId -ne $repoId -or !$seen.Add($row.id) -or $row.contentType -cnotin 'Issue', 'PullRequest') {
            throw 'Unexpected or duplicate Project item'
        }
        $fields = $row.memexProjectColumnValues
        $optionId = ($fields | Where-Object memexProjectColumnId -ceq 'Status').value.id
        $status = if ($optionId) { $options[$optionId] } else { '_noValue' }
        if (!$status) { throw 'Unknown Project Status option' }
        $countsFromRows[$status]++
        if ($row.contentType -ceq 'Issue') {
            $number = [long]($fields | Where-Object memexProjectColumnId -ceq 'Title').value.number
            if ($number -le 0 -or $statusByNumber.ContainsKey($number)) { throw 'Missing or duplicate Project issue number' }
            $statusByNumber[$number] = @{status = $status; state = $row.state}
        }
    }
}
if ($seen.Count -ne $board.totalCount.value -or
    ($counts.Keys | Where-Object { $counts[$_] -ne [int]$countsFromRows[$_] }) -or
    ($countsFromRows.Keys | Where-Object { !$counts.Contains($_) })) {
    throw 'Project partitions disagree with the board. Refresh the complete snapshot.'
}

$issues = @(foreach ($issue in (Read-GitHubPages "repos/$repo/issues?state=all&per_page=100")) {
    if ($issue.pull_request) { continue }
    $projectIssue = $statusByNumber[$issue.number]
    if ($projectIssue -and $projectIssue.state -cne $issue.state) { throw "Issue #$($issue.number) changed during the read" }
    if ($projectIssue) { $statusByNumber.Remove($issue.number) }
    $markers = [regex]::Matches([string]$issue.body, '(?m)^<!-- meklab-task: ([A-Z0-9-]+) -->\s*$')
    [ordered]@{
        number = $issue.number
        task_id = if ($markers.Count -eq 1) { $markers[0].Groups[1].Value }
        state = $issue.state
        assigned = [bool]$issue.assignees.Count
        labels = @($issue.labels | ForEach-Object name | Sort-Object)
        status = $projectIssue.status
        milestone = $issue.milestone.title
    }
})
if ($statusByNumber.Count) { throw 'A Project issue is missing from the repository read' }

[ordered]@{
    schema = 1
    repository = $repo
    project = 2
    captured_at_utc = [datetime]::UtcNow.ToString('o')
    project_counts = $counts
    known_labels = @(Read-GitHubPages "repos/$repo/labels?per_page=100" | ForEach-Object name | Sort-Object)
    issues = @($issues | Sort-Object { $_.number })
} | ConvertTo-Json -Depth 8
