$ErrorActionPreference = "Stop"

$version = "V0.3.0"
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

Write-Host "Building Tauri Windows application (release, Thin-LTO stripped, embedded assets)..."
cargo build --release --features custom-protocol --manifest-path "$PSScriptRoot\src-tauri\Cargo.toml"
if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }

$appExe = "$targetRoot\release\prism-time.exe"
$wv2Dll = "$targetRoot\release\WebView2Loader.dll"
$releaseDir = "$PSScriptRoot\release"
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

# Copy standalone binaries (skip overwrite gracefully if currently running in tray)
try {
    Copy-Item $appExe "$releaseDir\prism-time.exe" -Force -ErrorAction Stop
    Copy-Item $wv2Dll "$releaseDir\WebView2Loader.dll" -Force -ErrorAction Stop
} catch {
    Write-Warning "release\prism-time.exe is currently running; continuing to package single-EXE installer from fresh target-prism\release build."
}

Write-Host "Compressing single-EXE installer payload (Zlib level 9)..."
$payloadPath = "$targetRoot\installer-payload.zlib"
python -c @"
import struct, zlib, pathlib
files = [
    ('prism-time.exe', pathlib.Path(r'$appExe').read_bytes()),
    ('WebView2Loader.dll', pathlib.Path(r'$wv2Dll').read_bytes()),
    ('LICENSE', pathlib.Path(r'$PSScriptRoot\LICENSE').read_bytes()),
    ('README.md', pathlib.Path(r'$PSScriptRoot\README.md').read_bytes()),
]
buf = bytearray(b'PTI1')
buf += struct.pack('<I', len(files))
for name, data in files:
    nb = name.encode('utf-8')
    buf += struct.pack('<I', len(nb)) + nb
    buf += struct.pack('<I', len(data)) + data
compressed = zlib.compress(bytes(buf), level=9)
pathlib.Path(r'$payloadPath').write_bytes(compressed)
print(f'Payload raw={len(buf)/1024/1024:.2f} MiB -> compressed={len(compressed)/1024/1024:.2f} MiB')
"@
if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }

Write-Host "Compiling single-file Windows installer EXE..."
$env:PRISM_PAYLOAD_PATH = $payloadPath
cargo build --release --manifest-path "$PSScriptRoot\src-tauri\installer\Cargo.toml"
if ($LASTEXITCODE -ne 0) { Pop-Location; exit $LASTEXITCODE }
Pop-Location

$setupOut = "$releaseDir\PrismTime-$version-windows-x64-setup.exe"
Copy-Item "$targetRoot\release\prism-time-setup.exe" $setupOut -Force

Write-Host "Build completed successfully:"
Get-Item $appExe, $setupOut | Select-Object Name, Length, FullName
