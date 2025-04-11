# fuckmit

AI-powered git commit message generator implemented in Rust.

## Features

- Generate commit messages using AI based on your staged changes
- Support for multiple AI providers (OpenAI, Anthropic, Qwen)
- Customizable prompts for generating commit messages
- Exclude specific files from the diff (e.g., package-lock.json)
- Dry-run mode to preview commit messages without creating a commit

## Installation

### From Source

If you have already installed the Rust toolchain (including `cargo`), you can directly use the following command to install from the GitHub repository:

```bash
cargo install --locked --git https://github.com/mingeme/fuckmit
```

Or manually clone and build:

```bash
# Clone the repository
git clone https://github.com/mingeme/fuckmit.git
cd fuckmit

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

## Usage

```bash
# Show help
fuckmit --help

# Generate a commit message and create a commit
fuckmit

# Generate a commit message without creating a commit (dry-run mode)
fuckmit --dry-run
# or
fuckmit -d

# Authenticate with a provider
fuckmit auth add <provider> <apiKey>
# or
fuckmit auth use <provider>
# or set specific provider properties
fuckmit auth set <provider>.<model/api_key/endpoint> <value>

# Manage commit configurations
fuckmit config init        # Create a default commit configuration in current directory
fuckmit config init --global  # Create a default commit configuration in global config directory
fuckmit config show        # Show current commit configuration
fuckmit config list        # List all commit configurations
fuckmit config use <config>  # Set the current commit configuration
```

## Customizing Commit Messages

You can customize the prompts used for generating commit messages by creating a `.fuckmit.yml` or `.fuckmit.yaml` file either in your current working directory or in the global config directory (Linux: `~/.config/fuckmit/` or MacOS: `~/Library/Application Support/fuckmit/` or Windows: `C:\Users\<username>\AppData\Roaming\fuckmit` or `FUCKMIT_CONFIG_DIR` environment variable).

The file should have the following format:

```yaml
prompt:
  system: |
    Your custom system prompt here
  user: |
    Your custom user prompt template here

    {{diff}}

# Optional: exclude specific files from the diff
exclude:
  - "package-lock.json"
  - "**/node_modules/**"
  - "dist/**"
```

The `{{diff}}` placeholder will be replaced with the actual git diff content.

## Zsh Plugin

A Zsh plugin is available to provide convenient aliases and functions for `fuckmit`.

See the [plugin](https://github.com/mingeme/fuckmit-zsh) for more details and other installation methods.

## Development

```bash
# Run tests
cargo test

# Build the project
cargo build

# Run in development mode
cargo run -- --help
```

## License

MIT
