# Harness helper: drives the Settings "delete preset" confirmation message box.
# Dot-source after defining Assert. PostMessage keeps the harness thread free while the
# modal box runs inside the button handler.
Add-Type -Namespace Harness -Name Confirm -MemberDefinition @'
[DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindowExW(IntPtr parent, IntPtr after, string cls, string title);
[DllImport("user32.dll")] public static extern bool PostMessageW(IntPtr window, uint message, IntPtr wparam, IntPtr lparam);
[DllImport("user32.dll")] public static extern IntPtr GetDlgItem(IntPtr dialog, int id);
'@
function Click-Confirm([IntPtr]$Button, [IntPtr]$Owner, [bool]$Yes) {
    [Harness.Confirm]::PostMessageW($Button, 0x00F5, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
    $answer = if ($Yes) { 6 } else { 7 } # IDYES / IDNO
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    $target = [IntPtr]::Zero
    while ($target -eq [IntPtr]::Zero -and [DateTime]::UtcNow -lt $deadline) {
        $box = [IntPtr]::Zero
        while (($box = [Harness.Confirm]::FindWindowExW([IntPtr]::Zero, $box, '#32770', 'ElevenClock-rs')) -ne [IntPtr]::Zero) {
            if ($box -ne $Owner -and [Harness.Confirm]::GetDlgItem($box, 6) -ne [IntPtr]::Zero) {
                $target = [Harness.Confirm]::GetDlgItem($box, $answer); break
            }
        }
        if ($target -eq [IntPtr]::Zero) { Start-Sleep -Milliseconds 100 }
    }
    Assert ($target -ne [IntPtr]::Zero) 'deleting a preset asks for confirmation'
    [Harness.Confirm]::PostMessageW($target, 0x00F5, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null
    Start-Sleep -Milliseconds 500
}
