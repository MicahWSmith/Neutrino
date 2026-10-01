#!/usr/bin/env pwsh
# Packages Neutrino as a flat, relocatable Windows app folder.
$ErrorActionPreference = "Stop"

$rootDir = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $rootDir
try {
    cargo build -p neutrino-core --target wasm32-wasip2 --release
    cargo build -p neutrino-host --release

    $appDir = Join-Path $rootDir "target\Neutrino"
    $resourcesDir = Join-Path $appDir "Resources"

    if (Test-Path $appDir) {
        Remove-Item -Recurse -Force $appDir
    }
    New-Item -ItemType Directory -Path $resourcesDir -Force | Out-Null

    Copy-Item "target\release\neutrino.exe" (Join-Path $appDir "neutrino.exe")
    Copy-Item "target\wasm32-wasip2\release\neutrino_core.wasm" (Join-Path $resourcesDir "neutrino_core.wasm")
    Copy-Item -Recurse "ui" (Join-Path $resourcesDir "ui")

    Write-Host "Created $appDir"
    Write-Host "Run it with: $appDir\neutrino.exe"
}
finally {
    Pop-Location
}
