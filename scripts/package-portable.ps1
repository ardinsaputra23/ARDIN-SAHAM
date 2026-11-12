$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$binary = Join-Path $root 'src-tauri/target/release/Ardin Saham.exe'
if (!(Test-Path $binary)) { throw "Binary belum tersedia: $binary" }
$destination = Join-Path $root 'work/portable'
New-Item -ItemType Directory -Force $destination | Out-Null
Copy-Item $binary (Join-Path $destination 'Ardin Saham.exe') -Force
Copy-Item (Join-Path $root 'docs/PORTABLE.md') (Join-Path $destination 'BACA-DULU.txt') -Force
$version = (Get-Content (Join-Path $root 'package.json') -Raw | ConvertFrom-Json).version
@{
    product = 'ARDIN SAHAM'
    version = $version
    platform = 'Windows x64'
    commit = (git -C $root rev-parse HEAD)
    sourceCommit = $env:BUILD_SOURCE_SHA
    runtime = 'Microsoft WebView2 Runtime harus tersedia di Windows'
    dataDirectory = '%APPDATA%\id.ardin.saham'
} | ConvertTo-Json | Set-Content (Join-Path $destination 'BUILD-INFO.json') -Encoding utf8
Get-ChildItem $destination -File | Where-Object Name -ne 'SHA256SUMS.txt' | Sort-Object Name | ForEach-Object {
    "{0}  {1}" -f (Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant(), $_.Name
} | Set-Content (Join-Path $destination 'SHA256SUMS.txt') -Encoding ascii
Write-Host "Paket portabel: $destination"
