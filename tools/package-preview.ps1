[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [Parameter(Mandatory)][string]$ReferenceEnvironment,
    [Parameter(Mandatory)][string]$WorkEvidence,
    [switch]$LocalProof
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$output = [IO.Path]::GetFullPath($OutputDirectory)
$reference = (Resolve-Path -LiteralPath $ReferenceEnvironment).Path
$work = (Resolve-Path -LiteralPath $WorkEvidence).Path
if (-not $IsWindows -or [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -ne 'X64') {
    throw 'Use Windows x64 PowerShell 7 to build and verify this preview'
}
if (Test-Path -LiteralPath $output) { throw 'Release output directory must be new' }

$tag = 'v0.1.0-preview.1'
$target = 'x86_64-pc-windows-msvc'
$notes = 'docs/releases/v0.1.0-preview.1.md'
$policy = 'docs/releases/README.md'
$tracked = @('Cargo.toml','Cargo.lock','src','LICENSE','cases/server-inventory.json',
    'adapters/neoforge','tools/package-preview.ps1',$notes,$policy)
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

Push-Location -LiteralPath $root
try {
    $head = Run-Checked 'git' @('rev-parse','HEAD')
    $changes = @(Run-Checked 'git' (@('status','--porcelain','--untracked-files=all','--') + $tracked))
    if ($changes.Count -and -not $LocalProof) {
        throw 'Commit the release inputs first, or use -LocalProof for an unpublished test'
    }
    $compiler = Run-Checked 'rustc' @('--version')
    if ($compiler -notmatch '^rustc 1\.95\.0 ') { throw 'This release requires the verified Rust 1.95.0 toolchain' }
    $scratch = Join-Path $root ('.lab/release-work/' + [Guid]::NewGuid().ToString('N'))
    $build = Join-Path $root '.lab/release-target'
    $stage = Join-Path $scratch 'package'
    $installed = Join-Path $scratch 'installed'
    $privateChecks = Join-Path $scratch 'checks'
    New-Item -ItemType Directory -Force $stage,$installed,$privateChecks | Out-Null
    Run-Checked 'cargo' @('build','--locked','--release','--target',$target,'--target-dir',$build,
        '--bin','mek-lab','--bin','mek-work') | Out-Host
    $bin = Join-Path $build "$target/release"
    foreach ($file in 'mek-lab.exe','mek-work.exe') {
        Copy-Item -LiteralPath (Join-Path $bin $file) -Destination $stage
    }
    Copy-Item -LiteralPath (Join-Path $root 'LICENSE') -Destination $stage
    Copy-Item -LiteralPath (Join-Path $root 'cases/server-inventory.json') -Destination $stage
    Copy-Item -LiteralPath (Join-Path $root $notes) -Destination (Join-Path $stage 'RELEASE-NOTES.md')
    Copy-Item -LiteralPath (Join-Path $root $policy) -Destination (Join-Path $stage 'INSTALL.md')

    # Keep original attribution notices for dependencies compiled with the locked Rust build.
    $metadata = Run-Checked 'cargo' @('metadata','--locked','--format-version','1',
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
    Save-Json (Join-Path $stage 'MANIFEST.json') ([ordered]@{
        schema=1;version=$tag;target=$target;source_commit=$head;rustc=$compiler;
        release_inputs_committed=($changes.Count -eq 0);files=$files;dependencies=$dependencyRows
    })
    $name = "mekanism-rust-foundry-$tag-windows-x64.zip"
    $zip = Join-Path $scratch $name
    Zip-Release $stage $zip
    Expand-Archive -LiteralPath $zip -DestinationPath $installed
    $manifest = Read-Json (Join-Path $installed 'MANIFEST.json')
    $actualFiles = @(Get-ChildItem -LiteralPath $installed -Recurse -File)
    if ($actualFiles.Count -ne ($manifest.files.Count + 1)) { throw 'Archive contains unexpected files' }
    foreach ($file in $manifest.files) {
        $path = Join-Path $installed $file.path
        if ((File-Sha256 $path) -ne $file.sha256) { throw "Extracted hash mismatch: $($file.path)" }
    }

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
    } finally {
        Pop-Location
    }

    New-Item -ItemType Directory -Path $output | Out-Null
    Copy-Item -LiteralPath $zip -Destination $output
    $archiveHash = File-Sha256 (Join-Path $output $name)
    [IO.File]::WriteAllText((Join-Path $output 'SHA256SUMS'), ("$archiveHash  $name" + [Environment]::NewLine))
    Save-Json (Join-Path $output 'verification.json') ([ordered]@{
        schema=1;version=$tag;target=$target;source_commit=$head;rustc=$compiler;
        publication_candidate=(-not $LocalProof -and $changes.Count -eq 0);
        archive=$name;archive_sha256=$archiveHash;extracted_hashes_verified=$true;
        simulation_status=$simulationReport.status;simulation_cases=$simulationReport.cases.Count;
        reference_case=$referenceResult.case;reference_status=$referenceResult.status;
        reference_trace_seal=$seal;analysis_advanced_ticks=$analysis.advanced_ticks;
        analysis_binary_sha256=$analysisHash;qualified=$false
    })
    Write-Host "Verified preview in $output. Private logs remain in $privateChecks"
} finally {
    Pop-Location
}
