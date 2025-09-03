# Fuckmit - AI 智能 Git 提交信息生成器

一个 Rust 编写的命令行工具，通过集成多种 AI 提供商（OpenAI、Azure OpenAI、DeepSeek、Qwen 等），自动分析代码变更并生成符合规范的 Git 提交信息。

## 安装

### 二进制发布版

适用于 Windows、Mac OS(10.12+) 或 Linux，您可以在 [这里](https://github.com/mingeme/fuckmit/releases) 下载二进制发布版。

### Homebrew

```bash
brew tap mingeme/tap
brew install fuckmit
```

### 从 crates.io 安装

```bash
cargo install fuckmit
```

### 从源码安装

如果您已安装 Rust 工具链（包括 cargo），可以直接使用以下命令从 GitHub 仓库安装：

```bash
cargo install --locked --git https://github.com/mingeme/fuckmit
```

或者手动克隆并构建：

```bash
# 克隆仓库
git clone https://github.com/mingeme/fuckmit.git
cd fuckmit

# 构建项目
cargo build --release

# 安装二进制文件
cargo install --path .
```

## 支持的 AI 提供商

| 提供商       | 状态 | 支持的模型         |
| ------------ | ---- | ------------------ |
| OpenAI       | ✅   | GPT-3.5, GPT-4, 等 |
| Azure OpenAI | ✅   | GPT-3.5, GPT-4, 等 |
| DeepSeek     | ✅   | DeepSeek Chat      |
| Qwen         | ✅   | Qwen Turbo, 等     |

## 环境配置

### 环境变量配置

#### OpenAI

```bash
export OPENAI_API_KEY="your-openai-api-key"
export OPENAI_MODEL="gpt-4"  # 可选，默认为 gpt-3.5-turbo
export OPENAI_BASE_URL="https://api.openai.com/v1"  # 可选
```

#### Azure OpenAI

```bash
export AZURE_OPENAI_API_KEY="your-azure-api-key"
export AZURE_OPENAI_ENDPOINT="https://your-resource.openai.azure.com"
export AZURE_OPENAI_DEPLOYMENT="your-deployment-name"
export AZURE_OPENAI_API_VERSION="2024-02-15-preview"  # 可选
```

#### DeepSeek

```bash
export DEEPSEEK_API_KEY="your-deepseek-api-key"
export DEEPSEEK_MODEL="deepseek-chat"  # 可选
export DEEPSEEK_BASE_URL="https://api.deepseek.com/v1"  # 可选
```

#### Qwen

```bash
export QWEN_API_KEY="your-qwen-api-key"
export QWEN_MODEL="qwen-turbo"  # 可选
export QWEN_BASE_URL="https://dashscope.aliyuncs.com/compatible-mode/v1"  # 可选
```

#### 全局设置

```bash
export LLM_MODEL="deepseek/deepseek-chat"  # 必选
export LLM_TIMEOUT_SECONDS="30"  # 可选，超时设置
export LLM_MAX_RETRIES="3"  # 可选，重试次数
```

## 使用方法

### 基本用法

```bash
# 生成并提交 Git 提交信息
fuckmit

# 仅显示生成的提交信息，不实际提交
fuckmit --dry-run

# 使用提供商/模型格式
fuckmit --model openai/gpt-4

# 添加自定义规则
fuckmit --rules "使用中文提交信息"

# 添加变更上下文
fuckmit --context "修复了用户登录的bug"

# 同时使用规则和上下文
fuckmit --rules "使用简洁的描述" --context "重构了数据库连接逻辑"

# 自定义 AI 参数
fuckmit --max-tokens 1000 --temperature 0.5
```

### 命令行参数

- `-d, --dry-run`: 仅显示生成的提交信息，不执行提交
- `-m, --model <MODEL>`: 指定 AI 模型或使用 "provider/model" 格式
- `-r, --rules <RULES>`: 自定义提交信息生成规则
- `-c, --context <CONTEXT>`: 提供变更的额外上下文信息
- `--max-tokens <NUM>`: 生成消息的最大令牌数（默认：8192）
- `--temperature <NUM>`: AI 生成的温度参数，范围 0.0-2.0（默认：0.7）

## 许可证

本项目基于 MIT 许可证开源 - 详见 [LICENSE](LICENSE) 文件。
