# Settings.sh 使用说明

## 概述

`settings.sh` 是 Bendis 的环境配置脚本，用于设置 PATH 环境变量，使 `bendis` 和 `bender` 命令可用。

## 特性

- ✅ **兼容 bash 和 zsh**
- ✅ **静默加载**：不会干扰 Powerlevel10k instant prompt
- ✅ **自动检测路径**：无论从哪里 source 都能正确工作
- ✅ **智能 PATH 配置**：自动添加 bendis 和 bender 到 PATH

## 安装

### 永久配置（推荐）

#### Zsh 用户

在 `~/.zshrc` 中添加：

```bash
source /home/work1/Works/bendis/settings.sh
```

#### Bash 用户

在 `~/.bashrc` 中添加：

```bash
source /home/work1/Works/bendis/settings.sh
```

### 临时使用

```bash
source /home/work1/Works/bendis/settings.sh
```

## 使用方法

### 查看配置状态

脚本会静默加载（不输出任何信息），要查看配置状态，运行：

```bash
bendis-status
```

输出示例：
```
✓ Bendis environment configured
  BENDIS_ROOT: /home/work1/Works/bendis
  ✓ bendis: /home/work1/Works/bendis/bendis/target/release/bendis
  ✓ version: bendis 0.1.0
  ✓ bender: /home/work1/.cargo/bin/bender

Usage: bendis init    # Initialize project
       bendis update  # Update dependencies
       bendis <cmd>   # Pass through to bender
```

### 启用详细输出

如果你想在每次 source 时看到配置信息（不推荐，会破坏 Powerlevel10k），可以：

```bash
BENDIS_VERBOSE=1 source /home/work1/Works/bendis/settings.sh
```

或在 `~/.zshrc` 中：

```bash
export BENDIS_VERBOSE=1
source /home/work1/Works/bendis/settings.sh
```

## PATH 配置

`settings.sh` 会按以下顺序添加路径到 PATH：

1. **Bendis 二进制**：`${BENDIS_ROOT}/bendis/target/release`
2. **本地编译的 Bender**（如果存在）：`${BENDIS_ROOT}/bender/target/release`
3. **Cargo 安装的工具**：`$HOME/.cargo/bin`

## 兼容性说明

### Powerlevel10k Instant Prompt

如果你使用 Powerlevel10k 的 instant prompt 功能，`settings.sh` 会自动静默加载，不会输出任何信息。这样可以：

- ✅ 避免警告信息
- ✅ 保持快速启动
- ✅ 不会导致提示符跳动

### 检测逻辑

脚本使用以下逻辑决定是否输出：

```bash
if [[ -z "${ZSH_VERSION}" ]] || [[ -o interactive ]] && [[ -z "${POWERLEVEL9K_INSTANT_PROMPT}" ]]; then
    # 输出配置信息
fi
```

这意味着：
- 在 bash 中：正常输出（bash 不使用 instant prompt）
- 在 zsh 中：
  - 如果有 Powerlevel10k instant prompt：静默
  - 如果没有 instant prompt：正常输出
- 设置 `BENDIS_VERBOSE=1`：强制输出

## 验证安装

重新打开终端或运行：

```bash
source ~/.zshrc  # 或 source ~/.bashrc
```

然后验证：

```bash
# 检查 bendis
which bendis
# 输出: /home/work1/Works/bendis/bendis/target/release/bendis

# 检查版本
bendis --version
# 输出: bendis 0.1.0

# 查看配置状态
bendis-status
```

## 故障排除

### 问题：bendis: command not found

**原因**：settings.sh 没有被正确加载或 bendis 没有编译

**解决**：

1. 检查 `~/.zshrc` 或 `~/.bashrc` 中是否有 `source /path/to/bendis/settings.sh`
2. 确保路径正确
3. 重新加载配置：`source ~/.zshrc`
4. 检查是否编译了 bendis：`ls -lh /home/work1/Works/bendis/bendis/target/release/bendis`
5. 如果没有，运行：`cd /home/work1/Works/bendis && make build`

### 问题：Powerlevel10k 警告

**原因**：旧版本的 settings.sh 在加载时输出信息

**解决**：更新 settings.sh 到最新版本（已修复）

### 问题：bender not found

**原因**：bender 没有安装

**解决**：

```bash
# 安装 bender
curl --proto '=https' --tlsv1.2 https://pulp-platform.github.io/bender/init -sSf | sh
```

或使用 cargo 安装：

```bash
cargo install bender
```

### 问题：路径问题

**检查 BENDIS_ROOT**：

```bash
echo $BENDIS_ROOT
# 应该输出: /home/work1/Works/bendis
```

如果不正确，检查 settings.sh 的路径是否正确。

## 高级用法

### 自定义 PATH 顺序

如果你想改变 PATH 的优先级，可以修改 settings.sh 中的顺序。

### 禁用 Cargo bin

如果你不想自动添加 `~/.cargo/bin` 到 PATH，可以注释掉：

```bash
# if [ -d "$HOME/.cargo/bin" ]; then
#     export PATH="$HOME/.cargo/bin:${PATH}"
# fi
```

### 添加其他路径

你可以在 settings.sh 末尾添加其他路径：

```bash
# Add custom tools
export PATH="/path/to/custom/tools:${PATH}"
```

## 卸载

要停止使用 Bendis 环境配置：

1. 从 `~/.zshrc` 或 `~/.bashrc` 中删除 `source` 行
2. 重新打开终端或运行 `source ~/.zshrc`

## 总结

- **静默加载**：不会干扰终端启动
- **智能检测**：自动适配不同的 shell 和环境
- **手动查看**：使用 `bendis-status` 查看配置
- **兼容性好**：支持 bash、zsh 和 Powerlevel10k

完美集成到你的开发环境中！🚀
