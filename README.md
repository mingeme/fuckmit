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
fuckmit auth set <provider>.<model/apiKey/endpoint> <value>

# Manage commit configurations
fuckmit prompt init        # Create a default commit configuration in current directory
fuckmit prompt init --global  # Create a default commit configuration in global config directory
fuckmit prompt show        # Show current commit configuration
```

## Customizing Commit Messages

You can customize the prompts used for generating commit messages by creating a `.fuckmit.yml` or `.fuckmit.yaml` file either in your current working directory or in the global config directory (`~/.config/fuckmit/`).

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
