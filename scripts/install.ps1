<#
.SYNOPSIS
    Installs mycheatsheet on Windows, configuring PATH, Start Menu shortcut, and notes directory.

.DESCRIPTION
    Builds (or copies) the release binary to %LOCALAPPDATA%\Programs\mycheatsheet,
    deploys the application icon, sets up a Start Menu shortcut, updates the user's
    PATH environment variable, and creates a default config.toml.

.PARAMETER Uninstall
    Removes all installed components, shortcuts, configurations, and PATH entries.
#>

[CmdletBinding()]
param (
    [switch]$Uninstall
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoDir = Split-Path -Parent $ScriptDir
$NotesDir = Join-Path $RepoDir "notes"
$ReleaseBin = Join-Path $RepoDir "target\release\mycheatsheet.exe"

$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\mycheatsheet"
$InstalledExe = Join-Path $InstallDir "mycheatsheet.exe"
$InstalledIcon = Join-Path $InstallDir "icon.ico"
$SourceIcon = Join-Path $RepoDir "assets\icons\icon.ico"

$StartMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"
$ShortcutPath = Join-Path $StartMenuDir "mycheatsheet.lnk"

$ConfigDir = Join-Path $env:APPDATA "mycheatsheet"
$ConfigFile = Join-Path $ConfigDir "config.toml"

function Remove-PathEntry([string]$DirToRemove) {
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath) {
        $Parts = $UserPath -split ';' | Where-Object { $_ -ne $DirToRemove -and $_ -ne "" }
        $NewPath = $Parts -join ';'
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        Write-Host "  [Cleaned] Removed from User PATH: $DirToRemove"
    }
}

function Add-PathEntry([string]$DirToAdd) {
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $Parts = if ($UserPath) { $UserPath -split ';' } else { @() }
    if ($Parts -notcontains $DirToAdd) {
        $NewPath = ($Parts + $DirToAdd) -join ';'
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        $env:Path = "$env:Path;$DirToAdd"
        Write-Host "  [Configured] Added to User PATH: $DirToAdd"
    } else {
        Write-Host "  [Configured] Already present in User PATH: $DirToAdd"
    }
}

if ($Uninstall) {
    Write-Host "Uninstalling mycheatsheet..."

    if (Test-Path $ShortcutPath) {
        Remove-Item $ShortcutPath -Force
        Write-Host "  [Removed] Shortcut: $ShortcutPath"
    }

    if (Test-Path $InstallDir) {
        Remove-Item $InstallDir -Recurse -Force
        Write-Host "  [Removed] Installation directory: $InstallDir"
    }

    if (Test-Path $ConfigDir) {
        Remove-Item $ConfigDir -Recurse -Force
        Write-Host "  [Removed] Config directory: $ConfigDir"
    }

    $HomeNotesDir = Join-Path $env:USERPROFILE ".mycheatsheet"
    if (Test-Path $HomeNotesDir) {
        Remove-Item $HomeNotesDir -Recurse -Force
        Write-Host "  [Removed] Notes directory: $HomeNotesDir"
    }

    $DataDir = Join-Path $env:LOCALAPPDATA "mycheatsheet"
    if (Test-Path $DataDir) {
        Remove-Item $DataDir -Recurse -Force
        Write-Host "  [Removed] Data directory: $DataDir"
    }

    Remove-PathEntry $InstallDir

    Write-Host "`nmycheatsheet has been successfully uninstalled."
    Exit 0
}

Write-Host "Installing mycheatsheet on Windows..."

$CleanupTemp = $false
$TempDir = ""
if (-not (Test-Path $NotesDir)) {
    Write-Host "  [Fetch] Downloading repository notes and assets..."
    $TempDir = Join-Path ([System.IO.Path]::GetTempPath()) ("mycheatsheet-" + [System.Guid]::NewGuid().ToString().Substring(0, 8))
    git clone --depth 1 https://github.com/mnasies/mycheatsheet.git $TempDir
    $RepoDir = $TempDir
    $NotesDir = Join-Path $RepoDir "notes"
    $ReleaseBin = Join-Path $RepoDir "target\release\mycheatsheet.exe"
    $SourceIcon = Join-Path $RepoDir "assets\icons\icon.ico"
    $CleanupTemp = $true
}

# 1. Build release binary if missing
if (-not (Test-Path $ReleaseBin)) {
    Write-Host "Building release binary via cargo..."
    Push-Location $RepoDir
    try {
        cargo build --release
    } finally {
        Pop-Location
    }
}

# 2. Create installation directory and copy files
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Copy-Item $ReleaseBin -Destination $InstalledExe -Force
Write-Host "  [Installed] Binary -> $InstalledExe"

if (Test-Path $SourceIcon) {
    Copy-Item $SourceIcon -Destination $InstalledIcon -Force
    Write-Host "  [Installed] Icon -> $InstalledIcon"
}

# 3. Create Start Menu Shortcut
$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = $InstalledExe
$Shortcut.WorkingDirectory = $InstallDir
$Shortcut.Description = "Fast, offline cheatsheet reference for programming languages"
if (Test-Path $InstalledIcon) {
    $Shortcut.IconLocation = "$InstalledIcon,0"
}
$Shortcut.Save()
Write-Host "  [Installed] Start Menu Shortcut -> $ShortcutPath"

# 4. Update PATH
Add-PathEntry $InstallDir

# 5. Install default notes
$DataNotesDir = Join-Path $env:USERPROFILE ".mycheatsheet\notes"
if (-not (Test-Path $DataNotesDir)) {
    New-Item -ItemType Directory -Path $DataNotesDir -Force | Out-Null
}
Copy-Item "$NotesDir\*" -Destination $DataNotesDir -Recurse -Force
Write-Host "  [Installed] Default notes -> $DataNotesDir"

# 6. Create config.toml pointing to installed notes directory
if (-not (Test-Path $ConfigDir)) {
    New-Item -ItemType Directory -Path $ConfigDir -Force | Out-Null
}

$ConfigContent = @"
# mycheatsheet configuration
notes_dir = "$($DataNotesDir -replace '\\', '/')"
"@
Set-Content -Path $ConfigFile -Value $ConfigContent -Encoding UTF8
Write-Host "  [Configured] $ConfigFile (pointing to $DataNotesDir)"

if ($CleanupTemp -and $TempDir -and (Test-Path $TempDir)) {
    Remove-Item $TempDir -Recurse -Force
}

Write-Host "`nInstallation complete!"
Write-Host "You can now run 'mycheatsheet' from PowerShell or CMD, or launch it from the Start Menu."

