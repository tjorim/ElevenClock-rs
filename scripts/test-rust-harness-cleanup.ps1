# Failure injection for the native harnesses. Uses disposable LOCALAPPDATA;
# failures occur before any app launch. Each case runs in a fresh PowerShell.
param(
    [string]$Executable = 'target/x86_64-pc-windows-msvc/release/elevenclock-rs.exe',
    [ValidateSet('', 'SettingsPath', 'SettingsRemoval', 'TemplatesRemoval', 'TemplatesClipboard', 'EmptyClipboard')]
    [string]$Case = ''
)
$ErrorActionPreference = 'Stop'
if (!$Case) {
    foreach ($scenario in @('SettingsPath', 'SettingsRemoval', 'TemplatesRemoval', 'TemplatesClipboard', 'EmptyClipboard')) {
        & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $PSCommandPath -Executable $Executable -Case $scenario
        if ($LASTEXITCODE -ne 0) { throw "Cleanup regression failed: $scenario" }
    }
    exit 0
}
if ($Case -eq 'EmptyClipboard') {
    $harness = Get-Content (Join-Path $PSScriptRoot 'test-rust-settings.ps1') -Raw
    $definition = [regex]::Match($harness, "(?s)Add-Type @'\r?\n(.*?)\r?\n'@")
    Add-Type -TypeDefinition $definition.Groups[1].Value
    $clipboard = Get-Clipboard -Raw
    try {
        Set-Clipboard -Value 'clipboard cleanup regression'
        [SettingsNative]::ClearClipboard()
        if (![string]::IsNullOrEmpty((Get-Clipboard -Raw))) { throw 'Empty clipboard restoration failed' }
    } finally {
        if ([string]::IsNullOrEmpty($clipboard)) { [SettingsNative]::ClearClipboard() } else { Set-Clipboard -Value $clipboard }
    }
    Write-Output 'PASS: restoring an empty clipboard on Windows PowerShell 5.1'
    exit 0
}
$fixture = Join-Path $env:TEMP ('harness-cleanup-' + [guid]::NewGuid().ToString('N'))
$data = Join-Path $fixture 'ElevenClock-rs'
New-Item -ItemType Directory -Path $data -Force | Out-Null
$env:LOCALAPPDATA = $fixture
$preferences = Join-Path $data 'preferences.dat'
$library = Join-Path $data 'templates.json'
[IO.File]::WriteAllText($preferences, 'original preferences')
[IO.File]::WriteAllText($library, 'original library')
if ($Case -like '*Removal') { (Get-Item -LiteralPath $library).IsReadOnly = $true }
if ($Case -eq 'TemplatesClipboard') { function Get-Clipboard { throw 'Injected clipboard failure' } }
$failure = $null
try {
    if ($Case -like 'Settings*') {
        & (Join-Path $PSScriptRoot 'test-rust-settings.ps1') -Executable (Join-Path $fixture 'missing.exe')
    } else {
        & (Join-Path $PSScriptRoot 'test-rust-templates.ps1') -Executable $Executable
    }
} catch { $failure = $_ }
finally { (Get-Item -LiteralPath $library).IsReadOnly = $false }
if (!$failure) { throw "Expected setup failure: $Case" }
if ([IO.File]::ReadAllText($preferences) -ne 'original preferences' -or [IO.File]::ReadAllText($library) -ne 'original library') {
    throw "Saved data was lost after $Case"
}
# Ensure failure injection reached the intended setup operation.
if ($Case -eq 'TemplatesClipboard' -and $failure.ToString() -notlike '*Injected clipboard failure*') { throw $failure }
if ($Case -eq 'SettingsPath' -and $failure.ToString() -notlike '*missing.exe*') { throw $failure }
if ($Case -like '*Removal' -and $failure.ToString() -notlike '*templates.json*') { throw $failure }
Write-Output "PASS: $Case preserves preferences and templates"
