[CmdletBinding()]
param([Parameter(Mandatory)][string]$OutputDirectory)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'preview-zip.ps1')
$root = [IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Path $root | Out-Null

function New-FixtureZip([string]$Name, [string[]]$Entries, [string[]]$Contents) {
    $path = Join-Path $root "$Name.zip"
    $zip = [IO.Compression.ZipFile]::Open($path, [IO.Compression.ZipArchiveMode]::Create)
    try {
        for ($i = 0; $i -lt $Entries.Count; $i++) {
            $entry = $zip.CreateEntry($Entries[$i])
            $writer = [IO.StreamWriter]::new($entry.Open())
            try { $writer.Write($Contents[$i]) } finally { $writer.Dispose() }
        }
    } finally {
        $zip.Dispose()
    }
    $path
}

function Hash-Text([string]$Value) {
    [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData([Text.Encoding]::UTF8.GetBytes($Value)))
}

$files = @(
    @{ path='mek-lab.exe'; bytes=3; sha256=(Hash-Text 'lab') },
    @{ path='INSTALL.md'; bytes=5; sha256=(Hash-Text 'guide') }
)
$manifest = @{ files=$files } | ConvertTo-Json -Depth 5 -Compress
$names = @('mek-lab.exe', 'INSTALL.md', 'MANIFEST.json')
$contents = @('lab', 'guide', $manifest)
$valid = New-FixtureZip 'valid' $names $contents
$validHash = (Get-FileHash -LiteralPath $valid -Algorithm SHA256).Hash
$manifestHash = Hash-Text $manifest
Install-VerifiedPreviewZip $valid (Join-Path $root 'valid-install') $files $manifestHash $validHash

$forged = @{ files=@($files[0], $files[0]) } | ConvertTo-Json -Depth 5 -Compress
$controls = @(
    @{ Name='archive-digest'; Zip=$valid; Hash=('0' * 64); Error='Archive SHA-256 mismatch' },
    @{ Name='forged-inventory'; Zip=(New-FixtureZip 'forged-inventory' @('mek-lab.exe','unexpected.jar','MANIFEST.json') @('lab','junk',$forged)); Error='Unexpected or duplicate ZIP entry: unexpected.jar' },
    @{ Name='duplicate'; Zip=(New-FixtureZip 'duplicate' ($names + 'mek-lab.exe') ($contents + 'lab')); Error='Unexpected or duplicate ZIP entry: mek-lab.exe' },
    @{ Name='case-alias'; Zip=(New-FixtureZip 'case-alias' ($names + 'install.md') ($contents + 'guide')); Error='Unexpected or duplicate ZIP entry: install.md' },
    @{ Name='missing'; Zip=(New-FixtureZip 'missing' @('mek-lab.exe','MANIFEST.json') @('lab',$manifest)); Error='Missing ZIP entries: INSTALL.md' },
    @{ Name='manifest'; Zip=(New-FixtureZip 'manifest' $names @('lab','guide',$forged)); Error='Extracted manifest SHA-256 mismatch'; Extracts=$true },
    @{ Name='file'; Zip=(New-FixtureZip 'file' $names @('bad','guide',$manifest)); Error='Extracted size or SHA-256 mismatch: mek-lab.exe'; Extracts=$true }
)
foreach ($control in $controls) {
    $install = Join-Path $root "$($control.Name)-install"
    $hash = $control.Hash ?? (Get-FileHash -LiteralPath $control.Zip -Algorithm SHA256).Hash
    try {
        Install-VerifiedPreviewZip $control.Zip $install $files $manifestHash $hash
        throw "Unexpectedly accepted $($control.Name)"
    } catch {
        if ($_.Exception.Message -cne $control.Error) { throw }
    }
    if ((Test-Path -LiteralPath $install) -ne ($control.Extracts -eq $true)) {
        throw "Unexpected extraction state for $($control.Name)"
    }
    Write-Host "Rejected $($control.Name)"
}
Write-Host "Verified one valid ZIP and $($controls.Count) negative controls."
