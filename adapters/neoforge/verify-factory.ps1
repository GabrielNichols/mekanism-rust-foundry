param(
    [Parameter(Mandatory)][string]$BlockedRun,
    [Parameter(Mandatory)][string]$OpenRun,
    [Parameter(Mandatory)][string]$ControlOutput,
    [string]$Runner = './target/debug/mek-lab.exe'
)

$ErrorActionPreference = 'Stop'
$Runner = (Resolve-Path -LiteralPath $Runner).Path

function ReadRun($directory, $caseId, $blocked) {
    $seal = & $Runner verify-reference $directory | ConvertFrom-Json
    if ($LASTEXITCODE -ne 0) { throw "Trace verification failed for $caseId" }
    $trace = Get-Content -LiteralPath (Join-Path $directory 'trace.json') -Raw | ConvertFrom-Json -AsHashtable -Depth 100
    if ($trace.scenario -ne $caseId) { throw "Unexpected factory scenario $($trace.scenario)" }
    $result = Get-Content -LiteralPath (Join-Path $directory 'result.json') -Raw | ConvertFrom-Json
    $config = Get-FileHash -LiteralPath (Join-Path $directory 'effective-config.json')
    $save = Get-FileHash -LiteralPath (Join-Path $directory 'persisted-machine-projected.json')
    $first = $null
    $completed = $null
    $activeLane = if ($blocked) { 'B' } else { 'A' }
    $startFe = $null
    $endFe = $null
    foreach ($frame in $trace.frames) {
        $factory = $frame.state.factory.entries
        $lanes = $factory.lanes.entries
        $a = $lanes.A.entries
        $b = $lanes.B.entries
        $c = $lanes.C.entries
        if ($factory.chunk_forced.value -ne $true -or $c.progress.value -ne '0' -or
            $c.input.entries.count.value -ne '0' -or $c.output.entries.count.value -ne '0') {
            throw "$caseId invalid chunk or inactive lane at tick $($frame.tick)"
        }
        if ($blocked -and ($a.progress.value -ne '0' -or $a.input.entries.count.value -ne '1' -or
            $a.output.entries.count.value -ne '64')) {
            throw "$caseId blocked lane changed at tick $($frame.tick)"
        }
        $fe = [long]$factory.energy_fe.value
        if ($null -eq $startFe -and $fe -gt 0) { $startFe = $fe }
        if ($null -ne $endFe -and $fe -gt $endFe) { throw "$caseId energy increased after setup" }
        if ($null -ne $startFe) { $endFe = $fe }
        $lane = $lanes[$activeLane].entries
        if ($null -eq $first -and [int]$lane.progress.value -gt 0 -and
            $lane.output.entries.count.value -eq '0' -and $lane.input.entries.count.value -eq '1') {
            $first = $frame.sequence
        }
        if ($null -eq $completed -and $lane.output.entries.count.value -eq '2' -and
            $lane.output.entries.id.value -eq 'mekanism:dust_iron' -and $lane.input.entries.count.value -eq '0') {
            $completed = $frame.sequence
        }
    }
    if ($null -eq $first -or $null -eq $completed -or $first -ge $completed -or
        $null -eq $startFe -or $endFe -le 0 -or $endFe -ge $startFe) {
        throw "$caseId did not witness powered progress and completion with spare energy"
    }
    return @{
        trace = $trace; first = $first; completed = $completed; seal = $seal
        start_fe = $startFe; end_fe = $endFe; debit_fe = $startFe - $endFe
        config_sha256 = $config.Hash.ToLowerInvariant(); persisted_sha256 = $save.Hash.ToLowerInvariant()
        artifact_fingerprint = $result.artifact_fingerprint
    }
}

$blocked = ReadRun $BlockedRun 'LAB-FACTORY-BLOCKED-001' $true
$open = ReadRun $OpenRun 'LAB-FACTORY-OPEN-001' $false
if ($blocked.start_fe -ne $open.start_fe -or $blocked.config_sha256 -ne $open.config_sha256 -or
    $blocked.artifact_fingerprint -ne $open.artifact_fingerprint) {
    throw 'The two factory runs do not share the same effective reference profile and starting FE'
}
New-Item -ItemType Directory -Path $ControlOutput -ErrorAction Stop | Out-Null
$original = Join-Path $BlockedRun 'trace.json'
$copy = Join-Path $ControlOutput 'copy.json'
$mutant = Join-Path $ControlOutput 'progress-swap.json'
Copy-Item -LiteralPath $original -Destination $copy
$clean = & $Runner compare $original $copy | ConvertFrom-Json
if ($LASTEXITCODE -ne 0 -or -not $clean.equal) { throw 'Unchanged trace failed comparison' }
$lanes = $blocked.trace.frames[$blocked.first].state.factory.entries.lanes.entries
$lanes.A.entries.progress.value, $lanes.B.entries.progress.value =
    $lanes.B.entries.progress.value, $lanes.A.entries.progress.value
$blocked.trace | ConvertTo-Json -Depth 100 -Compress | Set-Content -LiteralPath $mutant -Encoding utf8NoBOM
$negative = & $Runner compare $original $mutant 2> (Join-Path $ControlOutput 'comparison.stderr.log') | ConvertFrom-Json
$path = "frames[$($blocked.first)].state.factory.lanes.A.progress"
if ($LASTEXITCODE -ne 1 -or $negative.equal -or $negative.first_difference -ne $path) {
    throw "Unexpected progress-swap verdict: $($negative | ConvertTo-Json -Compress)"
}
foreach ($run in $blocked, $open) {
    $run.active_tick = $run.trace.frames[$run.first].tick
    $run.completed_tick = $run.trace.frames[$run.completed].tick
    $run.Remove('trace')
}
@{blocked = $blocked; open = $open; negative_control = $negative; qualified = $false} | ConvertTo-Json -Depth 12
