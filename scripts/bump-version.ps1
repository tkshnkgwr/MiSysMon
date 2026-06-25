param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# セマンティックバージョン (X.Y.Z) のバリデーション
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

$InternalVersion = "$NewVersion.0"

Write-Host "Bumping version to $NewVersion (Internal: $InternalVersion)..."
# BOMなしUTF-8を生成するための .NET エンコーディングオブジェクト
$utf8 = New-Object System.Text.UTF8Encoding -ArgumentList $false

# PowerShell 5.1 のスクリピック文字化けを回避するため、日本語文字列を文字コードから動的生成
$versionLabel  = [char]0x30d0 + [char]0x30fc + [char]0x30b8 + [char]0x30e7 + [char]0x30f3          # "バージョン"
$internalLabel = [char]0x5185 + [char]0x90e8 + $versionLabel                                     # "内部バージョン"
$suitableLabel = [char]0x9069 + [char]0x5408 + $versionLabel                                     # "適合バージョン"

# 1. Cargo.toml の更新 (行配列処理)
$cargoPath = "Cargo.toml"
if (Test-Path $cargoPath) {
    Write-Host "Updating $cargoPath..."
    $lines = [System.IO.File]::ReadAllLines($cargoPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match '^version\s*=\s*"[^"]+"') {
            $lines[$i] = "version = `"$NewVersion`""
            break
        }
    }
    [System.IO.File]::WriteAllLines($cargoPath, $lines, $utf8)
} else {
    Write-Warning "$cargoPath not found."
}

# 2. docs/SPEC.md の更新 (バージョンおよび内部バージョンの動的置換・挿入)
$specPath = "docs/SPEC.md"
if (Test-Path $specPath) {
    Write-Host "Updating $specPath..."
    $lines = [System.IO.File]::ReadAllLines($specPath, $utf8)
    $newLines = [System.Collections.Generic.List[string]]::new()
    
    $hasInternal = $false
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*$internalLabel\*\*") {
            $hasInternal = $true
            break
        }
    }
    
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        if ($line -match "\*\*$versionLabel\*\*") {
            $line = "**$versionLabel**: $NewVersion"
            $newLines.Add($line)
            if (-not $hasInternal) {
                $newLines.Add("**$internalLabel**: $InternalVersion")
            }
        } elseif ($line -match "\*\*$internalLabel\*\*") {
            $line = "**$internalLabel**: $InternalVersion"
            $newLines.Add($line)
        } else {
            $newLines.Add($line)
        }
    }
    [System.IO.File]::WriteAllLines($specPath, $newLines.ToArray(), $utf8)
} else {
    Write-Warning "$specPath not found."
}

# 3. docs/TEST_REPORT.md の更新
$testPath = "docs/TEST_REPORT.md"
if (Test-Path $testPath) {
    Write-Host "Updating $testPath..."
    $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*$suitableLabel\*\*") {
            $lines[$i] = "**$suitableLabel**: mini-system-monitor v$NewVersion"
            break
        }
    }
    [System.IO.File]::WriteAllLines($testPath, $lines, $utf8)
} else {
    Write-Warning "$testPath not found."
}

Write-Host "Version bump completed successfully!"
