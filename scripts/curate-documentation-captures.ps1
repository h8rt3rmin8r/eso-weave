#!/usr/bin/env pwsh

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$CaptureRoot,

    [Parameter(Mandatory = $true)]
    [string]$MsiPropertiesSource
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$capturePath = (Resolve-Path -LiteralPath $CaptureRoot).Path
$targetRoot = Join-Path $repositoryRoot 'target'
$targetPrefix = $targetRoot.TrimEnd([System.IO.Path]::DirectorySeparatorChar) + [System.IO.Path]::DirectorySeparatorChar
if (-not $capturePath.StartsWith($targetPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "CaptureRoot must resolve below the repository target directory: $targetRoot"
}

$msiSource = (Resolve-Path -LiteralPath $MsiPropertiesSource).Path
$manifestPath = Join-Path $capturePath 'capture-manifest.json'
$manifest = Get-Content -Raw -LiteralPath $manifestPath | ConvertFrom-Json
if ($manifest.schema -ne 1 -or $manifest.generator -ne 'S083 deterministic screenshot sandbox') {
    throw 'CaptureRoot does not contain the expected S083 capture manifest.'
}

$destination = Join-Path $repositoryRoot 'docs/src/assets/screenshots'
New-Item -ItemType Directory -Path $destination -Force | Out-Null
Add-Type -AssemblyName System.Drawing

$captures = @(
    @{ Scene = 'first-launch'; Destination = 'first-launch.png'; Height = 640 },
    @{ Scene = 'healthy-system-state'; Destination = 'healthy-system-state.png'; Height = 640 },
    @{ Scene = 'pixelbeacon-lost'; Destination = 'pixelbeacon-signal-lost.png'; Height = 640 },
    @{ Scene = 'pixelbeacon-unmanaged'; Destination = 'pixelbeacon-unmanaged.png'; Height = 640 },
    @{ Scene = 'weaving-configuration'; Destination = 'weaving-configuration.png'; Height = 640 },
    @{ Scene = 'auto-potion-ready'; Destination = 'auto-potion-ready.png'; Height = 820 },
    @{ Scene = 'auto-potion-blocked'; Destination = 'auto-potion-blocked.png'; Height = 640 }
)

foreach ($entry in $captures) {
    $receipt = @($manifest.captures | Where-Object {
        $_.scene -eq $entry.Scene -and $_.theme -eq 'dark' -and $_.viewport -eq 'wide'
    })
    if ($receipt.Count -ne 1) {
        throw "Expected one dark-wide capture receipt for $($entry.Scene), found $($receipt.Count)."
    }
    if ($receipt[0].width -ne 1280 -or $receipt[0].height -ne 900) {
        throw "Unexpected source dimensions for $($entry.Scene)."
    }

    $source = Join-Path $capturePath $receipt[0].file
    $sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash.ToLowerInvariant()
    if ($sourceHash -ne $receipt[0].sha256) {
        throw "Source digest does not match the capture receipt for $($entry.Scene)."
    }

    $bitmap = [System.Drawing.Bitmap]::FromFile($source)
    try {
        $rectangle = [System.Drawing.Rectangle]::new(0, 0, 1280, $entry.Height)
        $cropped = $bitmap.Clone($rectangle, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
        try {
            $output = Join-Path $destination $entry.Destination
            $temporary = "$output.s084.tmp"
            $cropped.Save($temporary, [System.Drawing.Imaging.ImageFormat]::Png)
            Move-Item -Force -LiteralPath $temporary -Destination $output
        }
        finally {
            $cropped.Dispose()
        }
    }
    finally {
        $bitmap.Dispose()
    }
}

$msiDestination = Join-Path $destination 'windows-msi-properties-unblock.png'
Copy-Item -Force -LiteralPath $msiSource -Destination $msiDestination

Write-Output "Curated seven deterministic application captures and copied the supplied MSI image to $destination"
