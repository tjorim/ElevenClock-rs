# Development-only: checks the drop-box ACL recipe with two throwaway local accounts over
# loopback SMB. Run from an ELEVATED PowerShell on a disposable machine. It creates and then
# removes: two local users, C:\ecdrop-test and an SMB share. Results: C:\ecdrop-test-results.txt
param(
    [string]$TestExecutable = 'target/debug/deps/clock_launcher-626b25cc8b03ea10.exe',
    [string]$Launcher = 'target/debug/elevenclock-rs-launcher.exe',
    [switch]$DisposableMachine
)
$ErrorActionPreference = 'Stop'
if (-not $DisposableMachine) { throw 'This creates and deletes local users, a share and C:\ecdrop-test. Use only on a disposable machine, with -DisposableMachine.' }
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
if (-not ([Security.Principal.WindowsPrincipal]$identity).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'Run this from an elevated (administrator) PowerShell.'
}
$root = 'C:\ecdrop-test'
$results = 'C:\ecdrop-test-results.txt'
$share = 'ecdrop-reports'
$names = 'ecdrop1', 'ecdrop2'
$password = ConvertTo-SecureString ([guid]::NewGuid().ToString('N') + 'aA1!') -AsPlainText -Force
$log = New-Object Collections.Generic.List[string]
function Note([string]$line) { $log.Add($line); Write-Host $line }
$failures = New-Object Collections.Generic.List[string]
function Expect([bool]$condition, [string]$what) {
    if ($condition) { Note "  ok   $what" } else { $failures.Add($what); Note "  FAIL $what" }
}
$sources = (Resolve-Path $TestExecutable).Path, (Resolve-Path $Launcher).Path
try {
    foreach ($path in @($root)) { if (Test-Path $path) { Remove-Item $path -Recurse -Force } }
    New-Item -ItemType Directory "$root\bin", "$root\out", "$root\reports" | Out-Null
    Copy-Item $sources "$root\bin"
    icacls $root /grant 'Users:(OI)(CI)RX' | Out-Null
    icacls "$root\out" /grant '*S-1-1-0:(OI)(CI)M' | Out-Null
    foreach ($name in $names) {
        if (Get-LocalUser $name -ErrorAction SilentlyContinue) { Remove-LocalUser $name }
        New-LocalUser $name -Password $password -PasswordNeverExpires | Out-Null
    }
    # The recipe from docs/managed-deployment.md, applied to the reports folder.
    $reports = "$root\reports"
    icacls $reports /inheritance:r | Out-Null
    icacls $reports /grant '*S-1-5-32-544:(OI)(CI)F' '*S-1-5-18:(OI)(CI)F' | Out-Null
    icacls $reports /grant '*S-1-5-11:(WD)' | Out-Null
    icacls $reports /grant '*S-1-3-0:(OI)(IO)(WD,AD,WA,WEA,RA,RC,S)' | Out-Null
    if (Get-SmbShare $share -ErrorAction SilentlyContinue) { Remove-SmbShare $share -Force }
    New-SmbShare -Name $share -Path $reports -ChangeAccess 'Authenticated Users' -FullAccess 'Administrators' | Out-Null
    $unc = "\\localhost\$share"
    Note "Folder ACL:"; (icacls $reports) | ForEach-Object { Note "  $_" }

    $script:step = 0
    function As([string]$name, [string]$label, [string]$command) {
        $script:step++
        $credential = New-Object Management.Automation.PSCredential("$env:COMPUTERNAME\$name", $password)
        $out = "$root\out\step$($script:step)-$name.txt"
        $script = "$root\out\step$($script:step)-$name.ps1"
        Set-Content $script $command
        $process = Start-Process powershell.exe -Credential $credential -WorkingDirectory $root -Wait -PassThru -WindowStyle Hidden `
            -ArgumentList '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', $script -RedirectStandardOutput $out -RedirectStandardError "$out.err"
        Note "[$name] $label (exit $($process.ExitCode))"
        $lines = if (Test-Path $out) { @(Get-Content $out) } else { @() }
        $lines | ForEach-Object { Note "    $_" }
        if ((Test-Path "$out.err") -and (Get-Item "$out.err").Length) { Get-Content "$out.err" | Select-Object -First 3 | ForEach-Object { Note "    ERR $_" } }
        return $lines
    }

    $test = "$root\bin\$(Split-Path $sources[0] -Leaf)"
    $publish = "`$env:ELEVENCLOCK_DROP_BOX='$unc'; & '$test' drop_box_folder --ignored 2>&1 | Select-String 'test result|panicked|denied'"
    $first = As 'ecdrop1' 'publish (create + rewrite own file)' $publish
    Expect (($first -join '|') -match 'test result: ok') 'ecdrop1 creates and rewrites its own report'
    $probe = @"
`$d = '$unc'
`$f = Get-ChildItem -Force -Name -Path '$root\reports' -ErrorAction SilentlyContinue
function T(`$n,[scriptblock]`$b){ try{ & `$b; "ALLOWED  `$n" }catch{ "DENIED   `$n" } }
`$victim = '$unc\S-1-5-21-123.12345678-1234-4234-8234-123456789abc.json'
T 'list the folder' { if(-not (Get-ChildItem `$d -ErrorAction Stop)){ throw 'x' } }
T "read another user's file" { [IO.File]::ReadAllText(`$victim) | Out-Null }
T "overwrite another user's file" { [IO.File]::Open(`$victim,'OpenOrCreate','Write').Close() }
T "delete another user's file" { Remove-Item `$victim -ErrorAction Stop }
T "rename another user's file" { Rename-Item `$victim 'x.json' -ErrorAction Stop }
T 'create a file with a new name' { [IO.File]::Open("`$d\ecdrop2-own.json",'OpenOrCreate','Write').Close() }
"@
    $seen = As 'ecdrop2' 'probe (should be denied except a new name)' $probe
    foreach ($denied in 'list the folder', "read another user's file", "overwrite another user's file", "delete another user's file", "rename another user's file") {
        Expect ($seen -contains "DENIED   $denied") "ecdrop2 is denied: $denied"
    }
    Expect ($seen -contains 'ALLOWED  create a file with a new name') 'ecdrop2 can create a new name'
    $second = As 'ecdrop2' 'publish over the same name (should fail)' $publish
    Expect (($second -join '|') -match 'test result: FAILED') 'ecdrop2 cannot publish over ecdrop1''s report'

    # Summary as administrator: the file is owned by ecdrop1, so its SID does not match.
    $summary = "$root\summary.json"
    foreach ($skip in @($false, $true)) {
        $arguments = @('--inventory-summary', "`"$reports`"", "`"$summary`"", '3650') + $(if ($skip) { '--skip-owner-check' })
        $run = Start-Process "$root\bin\$(Split-Path $sources[1] -Leaf)" -ArgumentList $arguments -Wait -PassThru -WindowStyle Hidden
        $json = if (Test-Path $summary) { Get-Content $summary -Raw | ConvertFrom-Json }
        Note "[admin] summary skip-owner-check=$skip (exit $($run.ExitCode)): installations=$($json.counts.installations) unverified=$($json.unverified_owner_reports)"
        if ($skip) { Expect ($json.counts.installations -eq 1) 'summary with --skip-owner-check counts the report' }
        else { Expect ($json.counts.installations -eq 0 -and $json.unverified_owner_reports -eq 1) 'summary excludes a report whose owner does not match its SID' }
        if (Test-Path $summary) { Remove-Item $summary }
    }
    Note "Report owners:"
    Get-ChildItem $reports -Force | ForEach-Object { Note "  $($_.Name)  owner=$((Get-Acl $_.FullName).Owner)" }
}
catch { $failures.Add($_.Exception.Message); Note "FAILED: $($_.Exception.Message)" }
finally {
    if (Get-SmbShare $share -ErrorAction SilentlyContinue) { Remove-SmbShare $share -Force }
    foreach ($name in $names) { if (Get-LocalUser $name -ErrorAction SilentlyContinue) { Remove-LocalUser $name } }
    if (Test-Path $root) { Remove-Item $root -Recurse -Force -ErrorAction SilentlyContinue }
    Set-Content $results $log
    Write-Host "Done. Results saved to $results"
}
if ($failures.Count) { Write-Host "$($failures.Count) check(s) failed"; exit 1 }
