Write-Host "Installing xclock..."
# Check cargo
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Cargo not found. Please install Rust."
    exit 1
}

$RepoUrl = "https://github.com/xscriptordev/xclock.git"
$InstallDir = "$env:USERPROFILE\.cargo\bin"

# Check if we are in the repo
if (Test-Path "Cargo.toml") {
    $Content = Get-Content "Cargo.toml" -Raw
    if ($Content -match "xclock") {
        Write-Host "Detected local repository. Building from source..."
        cargo build --release
        
        if (!(Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Force -Path $InstallDir
        }
        
        Copy-Item "target\release\xclock.exe" -Destination "$InstallDir\xclock.exe" -Force
        Write-Host "Done! Ensure $InstallDir is in your PATH."
        exit 0
    }
}

$TempDir = [System.IO.Path]::GetTempPath() + "xclock_install"
if (Test-Path $TempDir) { Remove-Item -Recurse -Force $TempDir }

Write-Host "Cloning..."
git clone $RepoUrl $TempDir
Push-Location $TempDir

Write-Host "Building..."
cargo build --release

if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir
}

Write-Host "Copying to $InstallDir..."
Copy-Item "target\release\xclock.exe" -Destination "$InstallDir\xclock.exe" -Force

Write-Host "Done! Ensure $InstallDir is in your PATH."
Pop-Location
Remove-Item -Recurse -Force $TempDir
