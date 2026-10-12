# The soak run: the game plays itself for hours, is closed, and its record is
# read (procedures/overnight-soak.txt). One command, from anywhere, and
# nothing to answer while it runs:
#
#     pwsh tools/soak.ps1                 six hours
#     pwsh tools/soak.ps1 -Hours 8
#     pwsh tools/soak.ps1 -Minutes 4      a short one, to see that it works
#
# -Seed N fixes the first trail. -Sound plays the game's sound (it is muted
# otherwise). -AllowDisplayOff lets the screen sleep (it is kept on
# otherwise, with the machine, so the window keeps drawing).
#
# Leaves soak/<date-time>/: events.tsv (what the game did, src/soak.rs),
# game.log and game.out.log (what it printed), memory.tsv, run.txt, and
# report.txt, which is the thing to read. Exits 0 when the report found
# nothing wrong, 1 when it found something, 2 when the run could not be made.

param(
    [double]$Hours = 6,
    [double]$Minutes = 0,
    [long]$Seed = -1,
    [switch]$Sound,
    [switch]$AllowDisplayOff
)

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$secs = [int]($Minutes -gt 0 ? $Minutes * 60 : $Hours * 3600)
$dir = Join-Path $root ('soak\' + (Get-Date -Format 'yyyy-MM-dd-HHmm'))
New-Item -ItemType Directory -Force $dir | Out-Null
$report = Join-Path $dir 'report.txt'

# cargo is not always on the PATH of the shell that runs this.
$cargo = (Get-Command cargo -ErrorAction SilentlyContinue)?.Source
if (-not $cargo) { $cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe' }

Push-Location $root
try {
    & $cargo build *> (Join-Path $dir 'build.log')
    if ($LASTEXITCODE -ne 0) {
        "SOAK REPORT`n`nVERDICT: the game did not build, so nothing ran. See build.log." | Set-Content $report
        Get-Content $report
        exit 2
    }
    $exe = Join-Path $root 'target\debug\horse_heroes.exe'

    # Neither the machine nor, unless allowed, the screen sleeps meanwhile.
    Add-Type -Namespace Soak -Name Power -MemberDefinition `
        '[DllImport("kernel32.dll")] public static extern uint SetThreadExecutionState(uint flags);'
    $continuous = [uint32]2147483648
    $awake = $continuous -bor 1 -bor ($AllowDisplayOff ? 0 : 2)
    [Soak.Power]::SetThreadExecutionState($awake) | Out-Null

    $env:IDLE_SOAK = Join-Path $dir 'events.tsv'
    $env:IDLE_SOAK_SECS = $secs
    $arguments = @('--soak')   # a word the game does not read, so the list is never empty
    if (-not $Sound) { $arguments += '--mute' }
    if ($Seed -ge 0) { $arguments += '--seed', $Seed }
    $started = Get-Date
    $game = Start-Process -FilePath $exe -ArgumentList $arguments -WorkingDirectory $root -NoNewWindow -PassThru `
        -RedirectStandardOutput (Join-Path $dir 'game.out.log') `
        -RedirectStandardError (Join-Path $dir 'game.log')
    $null = $game.Handle   # or its exit code cannot be read afterwards
    "Soak run: process $($game.Id), $secs s, into $dir"

    # Its memory once a minute, until it has gone or is two minutes overdue.
    $memory = Join-Path $dir 'memory.tsv'
    "secs`tworking_set_mb`tprivate_mb" | Set-Content $memory
    $deadline = $started.AddSeconds($secs + 120)
    $nextSample = $started
    while (-not $game.HasExited -and (Get-Date) -lt $deadline) {
        if ((Get-Date) -ge $nextSample) {
            $nextSample = $nextSample.AddSeconds(60)
            $game.Refresh()
            if (-not $game.HasExited) {
                '{0:F0}`t{1:F1}`t{2:F1}' -f ((Get-Date) - $started).TotalSeconds,
                    ($game.WorkingSet64 / 1MB), ($game.PrivateMemorySize64 / 1MB) |
                    ForEach-Object { $_.Replace('`t', "`t") } | Add-Content $memory
            }
        }
        Start-Sleep -Seconds 2
    }

    # It exits by itself at its limit. If it has not, close the process this
    # script started, by its ID and no other way, and say so.
    $closed = 'itself'
    if (-not $game.HasExited) {
        $closed = 'window'
        $null = $game.CloseMainWindow()
        if (-not $game.WaitForExit(20000)) {
            $closed = 'killed'
            Stop-Process -Id $game.Id -Force
            $null = $game.WaitForExit(20000)
        }
    }
    @(
        "started=$($started.ToString('s'))"
        "ended=$((Get-Date).ToString('s'))"
        "asked_secs=$secs"
        "closed=$closed"
        "exit=$($game.ExitCode)"
    ) | Set-Content (Join-Path $dir 'run.txt')

    & $exe --soak-report $dir | Out-Null
    $code = $LASTEXITCODE
    if (-not (Test-Path $report)) {
        "SOAK REPORT`n`nVERDICT: the game wrote no events, so there is nothing to check. See game.log." |
            Set-Content $report
        $code = 2
    }
    Get-Content $report
    "`nThe report is $report"
    exit $code
}
finally {
    if ('Soak.Power' -as [type]) { [Soak.Power]::SetThreadExecutionState($continuous) | Out-Null }
    Pop-Location
}
