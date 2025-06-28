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

| Extension Name                           | Marketplace Link                                                                                      | Purpose                                                                                 |
|------------------------------------------|------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------|
| Rust Analyzer                           | [rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)         | Core Rust language support: code navigation, completion, diagnostics, and more. |
| Dependi                                 | [fill-labs.dependi](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi)                     | Dependency management and visualization.                                                |
| Dotenv (mikestead)                      | [mikestead.dotenv](https://marketplace.visualstudio.com/items?itemName=mikestead.dotenv)                       | Syntax highlighting and support for `.env` files[4].                                    |
| Dotenv Official (with Vault)             | [dotenv.dotenv-vscode](https://marketplace.visualstudio.com/items?itemName=dotenv.dotenv-vscode)                | Advanced `.env` file management and syncing[5].                                         |
| Even Better TOML                        | [tamasfe.even-better-toml](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)        | TOML file syntax highlighting and validation[6].                                        |
| GitHub Actions                          | [GitHub.vscode-github-actions](https://marketplace.visualstudio.com/items?itemName=GitHub.vscode-github-actions)| View/manage GitHub Actions and build status.                                            |
| GitHub Copilot                          | [GitHub.copilot](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot)                            | AI code completion and suggestions.                                             |
| GitHub Copilot Chat                     | [GitHub.copilot-chat](https://marketplace.visualstudio.com/items?itemName=GitHub.copilot-chat)                  | Chat-based prompts AI coding assistant.                                                         |
| GitHub Pull Requests & Issues            | [GitHub.vscode-pull-request-github](https://marketplace.visualstudio.com/items?itemName=GitHub.vscode-pull-request-github) | Manage pull requests and issues from within VS Code without commands.                                    |
| Codium                                   | [Codium.codium](https://marketplace.visualstudio.com/items?itemName=Codium.codium)                              | AI coding assistant alternative to copilot.                                                    |

Install these extensions by searching for their names in the VS Code Extensions Marketplace or by clicking the provided links. This setup will help ensure a robust and productive Rust development environment.

---

**3. Project Conventions**

- Please refer to our [Rust API Conventions Document](https://intelliconnectq.sharepoint.com/:w:/r/sites/TechnologyTeam/_layouts/15/Doc2.aspx?action=edit&sourcedoc=%7B688095ad-7aec-46c5-b894-6c6cfce210f2%7D&wdOrigin=TEAMS-MAGLEV.teamsSdk_ns.rwc&wdExp=TEAMS-TREATMENT&wdhostclicktime=1696314299612&web=1) for standardized API design, naming, and best practices.

---

## 4. Git Hooks Setup and Code Quality Checks

### Git Hooks Setup

To enable automated pre-commit checks (including running `cargo clippy`), you need to set up Git hooks:

- The hook scripts are stored in the `git-hooks` folder at the root of the repo.
- A setup script is provided to configure Git to use these hooks automatically.

### How to run the setup script

**After cloning the repository, run the setup once:**

- **Via PowerShell:**

```sh
  .\git-hooks\git_hooks_setup.ps1
```

* **Or (Windows only) via one-click executable:**

  * Locate and double-click the executable at:

    ```
    git-hooks\git_hooks_setup.exe
    ```

### What this setup does

* Configures Git to use the `git-hooks` directory as its hooks folder.
* All git hooks added henceforth will be tracked by git

---

### Code Formatting

* **Formatting:**
  Use [rustfmt](https://github.com/rust-lang/rustfmt) to format your code:

  ```sh
  cargo fmt
  ```


```
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
