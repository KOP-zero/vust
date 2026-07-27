# Vust 🚀

[English](#english) | [中文](#chinese)

---

<a id="english"></a>

## English

A full-stack scaffolding tool built with Rust and Vue, designed to help you quickly bootstrap full-stack applications.

### Features

- ⚡ **Fast Setup**: Create new full-stack projects with a single command
- 🎨 **Interactive Mode**: Use `-i` flag for an interactive project creation experience
- 🔧 **Development Server**: Start both frontend and backend development servers simultaneously
- 📦 **Production Build**: Build both frontend and backend with a single command
- 🗂️ **Customizable Output**: Built artifacts are organized in a default `output` folder

### Installation

#### From Source

```bash
# Clone the repository
git clone https://github.com/KOP-zero/vust.git
cd vust

# Build and install locally
cargo build --release

# The binary will be at ./target/release/vust
# You can run it directly:
./target/release/vust --help

# Or optionally add it to your PATH (Linux/macOS):
sudo cp ./target/release/vust /usr/local/bin/

# Or use cargo run for development:
cargo run -- --help
```

#### Or use directly without installation

```bash
# Run commands directly using cargo
cargo run -- new
cargo run -- new -i
cargo run -- dev
cargo run -- build
```

### Usage

#### Create a new project

```bash
# Quick creation with default settings
vust new

# Or using cargo run
cargo run -- new

# Interactive creation with custom options
vust new -i
# or
cargo run -- new -i
```

#### Development

```bash
# Start both frontend and backend in development mode
vust dev
# or
cargo run -- dev
```

#### Production Build

```bash
# Build both frontend and backend, output to ./output
vust build
# or
cargo run -- build
```

### Project Structure

```
my-project/
├── backend/          # Rust backend (Axum/Actix-web)
├── frontend/         # Vue 3 frontend
├── output/           # Built artifacts (after build command)
└── vust.toml         # Project configuration
```

### Requirements

- Rust 1.70+
- Node.js 16+
- pnpm/npm/yarn

### Development

```bash
# Build the CLI tool
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- new -i
```

### License

MIT

---

<a id="chinese"></a>

## 中文

一个基于 Rust 和 Vue 的全栈脚手架工具，帮助你快速创建全栈应用。

### 特性

- ⚡ **快速创建**：一条命令即可创建新的全栈项目
- 🎨 **交互模式**：使用 `-i` 参数进入交互式项目创建流程
- 🔧 **开发服务器**：同时启动前后端开发服务器
- 📦 **生产构建**：一条命令同时构建前后端
- 🗂️ **可配置输出**：构建产物默认输出到 `output` 文件夹

### 安装

#### 从源码安装

```bash
# 克隆仓库
git clone https://github.com/KOP-zero/vust.git
cd vust

# 构建 release 版本
cargo build --release

# 二进制文件位于 ./target/release/vust
# 可以直接运行：
./target/release/vust --help

# 或者可选择添加到 PATH（Linux/macOS）：
sudo cp ./target/release/vust /usr/local/bin/

# 或者在开发时使用 cargo run：
cargo run -- --help
```

#### 或者不安装直接使用

```bash
# 通过 cargo 直接运行命令
cargo run -- new
cargo run -- new -i
cargo run -- dev
cargo run -- build
```

### 使用指南

#### 创建新项目

```bash
# 快速创建，使用默认配置
vust new

# 或使用 cargo run
cargo run -- new

# 交互式创建，自定义选项
vust new -i
# 或
cargo run -- new -i
```

#### 开发模式

```bash
# 同时启动前后端开发服务器
vust dev
# 或
cargo run -- dev
```

#### 生产构建

```bash
# 构建前后端，输出到 ./output 目录
vust build
# 或
cargo run -- build
```

### 项目结构

```
my-project/
├── backend/          # Rust 后端（Axum/Actix-web）
├── frontend/         # Vue 3 前端
├── output/           # 构建产物（执行 build 后生成）
└── vust.toml         # 项目配置文件
```

### 环境要求

- Rust 1.70+
- Node.js 16+
- pnpm/npm/yarn

### 开发

```bash
# 构建 CLI 工具
cargo build --release

# 运行测试
cargo test

# 启用日志运行
RUST_LOG=debug cargo run -- new -i
```

### 许可证

MIT

---

## Contributing / 贡献

Contributions are welcome! Please feel free to submit a Pull Request.

欢迎贡献！欢迎提交 Pull Request。

---

## Support / 支持

If you encounter any issues, please file an issue on GitHub.

如果遇到任何问题，请在 GitHub 上提交 Issue。

---

**Made with ❤️ using Rust & Vue**
