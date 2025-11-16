# Cargo 版本管理指南

## 概述

Bendis 使用 Cargo 进行版本管理和发布。本文档说明如何管理版本、发布新版本以及安装。

## 版本管理

### 语义化版本（Semantic Versioning）

Bendis 遵循 [语义化版本规范](https://semver.org/)：

```
MAJOR.MINOR.PATCH
```

- **MAJOR**: 不兼容的 API 变更
- **MINOR**: 向下兼容的新功能
- **PATCH**: 向下兼容的问题修复

示例：
- `0.1.0` → `0.1.1`: Bug 修复
- `0.1.1` → `0.2.0`: 新功能
- `0.2.0` → `1.0.0`: 重大变更（API 不兼容）

### 更新版本号

编辑 `bendis/Cargo.toml`：

```toml
[package]
name = "bendis"
version = "0.2.0"  # 更新这里
edition = "2021"
```

或使用命令行工具（需要安装 `cargo-edit`）：

```bash
# 安装 cargo-edit
cargo install cargo-edit

# 更新版本
cd bendis
cargo set-version 0.2.0
```

### 版本发布检查清单

发布新版本前的检查：

```bash
cd bendis

# 1. 运行所有测试
cargo test

# 2. 检查代码
cargo check

# 3. 代码格式化
cargo fmt

# 4. Lint 检查
cargo clippy -- -D warnings

# 5. 构建文档
cargo doc --no-deps

# 6. 试运行发布（不实际发布）
cargo publish --dry-run

# 7. 检查包内容
cargo package --list
```

## 发布到 crates.io（公开发布）

### 前提条件

1. **注册 crates.io 账号**：https://crates.io/
2. **获取 API Token**：https://crates.io/me
3. **登录 Cargo**：
   ```bash
   cargo login <your-token>
   ```

### 发布步骤

```bash
cd bendis

# 1. 确保所有更改已提交
git status

# 2. 更新版本号
# 编辑 Cargo.toml，更新 version 字段

# 3. 更新 CHANGELOG.md
vim CHANGELOG.md

# 4. 提交版本更新
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# 5. 创建 Git tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# 6. 发布到 crates.io
cargo publish

# 7. 推送到远程仓库
git push origin master
git push origin v0.2.0
```

### 发布后验证

```bash
# 等待几分钟后，尝试安装
cargo install bendis

# 检查版本
bendis --version
```

## 发布到私有 Registry

如果你想发布到私有 Cargo registry（如 IHEP 内部）：

### 1. 配置私有 Registry

在 `~/.cargo/config.toml` 中添加：

```toml
[registries.ihep]
index = "https://git.ihep.ac.cn/cargo-registry/index"

[net]
git-fetch-with-cli = true
```

### 2. 修改 Cargo.toml

在 `bendis/Cargo.toml` 中添加：

```toml
[package]
publish = ["ihep"]  # 只允许发布到 ihep registry
```

### 3. 登录私有 Registry

```bash
cargo login --registry=ihep <your-token>
```

### 4. 发布到私有 Registry

```bash
cd bendis
cargo publish --registry=ihep
```

### 5. 从私有 Registry 安装

```bash
cargo install bendis --registry=ihep
```

## 使用 Cargo 安装 Bendis

### 从 crates.io 安装

```bash
# 安装最新版本
cargo install bendis

# 安装特定版本
cargo install bendis --version 0.1.0

# 强制重新安装
cargo install bendis --force
```

### 从 Git 仓库安装

```bash
# 从 GitHub 安装
cargo install --git https://github.com/your-org/bendis

# 从特定分支安装
cargo install --git https://github.com/your-org/bendis --branch develop

# 从特定 tag 安装
cargo install --git https://github.com/your-org/bendis --tag v0.1.0
```

### 从本地路径安装

```bash
# 从本地路径安装
cd /home/work1/Works/bendis/bendis
cargo install --path .

# 安装后的位置
which bendis
# 输出: ~/.cargo/bin/bendis
```

## 版本管理最佳实践

### 1. 保持 CHANGELOG.md 更新

每次发布新版本时，更新 `CHANGELOG.md`：

```markdown
## [0.2.0] - 2025-11-16

### Added
- 新增 bendis-status 函数
- 静默加载模式（兼容 Powerlevel10k）

### Changed
- 改进 settings.sh 的 shell 兼容性

### Fixed
- 修复 zsh 中路径检测问题
```

### 2. 使用 Git Tags

每个版本发布都应创建对应的 Git tag：

```bash
# 创建带注释的 tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# 推送 tag 到远程
git push origin v0.2.0

# 列出所有 tags
git tag -l

# 删除 tag（如果需要）
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0
```

### 3. 自动化版本发布

创建发布脚本 `bendis/scripts/release.sh`：

```bash
#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 0.2.0"
    exit 1
fi

VERSION=$1

echo "Releasing version $VERSION..."

# 1. 更新版本号
cargo set-version $VERSION

# 2. 运行测试
cargo test

# 3. 格式化和检查
cargo fmt
cargo clippy -- -D warnings

# 4. 提交更改
git add Cargo.toml
git commit -m "chore: bump version to $VERSION"

# 5. 创建 tag
git tag -a "v$VERSION" -m "Release version $VERSION"

# 6. 发布到 crates.io
cargo publish

# 7. 推送到远程
git push origin master
git push origin "v$VERSION"

echo "✓ Version $VERSION released successfully!"
```

使用：

```bash
chmod +x bendis/scripts/release.sh
cd bendis
./scripts/release.sh 0.2.0
```

## 依赖管理

### 更新依赖

```bash
cd bendis

# 检查过时的依赖
cargo outdated

# 更新所有依赖到兼容的最新版本
cargo update

# 更新特定依赖
cargo update -p clap
```

### 审计依赖安全性

```bash
# 安装 cargo-audit
cargo install cargo-audit

# 检查安全漏洞
cd bendis
cargo audit
```

## 构建和发布二进制文件

### 跨平台编译

```bash
# 安装交叉编译工具
cargo install cross

# Linux x86_64
cross build --release --target x86_64-unknown-linux-gnu

# Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# macOS
cross build --release --target x86_64-apple-darwin

# Windows
cross build --release --target x86_64-pc-windows-gnu
```

### 创建发布包

```bash
# 构建 release 版本
cargo build --release

# 创建发布目录
mkdir -p releases/bendis-0.1.0-linux-x64

# 复制文件
cp target/release/bendis releases/bendis-0.1.0-linux-x64/
cp README.md releases/bendis-0.1.0-linux-x64/
cp LICENSE-MIT releases/bendis-0.1.0-linux-x64/
cp LICENSE-APACHE releases/bendis-0.1.0-linux-x64/

# 打包
cd releases
tar -czf bendis-0.1.0-linux-x64.tar.gz bendis-0.1.0-linux-x64/

# 生成 SHA256 校验和
sha256sum bendis-0.1.0-linux-x64.tar.gz > bendis-0.1.0-linux-x64.tar.gz.sha256
```

## GitHub Releases 集成

### 使用 GitHub Actions 自动发布

创建 `.github/workflows/release.yml`：

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build
        run: cd bendis && cargo build --release

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: bendis/target/release/bendis
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## 常用命令速查

```bash
# 版本管理
cargo set-version 0.2.0          # 设置版本号
git tag -a v0.2.0 -m "Release"   # 创建 tag

# 发布
cargo publish --dry-run          # 试运行
cargo publish                    # 发布到 crates.io
cargo publish --registry=ihep    # 发布到私有 registry

# 安装
cargo install bendis             # 从 crates.io 安装
cargo install --path .           # 从本地安装
cargo install --force            # 强制重新安装

# 依赖管理
cargo update                     # 更新依赖
cargo outdated                   # 检查过时依赖
cargo audit                      # 安全审计

# 检查
cargo test                       # 运行测试
cargo check                      # 检查代码
cargo clippy                     # Lint 检查
cargo fmt                        # 格式化代码
```

## 总结

使用 Cargo 进行版本管理的优势：

1. ✅ **标准化**：遵循 Rust 生态系统标准
2. ✅ **简单安装**：用户可以用 `cargo install bendis` 一键安装
3. ✅ **版本控制**：语义化版本，清晰的依赖管理
4. ✅ **自动化**：可以集成 CI/CD 自动发布
5. ✅ **私有 Registry**：支持内网部署

现在 Bendis 已经完全准备好使用 Cargo 进行版本管理了！🚀
