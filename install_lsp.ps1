$ErrorActionPreference = "Stop"

$LATEST = (Invoke-RestMethod "https://api.github.com/repos/opmr0/sinbo/releases/latest").tag_name
$DEST = "$env:USERPROFILE\.cargo\bin"

Write-Host "Installing sinbo $LATEST..."
$URL = "https://github.com/opmr0/sinbo/releases/download/$LATEST/sinbo-windows-x86_64.zip"
$ZIP = "$env:TEMP\sinbo.zip"
Invoke-WebRequest -Uri $URL -OutFile $ZIP
Expand-Archive -Path $ZIP -DestinationPath $env:TEMP -Force
Move-Item -Force "$env:TEMP\sinbo-windows-x86_64.exe" "$DEST\sinbo.exe"
Remove-Item $ZIP
Write-Host "sinbo installed successfully!"

Write-Host "Installing sinbo-lsp..."
$LSP_URL = "https://github.com/opmr0/sinbo/releases/download/$LATEST/sinbo-lsp-windows-x86_64.zip"
$LSP_ZIP = "$env:TEMP\sinbo-lsp.zip"
Invoke-WebRequest -Uri $LSP_URL -OutFile $LSP_ZIP
Expand-Archive -Path $LSP_ZIP -DestinationPath $env:TEMP -Force
Move-Item -Force "$env:TEMP\sinbo-lsp-windows-x86_64.exe" "$DEST\sinbo-lsp.exe"
Remove-Item $LSP_ZIP
Write-Host "sinbo-lsp installed successfully!"

Write-Host ""
Write-Host "Run 'sinbo --help' to get started"
Write-Host "See https://github.com/opmr0/sinbo/sinbo-lsp for editor setup"