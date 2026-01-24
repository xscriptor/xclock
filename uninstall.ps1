$InstallDir = "$env:USERPROFILE\.cargo\bin"
$ExePath = "$InstallDir\xclock.exe"

if (Test-Path $ExePath) {
    Remove-Item $ExePath -ErrorAction SilentlyContinue
    Write-Host "Uninstalled xclock from $InstallDir."
} else {
    Write-Host "xclock not found in $InstallDir."
}
