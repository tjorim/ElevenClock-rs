# Development-only, independent UIA identity check for the real clock-app (#51).
# Starts the app quietly, records where its overlays sit, stops it so the native
# tray is uncovered, then uses managed UI Automation (not the app's selector) to
# find the clock, the bell, Show Desktop and the other tray buttons on every
# taskbar. Each overlay must match exactly one clock element owned by that
# taskbar's Explorer. A child process then hit-tests a grid over each button
# with IUIAutomation::ElementFromPoint: interior points must resolve to that
# button, and no overlay point may resolve to another button. No input is sent
# and no setting is changed; the caller restarts any clock it stopped.
param(
    [string]$Executable,
    [string]$OutputDirectory,
    [ValidateRange(1,16)][int]$Step = 4,
    # Internal: the child process that runs the queued point checks.
    [string]$SampleJobs
)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
Add-Type @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public static class UiaIdentity {
    [StructLayout(LayoutKind.Sequential)] public struct Rect { public int Left,Top,Right,Bottom; }
    delegate bool EnumProc(IntPtr window,IntPtr parameter);
    [DllImport("user32.dll")] static extern bool EnumWindows(EnumProc callback,IntPtr parameter);
    [DllImport("user32.dll",CharSet=CharSet.Unicode)] static extern IntPtr FindWindowExW(IntPtr parent,IntPtr after,string cls,string title);
    [DllImport("user32.dll",CharSet=CharSet.Unicode)] static extern int GetClassNameW(IntPtr window,StringBuilder text,int size);
    [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr window,out uint process);
    [DllImport("user32.dll")] static extern bool IsWindowVisible(IntPtr window);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr window,out Rect rect);
    [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr window);
    [DllImport("user32.dll")] public static extern IntPtr SetThreadDpiAwarenessContext(IntPtr context);
    public static IntPtr[] Taskbars() {
        var found=new List<IntPtr>();
        foreach(var cls in new[]{"Shell_TrayWnd","Shell_SecondaryTrayWnd"}) {
            var after=IntPtr.Zero;
            for(int count=0;count<16;count++) {
                after=FindWindowExW(IntPtr.Zero,after,cls,null);
                if(after==IntPtr.Zero) break;
                found.Add(after);
            }
        }
        return found.ToArray();
    }
    public static Rect[] Overlays(uint process) {
        var found=new List<Rect>();
        EnumWindows((window,parameter)=> {
            uint owner; GetWindowThreadProcessId(window,out owner);
            var text=new StringBuilder(64);
            if(owner==process && IsWindowVisible(window) && GetClassNameW(window,text,text.Capacity)>0 && text.ToString()=="ElevenClock-rs.Overlay") {
                Rect r; GetWindowRect(window,out r); found.Add(r);
            }
            return true;
        },IntPtr.Zero);
        return found.ToArray();
    }
    public static string ClassOf(IntPtr window) { var text=new StringBuilder(64); GetClassNameW(window,text,text.Capacity); return text.ToString(); }
}
// The managed client stops at Shell_TrayWnd instead of hit-testing into the XAML
// island, so point checks use IUIAutomation::ElementFromPoint. Only the vtable
// prefix that is called is declared; placeholders keep the slots in order.
[StructLayout(LayoutKind.Sequential)] public struct UiaPoint { public int X,Y; }
[ComImport, Guid("30cbe57d-d9d0-452a-ab13-7ac5ac4825ee"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomation {
    void CompareElements(); void CompareRuntimeIds(); void GetRootElement(); void ElementFromHandle();
    IUIAutomationElement ElementFromPoint(UiaPoint point);
    void GetFocusedElement(); void GetRootElementBuildCache(); void ElementFromHandleBuildCache(); void ElementFromPointBuildCache();
    void GetFocusedElementBuildCache(); void CreateTreeWalker(); void ControlViewWalker(); void ContentViewWalker();
    IUIAutomationTreeWalker RawViewWalker { get; }
}
[ComImport, Guid("d22108aa-8ac5-49a5-837b-37bbb3d7591e"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomationElement {
    void SetFocus(); void GetRuntimeId(); void FindFirst(); void FindAll(); void FindFirstBuildCache(); void FindAllBuildCache(); void BuildUpdatedCache();
    [return: MarshalAs(UnmanagedType.Struct)] object GetCurrentPropertyValue(int property);
}
[ComImport, Guid("4042c624-389c-4afc-a630-9df854a541fc"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
public interface IUIAutomationTreeWalker {
    IUIAutomationElement GetParentElement(IUIAutomationElement element);
}
public static class NativeHit {
    static readonly IUIAutomation automation = (IUIAutomation)Activator.CreateInstance(Type.GetTypeFromCLSID(new Guid("ff48dba4-60ef-4201-aa87-54103eef594e")));
    // Runtime id of the SystemTrayIcon button containing the hit element, or
    // "class[automation id]" of the hit element when there is none.
    public static string TrayAt(int x,int y) {
        var walker=automation.RawViewWalker;
        var hit=automation.ElementFromPoint(new UiaPoint { X=x,Y=y });
        var node=hit;
        for(int depth=0;node!=null && depth<12;depth++) {
            if((node.GetCurrentPropertyValue(30011) as string)=="SystemTrayIcon") return Id(node);
            node=walker.GetParentElement(node);
        }
        return "none:"+(hit.GetCurrentPropertyValue(30012) as string)+"["+(hit.GetCurrentPropertyValue(30011) as string)+"]";
    }
    static string Id(IUIAutomationElement element) {
        var id=element.GetCurrentPropertyValue(30000) as int[];
        return id==null ? "none:no runtime id" : string.Join(".",id);
    }
}
'@
# Physical coordinates for both the native window queries and UI Automation.
[UiaIdentity]::SetThreadDpiAwarenessContext([IntPtr](-4)) | Out-Null
if ($SampleJobs) {
    # Child mode. Once System.Windows.Automation has been used in a process, its
    # client-side proxies make ElementFromPoint stop at Shell_TrayWnd, so the
    # point checks run here, in a process that never loads the managed client.
    $work = Get-Content $SampleJobs -Raw | ConvertFrom-Json
    $done = foreach ($job in $work.jobs) {
        $hits = 0; $edge = 0; $interior = 0; $foreign = 0; $misses = @(); $watch = [Diagnostics.Stopwatch]::StartNew()
        $r = $job.rect
        foreach ($p in $job.points) {
            $got = [NativeHit]::TrayAt($p[0], $p[1])
            if ($got -eq $job.want) { $hits++; continue }
            # UIA bounds are rounded from fractional DIPs, so a boundary pixel may
            # belong to the gap or the adjoining button; interior pixels may not.
            if ($p[0] -eq $r.left -or $p[0] -eq $r.right - 1 -or $p[1] -eq $r.top -or $p[1] -eq $r.bottom - 1) { $edge++ } else { $interior++ }
            if (!$got.StartsWith('none:')) { $foreign++ }
            if ($misses.Count -lt 20) {
                $what = if ($work.names.PSObject.Properties[$got]) { $work.names.$got } else { $got }
                $misses += "($($p[0]),$($p[1])) -> $what"
            }
        }
        [ordered]@{ points = @($job.points).Count; hits = $hits; edgeMisses = $edge; interiorMisses = $interior; otherButton = $foreign; firstMisses = $misses; elapsedMs = $watch.ElapsedMilliseconds }
    }
    ConvertTo-Json -InputObject @($done) -Depth 4 -Compress
    return
}
if (!$Executable -or !$OutputDirectory) { throw '-Executable and -OutputDirectory are required' }
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$exe = (Resolve-Path $Executable).Path
if (Get-Process -Name elevenclock-rs -ErrorAction SilentlyContinue) { throw 'Stop every running ElevenClock-rs first; this check needs its own instance.' }

$A = [System.Windows.Automation.AutomationElement]
$walker = [System.Windows.Automation.TreeWalker]::RawViewWalker
function Rect-Of($r) { [ordered]@{ left = $r.Left; top = $r.Top; right = $r.Right; bottom = $r.Bottom } }
function Uia-Rect($element) {
    $b = $element.Current.BoundingRectangle
    if ($b.IsEmpty) { return $null }
    [ordered]@{ left = [int][Math]::Round($b.Left); top = [int][Math]::Round($b.Top); right = [int][Math]::Round($b.Right); bottom = [int][Math]::Round($b.Bottom) }
}
function Describe($element) {
    $c = $element.Current
    [ordered]@{ automationId = $c.AutomationId; class = $c.ClassName; name = $c.Name; controlType = $c.ControlType.ProgrammaticName; framework = $c.FrameworkId; processId = $c.ProcessId; offscreen = $c.IsOffscreen; rect = (Uia-Rect $element) }
}
function Children($element) {
    $out = @(); $child = $walker.GetFirstChild($element)
    while ($child) { $out += $child; $child = $walker.GetNextSibling($child) }
    return $out
}
function Tray-Buttons($root) {
    # Breadth-first over the raw view, bounded: the tray buttons sit a few levels down.
    $queue = [Collections.Generic.Queue[object]]::new(); $queue.Enqueue(@($root, 0)); $found = @(); $visited = 0
    while ($queue.Count -and $visited -lt 4000) {
        $item = $queue.Dequeue(); $visited++
        foreach ($child in (Children $item[0])) {
            if ($child.Current.AutomationId -eq 'SystemTrayIcon') { $found += $child }
            elseif ($item[1] -lt 6) { $queue.Enqueue(@($child, $item[1] + 1)) }
        }
    }
    return $found
}
function Parentage($element, $root) {
    $chain = @(); $node = $element
    while ($node -and $chain.Count -lt 16) {
        $c = $node.Current
        $chain += "$($c.ClassName)[$($c.AutomationId)]"
        if ([System.Windows.Automation.Automation]::Compare($node, $root)) { return [ordered]@{ chain = $chain; reachesTaskbar = $true } }
        $node = $walker.GetParent($node)
    }
    [ordered]@{ chain = $chain; reachesTaskbar = $false }
}
function Runtime-Id($element) { $element.GetRuntimeId() -join '.' }
function Points($r) {
    $xs = @(for ($x = $r.left; $x -lt $r.right; $x += $Step) { $x }) + ($r.right - 1) | Select-Object -Unique
    $ys = @(for ($y = $r.top; $y -lt $r.bottom; $y += $Step) { $y }) + ($r.bottom - 1) | Select-Object -Unique
    foreach ($y in $ys) { foreach ($x in $xs) { ,@($x, $y) } }
}
$jobs = [Collections.Generic.List[object]]::new()
$names = @{}
# Queue a point check; the child process fills in hits after the tree walk.
function Sample-Points($r, $expected, [string]$label) {
    $job = [ordered]@{ target = $label; class = $expected.Current.ClassName; name = $expected.Current.Name; runtimeId = (Runtime-Id $expected); rect = $r; points = 0; hits = 0; firstMisses = @(); elapsedMs = 0 }
    $jobs.Add($job)
    return $job
}
function Capture([string]$name, $r) {
    $bitmap = [Drawing.Bitmap]::new($r.Right - $r.Left, $r.Bottom - $r.Top)
    $graphics = [Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.CopyFromScreen($r.Left, $r.Top, 0, 0, $bitmap.Size)
        $bitmap.Save((Join-Path $OutputDirectory "$name.png"), [Drawing.Imaging.ImageFormat]::Png)
    } finally { $graphics.Dispose(); $bitmap.Dispose() }
}

$version = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
$results = [ordered]@{
    executable = $exe
    sha256 = (Get-FileHash $exe -Algorithm SHA256).Hash
    # ProductName still says "Windows 10" on Windows 11; derive the major version from the build.
    os = "Windows $(if ([int]$version.CurrentBuild -ge 22000) { 11 } else { 10 }) $($version.EditionID) $($version.DisplayVersion) $($version.CurrentBuild).$($version.UBR) $env:PROCESSOR_ARCHITECTURE"
    step = $Step
    taskbars = @()
}
$app = $null
try {
    $app = Start-Process $exe -ArgumentList '--startup' -PassThru
    $deadline = [DateTime]::UtcNow.AddSeconds(30); $stableSince = $null; $last = ''
    while ([DateTime]::UtcNow -lt $deadline) {
        $now = ([UiaIdentity]::Overlays([uint32]$app.Id) | ForEach-Object { "$($_.Left),$($_.Top),$($_.Right),$($_.Bottom)" }) -join ';'
        if ($now -and $now -eq $last) { if (((Get-Date) - $stableSince).TotalSeconds -ge 2) { break } } else { $stableSince = Get-Date; $last = $now }
        Start-Sleep -Milliseconds 200
    }
    $overlays = @([UiaIdentity]::Overlays([uint32]$app.Id))
    if (!$overlays.Count) { throw 'No visible overlay appeared within 30 seconds' }
    $bars = @([UiaIdentity]::Taskbars())
    $index = 0
    foreach ($bar in $bars) { $r = New-Object UiaIdentity+Rect; [UiaIdentity]::GetWindowRect($bar, [ref]$r) | Out-Null; Capture "taskbar$index-overlay" $r; $index++ }
    $app.Kill(); $app.WaitForExit(10000) | Out-Null
    Start-Sleep -Milliseconds 1500 # let Explorer repaint the uncovered clock

    $index = 0
    foreach ($bar in $bars) {
        $r = New-Object UiaIdentity+Rect; [UiaIdentity]::GetWindowRect($bar, [ref]$r) | Out-Null
        Capture "taskbar$index-native" $r
        $shellPid = [uint32]0; [UiaIdentity]::GetWindowThreadProcessId($bar, [ref]$shellPid) | Out-Null
        $entry = [ordered]@{ index = $index; class = [UiaIdentity]::ClassOf($bar); bounds = (Rect-Of $r); dpi = [UiaIdentity]::GetDpiForWindow($bar); shellPid = $shellPid; checks = [ordered]@{} }
        $root = $A::FromHandle($bar)
        $buttons = @(Tray-Buttons $root)
        $entry.trayButtons = @($buttons | ForEach-Object { Describe $_ })
        foreach ($b in $buttons) { $names[(Runtime-Id $b)] = "$($b.Current.ClassName) '$($b.Current.Name)'" }
        $clocks = @($buttons | Where-Object { $_.Current.ClassName -in 'SystemTray.OmniButton','SystemTray.OmniButtonLeft' -and @(Children $_ | Where-Object { $_.Current.AutomationId -eq 'TimeInnerTextBlock' }).Count })
        $overlay = @($overlays | Where-Object { $_.Left -ge $r.Left -and $_.Right -le $r.Right -and $_.Top -ge $r.Top -and $_.Bottom -le $r.Bottom })
        $entry.overlay = if ($overlay.Count -eq 1) { Rect-Of $overlay[0] } else { $null }
        $entry.checks.oneClock = ($clocks.Count -eq 1)
        $entry.checks.oneOverlay = ($overlay.Count -eq 1)
        if ($clocks.Count -eq 1) {
            $clock = $clocks[0]
            $entry.clock = Describe $clock
            $entry.clock.parentage = Parentage $clock $root
            $entry.checks.clockOwnedByTaskbarShell = ($clock.Current.ProcessId -eq $shellPid)
            $entry.checks.clockParentReachesTaskbar = $entry.clock.parentage.reachesTaskbar
            # A split clock has a separate bell (OmniButtonRight); a merged one does not.
            # Show Desktop is required only on the primary taskbar.
            $split = $clock.Current.ClassName -eq 'SystemTray.OmniButtonLeft'
            $required = @(if ($split) { 'SystemTray.OmniButtonRight' }) + @(if ($entry.class -eq 'Shell_TrayWnd') { 'SystemTray.ShowDesktopButton' })
            foreach ($class in $required) { $entry.checks."present_$class" = (@($buttons | Where-Object { $_.Current.ClassName -eq $class }).Count -eq 1) }
            $entry.points = @()
            if ($overlay.Count -eq 1) {
                $o = $entry.overlay; $c = $entry.clock.rect
                $entry.checks.overlayMatchesClock = ($c -and [Math]::Abs($c.left - $o.left) -le 1 -and [Math]::Abs($c.top - $o.top) -le 1 -and [Math]::Abs($c.right - $o.right) -le 1 -and [Math]::Abs($c.bottom - $o.bottom) -le 1)
                $entry.points += Sample-Points $o $clock 'clock'
            }
            # Every other on-screen tray button is a neighbour: its points must hit
            # itself, and none may overlap the overlay.
            $entry.neighbours = @()
            foreach ($b in ($buttons | Where-Object { $_ -ne $clock -and !$_.Current.IsOffscreen })) {
                $d = Describe $b
                if (!$d.rect) { continue }
                $d.parentage = Parentage $b $root
                $entry.neighbours += $d
                $label = if ($split -and $d.class -eq 'SystemTray.OmniButtonRight') { 'bell' } elseif ($d.class -eq 'SystemTray.ShowDesktopButton') { 'showDesktop' } else { "neighbour$($entry.neighbours.Count)" }
                $entry.points += Sample-Points $d.rect $b $label
                $o = $entry.overlay; $n = $d.rect
                if ($o) { $entry.checks."overlayClearOf_$label" = ($n.right -le $o.left -or $n.left -ge $o.right -or $n.bottom -le $o.top -or $n.top -ge $o.bottom) }
            }
        }
        $results.taskbars += $entry
        $index++
    }
    $jobFile = Join-Path $OutputDirectory 'point-jobs.json'
    @{ names = $names; jobs = @($jobs | ForEach-Object { @{ want = $_.runtimeId; rect = $_.rect; points = @(Points $_.rect) } }) } | ConvertTo-Json -Depth 6 -Compress | Set-Content -Encoding UTF8 $jobFile
    $sampled = @(powershell.exe -NoProfile -ExecutionPolicy Bypass -File $PSCommandPath -SampleJobs $jobFile | ConvertFrom-Json | ForEach-Object { $_ })
    Remove-Item $jobFile
    if ($sampled.Count -ne $jobs.Count) { throw "Point sampling returned $($sampled.Count) of $($jobs.Count) results" }
    for ($i = 0; $i -lt $jobs.Count; $i++) { foreach ($key in 'points','hits','edgeMisses','interiorMisses','otherButton','firstMisses','elapsedMs') { $jobs[$i][$key] = $sampled[$i].$key } }
    foreach ($entry in $results.taskbars) {
        foreach ($sample in @($entry.points)) {
            # Interior points must all hit; the overlay may cover no other button at all.
            $entry.checks."points_$($sample.target)" = ($sample.points -gt 0 -and $sample.interiorMisses -eq 0 -and ($sample.target -ne 'clock' -or $sample.otherButton -eq 0))
        }
        $entry.passed = -not ($entry.checks.Values -contains $false)
    }
    $results.passed = ($results.taskbars.Count -gt 0) -and -not ($results.taskbars | Where-Object { -not $_.passed })
} finally {
    if ($app -and !$app.HasExited) { $app.Kill() }
    $results | ConvertTo-Json -Depth 10 | Set-Content -Encoding UTF8 (Join-Path $OutputDirectory 'uia-identity.json')
}
if (!$results.passed) { throw 'UIA identity check failed; see uia-identity.json' }
'UIA identity check passed'
