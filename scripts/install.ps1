# Usage: powershell -ExecutionPolicy Bypass -File install.ps1
# Note: Sample script to install ace on Windows
function Get-AceDownloadUrl {
    $arch = $env:PROCESSOR_ARCHITECTURE.ToLower()
    # available arch: AMD64, ARM64
    if ($arch -eq "amd64" || $arch -eq "arm64") {
        # TODO: get latest release, currently hardcode to cf-tool v1.0.9
        # Download latest dotnet/codeformatter release from github

        $file = "ace-${arch}.exe"

        $releases = "https://api.github.com/repos/dianhsu/ace/releases"

        Write-Host Determining latest release
        $tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name

        $downloadUrl = "https://github.com/dianhsu/ace/releases/download/$tag/$file"
        return $downloadUrl
    }
    else {
        Write-Host -ForegroundColor Red "Unsupported architecture: $arch"
        return ""
    }
}

function Update-Path {
    param(
        [string]$installDir
    )   
    $env:Path.Split(';') | ForEach-Object {
        if ($_ -eq $installDir) {
            Write-Warning "Already in Path: $installDir"
            exit 0
        }
    }
    $env:Path = "$env:Path;$installDir"
    Write-Host -ForegroundColor Green "Update Path: $env:Path"
    [Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
}

$installDir = "$env:USERPROFILE\cf-tool"
$downloadUrl = Get-AceDownloadUrl 
if ($downloadUrl -eq "") {
    exit 1
}
# Create install dir if not exist
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Force -Path $installDir
}
# Download ace.exe
Invoke-WebRequest $downloadUrl -OutFile "$installDir\ace.exe"
# Update Path env
Update-Path $installDir
