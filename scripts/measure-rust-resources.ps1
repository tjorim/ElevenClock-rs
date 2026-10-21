# Development-only resource sampler for R13; never part of the installed app.
#
# Samples every elevenclock-rs process (the app and any helper) at a fixed interval and
# reports, per the acceptance contract: total CPU as CPU-time delta / (wall time x logical
# processors), private working set, handles, threads, GDI/USER objects and top-level
# windows. Timer/discovery rates and hooks are not observable from outside the process;
# record them from the R11 in-app diagnostics instead, and the summary lists them as gaps.
# It only reads public process counters and never starts, stops or controls the app.
#
# Each process is tracked by PID and start time, so CPU stays correct when a helper
# starts or exits mid-run: a process that starts between samples contributes all of its
# CPU, and a process that exits loses only the CPU it used since its last sample, which
# is reported as an accounting gap rather than hidden. Rows are appended as they are
# taken, and the summary is written even if the app exits or the run is interrupted, so a
# long soak never loses its raw samples.
param(
    [int]$DurationSeconds = 600,
    [int]$IntervalSeconds = 10,
    [string]$Label = 'unlabelled',
    # Free text recorded with the run: the clock format and any workload or transitions.
    [string]$Workload = '',
    [string]$OutputDirectory = 'native-evidence/resources'
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$logical = [Environment]::ProcessorCount
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$base = Join-Path $OutputDirectory "$stamp-$Label"
$csv = "$base.csv"
$eventsCsv = "$base-processes.csv"

Add-Type -Namespace ElevenClockMeasure -Name Native -MemberDefinition @'
[DllImport("kernel32.dll", SetLastError = true)]
public static extern System.IntPtr OpenProcess(uint access, bool inherit, uint pid);
[DllImport("kernel32.dll")]
public static extern bool CloseHandle(System.IntPtr handle);
[DllImport("user32.dll")]
public static extern bool SetProcessDPIAware();
[DllImport("user32.dll")]
public static extern uint GetGuiResources(System.IntPtr process, uint flags);
public delegate bool EnumProc(System.IntPtr hwnd, System.IntPtr lParam);
[DllImport("user32.dll")]
public static extern bool EnumWindows(EnumProc callback, System.IntPtr lParam);
[DllImport("user32.dll")]
public static extern uint GetWindowThreadProcessId(System.IntPtr hwnd, out uint pid);
public static System.Collections.Generic.Dictionary<uint, int> TopLevelWindows() {
    var counts = new System.Collections.Generic.Dictionary<uint, int>();
    EnumWindows((hwnd, _) => {
        uint pid; GetWindowThreadProcessId(hwnd, out pid);
        int n; counts.TryGetValue(pid, out n); counts[pid] = n + 1;
        return true;
    }, System.IntPtr.Zero);
    return counts;
}
'@
# Report monitor bounds in physical pixels rather than this host's DPI-virtualized view.
[void][ElevenClockMeasure.Native]::SetProcessDPIAware()

function Get-GuiObjects([int]$ProcessId) {
    # PROCESS_QUERY_LIMITED_INFORMATION is enough for GetGuiResources.
    $handle = [ElevenClockMeasure.Native]::OpenProcess(0x1000, $false, [uint32]$ProcessId)
    if ($handle -eq [IntPtr]::Zero) { return @(-1, -1) }
    try {
        return @([int][ElevenClockMeasure.Native]::GetGuiResources($handle, 0),
                 [int][ElevenClockMeasure.Native]::GetGuiResources($handle, 1))
    } finally { [void][ElevenClockMeasure.Native]::CloseHandle($handle) }
}

function Snapshot {
    $time = [DateTime]::UtcNow
    $windows = [ElevenClockMeasure.Native]::TopLevelWindows()
    $entries = @{}
    foreach ($process in @(Get-Process -Name 'elevenclock-rs*' -ErrorAction SilentlyContinue)) {
        try {
            $start = $process.StartTime.ToUniversalTime()
            # Private working set (the contract's metric), not private bytes/commit.
            $counter = Get-CimInstance Win32_PerfFormattedData_PerfProc_Process -Filter "IDProcess = $($process.Id)"
            $gui = Get-GuiObjects $process.Id
            $count = 0; [void]$windows.TryGetValue([uint32]$process.Id, [ref]$count)
            $entries["$($process.Id)@$($start.Ticks)"] = [pscustomobject]@{
                Id = $process.Id; Name = $process.ProcessName; Path = $process.Path; Start = $start
                Cpu = $process.TotalProcessorTime.TotalSeconds
                Private = [int64]$counter.WorkingSetPrivate
                Handles = $process.HandleCount; Threads = $process.Threads.Count
                Gdi = $gui[0]; User = $gui[1]; Windows = $count
            }
        } catch {
            # The process exited while it was being read; the next sample records the exit.
        }
    }
    [pscustomobject]@{ Time = $time; Entries = $entries }
}

function Get-Environment($snapshot) {
    $os = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
    $cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
    $machine = Get-CimInstance Win32_ComputerSystem
    $personalize = Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -ErrorAction SilentlyContinue
    Add-Type -AssemblyName System.Windows.Forms
    $dpi = (Get-ItemProperty 'HKCU:\Control Panel\Desktop\WindowMetrics' -ErrorAction SilentlyContinue).AppliedDPI
    $executables = @($snapshot.Entries.Values | Where-Object Path | Sort-Object Path -Unique | ForEach-Object {
        $item = Get-Item $_.Path
        [ordered]@{
            path = $_.Path; sha256 = (Get-FileHash $_.Path -Algorithm SHA256).Hash
            fileVersion = $item.VersionInfo.FileVersion; lastWriteUtc = $item.LastWriteTimeUtc.ToString('o')
        }
    })
    $commit = $null; $rustc = $null
    try { $commit = (git rev-parse HEAD 2>$null) } catch {}
    try { $rustc = (rustc -V 2>$null) } catch {}
    [ordered]@{
        capturedUtc = $snapshot.Time.ToString('o')
        executables = $executables
        # The checkout's HEAD where the sampler ran; it identifies the candidate only if the
        # executable above was built from it. The app's own Support report states its commit.
        checkoutCommit = $commit; rustc = $rustc
        windows = [ordered]@{
            edition = $os.EditionID; build = $os.CurrentBuild; ubr = $os.UBR
            displayVersion = $os.DisplayVersion; architecture = $env:PROCESSOR_ARCHITECTURE
        }
        cpu = $cpu.Name.Trim(); logicalProcessors = $logical
        machine = "$($machine.Manufacturer) $($machine.Model)"
        theme = [ordered]@{
            appsUseLightTheme = $personalize.AppsUseLightTheme
            systemUsesLightTheme = $personalize.SystemUsesLightTheme
        }
        # AppliedDPI is the session's system DPI; per-monitor DPI comes from Support diagnostics.
        systemDpi = $dpi
        monitors = @([System.Windows.Forms.Screen]::AllScreens | ForEach-Object {
            [ordered]@{
                device = $_.DeviceName; primary = $_.Primary
                bounds = "$($_.Bounds.Width)x$($_.Bounds.Height) at ($($_.Bounds.X), $($_.Bounds.Y))"
            }
        })
    }
}

function Write-ProcessEvent($kind, $time, $entry) {
    [pscustomobject]@{
        UtcTime = $time.ToString('o'); Event = $kind; Id = $entry.Id; Name = $entry.Name
        StartUtc = $entry.Start.ToString('o'); Path = $entry.Path
    } | Export-Csv -Path $eventsCsv -NoTypeInformation -Append
}

# Least-squares slope per hour, so a soak's growth is visible beyond start/end noise.
function Get-Trend($rows, [string]$field) {
    $points = @($rows | Where-Object { $_.Processes -gt 0 })
    if ($points.Count -eq 0) { return $null }
    $origin = [DateTime]::Parse($points[0].UtcTime).ToUniversalTime()
    $xs = @($points | ForEach-Object { ([DateTime]::Parse($_.UtcTime).ToUniversalTime() - $origin).TotalHours })
    $ys = @($points | ForEach-Object { [double]$_.$field })
    $stats = $ys | Measure-Object -Minimum -Maximum -Average
    $slope = $null
    if ($points.Count -ge 2) {
        $mx = ($xs | Measure-Object -Average).Average; $my = $stats.Average
        $num = 0.0; $den = 0.0
        for ($i = 0; $i -lt $xs.Count; $i++) {
            $num += ($xs[$i] - $mx) * ($ys[$i] - $my); $den += ($xs[$i] - $mx) * ($xs[$i] - $mx)
        }
        if ($den -gt 0) { $slope = [math]::Round($num / $den, 4) }
    }
    [ordered]@{
        start = $ys[0]; end = $ys[-1]; min = $stats.Minimum; max = $stats.Maximum
        mean = [math]::Round($stats.Average, 2); slopePerHour = $slope
    }
}

$first = Snapshot
if ($first.Entries.Count -eq 0) { throw 'No elevenclock-rs process is running; start the app first.' }
Get-Environment $first | ConvertTo-Json -Depth 5 | Set-Content -Path "$base-environment.json" -Encoding utf8
foreach ($entry in $first.Entries.Values) { Write-ProcessEvent 'present' $first.Time $entry }

$rows = New-Object System.Collections.Generic.List[object]
$previous = $first
$deadline = $first.Time.AddSeconds($DurationSeconds)
$accountedCpu = 0.0; $gapIntervals = 0; $zeroSamples = 0; $interrupted = $false
$seen = @{}; foreach ($key in $first.Entries.Keys) { $seen[$key] = $true }
try {
    while ($previous.Time -lt $deadline) {
        Start-Sleep -Seconds $IntervalSeconds
        $now = Snapshot
        $wall = ($now.Time - $previous.Time).TotalSeconds
        $delta = 0.0; $started = 0; $exited = 0
        foreach ($key in $now.Entries.Keys) {
            $entry = $now.Entries[$key]
            if ($previous.Entries.ContainsKey($key)) {
                $delta += $entry.Cpu - $previous.Entries[$key].Cpu
            } else {
                # Started since the last sample: all of its CPU belongs to this interval.
                $started++; $delta += $entry.Cpu; $seen[$key] = $true
                Write-ProcessEvent 'started' $now.Time $entry
            }
        }
        foreach ($key in $previous.Entries.Keys) {
            if (-not $now.Entries.ContainsKey($key)) {
                $exited++; Write-ProcessEvent 'exited' $now.Time $previous.Entries[$key]
            }
        }
        if ($exited -gt 0) { $gapIntervals++ }
        if ($now.Entries.Count -eq 0) { $zeroSamples++ }
        $accountedCpu += $delta
        $values = @($now.Entries.Values)
        $row = [pscustomobject]@{
            UtcTime = $now.Time.ToString('o'); Processes = $values.Count
            Started = $started; Exited = $exited
            CpuPercentOfMachine = [math]::Round(100 * $delta / ($wall * $logical), 4)
            PrivateMiB = [math]::Round((($values | Measure-Object Private -Sum).Sum) / 1MB, 2)
            Handles = [int](($values | Measure-Object Handles -Sum).Sum)
            Threads = [int](($values | Measure-Object Threads -Sum).Sum)
            GdiObjects = [int](($values | Measure-Object Gdi -Sum).Sum)
            UserObjects = [int](($values | Measure-Object User -Sum).Sum)
            TopLevelWindows = [int](($values | Measure-Object Windows -Sum).Sum)
        }
        $row | Export-Csv -Path $csv -NoTypeInformation -Append
        $rows.Add($row)
        $previous = $now
    }
} catch {
    $interrupted = $true
    Write-Warning "Sampling stopped early: $_"
} finally {
    $wallTotal = ($previous.Time - $first.Time).TotalSeconds
    $cpuPercent = $null
    if ($wallTotal -gt 0) { $cpuPercent = [math]::Round(100 * $accountedCpu / ($wallTotal * $logical), 4) }
    $privateMax = ($rows | Measure-Object PrivateMiB -Maximum).Maximum
    $gaps = @(
        'Timer-wake and discovery rates: not observable externally; take them from Support diagnostics.'
        'Hooks: not observable externally; GDI/USER object and window counts are the external proxy.'
    )
    if ($gapIntervals -gt 0) { $gaps += "$gapIntervals interval(s) lost the CPU of a process that exited since its previous sample." }
    if ($zeroSamples -gt 0) { $gaps += "$zeroSamples sample(s) found no elevenclock-rs process." }
    $summary = [ordered]@{
        label = $Label; workload = $Workload; csv = $csv; processEvents = $eventsCsv
        environment = "$base-environment.json"
        intervalSeconds = $IntervalSeconds; requestedSeconds = $DurationSeconds
        wallSeconds = [math]::Round($wallTotal, 1); samples = $rows.Count
        complete = (-not $interrupted) -and ($zeroSamples -eq 0) -and ($wallTotal -ge $DurationSeconds)
        logicalProcessors = $logical
        processesSeen = $seen.Count
        processesStarted = ($rows | Measure-Object Started -Sum).Sum
        processesExited = ($rows | Measure-Object Exited -Sum).Sum
        cpuPercentOfMachine = $cpuPercent
        privateMiBMax = $privateMax
        trends = [ordered]@{
            privateMiB = Get-Trend $rows 'PrivateMiB'; handles = Get-Trend $rows 'Handles'
            threads = Get-Trend $rows 'Threads'; processes = Get-Trend $rows 'Processes'
            gdiObjects = Get-Trend $rows 'GdiObjects'; userObjects = Get-Trend $rows 'UserObjects'
            topLevelWindows = Get-Trend $rows 'TopLevelWindows'
        }
        targets = 'CPU < 0.2% of machine; private working set < 40 MiB (reference machine only)'
        cpuTargetMet = ($null -ne $cpuPercent) -and ($cpuPercent -lt 0.2)
        memoryTargetMet = ($null -ne $privateMax) -and ($privateMax -lt 40)
        instrumentationGaps = $gaps
    }
    $summary | ConvertTo-Json -Depth 5 | Tee-Object -FilePath "$base.json"
}
if (-not $summary.complete) { exit 1 }
