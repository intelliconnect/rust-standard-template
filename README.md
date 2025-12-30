## Rust API Template: Developer Environment Setup

To ensure a smooth onboarding experience and consistent development practices, follow the instructions below to set up your Rust development environment. This guide lists the essential tools and recommended Visual Studio Code (VS Code) extensions for working with this standardized Rust API template.

---

**1. Install Rust Toolchain**

- **Rustup**: The official Rust toolchain installer.
  Install by running:
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- This will install `rustc` (the compiler), `cargo` (the package manager), and other core tools.

---

## Recommended VS Code Extensions for Rust Development

Below is a table of recommended Visual Studio Code extensions for your Rust API project, each with a direct link to its Marketplace page for easy installation:

| Extension Name                | Marketplace Link                                                                                                           | Purpose                                                                         |
| ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| Rust Analyzer                 | [rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)                     | Core Rust language support: code navigation, completion, diagnostics, and more. |
| Dependi                       | [fill-labs.dependi](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi)                                 | Dependency management and visualization.                                        |
| Dotenv (mikestead)            | [mikestead.dotenv](https://marketplace.visualstudio.com/items?itemName=mikestead.dotenv)                                   | Syntax highlighting and support for `.env` files[4].                            |
| Dotenv Official (with Vault)  | [dotenv.dotenv-vscode](https://marketplace.visualstudio.com/items?itemName=dotenv.dotenv-vscode)                           | Advanced `.env` file management and syncing[5].                                 |
| Even Better TOML              | [tamasfe.even-better-toml](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)                   | TOML file syntax highlighting and validation[6].                                |
| GitHub Actions                | [GitHub.vscode-github-actions](https://marketplace.visualstudio.com/items?itemName=GitHub.vscode-github-actions)           | View/manage GitHub Actions and build status.                                    |
| GitHub Copilot                | [GitHub.copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot)                                       | AI code completion and suggestions.                                             |
| GitHub Copilot Chat           | [GitHub.copilot-chat](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot-chat)                             | Chat-based prompts AI coding assistant.                                         |
| GitHub Pull Requests & Issues | [GitHub.vscode-pull-request-github](https://marketplace.visualstudio.com/items?itemName=GitHub.vscode-pull-request-github) | Manage pull requests and issues from within VS Code without commands.           |
| Codium                        | [Codium.codium](https://marketplace.visualstudio.com/items?itemName=Codium.codium)                                         | AI coding assistant alternative to copilot.                                     |

Install these extensions by searching for their names in the VS Code Extensions Marketplace or by clicking the provided links. This setup will help ensure a robust and productive Rust development environment.

---

**3. Project Conventions**

- Please refer to our [Rust API Conventions Document](https://intelliconnectq.sharepoint.com/:w:/r/sites/TechnologyTeam/_layouts/15/Doc2.aspx?action=edit&sourcedoc=%7B688095ad-7aec-46c5-b894-6c6cfce210f2%7D&wdOrigin=TEAMS-MAGLEV.teamsSdk_ns.rwc&wdExp=TEAMS-TREATMENT&wdhostclicktime=1696314299612&web=1) for standardized API design, naming, and best practices.

---

## 4. Git Hooks Setup and Code Quality Checks

### Quick Setup

Run this once after cloning:

```sh
.\git_hooks_setup.ps1
```

Done. Git will now automatically check your code before each commit.

### What Gets Checked

- **Branch names** - Must follow `<type>/<name>` format (e.g., `feature/add-login`)
- **Commit messages** - Must follow `<type>: description` format (e.g., `feat: add login`)
- **Code quality** - Runs `cargo clippy` to catch Rust mistakes
- **File names** - No weird Unicode characters
- **Whitespace** - No trailing spaces

### Branch Types

`feature/` `bugfix/` `hotfix/` `release/` `add/` `refactor/`

**Example:**

```sh
git checkout -b feature/add-expense-tracking
```

### Commit Types

`feat` `fix` `chore` `docs` `style` `refactor` `test` `perf` `ci`

**Example:**

```sh
git commit -m "feat: add two-factor authentication"
```

### Troubleshooting

**Hooks not running?**

```sh
git config core.hooksPath
```

Should show `git-hooks`. If not, run setup script again.

**"Permission denied" error?**

```powershell
git update-index --chmod=+x .scripts/check-branch-name.sh
git update-index --chmod=+x .scripts/check-commit-msg.sh
git update-index --chmod=+x git-hooks/pre-commit
```

**Need to skip checks?**

```sh
git commit --no-verify -m "message"
```

(Don't make this a habit!)

---

### Code Formatting

Format your code regularly:

```sh
cargo fmt
```
---

**5. Quick Start**

1. Clone the repository and open it in VS Code.
2. Ensure all recommended extensions are installed.
3. Build and run the project:
   ```sh
   cargo build
   cargo run
   ```
4. Use `cargo clippy` and `cargo fmt` regularly to maintain code quality.

---

**6. Additional Resources**

- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

By following these steps and using the listed tools, you will have a robust and productive Rust development environment tailored for standardized API projects.

## Disclaimer:

This project and its accompanying installation documentation are provided "as is" without any warranties or guarantees of any kind, either express or implied. While we have made every effort to ensure the accuracy and functionality of the code and instructions provided, we cannot guarantee that this solution will work in all environments or configurations. Use of this project is at your own risk. We are not responsible for any issues, damages, or data loss that may occur as a result of using this project or following the installation instructions. It is strongly recommended that you thoroughly test the solution in a non-production environment before deploying it to your production systems. Additionally, ensure that your environment meets all prerequisites and that you have the appropriate expertise to manage and troubleshoot any issues that may arise. By using this project, you acknowledge and agree to the terms of this disclaimer.
