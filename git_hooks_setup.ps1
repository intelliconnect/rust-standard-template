# setup.ps1
$repoRoot = git rev-parse --show-toplevel
$hookPath = Join-Path $repoRoot "git-hooks"

# Set Git to use versioned hook directory
git config core.hooksPath "$hookPath"

Write-Host "✅ Git is now using hooks from: $hookPath"
Write-Host "💡 Your pre-commit hook is now active."
