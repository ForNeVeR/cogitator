param (
    $WindivertUrl = 'https://reqrypt.org/download/WinDivert-1.1.8-MSVC.zip',
    $WindivertSha256 = 'F21CC27CAAC865798D20BFD4BF9EB1D3FB314A132879D9A120901928C9DB1068',
    $DownloadLocation = "$PSScriptRoot/../windivert"
)

$ErrorActionPreference = 'Stop'

function Check-Hash($path, $hash) {
    if ((Get-FileHash -Algorithm SHA256 $path).Hash -ne $hash) {
        Write-Warning "$path has invalid hash"
        $false
    } else {
        $true
    }
}

function Ensure-WindivertDownloaded {
    $zipFile = "$DownloadLocation/windivert.zip"
    if (-not (Test-Path $DownloadLocation -ErrorAction Ignore)) {
        Write-Output "Creating $DownloadLocation"
        New-Item $DownloadLocation -Type Directory | Out-Null
    }

    if (Test-Path $zipFile) {
        Write-Output "$zipFile exists, checking hash..."
        if ((Check-Hash $zipFile $WindivertSha256)) {
            Write-Output 'Hash ok'
        } else {
            Write-Warning "Redownloading $zipFile"
            Remove-Item $zipFile
        }
    }

    if (-not (Test-Path $zipFile)) {
        Write-Output "Downloading from $WindivertUrl..."
        Invoke-WebRequest $WindivertUrl -OutFile $zipFile
        if ((Check-Hash $zipFile $WindivertSha256)) {
            Write-Output 'Hash ok'
        } else {
            throw "$zipFile has invalid hash after download; try to restart script"
        }
    }
}

Ensure-WindivertDownloaded
