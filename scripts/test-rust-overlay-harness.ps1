# Exercise the actual wait function without loading the desktop harness or Win32.
$ErrorActionPreference = 'Stop'
$tokens=$null;$parseErrors=$null
$ast=[System.Management.Automation.Language.Parser]::ParseFile(
    (Join-Path $PSScriptRoot 'test-rust-overlay.ps1'),[ref]$tokens,[ref]$parseErrors)
if ($parseErrors.Count) {throw $parseErrors}
$waitFunction=$ast.Find({param($node)
    $node -is [System.Management.Automation.Language.FunctionDefinitionAst] -and $node.Name -eq 'Wait-Overlay'
},$true)
if (!$waitFunction) {throw 'Wait-Overlay function missing'}
Add-Type @'
namespace Windows.Forms {
    public static class Application { public static void DoEvents() {} }
}
public static class OverlayEvidence {
    public static bool IsWindowVisible(System.IntPtr h) { return h.ToInt64()==1; }
}
'@
. ([ScriptBlock]::Create($waitFunction.Extent.Text))
function Overlay {
    if ($script:handles.Count) {return [IntPtr]$script:handles.Dequeue()}
    return [IntPtr]1
}
function Capture($name) {$script:captured=$name}
foreach ($case in @(
    @{name='absent is hidden before recovery';handles=@(0,1);visible=$false;expected=0},
    @{name='destruction between visible samples';handles=@(1,0,1);visible=$false;expected=0},
    @{name='existing hidden window';handles=@(2);visible=$false;expected=2},
    @{name='missing window does not satisfy visible wait';handles=@(0,1);visible=$true;expected=1}
)) {
    $script:handles=[Collections.Generic.Queue[int]]::new()
    foreach ($handle in $case.handles) {$script:handles.Enqueue($handle)}
    $result=Wait-Overlay $case.visible 1
    if ($result.ToInt64() -ne $case.expected) {throw $case.name}
    Write-Output "Passed: $($case.name)"
}
$script:handles=[Collections.Generic.Queue[int]]::new();$script:captured=$null
try {
    Wait-Overlay $false 1 | Out-Null
    throw 'Accepted an always-visible overlay'
} catch {
    if ($_.Exception.Message -ne 'Overlay visible=False not observed' -or $script:captured -ne 'wait-failed') {throw}
}
Write-Output 'Passed: always-visible overlay rejected with failure capture'
