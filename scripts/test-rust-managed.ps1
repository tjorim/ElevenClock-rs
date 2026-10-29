# Development harness for an explicitly disposable Windows account. Never shipped.
# Exercises the managed ZIP (ADR 0004) with the real launcher: the launcher copies the
# app from a temporary "source" folder to %LOCALAPPDATA%\Programs\ElevenClock-rs, the
# app recognises managed mode, the Startup option registers the launcher, a changed
# source is picked up on the next launch, and the installer refuses while the managed
# copy is present. This writes real HKCU startup values.
param(
    [string]$PackageDirectory = 'dist/candidate',
    [string]$OutputDirectory = 'native-evidence/managed',
    [switch]$DisposableAccount
)
$ErrorActionPreference = 'Stop'
if (!$DisposableAccount) { throw 'Use only in a disposable account, with -DisposableAccount' }
$packageRoot = (Resolve-Path -LiteralPath $PackageDirectory).Path
# The published assets sit directly in the directory (as downloaded from CI).
$installer = (Get-ChildItem -LiteralPath $packageRoot -Filter '*-setup.exe' | Select-Object -ExpandProperty FullName -First 1)
$managedZip = (Get-ChildItem -LiteralPath $packageRoot -Filter '*-managed.zip' | Select-Object -ExpandProperty FullName -First 1)
if (!$installer -or !$managedZip) { throw 'The package directory needs the setup and managed assets' }
$install = Join-Path $env:LOCALAPPDATA 'Programs/ElevenClock-rs'
$shortcut = Join-Path ([Environment]::GetFolderPath('Programs')) 'ElevenClock-rs.lnk'
$run = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
$approved = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run'
$value = 'ElevenClockRust'
function Assert($condition, [string]$message) { if (!$condition) { throw $message } }
function Read-Run([string]$name) { (Get-ItemProperty -LiteralPath $run -Name $name -ErrorAction SilentlyContinue).$name }
Assert (!(Test-Path $install) -and !(Test-Path $shortcut)) 'Account already has an installation'
Assert (!(Get-Process -Name elevenclock-rs,ElevenClock -ErrorAction SilentlyContinue)) 'Exit all Rust and legacy overlay apps before testing'
Assert ($null -eq (Read-Run $value)) 'Account already owns startup registration'
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$evidence = (Resolve-Path -LiteralPath $OutputDirectory).Path
$results = [ordered]@{ artifact = (Split-Path $managedZip -Leaf); status = 'failed or interrupted' }
Add-Type @'
using System;
using System.Runtime.InteropServices;
using System.Text;
public static class ManagedWindow {
  delegate bool Callback(IntPtr window, IntPtr data);
  [DllImport("user32.dll")] static extern bool EnumWindows(Callback callback, IntPtr data);
  [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr window, out uint pid);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern int GetWindowText(IntPtr window, StringBuilder text, int max);
  [DllImport("user32.dll")] public static extern IntPtr SendMessage(IntPtr window, uint msg, IntPtr w, IntPtr l);
  [DllImport("user32.dll", CharSet=CharSet.Unicode)] static extern IntPtr SendMessage(IntPtr window, uint msg, int w, StringBuilder text);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr window);
  [DllImport("user32.dll")] public static extern IntPtr GetDlgItem(IntPtr dialog, int id);
  public static IntPtr Settings(int process) {
    IntPtr result = IntPtr.Zero;
    EnumWindows((window, data) => {
      uint pid; GetWindowThreadProcessId(window, out pid);
      var title = new StringBuilder(256); GetWindowText(window, title, 256);
      if (pid == process && title.ToString() == "ElevenClock-rs Settings") result = window;
      return true;
    }, IntPtr.Zero);
    return result;
  }
  public static string Text(IntPtr control) {
    var text = new StringBuilder(16384); SendMessage(control, 0x000D, 16384, text); // WM_GETTEXT
    return text.ToString().Replace("\r\n", "\n");
  }
}
'@
# Pages are children of the dialog and controls children of pages (settings.rc).
$PAGE = @{ Startup = 113; Support = 114 }
function Control([IntPtr]$window, [int]$page, [int]$id) {
    [ManagedWindow]::GetDlgItem([ManagedWindow]::GetDlgItem($window, $page), $id)
}
function Wait-App {
    $deadline = [DateTime]::UtcNow.AddSeconds(20)
    do {
        $apps = @(Get-Process -Name elevenclock-rs -ErrorAction SilentlyContinue | Where-Object { [ManagedWindow]::Settings($_.Id) -ne [IntPtr]::Zero })
        if ($apps.Count -eq 1) { return $apps[0] }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw 'Launch did not open one Settings window'
}
function Exit-App($app) {
    $window = [ManagedWindow]::Settings($app.Id)
    Assert ($window -ne [IntPtr]::Zero) 'No Settings window for Exit'
    [ManagedWindow]::SendMessage($window, 0x111, [IntPtr]1003, [IntPtr]::Zero) | Out-Null
    Assert ($app.WaitForExit(20000)) 'App did not exit through Settings'
}
function Invoke-Launcher([string]$launcher, [string[]]$arguments = @()) {
    $p = if ($arguments) { Start-Process -FilePath $launcher -ArgumentList $arguments -PassThru } else { Start-Process -FilePath $launcher -PassThru }
    Assert ($p.WaitForExit(60000)) 'Launcher did not finish'
    Assert ($p.ExitCode -eq 0) "Launcher failed ($($p.ExitCode))"
}
function Hash([string]$file) { (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash }
# Waits for the Support page to show a line; its text follows the registry and diagnostics.
function Wait-Report($window, [string]$expected) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $report = [ManagedWindow]::Text((Control $window $PAGE.Support 1503))
        if ($report.Contains($expected)) { return $report }
        Start-Sleep -Milliseconds 200
    } while ([DateTime]::UtcNow -lt $deadline)
    throw "The diagnostics report never showed '$expected': $report"
}
try {
    $work = Join-Path ([IO.Path]::GetTempPath()) "managed-smoke-$([guid]::NewGuid().ToString('N'))"
    New-Item -ItemType Directory -Path $work | Out-Null
    Expand-Archive -LiteralPath $managedZip -DestinationPath "$work/source"
    $source = Join-Path $work 'source/ElevenClock-rs'
    $sourceLauncher = Join-Path $source 'elevenclock-rs-launcher.exe'
    $sourceApp = Join-Path $source 'elevenclock-rs.data'
    Assert ((Test-Path $sourceLauncher) -and (Test-Path $sourceApp)) 'Managed ZIP lacks the launcher or the app (elevenclock-rs.data)'

    # 1. First launch: the launcher deploys the local copy and starts it.
    Invoke-Launcher $sourceLauncher
    $localApp = Join-Path $install 'elevenclock-rs.exe'
    $localLauncher = Join-Path $install 'elevenclock-rs-launcher.exe'
    Assert ((Hash $localApp) -eq (Hash $sourceApp)) 'Local app differs from the central app'
    Assert (Test-Path $localLauncher) 'Launcher did not copy itself next to the app'
    $app = Wait-App
    $results.deploy = 'passed: local copy matches the source and the app started'

    # 2. The app is in managed mode: the version carries the note and the report says so.
    $window = [ManagedWindow]::Settings($app.Id)
    $status = [ManagedWindow]::Text((Control $window $PAGE.Support 1501))
    Assert ($status -like 'ElevenClock-rs version * (managed by your organization)*') "Support status lacks the managed note: $status"
    $report = Wait-Report $window 'Managed: yes'
    Assert ($report.Contains('Startup: off')) "Startup should start off: $report"
    foreach ($id in 1108, 1109) {
        Assert (![ManagedWindow]::IsWindowVisible((Control $window 0 $id))) "Managed mode must hide update control $id"
    }
    [ManagedWindow]::SendMessage($window, 0x0111, [IntPtr]1109, [IntPtr]::Zero) | Out-Null
    $report = [ManagedWindow]::Text((Control $window $PAGE.Support 1503))
    Assert ($report -like '*Update checks: disabled (managed)*') 'Managed update policy missing from diagnostics'
    Assert ($report -like '*Last update check: not checked*') 'Synthetic Check now must not start a managed check'
    $results.managedMode = 'passed: version note and "Managed: yes" in diagnostics'

    # 3. Startup stays the user's choice, but registers the sibling launcher.
    Assert ($null -eq (Read-Run $value)) 'The launcher must not register startup on its own'
    $check = Control $window $PAGE.Startup 1402
    [ManagedWindow]::SendMessage($check, 0x00F5, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null  # BM_CLICK
    $expected = '"' + $localLauncher + '" --startup'
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    while ((Read-Run $value) -ne $expected -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 200 }
    Assert ((Read-Run $value) -eq $expected) "Startup registered '$(Read-Run $value)', expected the launcher"
    Wait-Report $window 'Startup: launcher' | Out-Null
    $results.startup = 'passed: the Run value targets the local launcher with --startup'
    Exit-App $app

    # 4. A changed central copy is picked up by the next launch.
    $oldHash = Hash $localApp
    $stream = [IO.File]::Open($sourceApp, 'Append')
    try { $stream.WriteByte(0) } finally { $stream.Dispose() }  # still a valid PE; a different hash
    Assert ((Hash $sourceApp) -ne $oldHash) 'The test did not change the source'
    # The shipped list now names the old app: the next launch must refuse the copy.
    Invoke-Launcher $sourceLauncher
    Assert ((Hash $localApp) -eq $oldHash) 'Launcher replaced the local copy although the checksum list did not match'
    Exit-App (Wait-App)
    # An admin replaces the list together with the app.
    Set-Content -LiteralPath (Join-Path $source 'SHA256SUMS.txt') -Value "$((Hash $sourceApp).ToLowerInvariant())  elevenclock-rs.data" -Encoding ascii
    Invoke-Launcher $sourceLauncher
    Assert ((Hash $localApp) -eq (Hash $sourceApp)) 'Launcher did not update the local copy'
    $app = Wait-App
    Exit-App $app
    $results.update = 'passed: the next launch replaced the local copy'

    # 5. The installer refuses while the managed copy is present, and changes nothing.
    $before = Hash $localApp
    $setup = Start-Process -FilePath $installer -ArgumentList '/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', "/LOG=`"$evidence/refused-install.log`"" -PassThru
    Assert ($setup.WaitForExit(120000)) 'Installer timed out'
    Assert ($setup.ExitCode -ne 0) 'Installer ran although a managed copy is present'
    Assert (Select-String -LiteralPath "$evidence/refused-install.log" -SimpleMatch 'refused: managed copy present' -Quiet) 'The installer stopped, but not because of the managed copy'
    Assert ((Hash $localApp) -eq $before) 'Refused installer changed the managed app'
    Assert (Test-Path $localLauncher) 'Refused installer removed the launcher'
    Assert (!(Test-Path (Join-Path $install 'unins000.exe')) -and !(Test-Path $shortcut)) 'Refused installer left installer files'
    $results.installerRefusal = 'passed'

    # 6. Uninstall through the launcher removes the managed files and the Run value.
    Invoke-Launcher $sourceLauncher @('--uninstall')
    Assert ($null -eq (Read-Run $value)) 'Managed startup value survived uninstall'
    Assert (!(Test-Path $localApp)) 'Managed uninstall left the app'
    $results.uninstall = 'passed'
    $results.status = 'passed'
} finally {
    Remove-ItemProperty -LiteralPath $run -Name $value -ErrorAction SilentlyContinue
    Remove-ItemProperty -LiteralPath $approved -Name $value -ErrorAction SilentlyContinue
    $results | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath "$evidence/results.json" -Encoding UTF8
}
