param([string]$RuntimeRoot='.lab/runtime/client-21.1.200',[Parameter(Mandatory=$true)][string]$Jdk)
$ErrorActionPreference='Stop'
$root=(Get-Location).Path
$lib=Join-Path (Resolve-Path -LiteralPath $RuntimeRoot).Path 'libraries'
$priority=@((Join-Path $lib 'net/neoforged/neoforge/21.1.200/neoforge-21.1.200-client.jar'),(Join-Path $lib 'net/neoforged/neoforge/21.1.200/neoforge-21.1.200-universal.jar'))
$rest=@(Get-ChildItem -LiteralPath $lib -Filter '*.jar' -Recurse -File | Where-Object {$_.FullName -notin $priority -and $_.Name -notmatch '(slim|extra|unpacked)'} | Select-Object -ExpandProperty FullName)
$classes=Join-Path $root '.lab/bridge-classes'
New-Item -ItemType Directory -Force -Path $classes | Out-Null
$sources=@(Get-ChildItem -LiteralPath 'adapters/neoforge/src' -Filter '*.java' -Recurse -File | Select-Object -ExpandProperty FullName)
& (Join-Path $Jdk 'bin/javac.exe') --release 21 -proc:none -cp (($priority+$rest) -join ';') -d $classes @sources
if($LASTEXITCODE -ne 0){throw 'Bridge javac failed'}
Copy-Item -LiteralPath 'adapters/neoforge/resources/META-INF' -Destination $classes -Recurse -Force
$jar=Join-Path $root '.lab/cache/meklab-bridge.jar'
if(Test-Path -LiteralPath $jar){Remove-Item -LiteralPath $jar}
Add-Type -AssemblyName System.IO.Compression.FileSystem
[IO.Compression.ZipFile]::CreateFromDirectory($classes,$jar)
@{bridge_sha256=(Get-FileHash -LiteralPath $jar -Algorithm SHA256).Hash.ToLowerInvariant()} | ConvertTo-Json -Compress
