# Git Hooks Setup

This folder contains the shared Git hooks and setup scripts for this repository.

## Files

- `pre-commit`
  The actual Git pre-commit hook script. Runs custom defined rules.
  **Must have LF line endings and no file extension.**

- `git_hooks_setup.ps1`
  PowerShell script to configure Git to use this folder as the hooks directory by setting `core.hooksPath`.
  Also validates the existence and content of the `pre-commit` hook.

- `git_hooks_setup.exe`
  Executable version of `git_hooks_setup.ps1` created with `ps2exe`. Provides a one-click setup option for Windows users.

## Tools Used

- [`ps2exe`](https://github.com/MScholtes/PS2EXE) PowerShell module
  Converts `git_hooks_setup.ps1` into `git_hooks_setup.exe`.

## Command to build the executable

Run this command from the repository root or inside the `git-hooks` folder:

```powershell
Invoke-ps2exe .\git-hooks\git_hooks_setup.ps1 .\git-hooks\git_hooks_setup.exe
````

## Setup Instructions for Developers

1. After cloning the repository, run the setup script once to enable Git hooks:

* Using PowerShell:

  ```powershell
  .\git-hooks\git_hooks_setup.ps1
  ```

* Or double-click the executable (Windows only):

  ```
  git-hooks\git_hooks_setup.exe
  ```

2. This will configure Git to use the hooks in the `git-hooks` folder automatically.
