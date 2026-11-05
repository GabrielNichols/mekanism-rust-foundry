param([Parameter(Mandatory=$true)][string]$Jdk,[ValidateRange(1,900)][int]$BootTimeoutSeconds=300,[string]$Runner='./target/debug/mek-lab.exe')
$ErrorActionPreference='Stop'
$root=(Get-Location).Path
$Runner=(Resolve-Path -LiteralPath $Runner).Path
$Jdk=(Resolve-Path -LiteralPath $Jdk).Path
if((Get-Content -LiteralPath (Join-Path $Jdk 'release') -Raw) -notmatch 'JAVA_VERSION="21\.') { throw 'A private JDK21 path is required' }
$cache=Join-Path $root '.lab/cache'
New-Item -ItemType Directory -Force -Path $cache | Out-Null
function FetchPinned($url,$path,$hash) {
    if(-not(Test-Path -LiteralPath $path)){Invoke-WebRequest -Uri $url -OutFile $path}
    if((Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant() -ne $hash){throw "Pinned artifact mismatch: $path"}
}
$installer=Join-Path $cache 'neoforge-21.1.200-installer.jar'
FetchPinned 'https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.200/neoforge-21.1.200-installer.jar' $installer '3c4c909f6f6340ad77939d75e85d0e46323ea4bf1a8acdd7c85bfc0862c3492a'
$metadata=Join-Path $cache 'mekanism-version.json'
if(-not(Test-Path -LiteralPath $metadata)) {
    $versions=Invoke-RestMethod -Uri 'https://api.modrinth.com/v2/project/mekanism/version?game_versions=%5B%221.21.1%22%5D'
    $version=@($versions | Where-Object {$_.version_number -eq '10.7.19.85'}) | Select-Object -First 1
    if(-not $version){throw 'Pinned Mekanism release unavailable'}
    $version | ConvertTo-Json -Depth 12 | Set-Content -LiteralPath $metadata
}
$version=Get-Content -LiteralPath $metadata -Raw | ConvertFrom-Json
$file=@($version.files | Where-Object {$_.primary})[0]
$mekanism=Join-Path $cache 'Mekanism-1.21.1-10.7.19.85.jar'
FetchPinned $file.url $mekanism '004dbc9f3106f4d192aeaa1ee1190dd16ec9ca8059ed3d093b80034f4c574f43'
$serverRoot=Join-Path $root '.lab/runtime/neoforge-21.1.200'
$clientRoot=Join-Path $root '.lab/runtime/client-21.1.200'
foreach($target in @(@{path=$serverRoot;flag='--installServer';ready='libraries/net/neoforged/neoforge/21.1.200/win_args.txt'},@{path=$clientRoot;flag='--installClient';ready='versions/neoforge-21.1.200/neoforge-21.1.200.json'})) {
    if(Test-Path -LiteralPath (Join-Path $target.path $target.ready)){continue}
    New-Item -ItemType Directory -Force -Path $target.path | Out-Null
    if($target.flag -eq '--installClient'){'{"profiles":{}}' | Set-Content -LiteralPath (Join-Path $target.path 'launcher_profiles.json')}
    & (Join-Path $Jdk 'bin/java.exe') -jar $installer $target.flag $target.path
    if($LASTEXITCODE -ne 0){throw 'NeoForge installation failed'}
}
& (Join-Path $root 'adapters/neoforge/prepare-client.ps1') -ClientRoot $clientRoot -Runner $Runner
& (Join-Path $root 'adapters/neoforge/build.ps1') -Jdk $Jdk
$artifacts=[System.Collections.Generic.List[object]]::new()
function AddArtifact($role,$path){$artifacts.Add(@{role=$role;path=$path;sha256=(Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()})}
AddArtifact 'java' (Join-Path $Jdk 'bin/java.exe')
AddArtifact 'jdk_release' (Join-Path $Jdk 'release')
AddArtifact 'jdk_modules' (Join-Path $Jdk 'lib/modules')
AddArtifact 'jvm' (Join-Path $Jdk 'bin/server/jvm.dll')
AddArtifact 'runner' $Runner
AddArtifact 'bridge' (Join-Path $cache 'meklab-bridge.jar')
AddArtifact 'mekanism' $mekanism
$n=0
foreach($path in @($serverRoot,$clientRoot)){
    foreach($item in (Get-ChildItem -LiteralPath $path -Recurse -File | Where-Object {$_.Extension -in @('.jar','.dll') -or $_.Name -in @('win_args.txt','launch.json') -or $_.FullName.Contains('\assets\')} | Sort-Object FullName)) {
        AddArtifact ('runtime_'+$n) $item.FullName;$n++
    }
}
@{schema=1;lock=@{schema=1;artifacts=@($artifacts)};server_root=$serverRoot;client_root=$clientRoot;boot_timeout_seconds=$BootTimeoutSeconds} | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath (Join-Path $root '.lab/environment.json')
New-Item -ItemType Directory -Force -Path (Join-Path $root '.lab/evidence') | Out-Null
@{prepared=$true;artifacts=$artifacts.Count;runtime_qualified=$false;audio_qualified=$false} | ConvertTo-Json -Compress
