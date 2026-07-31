# Open Longevity installer / updater for Windows
# Usage: irm https://openlongevity.life/install.ps1 | iex

$ErrorActionPreference = "Stop"
$repository = "edison7009/OpenLongevity"
$headers = @{ Accept = "application/vnd.github+json"; "User-Agent" = "Open-Longevity-Installer" }

Write-Host ""
Write-Host "  Open Longevity Installer" -ForegroundColor Cyan
Write-Host "  ------------------------" -ForegroundColor DarkGray
Write-Host "  Checking the latest Windows release..." -ForegroundColor Gray

$latestVersion = $null
$downloadUrl = $null
try {
  $latestVersion = (Invoke-RestMethod "https://openlongevity.life/version.json?platform=windows" -TimeoutSec 10).version
  if ($latestVersion) { $downloadUrl = "https://openlongevity.life/download/windows" }
} catch {}

if (-not $latestVersion) {
  try {
    $release = Invoke-RestMethod "https://api.github.com/repos/$repository/releases/latest" -Headers $headers -TimeoutSec 15
    $asset = $release.assets | Where-Object { $_.name -like "*_Windows_x64-setup.exe" } | Select-Object -First 1
    if ($asset) {
      $latestVersion = $release.tag_name -replace '^v',''
      $downloadUrl = $asset.browser_download_url
    }
  } catch {}
}

$installedVersion = $null
$registryPaths = @(
  "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*",
  "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*",
  "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*"
)
foreach ($path in $registryPaths) {
  $entry = Get-ItemProperty $path -ErrorAction SilentlyContinue |
    Where-Object { $_.DisplayName -like "Open Longevity*" } | Select-Object -First 1
  if ($entry) { $installedVersion = $entry.DisplayVersion; break }
}

if (-not $latestVersion -or -not $downloadUrl) {
  Write-Host ""
  Write-Host "  An installer is not available yet. Please try again in about 10 minutes." -ForegroundColor Yellow
  exit 0
}

Write-Host "  Latest   : v$latestVersion" -ForegroundColor Green
if ($installedVersion) {
  Write-Host "  Installed: v$installedVersion" -ForegroundColor Gray
  if ($installedVersion -eq $latestVersion) {
    Write-Host ""
    Write-Host "  Open Longevity is already up to date." -ForegroundColor Green
    exit 0
  }
  Write-Host "  Upgrading v$installedVersion -> v$latestVersion..." -ForegroundColor Yellow
} else {
  Write-Host "  Performing a fresh installation..." -ForegroundColor Gray
}

$installerPath = Join-Path ([IO.Path]::GetTempPath()) "Open-Longevity-$latestVersion-setup.exe"
try {
  Write-Host "  Downloading..." -ForegroundColor Gray
  Invoke-WebRequest $downloadUrl -Headers $headers -OutFile $installerPath -UseBasicParsing
  if ((Get-Item -LiteralPath $installerPath).Length -eq 0) { throw "The installer download is empty." }
  Write-Host "  Starting setup..." -ForegroundColor Gray
  Start-Process -FilePath $installerPath -Wait
  Write-Host ""
  Write-Host "  Open Longevity v$latestVersion is installed." -ForegroundColor Green
} catch {
  Write-Host ""
  Write-Host "  Installation failed: $($_.Exception.Message)" -ForegroundColor Red
  Write-Host "  Please wait a few minutes and run the command again." -ForegroundColor DarkYellow
  exit 1
} finally {
  Remove-Item -LiteralPath $installerPath -Force -ErrorAction SilentlyContinue
}
