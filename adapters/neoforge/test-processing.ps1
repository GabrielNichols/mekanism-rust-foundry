param([string]$Environment='.lab/environment.json',[string]$Case='cases/enrichment-lifecycle.json',[ValidateRange(3,10)][int]$Pairs=3,[Parameter(Mandatory=$true)][string]$OutputRoot)
$ErrorActionPreference='Stop'
$root=(Get-Location).Path
$privateRoot=[IO.Path]::GetFullPath((Join-Path $root '.lab/evidence'))+[IO.Path]::DirectorySeparatorChar
$OutputRoot=[IO.Path]::GetFullPath($OutputRoot)
if(-not $OutputRoot.StartsWith($privateRoot,[StringComparison]::OrdinalIgnoreCase)){throw 'Campaign output must be a new directory under .lab/evidence'}
if(Test-Path -LiteralPath $OutputRoot){throw 'Existing evidence cannot be reused'}
New-Item -ItemType Directory -Path $OutputRoot | Out-Null
$processor=Get-CimInstance Win32_Processor
$operatingSystem=Get-CimInstance Win32_OperatingSystem
@{sampled_utc=(Get-Date).ToUniversalTime().ToString('o');cpu=$processor.Name;logical_processors=$processor.NumberOfLogicalProcessors;os_build=$operatingSystem.BuildNumber;total_memory_kib=$operatingSystem.TotalVisibleMemorySize;free_memory_kib=$operatingSystem.FreePhysicalMemory;phase='functional_campaign';quiescent_host_proven=$false} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $OutputRoot 'host-profile.json') -Encoding utf8NoBOM
$runner=Join-Path $root 'target/debug/mek-lab.exe'
$workRunner=Join-Path $root 'target/debug/mek-work.exe'
$template=Get-Content -LiteralPath $Case -Raw | ConvertFrom-Json -Depth 80
if($template.client -or -not $template.require_persisted_machine -or $template.observer_mode -ne 'every_tick'){throw 'Requires server case with every_tick and independent persistence'}
$detailCase=Join-Path $OutputRoot 'detail-case.json'
$terminalCase=Join-Path $OutputRoot 'terminal-case.json'
$template | ConvertTo-Json -Depth 80 | Set-Content -LiteralPath $detailCase -Encoding utf8NoBOM
$template.observer_mode='terminal'
$template | ConvertTo-Json -Depth 80 | Set-Content -LiteralPath $terminalCase -Encoding utf8NoBOM
$runs=[System.Collections.Generic.List[object]]::new()
function RunCase($name,$casePath) {
    $output=Join-Path $OutputRoot $name
    & $runner run-reference $Environment $casePath $output
    if($LASTEXITCODE -ne 0){throw "Reference failed: $name"}
    $result=Get-Content -LiteralPath (Join-Path $output 'result.json') -Raw | ConvertFrom-Json
    $sealText=& $runner verify-reference $output
    if($LASTEXITCODE -ne 0){throw "Reference evidence verification failed: $name"}
    $seal=$sealText | ConvertFrom-Json
    $performancePath=Join-Path $output 'performance.json'
    $performance=if(Test-Path -LiteralPath $performancePath){Get-Content -LiteralPath $performancePath -Raw | ConvertFrom-Json}else{$null}
    $caseSpec=Get-Content -LiteralPath $casePath -Raw | ConvertFrom-Json -Depth 80
    $work=$null
    if($caseSpec.require_persisted_machine){
        $workText=& $workRunner $output
        if($LASTEXITCODE -ne 0){throw "Work accounting failed: $name"}
        $workText | Set-Content -LiteralPath (Join-Path $output 'work.json') -Encoding utf8NoBOM
        $work=$workText | ConvertFrom-Json
    }
    $runs.Add(@{name=$name;case=$result.case;status=$result.status;frames=$seal.frames;trace_sha256=$seal.sha256;performance=$performance;work=$work})
}
foreach($n in 1..$Pairs){
    # Alternate launch order; each execution still receives a new world and JVM.
    $order=if($n%2 -eq 1){@('detail','terminal')}else{@('terminal','detail')}
    foreach($mode in $order){$casePath=if($mode -eq 'detail'){$detailCase}else{$terminalCase};RunCase ($mode+'-'+$n) $casePath}
    $report=& $runner compare-observer (Join-Path $OutputRoot ('detail-'+$n)) (Join-Path $OutputRoot ('terminal-'+$n))
    if($LASTEXITCODE -ne 0){throw "Observer comparison failed: pair $n"}
    $report | Set-Content -LiteralPath (Join-Path $OutputRoot ('observer-'+$n+'.json')) -Encoding utf8NoBOM
    if($n -gt 1){foreach($mode in @('detail','terminal')){
        $report=& $runner compare (Join-Path $OutputRoot ($mode+'-1/trace.json')) (Join-Path $OutputRoot ($mode+'-'+$n+'/trace.json'))
        if($LASTEXITCODE -ne 0){throw "A/A mismatch: $mode $n"}
        $report | Set-Content -LiteralPath (Join-Path $OutputRoot ('aa-'+$mode+'-'+$n+'.json')) -Encoding utf8NoBOM
    }}
}
RunCase 'client-regression' (Join-Path $root 'cases/client-processing-menu.json')
@{status='functional_campaign_pass';pairs=$Pairs;runtime_qualified=$false;optimization_proven=$false;port_equivalence_proven=$false;runs=@($runs);scope='Original Java lifecycle, fresh A/A, every-tick versus terminal observer, independent saved NBT, real client placement/menu regression'} | ConvertTo-Json -Depth 15 | Set-Content -LiteralPath (Join-Path $OutputRoot 'campaign.json') -Encoding utf8NoBOM
Get-Content -LiteralPath (Join-Path $OutputRoot 'campaign.json')
