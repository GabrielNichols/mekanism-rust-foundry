param([Parameter(Mandatory=$true)][string]$ClientRoot)
$ErrorActionPreference = 'Stop'
$ClientRoot = (Resolve-Path -LiteralPath $ClientRoot).Path
$neo = Get-Content -LiteralPath (Join-Path $ClientRoot 'versions/neoforge-21.1.200/neoforge-21.1.200.json') -Raw | ConvertFrom-Json
$base = Get-Content -LiteralPath (Join-Path $ClientRoot 'versions/1.21.1/1.21.1.json') -Raw | ConvertFrom-Json
$allNames=@($neo.libraries.name)+@($base.libraries.name)
$allNames | ConvertTo-Json | Set-Content -LiteralPath '.lab/cache/library-names.json'
$selectedNames=& '.\target\debug\mek-lab.exe' select-windows-x64 '.lab/cache/library-names.json' | ConvertFrom-Json
if($LASTEXITCODE -ne 0){throw 'Rust platform selection failed'}
function Allowed($entry) {
    if (-not $entry.rules) { return $true }
    $result = $false
    foreach($rule in $entry.rules) {
        $match = (-not $rule.os -or (-not $rule.os.name -or $rule.os.name -eq 'windows')) -and (-not $rule.features)
        if($match) { $result = $rule.action -eq 'allow' }
    }
    return $result
}
function FetchVerified($url,$destination,$sha1) {
    if(-not(Test-Path -LiteralPath $destination)) {
        New-Item -ItemType Directory -Force -Path (Split-Path $destination) | Out-Null
        Invoke-WebRequest -Uri $url -OutFile $destination
    }
    if((Get-FileHash -LiteralPath $destination -Algorithm SHA1).Hash.ToLowerInvariant() -ne $sha1) { throw "Hash mismatch: $destination" }
}
$jars = [System.Collections.Generic.List[string]]::new()
$seen = @{}
foreach($manifest in @($neo,$base)) {
    foreach($library in $manifest.libraries) {
        if(-not(Allowed $library) -or -not $library.downloads.artifact -or $library.name -notin $selectedNames) { continue }
        $artifact=$library.downloads.artifact
        $path=Join-Path $ClientRoot ('libraries/'+$artifact.path)
        FetchVerified $artifact.url $path $artifact.sha1
        if(-not $seen.ContainsKey($path)) { $jars.Add($path); $seen[$path]=$true }
    }
}
$assets=Join-Path $ClientRoot 'assets'
$indexPath=Join-Path $assets ('indexes/'+$base.assetIndex.id+'.json')
FetchVerified $base.assetIndex.url $indexPath $base.assetIndex.sha1
$index=Get-Content -LiteralPath $indexPath -Raw | ConvertFrom-Json
$selected=@($index.objects.PSObject.Properties | Where-Object { $_.Name -match '(^icons/|/font/|/lang/en_us\.json$)' })
foreach($object in $selected) {
    $hash=$object.Value.hash
    FetchVerified ('https://resources.download.minecraft.net/'+$hash.Substring(0,2)+'/'+$hash) (Join-Path $assets ('objects/'+$hash.Substring(0,2)+'/'+$hash)) $hash
}
$natives=Join-Path $ClientRoot 'natives'
New-Item -ItemType Directory -Force -Path $natives | Out-Null
Add-Type -AssemblyName System.IO.Compression.FileSystem
foreach($path in $jars) {
    if($path -notmatch 'natives-windows') { continue }
    $zip=[IO.Compression.ZipFile]::OpenRead($path)
    try { foreach($entry in $zip.Entries) { if($entry.Name.EndsWith('.dll')) { [IO.Compression.ZipFileExtensions]::ExtractToFile($entry,(Join-Path $natives $entry.Name),$true) } } } finally { $zip.Dispose() }
}
$values=@{'library_directory'=(Join-Path $ClientRoot 'libraries');'classpath_separator'=';';'version_name'='neoforge-21.1.200';'natives_directory'=$natives;'launcher_name'='mek-lab';'launcher_version'='0.1'}
function Expand($value) { foreach($key in $values.Keys) { $value=$value.Replace(('${'+$key+'}'),$values[$key]) }; return $value }
$argsOut=[System.Collections.Generic.List[string]]::new()
foreach($argument in $base.arguments.jvm) {
    if($argument -is [string]) {
        if($argument -eq '${classpath}') { $argsOut.Add(($jars -join ';')) } else { $argsOut.Add((Expand $argument)) }
    } elseif(Allowed $argument) { foreach($v in @($argument.value)) { $argsOut.Add((Expand $v)) } }
}
foreach($v in $neo.arguments.jvm) { $argsOut.Add((Expand $v)) }
$argsOut.Add($neo.mainClass)
foreach($v in @('--username','MekLabP1','--uuid','00000000000000000000000000000001','--accessToken','0','--version','neoforge-21.1.200','--gameDir','{game_dir}','--assetsDir',$assets,'--assetIndex',$base.assetIndex.id,'--userType','msa','--versionType','release')) { $argsOut.Add($v) }
foreach($v in $neo.arguments.game) { $argsOut.Add($v) }
foreach($v in @('--width','854','--height','480','--quickPlayMultiplayer','{server_address}')) { $argsOut.Add($v) }
@{args=@($argsOut);assets_profile='en_us_fonts_icons_silent';downloaded_asset_objects=$selected.Count;libraries=@($jars)} | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $ClientRoot 'launch.json')
@{prepared=$true;libraries=$jars.Count;asset_objects=$selected.Count;audio_qualified=$false} | ConvertTo-Json -Compress
