$ErrorActionPreference='Stop'
$PSNativeCommandUseErrorActionPreference=$true
$paths=git -c core.quotepath=false ls-files
$forbidden=@($paths|Where-Object {
    $_ -match '(^|/)(\.lab|\.local|target|world|playerdata|screenshots)(/|$)' -or
    $_ -match '\.(jar|class|log|jfr|hprof|mca|zip|exe|dll)$'
})
if($forbidden.Count){throw ('Private runtime or binary files in the checkout index: '+($forbidden -join ', '))}
$images=@($paths|Where-Object {$_ -match '\.(png|jpg|jpeg|webp|svg)$'})
$allowedImages=@(
    'assets/foundry-header.jpg'
    'assets/badges/parity-lab.svg'
    'assets/badges/rust.svg'
    'assets/badges/java.svg'
    'assets/badges/minecraft-java.svg'
    'assets/badges/neoforge.svg'
    'assets/badges/mekanism-core.svg'
    'assets/badges/license.svg'
)
if($images|Where-Object {$_ -cnotin $allowedImages}){
    throw 'Unexpected image in the public tree. Review provenance before extending the allowlist.'
}
if($images -and (Get-Item -LiteralPath 'assets/foundry-header.jpg').Length -gt 1MB){
    throw 'Foundry social artwork exceeds 1 MiB'
}
Write-Host "Checked $($paths.Count) tracked paths for private runtime and package files."
