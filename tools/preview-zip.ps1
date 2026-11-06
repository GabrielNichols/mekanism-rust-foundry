function Install-VerifiedPreviewZip {
    param(
        [Parameter(Mandatory)][string]$ZipPath,
        [Parameter(Mandatory)][string]$InstallPath,
        [Parameter(Mandatory)][System.Collections.IDictionary[]]$Files,
        [Parameter(Mandatory)][string]$ManifestSha256,
        [Parameter(Mandatory)][string]$ArchiveSha256
    )

    if (Test-Path -LiteralPath $InstallPath) { throw 'Install directory must be new' }
    if ((Get-FileHash -LiteralPath $ZipPath -Algorithm SHA256).Hash -ine $ArchiveSha256) {
        throw 'Archive SHA-256 mismatch'
    }

    # The caller provides the staging inventory, independent of the archive's own manifest.
    $expected = [Collections.Generic.Dictionary[string,string]]::new([StringComparer]::OrdinalIgnoreCase)
    $expected.Add('MANIFEST.json', 'MANIFEST.json')
    foreach ($file in $Files) {
        $expected.Add($file.path, $file.path)
    }
    $archive = [IO.Compression.ZipFile]::OpenRead($ZipPath)
    try {
        foreach ($entry in $archive.Entries) {
            $name = $entry.FullName
            if (-not $expected.ContainsKey($name) -or $name -cne $expected[$name]) {
                throw "Unexpected or duplicate ZIP entry: $name"
            }
            $unixType = ($entry.ExternalAttributes -shr 16) -band 0xF000
            if (($entry.ExternalAttributes -band 0x410) -or $unixType -notin 0,0x8000) {
                throw "ZIP entry must be a regular file: $name"
            }
            [void]$expected.Remove($name)
        }
        if ($expected.Count) { throw "Missing ZIP entries: $($expected.Keys -join ', ')" }
    } finally {
        $archive.Dispose()
    }

    Expand-Archive -LiteralPath $ZipPath -DestinationPath $InstallPath
    $manifest = Join-Path $InstallPath 'MANIFEST.json'
    if ((Get-FileHash -LiteralPath $manifest -Algorithm SHA256).Hash -ine $ManifestSha256) {
        throw 'Extracted manifest SHA-256 mismatch'
    }
    foreach ($file in $Files) {
        $path = Join-Path $InstallPath $file.path
        if ((Get-Item -LiteralPath $path).Length -ne $file.bytes -or
            (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash -ine $file.sha256) {
            throw "Extracted size or SHA-256 mismatch: $($file.path)"
        }
    }
}
