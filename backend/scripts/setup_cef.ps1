# CEF version to download
$CEF_VERSION = "120.1.10+g3ce3184+chromium-120.0.6099.129"
$CEF_PLATFORM = "windows64"

# Create directories
New-Item -ItemType Directory -Force -Path "cef"
Set-Location "cef"

# Download CEF binary distribution
$CEF_DOWNLOAD_URL = "https://cef-builds.spotifycdn.com/cef_binary_${CEF_VERSION}_${CEF_PLATFORM}.tar.bz2"
$CEF_ARCHIVE = "cef_binary.tar.bz2"

Write-Host "Downloading CEF from $CEF_DOWNLOAD_URL"
Invoke-WebRequest -Uri $CEF_DOWNLOAD_URL -OutFile $CEF_ARCHIVE

# Extract archive
Write-Host "Extracting CEF archive"
tar -xf $CEF_ARCHIVE

# Move files to the right place
Write-Host "Setting up CEF files"
$CEF_DIR = "cef_binary_${CEF_VERSION}_${CEF_PLATFORM}"
New-Item -ItemType Directory -Force -Path "Release"
Copy-Item "$CEF_DIR/Release/*" "Release/" -Recurse -Force
Copy-Item "$CEF_DIR/Resources/*" "Release/" -Recurse -Force
Copy-Item "$CEF_DIR/include" "." -Recurse -Force

# Clean up
Write-Host "Cleaning up"
Remove-Item $CEF_ARCHIVE
Remove-Item $CEF_DIR -Recurse -Force

Write-Host "CEF setup complete" 