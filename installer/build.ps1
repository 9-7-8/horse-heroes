# Builds the release, stages exactly what is installed, and compiles the
# installer (#190, #192). One command, from anywhere:
#
#     pwsh installer/build.ps1
#
# Leaves target/stage/ (the installed files) and
# target/installer/HorseHeroes-Setup.exe. The version comes from Cargo.toml
# and nowhere else. Nothing is signed: 0.x releases go out unsigned (#189).

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$stage = Join-Path $root 'target\stage'
$out = Join-Path $root 'target\installer'
$repo = 'https://github.com/9-7-8/horse-heroes'

# cargo is not always on the PATH of the shell that runs this.
$cargo = (Get-Command cargo -ErrorAction SilentlyContinue)?.Source
if (-not $cargo) { $cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe' }

$iscc = (Get-Command iscc -ErrorAction SilentlyContinue)?.Source
if (-not $iscc) {
    $iscc = @(
        (Join-Path $env:LOCALAPPDATA 'Programs\Inno Setup 6\ISCC.exe'),
        (Join-Path ${env:ProgramFiles(x86)} 'Inno Setup 6\ISCC.exe')
    ) | Where-Object { Test-Path $_ } | Select-Object -First 1
}
if (-not $iscc) { throw 'Inno Setup 6 is not installed: winget install JRSoftware.InnoSetup' }

Push-Location $root
try {
    $meta = & $cargo metadata --format-version 1 --locked --filter-platform x86_64-pc-windows-msvc |
        ConvertFrom-Json
    if ($LASTEXITCODE) { throw 'cargo metadata failed' }
    $version = ($meta.packages | Where-Object name -eq 'horse_heroes').version
    Write-Host "Horse Heroes $version"

    # The C runtime is linked in, so a machine with no Visual C++
    # redistributable can run the game.
    $env:RUSTFLAGS = '-C target-feature=+crt-static'
    & $cargo build --release --locked
    if ($LASTEXITCODE) { throw 'cargo build failed' }

    if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
    New-Item -ItemType Directory -Force $stage, $out | Out-Null

    # The game's data is compiled into the executable. When an asset is first
    # loaded from disk, it is copied here too.
    Copy-Item 'target\release\horse_heroes.exe' $stage
    Copy-Item 'LICENSE' (Join-Path $stage 'LICENSE.txt')
    Set-Content (Join-Path $stage 'version.txt') $version -NoNewline
    Set-Content (Join-Path $stage 'SOURCE.txt') @"
Horse Heroes $version

Horse Heroes is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version. It comes with no warranty. The licence is in LICENSE.txt.

The source of this version:
    $repo/tree/v$version

The licences of the libraries built into it are in THIRD-PARTY-NOTICES.txt.
"@

    # Third-party notices: every crate in the build, and each distinct
    # licence text found in those crates, printed once.
    $crates = $meta.packages | Where-Object name -ne 'horse_heroes' | Sort-Object name, version
    $texts = [ordered]@{}
    foreach ($crate in $crates) {
        $dir = Split-Path $crate.manifest_path -Parent
        $files = Get-ChildItem $dir -File |
            Where-Object Name -match '^(LICEN[SC]E|COPYING|NOTICE|UNLICENSE|COPYRIGHT)' |
            Sort-Object Name
        foreach ($file in $files) {
            $text = (Get-Content $file.FullName -Raw) -replace "`r`n", "`n"
            if (-not $text) { continue }
            $text = $text.Trim()
            if (-not $texts.Contains($text)) { $texts[$text] = [System.Collections.Generic.List[string]]::new() }
            $texts[$text].Add("$($crate.name) $($crate.version)")
        }
    }
    $rule = '-' * 78
    $notices = [System.Text.StringBuilder]::new()
    [void]$notices.AppendLine("Horse Heroes $version is built from the libraries below, and from the Rust")
    [void]$notices.AppendLine('standard library (MIT or Apache-2.0, https://github.com/rust-lang/rust).')
    [void]$notices.AppendLine('Their licence texts follow the list.')
    [void]$notices.AppendLine()
    foreach ($crate in $crates) {
        [void]$notices.AppendLine("$($crate.name) $($crate.version)  [$($crate.license)]  $($crate.repository)")
    }
    foreach ($text in $texts.Keys) {
        [void]$notices.AppendLine()
        [void]$notices.AppendLine($rule)
        [void]$notices.AppendLine('Applies to: ' + (($texts[$text] | Select-Object -Unique) -join ', '))
        [void]$notices.AppendLine($rule)
        [void]$notices.AppendLine($text)
    }
    Set-Content (Join-Path $stage 'THIRD-PARTY-NOTICES.txt') $notices.ToString()

    & $iscc /Qp "/DAppVersion=$version" "/DStageDir=$stage" "/DOutputDir=$out" 'installer\horse-heroes.iss'
    if ($LASTEXITCODE) { throw 'the installer did not compile' }

    $setup = Join-Path $out 'HorseHeroes-Setup.exe'
    $hash = (Get-FileHash $setup -Algorithm SHA256).Hash.ToLower()
    Set-Content (Join-Path $out 'HorseHeroes-Setup.exe.sha256') "$hash  HorseHeroes-Setup.exe"
    Write-Host "Staged:    $stage"
    Write-Host "Installer: $setup"
    Write-Host "SHA-256:   $hash"
}
finally {
    Pop-Location
}
