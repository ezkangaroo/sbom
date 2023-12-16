#Requires -Version 5

<#
    .SYNOPSIS
    Download and install the latest available SBOM release from GitHub.
#>

[CmdletBinding()]
Param()

$OldErrorActionPref = $ErrorActionPreference
$ErrorActionPreference = "Stop"

$app = "sbom"
$github = "https://github.com"
$owner = "ezkangaroo"
$repo = "sbom"
$latestUri = "$github/$owner/$repo/releases/$releaseTag"
$userExtractDir = "$env:LOCALAPPDATA\sbom"
$allUsersExtractDir = "$env:PROGRAMFILES\sbom"

$extractDir = "$userExtractDir"

Write-Verbose "Looking up release ($releaseTag)..."

[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12

$headers = @{
    'Accept' = 'application/json'
}

$release = Invoke-RestMethod -Uri $latestUri -Method Get -Headers $headers
$releaseVersion = $release.tag_name;
$downloadUri = "$github/$owner/$repo/releases/download/$releaseVersion/$($app)-x86_64-pc-windows-gnu.zip"

Write-Output "Downloading: $downloadUri"

$TempDir = Join-Path ([System.IO.Path]::GetTempPath()) "sbom"
if (![System.IO.Directory]::Exists($TempDir)) {[void][System.IO.Directory]::CreateDirectory($TempDir)}

$zipFile = "$TempDir\sbom.zip"

(New-Object System.Net.WebClient).DownloadFile($downloadUri, $zipFile)

Expand-Archive -Path $zipFile -DestinationPath $extractDir -Force

$ErrorActionPreference = $OldErrorActionPref

$cmdapp = "$extractDir\sbom.exe"
$env:Path += ";$extractDir"
Write-Host "Installed sbom at: $sbom"
Write-Host "Get started by running: sbom.exe --help"
Write-Host "------"
Write-Host "Running sbom.exe --version"

& $cmdapp --version