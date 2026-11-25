$ErrorActionPreference = "Stop"

$targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "$env:USERPROFILE\.cargo\target-prism" }
$env:CARGO_TARGET_DIR = $targetRoot
$toolsDir = "$targetRoot\tools"
$env:PATH = "$env:USERPROFILE\.cargo\bin;$toolsDir;$env:PATH"

New-Item -ItemType Directory -Force -Path $toolsDir | Out-Null
$shimSrc = "$PSScriptRoot\src-tauri\tools\dlltool_shim.rs"
$shimExe = "$toolsDir\dlltool.exe"
$needShimBuild = (-not (Test-Path $shimExe)) -or ((Get-Item $shimSrc).LastWriteTimeUtc -gt (Get-Item $shimExe).LastWriteTimeUtc)
if ($needShimBuild) {
    Write-Host "Building pure-Rust GNU dlltool shim..."
    rustc $shimSrc -O -o $shimExe
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

Write-Host "Building frontend production assets..."
Push-Location $PSScriptRoot
npm run build
if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }

Write-Host "Building Tauri Windows application (release, standalone embedded assets)..."
cargo build --release --features custom-protocol --manifest-path "$PSScriptRoot\src-tauri\Cargo.toml"
if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
Pop-Location

$releaseDir = "$PSScriptRoot\release"
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null
Copy-Item "$targetRoot\release\prism-time.exe" "$releaseDir\prism-time.exe" -Force
Copy-Item "$targetRoot\release\WebView2Loader.dll" "$releaseDir\WebView2Loader.dll" -Force

Write-Host "Build completed successfully:"
Get-Item "$releaseDir\prism-time.exe", "$releaseDir\WebView2Loader.dll" | Select-Object Name, Length, FullName
