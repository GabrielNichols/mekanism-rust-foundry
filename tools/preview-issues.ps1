[CmdletBinding(DefaultParameterSetName='Observed')]
param(
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string[]]$Ids,
    [Parameter(Mandatory,ParameterSetName='Observed')][string]$ObservedIssues,
    [Parameter(Mandatory,ParameterSetName='Remote')]
    [ValidatePattern('^[A-Za-z0-9-]+/[A-Za-z0-9_.-]+$')][string]$Repository
)
$ErrorActionPreference='Stop'
$manifest=Get-Content -LiteralPath (Join-Path $PSScriptRoot '../docs/planning/issue-queue.json') -Raw | ConvertFrom-Json -AsHashtable
if($manifest.schema -ne 1){throw 'Unsupported issue queue schema'}
if($PSCmdlet.ParameterSetName -eq 'Remote'){
    $pages=gh api --paginate --slurp "repos/$Repository/issues?state=all&per_page=100"
    if($LASTEXITCODE -ne 0){throw 'GitHub issue read failed'}
    $observed=@(foreach($page in ($pages|ConvertFrom-Json -AsHashtable -NoEnumerate)){foreach($issue in $page){if(-not $issue.Contains('pull_request')){$issue}}})
} else {
    $observed=Get-Content -LiteralPath $ObservedIssues -Raw | ConvertFrom-Json -AsHashtable -NoEnumerate
    if($observed -isnot [array]){throw 'Observed issue snapshot must be a JSON array'}
}
$byId=@{}
foreach($task in $manifest.tasks){
    if($task.id -cnotmatch '^W1(?:-[A-Z]+)?$' -or $byId.ContainsKey($task.id)){throw 'Invalid or duplicate manifest task ID'}
    $byId[$task.id]=$task
}
$selected=[System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
$plan=@(foreach($id in $Ids){
    if(-not $byId.ContainsKey($id)){throw "Unknown issue ID $id"}
    if(-not $selected.Add($id)){continue}
    $task=$byId[$id]
    $marker='<!-- meklab-task: '+$task.id+' -->'
    $title='['+$task.id+'] '+$task.title
    $marked=@($observed|Where-Object { ([string]$_.body).Contains($marker,[StringComparison]::Ordinal) })
    $titled=@($observed|Where-Object { ([string]$_.title).StartsWith('['+$task.id+']',[StringComparison]::OrdinalIgnoreCase) })
    $matched=@(($marked+$titled)|Sort-Object -Property number -Unique)
    if($matched.Count -eq 1 -and $marked.Count -eq 1){
        [ordered]@{id=$task.id;action='preserve_existing';issue_number=$matched[0].number;state=$matched[0].state}
    } elseif($matched.Count){
        [ordered]@{id=$task.id;action='conflict';issue_numbers=@($matched.number)}
    } else {
        $sections=@("## Scope",$task.scope,"## Dependencies",$task.depends_on,
            "## Proposed owner",$task.owner,"## Acceptance",$task.acceptance,
            "## Negative control",$task.negative_control,"## Evidence",$task.evidence,"## First PR",$task.first_pr)
        [ordered]@{id=$task.id;action='prepare_create';title=$title;body=$marker+[Environment]::NewLine+[Environment]::NewLine+($sections -join ([Environment]::NewLine+[Environment]::NewLine))}
    }
})
[ordered]@{schema=1;mode='read_only_preview';repository=$Repository;tasks=$plan}|ConvertTo-Json -Depth 12
