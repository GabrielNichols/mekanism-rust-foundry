[CmdletBinding(DefaultParameterSetName='Release')]
param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [Parameter(Mandatory, ParameterSetName='Release')][string]$ReferenceEnvironment,
    [Parameter(Mandatory, ParameterSetName='Release')][string]$WorkEvidence,
    [Parameter(ParameterSetName='Release')][switch]$LocalProof,
    [Parameter(Mandatory, ParameterSetName='DryRun')][switch]$DryRun,
    [Parameter(Mandatory, ParameterSetName='DryRun')]
    [ValidatePattern('^[a-fA-F0-9]{40}\z')][string]$ExpectedCommit
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'preview-zip.ps1')
$root = Split-Path -Parent $PSScriptRoot
$output = [IO.Path]::GetFullPath($OutputDirectory)
$isDryRun = $PSCmdlet.ParameterSetName -eq 'DryRun'
if (-not $isDryRun) {
    $reference = (Resolve-Path -LiteralPath $ReferenceEnvironment).Path
    $work = (Resolve-Path -LiteralPath $WorkEvidence).Path
}
if (-not $IsWindows -or [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -ne 'X64') {
    throw 'Use Windows x64 PowerShell 7 to build and verify this preview'
}
if (Test-Path -LiteralPath $output) { throw 'Release output directory must be new' }

$version = if ($isDryRun) { 'dry-run' } else { 'v0.1.0-preview.1' }
$cargoLock = if ($isDryRun) { '--frozen' } else { '--locked' }
$target = 'x86_64-pc-windows-msvc'
$notes = 'docs/releases/v0.1.0-preview.1.md'
$policy = 'docs/releases/README.md'
$tracked = @('Cargo.toml','Cargo.lock','src','LICENSE','cases/server-inventory.json',
    'adapters/neoforge','tools/package-preview.ps1','tools/preview-zip.ps1',$notes,$policy)
function Read-Json([string]$Path) {
    Get-Content -LiteralPath $Path -Raw | ConvertFrom-Json -AsHashtable
}
function File-Sha256([string]$Path) {
    (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}
function Save-Json([string]$Path, $Value) {
    [IO.File]::WriteAllText($Path, (($Value | ConvertTo-Json -Depth 30) + [Environment]::NewLine))
}
function Run-Checked([string]$Program, [string[]]$Arguments) {
    & $Program @Arguments
    if ($LASTEXITCODE -ne 0) { throw "$Program failed with exit code $LASTEXITCODE" }
}
function Zip-Release([string]$Source, [string]$ZipPath) {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [IO.Compression.ZipFile]::Open($ZipPath, [IO.Compression.ZipArchiveMode]::Create)
    try {
        $names = [string[]]@(Get-ChildItem -LiteralPath $Source -Recurse -File | ForEach-Object {
            [IO.Path]::GetRelativePath($Source, $_.FullName).Replace('\','/')
        })
        [Array]::Sort($names, [StringComparer]::Ordinal)
        foreach ($name in $names) {
            $entry = $zip.CreateEntry($name, [IO.Compression.CompressionLevel]::Optimal)
            $entry.LastWriteTime = [DateTimeOffset]::new(1980, 1, 1, 0, 0, 0, [TimeSpan]::Zero)
            $from = [IO.File]::OpenRead((Join-Path $Source $name))
            try {
                $to = $entry.Open()
                try { $from.CopyTo($to) } finally { $to.Dispose() }
            } finally {
                $from.Dispose()
            }
        }
    } finally {
        $zip.Dispose()
    }
}

function Assert-DryRunSource {
    if ((Run-Checked 'git' @('rev-parse','HEAD')) -cne $ExpectedCommit.ToLowerInvariant()) {
        throw 'Dry run HEAD does not match ExpectedCommit'
    }
    if (@(Run-Checked 'git' @('status','--porcelain=v1','--untracked-files=all')).Count) {
        throw 'Dry run requires a clean checkout of ExpectedCommit'
    }
}

Push-Location -LiteralPath $root
try {
    $head = Run-Checked 'git' @('rev-parse','HEAD')
    $changes = @(Run-Checked 'git' (@('status','--porcelain','--untracked-files=all','--') + $tracked))
    if ($isDryRun) {
        Assert-DryRunSource
        $head = $ExpectedCommit.ToLowerInvariant()
    } elseif ($changes.Count -and -not $LocalProof) {
        throw 'Commit the release inputs first, or use -LocalProof for an unpublished test'
    }
    $compiler = Run-Checked 'rustc' @('--version')
    if ($compiler -notmatch '^rustc 1\.95\.0 ') { throw 'This release requires the verified Rust 1.95.0 toolchain' }
    $scratch = Join-Path $root ('.lab/release-work/' + [Guid]::NewGuid().ToString('N'))
    $buildDirectory = if ($isDryRun) { '.lab/dry-run-target' } else { '.lab/release-target' }
    $build = Join-Path $root $buildDirectory
    $stage = Join-Path $scratch 'package'
    $installed = Join-Path $scratch 'installed'
    $privateChecks = Join-Path $scratch 'checks'
    New-Item -ItemType Directory -Path $stage,$privateChecks -Force | Out-Null
    Run-Checked 'cargo' @('build',$cargoLock,'--release','--target',$target,'--target-dir',$build,
        '--bin','mek-lab','--bin','mek-work') | Out-Host
    $bin = Join-Path $build "$target/release"
    $core = [ordered]@{
        'mek-lab.exe' = Join-Path $bin 'mek-lab.exe'
        'mek-work.exe' = Join-Path $bin 'mek-work.exe'
        'LICENSE' = Join-Path $root 'LICENSE'
        'server-inventory.json' = Join-Path $root 'cases/server-inventory.json'
        'RELEASE-NOTES.md' = Join-Path $root $notes
        'INSTALL.md' = Join-Path $root $policy
    }
    foreach ($file in $core.GetEnumerator()) {
        Copy-Item -LiteralPath $file.Value -Destination (Join-Path $stage $file.Key)
    }

    # Keep original attribution notices for dependencies compiled with the locked Rust build.
    $metadata = Run-Checked 'cargo' @('metadata',$cargoLock,'--format-version','1',
        '--filter-platform',$target) | ConvertFrom-Json -AsHashtable
    $packages = @($metadata.packages | Where-Object { $_.id -ne $metadata.resolve.root } | Sort-Object name,version)
    $dependencyRows = @(foreach ($package in $packages) {
        if ($package.source -notlike 'registry+*' -or -not $package.license) {
            throw "Review dependency provenance and notices for $($package.name)"
        }
        $id = "$($package.name)-$($package.version)"
        $crateRoot = Split-Path -Parent $package.manifest_path
        $notices = @(Get-ChildItem -LiteralPath $crateRoot -File |
            Where-Object Name -Match '^(LICENSE|LICENCE|COPYING|NOTICE)([._-]|$)' |
            Sort-Object Name)
        if (-not $notices.Count) { throw "Missing shipped license notices for $id" }
        $to = Join-Path $stage "licenses/$id"
        New-Item -ItemType Directory -Force $to | Out-Null
        foreach ($notice in $notices) {
            Copy-Item -LiteralPath $notice.FullName -Destination $to
        }
        [ordered]@{name=$package.name;version=$package.version;license=$package.license;
            notices=@($notices | ForEach-Object { "licenses/$id/$($_.Name)" })}
    })
    $files = @(Get-ChildItem -LiteralPath $stage -Recurse -File | Sort-Object FullName |
        ForEach-Object {
            [ordered]@{path=[IO.Path]::GetRelativePath($stage, $_.FullName).Replace('\','/');
                sha256=(File-Sha256 $_.FullName);bytes=$_.Length}
        })
    $allowlist = @($core.Keys) + @($dependencyRows | ForEach-Object { $_.notices })
    if ($files.Count -ne $allowlist.Count -or (Compare-Object $allowlist $files.path -CaseSensitive)) {
        throw 'Staging files differ from the package allowlist'
    }
    $manifestPath = Join-Path $stage 'MANIFEST.json'
    Save-Json $manifestPath ([ordered]@{
        schema=1;version=$version;target=$target;source_commit=$head;rustc=$compiler;
        release_inputs_committed=($changes.Count -eq 0);files=$files;dependencies=$dependencyRows
    })
    $manifestHash = File-Sha256 $manifestPath
    $label = if ($isDryRun) { "$version-$head" } else { $version }
    $name = "mekanism-rust-foundry-$label-windows-x64.zip"
    $zip = Join-Path $scratch $name
    Zip-Release $stage $zip
    $archiveHash = File-Sha256 $zip
    Install-VerifiedPreviewZip $zip $installed $files $manifestHash $archiveHash

    Push-Location -LiteralPath $installed
    try {
        $lab = Join-Path $installed 'mek-lab.exe'
        $analyzer = Join-Path $installed 'mek-work.exe'
        $simulation = Join-Path $privateChecks 'simulation'
        Run-Checked $lab @('simulate-observer',$simulation) | Out-Host
        $simulationReport = Read-Json (Join-Path $simulation 'result.json')
        if ($simulationReport.status -ne 'pass' -or $simulationReport.qualified -ne $false) {
            throw 'Unexpected simulated observer verdict'
        }

        if ($isDryRun) {
            & {
                $PSNativeCommandUseErrorActionPreference = $false
                $message = (& $analyzer 2>&1 | Out-String).Trim()
                if ($LASTEXITCODE -ne 1 -or $message -cne 'usage: mek-work RUN_DIRECTORY') {
                    throw 'Unexpected installed analyzer usage result'
                }
            }
        } else {
            $environment = Read-Json $reference
            $runner = @($environment.lock.artifacts | Where-Object role -eq 'runner')
            if ($runner.Count -ne 1) { throw 'Expected exactly one locked reference runner' }
            # Keep all runtime pins; explicitly bind the extracted executable for this attempt.
            $runner[0].path = $lab
            $runner[0].sha256 = File-Sha256 $lab
            $bound = Join-Path $privateChecks 'environment.json'
            Save-Json $bound $environment
            $run = Join-Path $privateChecks 'reference'
            Run-Checked $lab @('run-reference',$bound,(Join-Path $installed 'server-inventory.json'),$run) | Out-Host
            $seal = Run-Checked $lab @('verify-reference',$run) | ConvertFrom-Json -AsHashtable
            $referenceResult = Read-Json (Join-Path $run 'result.json')
            if ($referenceResult.status -ne 'functional_pass' -or $referenceResult.qualified -ne $false -or $seal.frames -lt 1) {
                throw 'Unexpected reference verdict'
            }
            $analysis = Run-Checked $analyzer @($work) | ConvertFrom-Json -AsHashtable
            $analysisHash = File-Sha256 $analyzer
            if ($analysis.analysis_binary_sha256 -ne $analysisHash -or $analysis.advanced_ticks -lt 1) {
                throw 'Extracted analyzer did not process the supplied evidence'
            }
            Save-Json (Join-Path $privateChecks 'work.json') $analysis
        }
    } finally {
        Pop-Location
    }

    if ($isDryRun) { Assert-DryRunSource }
    New-Item -ItemType Directory -Path $output | Out-Null
    Copy-Item -LiteralPath $zip -Destination $output
    $checksumPath = Join-Path $output 'SHA256SUMS'
    [IO.File]::WriteAllText($checksumPath, ("$archiveHash  $name" + [Environment]::NewLine))
    $copiedHash = File-Sha256 (Join-Path $output $name)
    if ($copiedHash -cne $archiveHash -or
        [IO.File]::ReadAllText($checksumPath) -cne ("$copiedHash  $name" + [Environment]::NewLine)) {
        throw 'Output archive or SHA256SUMS mismatch'
    }
    $report = [ordered]@{
        schema=1;version=$version;target=$target;source_commit=$head;rustc=$compiler;
        publication_candidate=(-not $isDryRun -and -not $LocalProof -and $changes.Count -eq 0);
        archive=$name;archive_sha256=$archiveHash;extracted_hashes_verified=$true;
        simulation_status=$simulationReport.status;simulation_cases=$simulationReport.cases.Count
    }
    if ($isDryRun) {
        $report.dry_run = $true
        $report.manifest_sha256 = $manifestHash
        $report.archive_entries = $files.Count + 1
        $report.analyzer_usage_exit = 1
    } else {
        $report.reference_case = $referenceResult.case
        $report.reference_status = $referenceResult.status
        $report.reference_trace_seal = $seal
        $report.analysis_advanced_ticks = $analysis.advanced_ticks
        $report.analysis_binary_sha256 = $analysisHash
    }
    $report.qualified = $false
    Save-Json (Join-Path $output 'verification.json') $report
    Write-Host "Verified preview in $output. Private logs remain in $privateChecks"
} finally {
    Pop-Location
}
