param([string]$Environment='.lab/environment.json',[Parameter(Mandatory=$true)][string]$OutputRoot)
$ErrorActionPreference='Stop'
$root=(Get-Location).Path
$privateRoot=[IO.Path]::GetFullPath((Join-Path $root '.lab/evidence'))+[IO.Path]::DirectorySeparatorChar
$OutputRoot=[IO.Path]::GetFullPath($OutputRoot)
if(-not $OutputRoot.StartsWith($privateRoot,[StringComparison]::OrdinalIgnoreCase)){throw 'Campaign output must be a new directory under .lab/evidence'}
if(Test-Path -LiteralPath $OutputRoot){throw 'Campaign output already exists; old evidence cannot be reused'}
New-Item -ItemType Directory -Path $OutputRoot | Out-Null
$runner=Join-Path $root 'target/debug/mek-lab.exe'
$runs=[System.Collections.Generic.List[object]]::new()
function RunCase($name,$case) {
    $output=Join-Path $OutputRoot $name
    & $runner run-reference $Environment $case $output
    if($LASTEXITCODE -ne 0){throw "Reference run failed: $name"}
    $result=Get-Content -LiteralPath (Join-Path $output 'result.json') -Raw | ConvertFrom-Json
    $sealText=& $runner verify-reference $output
    if($LASTEXITCODE -ne 0){throw "Reference evidence verification failed: $name"}
    $seal=$sealText | ConvertFrom-Json
    $runs.Add(@{name=$name;case=$result.case;status=$result.status;trace_sha256=$seal.sha256})
}
foreach($n in 1..3){RunCase ('server-'+$n) (Join-Path $root 'cases/server-inventory.json')}
foreach($n in 2..3) {
    $comparison=& $runner compare (Join-Path $OutputRoot 'server-1/trace.json') (Join-Path $OutputRoot ('server-'+$n+'/trace.json'))
    if($LASTEXITCODE -ne 0){throw 'Independent Java A/A comparison failed'}
    $comparison | Set-Content -LiteralPath (Join-Path $OutputRoot ('aa-1-'+$n+'.json'))
}
RunCase 'client-1' (Join-Path $root 'cases/client-placement-menu.json')
@{status='functional_campaign_pass';runtime_qualified=$false;optimization_proven=$false;port_equivalence_proven=$false;runs=@($runs);scope='one server fixture and one real client keybinding/Screen scenario'} | ConvertTo-Json -Depth 7 | Set-Content -LiteralPath (Join-Path $OutputRoot 'campaign.json')
Get-Content -LiteralPath (Join-Path $OutputRoot 'campaign.json')
