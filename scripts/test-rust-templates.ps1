# Native template-flow smoke test, without foreground keyboard injection. Uses
# real controls, the real second-instance handoff and an optional owned URL handler.
# Saves/restores the current user's preferences, template library and clipboard.
param([string]$Executable = 'target/x86_64-pc-windows-msvc/release/elevenclock-rs.exe',
      [string]$OutputDirectory = 'native-evidence/templates')
$ErrorActionPreference = 'Stop'
if (Get-Process elevenclock-rs -ErrorAction SilentlyContinue) { throw 'Exit ElevenClock-rs before the template smoke test' }
Add-Type -AssemblyName System.Drawing
$harness = Get-Content (Join-Path $PSScriptRoot 'test-rust-settings.ps1') -Raw
$definition = [regex]::Match($harness, "(?s)Add-Type @'\r?\n(.*?)\r?\n'@")
if (!$definition.Success) { throw 'Settings native test helpers unavailable' }
Add-Type -TypeDefinition $definition.Groups[1].Value
[SettingsNative]::SetThreadDpiAwarenessContext([IntPtr](-4)) | Out-Null
$work = Join-Path $env:TEMP ('template-smoke-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $work | Out-Null
$exe = (Copy-Item -LiteralPath $Executable -Destination $work -PassThru).FullName
$data = Join-Path $env:LOCALAPPDATA 'ElevenClock-rs'
New-Item -ItemType Directory -Path $data -Force | Out-Null
$preferences = Join-Path $data 'preferences.dat'
$library = Join-Path $data 'templates.json'
$backups = @{}
foreach ($path in @($preferences, $library)) {
    $backups[$path] = if (Test-Path -LiteralPath $path) { [IO.File]::ReadAllBytes($path) } else { $null }
    if (Test-Path -LiteralPath $path) { Remove-Item -LiteralPath $path }
}
$clipboard = Get-Clipboard -Raw
$protocol = 'HKCU:\Software\Classes\elevenclock'
$protocolWasPresent = Test-Path -LiteralPath $protocol
$ownedCommand = '"' + $exe + '" --template "%1"'
$app = $null
function Assert($value, [string]$message) { if (!$value) { throw $message }; Write-Output "PASS: $message" }
function Control([int]$page, [int]$id) {
    $parent = if ($page) { [SettingsNative]::GetDlgItem($window, $page) } else { $window }
    [SettingsNative]::GetDlgItem($parent, $id)
}
function Click([int]$page, [int]$id) {
    [SettingsNative]::SendMessageW((Control $page $id), 0x00F5, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
    Start-Sleep -Milliseconds 500
}
function Text([int]$page, [int]$id) { [SettingsNative]::Text((Control $page $id)) }
function Launch-Link([string]$link) {
    $sender = Start-Process -FilePath $exe -ArgumentList '--template', ('"' + $link + '"') -PassThru -WindowStyle Hidden
    Assert ($sender.WaitForExit(10000) -and $sender.ExitCode -eq 0) 'second-instance template sender exits successfully'
    Start-Sleep -Milliseconds 400
}
try {
    $first = 'elevenclock://template?v=1&format=%25H%3A%25M&font=9&theme=dark'
    # The window under test must be visible; SW_HIDE would hide its first ShowWindow.
    $app = Start-Process -FilePath $exe -ArgumentList '--template', ('"' + $first + '"') -PassThru
    $deadline = [DateTime]::UtcNow.AddSeconds(15)
    do { Start-Sleep -Milliseconds 100; $window = [SettingsNative]::Settings($app.Id) }
    while ($window -eq [IntPtr]::Zero -and [DateTime]::UtcNow -lt $deadline)
    Assert ($window -ne [IntPtr]::Zero) 'template first launch opens Settings'
    foreach ($id in 1601..1610) { Assert ((Control 115 $id) -ne [IntPtr]::Zero) "Templates control $id exists" }
    Assert ((Text 115 1607) -like 'https://tjorim.github.io/ElevenClock-rs/#template=1*theme=dark') 'first-launch link is offered for review'
    $tab = Control 0 1100
    foreach ($id in @(1602,1604,1605,1601,1603,1606,1609,1610,1607,1608)) {
        $tab = [SettingsNative]::GetNextDlgTabItem($window, $tab, $false)
        Assert ([SettingsNative]::GetDlgCtrlID($tab) -eq $id) "native Templates tab order reaches $id"
    }
    Assert ([SettingsNative]::AccessibleName((Control 115 1606)) -eq 'Copy share link') 'Share action has its accessible name'
    Assert ([SettingsNative]::AccessibleName((Control 115 1608)) -eq 'Review template') 'Review action has its accessible name'
    Assert ((Text 111 1202) -eq "%H:%M`n%g%V.%u") 'incoming link leaves the default draft unchanged'
    Click 115 1608
    Assert ((Text 111 1202) -eq '%H:%M' -and (Text 112 1303) -eq '9') 'Review stages template presentation'
    Click 0 1105
    Assert ((Text 0 1102) -eq 'Settings applied.') 'template uses the normal Apply path'
    $saved = [IO.File]::ReadAllText($preferences)
    [SettingsNative]::SetText((Control 111 1202), '%d')
    Assert ((Text 111 1202) -eq '%d') 'native edit accepts the unsaved draft'
    # WM_SETTEXT does not emit EN_CHANGE for a multiline edit. Queue the same
    # notification actual typing emits so the controller owns the test draft.
    [SettingsNative]::SendMessageW((Control 0 111), 0x0111, [IntPtr](1202 -bor (0x0300 -shl 16)), (Control 111 1202)) | Out-Null
    Start-Sleep -Milliseconds 300
    Assert ((Text 111 1202) -eq '%d') 'unsaved draft survives the ordinary message pump'
    $second = 'elevenclock://template?v=1&format=%25g%25V.%25u&font=11&theme=light'
    Launch-Link $second
    Assert ((Text 111 1202) -eq '%d') "second-instance link preserves unsaved edits (found: $(Text 111 1202))"
    Assert ((Text 115 1607) -like '*font=11&theme=light') 'second-instance handoff transfers appearance'
    Click 115 1608
    Assert ((Text 111 1202) -eq '%g%V.%u' -and (Text 112 1303) -eq '11') 'second template stages format and font'
    Assert ([IO.File]::ReadAllText($preferences) -eq $saved) 'Review never persists before Apply'
    [SettingsNative]::SetText((Control 115 1601), 'Template smoke')
    Click 115 1603
    $stored = Get-Content -LiteralPath $library -Raw | ConvertFrom-Json
    Assert ($stored.presets.Count -eq 1 -and $stored.presets[0].name -eq 'Template smoke') 'named preset is saved separately'
    Click 115 1606
    Assert ((Get-Clipboard -Raw).Trim() -eq $stored.presets[0].link) 'sharing copies the saved presentation link'
    Click 0 1104
    Click 115 1604
    Assert ((Text 111 1202) -eq '%g%V.%u' -and (Text 112 1303) -eq '11') 'saved local preset loads into the draft'
    Click 115 1605
    Assert (@((Get-Content -LiteralPath $library -Raw | ConvertFrom-Json).presets).Count -eq 0) 'saved preset can be deleted'
    if (!$protocolWasPresent) {
        Click 115 1610
        Assert ((Get-Item -LiteralPath ($protocol + '\shell\open\command')).GetValue('') -eq $ownedCommand) 'template protocol is quoted and registered per-user'
        Start-Process -FilePath $first -WindowStyle Hidden
        $deadline = [DateTime]::UtcNow.AddSeconds(10)
        while ((Text 115 1607) -notlike '*font=9&theme=dark' -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
        Assert ((Text 115 1607) -like '*font=9&theme=dark') 'Windows protocol dispatch delivers a template to the running app'
        $unicode = 'elevenclock://template?v=1&format=%25H%3A%25M%0A%C3%A9%20%26%20%E6%97%A5%20%25%25&font=9&theme=dark'
        Start-Process -FilePath $unicode -WindowStyle Hidden
        $deadline = [DateTime]::UtcNow.AddSeconds(10)
        while ((Text 115 1607) -notlike '*%C3%A9%20%26%20%E6%97%A5*' -and [DateTime]::UtcNow -lt $deadline) { Start-Sleep -Milliseconds 100 }
        Assert ((Text 115 1607) -like '*%C3%A9%20%26%20%E6%97%A5%20%25%25*') 'Windows protocol dispatch preserves Unicode, ampersands and literal percent signs'
        Click 115 1610
        Assert (!(Test-Path -LiteralPath $protocol)) 'disabling template links removes the owned registration'
    } else { Write-Output 'SKIP: existing URL handler preserved; scratch-key registration tests cover ownership' }
    New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
    $rect = New-Object SettingsNative+Rect
    [SettingsNative]::GetWindowRect($window, [ref]$rect) | Out-Null
    $bitmap = [Drawing.Bitmap]::new($rect.Right - $rect.Left, $rect.Bottom - $rect.Top)
    try {
        $graphics = [Drawing.Graphics]::FromImage($bitmap)
        try { $dc = $graphics.GetHdc(); try { [SettingsNative]::PrintWindow($window, $dc, 2) | Out-Null } finally { $graphics.ReleaseHdc($dc) } }
        finally { $graphics.Dispose() }
        $bitmap.Save((Join-Path (Resolve-Path $OutputDirectory) 'templates.png'), [Drawing.Imaging.ImageFormat]::Png)
    } finally { $bitmap.Dispose() }
    Click 0 2
    Assert ([IO.File]::ReadAllText($preferences) -eq $saved) 'Cancel discards the imported draft'
} finally {
    if (!$protocolWasPresent -and (Test-Path -LiteralPath ($protocol + '\shell\open\command'))) {
        if ((Get-Item -LiteralPath ($protocol + '\shell\open\command')).GetValue('') -eq $ownedCommand) { Remove-Item -LiteralPath $protocol -Recurse }
    }
    Get-Process elevenclock-rs -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe } | ForEach-Object {
        Stop-Process -Id $_.Id -Force
        $_.WaitForExit()
    }
    foreach ($path in @($preferences, $library)) {
        if ($null -eq $backups[$path]) { if (Test-Path -LiteralPath $path) { Remove-Item -LiteralPath $path } }
        else { [IO.File]::WriteAllBytes($path, [byte[]]$backups[$path]) }
    }
    if ($null -eq $clipboard) { Set-Clipboard -Value '' } else { Set-Clipboard -Value $clipboard }
    # The disposable executable is removed by exact file path, without recursion.
    Remove-Item -LiteralPath $exe -ErrorAction SilentlyContinue
    Write-Output ('Evidence: ' + $OutputDirectory)
}
