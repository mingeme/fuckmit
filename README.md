# Fuckmit - AI-Powered Git Commit Message Generator

*Read this in other languages: [简体中文](README.zh-CN.md)*

A command-line tool written in Rust that automatically analyzes code changes and generates standardized Git commit messages by integrating with various AI providers (OpenAI, Azure OpenAI, DeepSeek, Qwen, etc.).

## Installation

### Binary Releases

For Windows, Mac OS (10.12+) or Linux, you can download binary releases [here](https://github.com/mingeme/fuckmit/releases).

### Homebrew

```bash
brew tap mingeme/tap
brew install fuckmit
```

### Install from crates.io

```bash
cargo install fuckmit
```

### Install from Source

If you have the Rust toolchain installed (including cargo), you can install directly from the GitHub repository:

```bash
cargo install --locked --git https://github.com/mingeme/fuckmit
```

Or clone and build manually:

```bash
# Clone repository
git clone https://github.com/mingeme/fuckmit.git
cd fuckmit

# Build project
cargo build --release

# Install binary
cargo install --path .
```

## Supported AI Providers

| Provider     | Status | Supported Models   |
| ------------ | ------ | ------------------ |
| OpenAI       | ✅     | GPT-3.5, GPT-4, etc |
| Azure OpenAI | ✅     | GPT-3.5, GPT-4, etc |
| DeepSeek     | ✅     | DeepSeek Chat      |
| Qwen         | ✅     | Qwen Turbo, etc    |

## Environment Configuration

### Environment Variables

#### OpenAI

```bash
export OPENAI_API_KEY="your-openai-api-key"
export OPENAI_MODEL="gpt-4"  # Optional, defaults to gpt-3.5-turbo
export OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional
```

#### Azure OpenAI

```bash
export AZURE_OPENAI_API_KEY="your-azure-api-key"
export AZURE_OPENAI_ENDPOINT="https://your-resource.openai.azure.com"
export AZURE_OPENAI_DEPLOYMENT="your-deployment-name"
export AZURE_OPENAI_API_VERSION="2024-02-15-preview"  # Optional
```

#### DeepSeek

```bash
export DEEPSEEK_API_KEY="your-deepseek-api-key"
export DEEPSEEK_MODEL="deepseek-chat"  # Optional
export DEEPSEEK_BASE_URL="https://api.deepseek.com/v1"  # Optional
```

#### Qwen

```bash
export QWEN_API_KEY="your-qwen-api-key"
export QWEN_MODEL="qwen-turbo"  # Optional
export QWEN_BASE_URL="https://dashscope.aliyuncs.com/compatible-mode/v1"  # Optional
```

#### Global Settings

```bash
export LLM_MODEL="deepseek/deepseek-chat"  # Required
export LLM_TIMEOUT_SECONDS="30"  # Optional, timeout setting
export LLM_MAX_RETRIES="3"  # Optional, retry count
```

## Usage

### Basic Usage

```bash
# Generate and commit Git commit message
fuckmit

# Only display generated commit message, don't actually commit
fuckmit --dry-run

# Use provider/model format
fuckmit --model openai/gpt-4

# Add custom rules
fuckmit --rules "Use English commit messages"

# Add change context
fuckmit --context "Fixed user login bug"

# Use rules and context together
fuckmit --rules "Use concise descriptions" --context "Refactored database connection logic"

# Custom AI parameters
fuckmit --max-tokens 1000 --temperature 0.5
```

### Command Line Arguments

- `-d, --dry-run`: Only display generated commit message, don't execute commit
- `-m, --model <MODEL>`: Specify AI model or use "provider/model" format
- `-r, --rules <RULES>`: Custom commit message generation rules
- `-c, --context <CONTEXT>`: Provide additional context for changes
- `--max-tokens <NUM>`: Maximum tokens for generated message (default: 8192)
- `--temperature <NUM>`: AI generation temperature parameter, range 0.0-2.0 (default: 0.7)

## License

This project is open source under the MIT License - see the [LICENSE](LICENSE) file for details.
