#!/usr/bin/env pwsh
# Packages Neutrino as a flat, relocatable Windows app folder.
param(
    [Parameter(Mandatory = $true)]
    [string]$Name,

    [Parameter(Mandatory = $true)]
    [string]$UiDir
)
$ErrorActionPreference = "Stop"

$uiDirResolved = (Resolve-Path $UiDir).Path
$rootDir = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $rootDir
try {
    cargo build -p neutrino-core --target wasm32-wasip2 --release
    cargo build -p neutrino-host --release

    $appDir = Join-Path $rootDir "target\$Name"
    $resourcesDir = Join-Path $appDir "Resources"

    if (Test-Path $appDir) {
        Remove-Item -Recurse -Force $appDir
    }
    New-Item -ItemType Directory -Path $resourcesDir -Force | Out-Null

    Copy-Item "target\release\neutrino.exe" (Join-Path $appDir "$Name.exe")
    Copy-Item "target\wasm32-wasip2\release\neutrino_core.wasm" (Join-Path $resourcesDir "neutrino_core.wasm")
    Copy-Item -Recurse $uiDirResolved (Join-Path $resourcesDir "ui")

    Write-Host "Created $appDir"
    Write-Host "Run it with: $appDir\$Name.exe"
}
finally {
    Pop-Location
}
