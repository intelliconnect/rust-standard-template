# setup.ps1

# Goes to project root where git is initialized
Set-Location -Path (git rev-parse --show-toplevel)

# Ensure git is available
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Error "❌ Git is not installed or not available in PATH."
    exit 1
}

# Get repo root directory
$repoRoot = git rev-parse --show-toplevel 2>$null
if (-not $repoRoot) {
    Write-Error "❌ This directory is not a Git repository."
    exit 1
}

$hookPath = Join-Path $repoRoot "git-hooks"
$preCommitFile = Join-Path $hookPath "pre-commit"

# Check if pre-commit file exists and is not empty
if (-not (Test-Path $preCommitFile)) {
    Write-Error "❌ pre-commit file not found at: $preCommitFile"
    exit 1
}
if ((Get-Content $preCommitFile | Measure-Object -Line).Lines -eq 0) {
    Write-Error "❌ pre-commit file is empty. Add logic before using this hook."
    exit 1
}

# Set hooksPath
git config core.hooksPath "$hookPath"

Write-Host "`n✅ Git is now using hooks from: $hookPath"
Write-Host "💡 Your git hooks are now active.`n"
