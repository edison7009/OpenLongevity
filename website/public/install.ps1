$ErrorActionPreference = "Stop"

$repository = "edison7009/OpenLongevity"
$headers = @{
  Accept = "application/vnd.github+json"
  "User-Agent" = "Open-Longevity-Installer"
}

Write-Host "Finding the latest Open Longevity release..." -ForegroundColor Cyan

$release = Invoke-RestMethod `
  -Uri "https://api.github.com/repos/$repository/releases/latest" `
  -Headers $headers

$asset = $release.assets |
  Where-Object { $_.name -like "*_Windows_x64-setup.exe" } |
  Select-Object -First 1

if (-not $asset) {
  throw "The latest release does not contain a Windows x64 installer."
}

$installerPath = Join-Path ([IO.Path]::GetTempPath()) $asset.name

Write-Host "Downloading $($asset.name)..." -ForegroundColor Cyan
Invoke-WebRequest `
  -Uri $asset.browser_download_url `
  -Headers $headers `
  -OutFile $installerPath

if ((Get-Item -LiteralPath $installerPath).Length -eq 0) {
  throw "The installer download is empty."
}

Write-Host "Starting Open Longevity setup..." -ForegroundColor Green
Start-Process -FilePath $installerPath -Wait

Remove-Item -LiteralPath $installerPath -Force -ErrorAction SilentlyContinue
