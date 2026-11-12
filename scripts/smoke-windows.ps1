# Only run on an ephemeral CI runner: never overwrite a personal database.
$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true') { throw 'Smoke test ini hanya untuk runner GitHub Actions.' }
$root = Split-Path $PSScriptRoot -Parent
$binary = Join-Path $root 'work/portable/Ardin Saham.exe'
$data = Join-Path $env:APPDATA 'id.ardin.saham'
$db = Join-Path $data 'ardin-saham.db'
if (Test-Path $db) { throw 'Database sudah ada; tes dibatalkan untuk melindungi data.' }
$adapter = Join-Path $root 'core/target/debug/examples/test_rpc.exe'
$rules = @()
$process = $null
function Invoke-Core($command, $arguments) {
    $request = @{cmd=$command; args=$arguments} | ConvertTo-Json -Depth 10 -Compress
    $response = $request | & $adapter | ConvertFrom-Json
    if ($LASTEXITCODE -ne 0 -or $response.error) { throw "Core gagal: $($response.error)" }
    return $response.ok
}
function Open-App {
    $script:process = Start-Process -FilePath $binary -WorkingDirectory (Split-Path $binary) -PassThru
    $deadline = (Get-Date).AddSeconds(45)
    do {
        Start-Sleep -Milliseconds 500
        $script:process.Refresh()
        if ($script:process.HasExited) { throw "Aplikasi keluar: $($script:process.ExitCode)" }
        if ($script:process.MainWindowHandle -ne 0 -and $script:process.MainWindowTitle -eq 'ARDIN SAHAM') { return }
    } while ((Get-Date) -lt $deadline)
    throw 'Jendela ARDIN SAHAM tidak muncul dalam 45 detik.'
}
function Close-App {
    if (!$script:process.CloseMainWindow()) { throw 'Tidak dapat menutup jendela aplikasi.' }
    if (!$script:process.WaitForExit(15000)) { throw 'Aplikasi tidak keluar setelah jendela ditutup.' }
    if ($script:process.ExitCode -ne 0) { throw "Aplikasi keluar dengan error: $($script:process.ExitCode)" }
    $script:process = $null
}
try {
    $webviews = @(Get-ChildItem "${env:ProgramFiles(x86)}\Microsoft\EdgeWebView\Application" -Filter msedgewebview2.exe -Recurse -ErrorAction SilentlyContinue)
    if (!$webviews.Count) { throw 'WebView2 Runtime tidak ditemukan pada runner.' }
    $programs = @($binary) + @($webviews.FullName)
    foreach ($program in $programs) {
        $name = 'ArdinSmoke-' + [guid]::NewGuid().ToString()
        New-NetFirewallRule -Name $name -DisplayName $name -Direction Outbound -Program $program -Action Block | Out-Null
        $rules += $name
    }
    # First launch must create the permanent database itself.
    Open-App
    Close-App
    if (!(Test-Path $db)) { throw 'Database lokal tidak dibuat oleh aplikasi native.' }
    $env:ARDIN_TEST_DB = $db
    Invoke-Core 'save_stock' @{code='TEST'; name='Smoke test'; sector='Pengujian'} | Out-Null
    Invoke-Core 'save_event' @{event=@{id=0; date='2026-01-01'; kind='OPENING'; stock='TEST'; shares=100; price='1000'; fee='0'; amount='0'; journal='Jurnal tetap ada setelah aplikasi ditutup'}} | Out-Null
    Open-App
    Close-App
    $state = Invoke-Core 'read_state' @{}
    if ($state.events.Count -ne 1 -or $state.events[0].journal -ne 'Jurnal tetap ada setelah aplikasi ditutup' -or $state.ledger.positions.TEST.shares -ne 100) {
        throw 'Data berubah setelah buka/tutup aplikasi native.'
    }
    Write-Host 'PASS: native window, outbound firewall block, local database creation and persistence across restart.'
} finally {
    if ($process -and !$process.HasExited) { Stop-Process -Id $process.Id -Force }
    foreach ($name in $rules) { Remove-NetFirewallRule -Name $name -ErrorAction SilentlyContinue }
    Remove-Item Env:ARDIN_TEST_DB -ErrorAction SilentlyContinue
}
