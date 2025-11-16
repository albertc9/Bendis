# Bendis 快速入门指南

## 安装 (Installation)

### 1. 构建 Bendis (Build Bendis)

```bash
cd /path/to/bendis
make build
```

或者手动构建：

```bash
cd bendis
cargo build --release
```

### 2. 配置环境 (Setup Environment)

在 `~/.bashrc` 或 `~/.zshrc` 中添加：

```bash
source /path/to/bendis/settings.sh
```

然后重新加载：

```bash
source ~/.bashrc  # 或 source ~/.zshrc
```

### 3. 验证安装 (Verify)

```bash
bendis --version
# 输出: bendis 0.1.0
```

## 使用流程 (Usage Workflow)

### 第一次使用 (First Time)

```bash
# 1. 进入你的硬件项目目录
cd /path/to/your/project

# 2. 初始化 Bendis
bendis init

# 3. 编辑配置文件
vim .bendis/Bender.yml
```

在 `.bendis/Bender.yml` 中添加依赖：

```yaml
package:
  name: my_project
  authors: ["Your Name <you@example.com>"]

dependencies:
  common_cells: { git: "https://github.com/pulp-platform/common_cells.git", version: 1.21.0 }
  axi: { git: "https://github.com/pulp-platform/axi.git", version: 0.39.0 }

sources:
  - rtl/top.sv
  - rtl/module.sv
```

```bash
# 4. 更新依赖
bendis update
```

### 日常使用 (Daily Use)

#### 添加新依赖 (Add Dependency)

```bash
# 编辑 .bendis/Bender.yml，添加新的依赖
vim .bendis/Bender.yml

# 运行更新
bendis update
```

#### 更新依赖版本 (Update Version)

```bash
# 修改 .bendis/Bender.yml 中的版本号
vim .bendis/Bender.yml

# 运行更新
bendis update
```

#### 使用本地覆盖 (Local Override)

编辑 `.bendis/.bender.yml`：

```yaml
overrides:
  common_cells: { path: "../my_custom_common_cells" }
```

然后运行：

```bash
bendis update
```

#### 生成仿真脚本 (Generate Simulation Script)

```bash
# 生成 ModelSim 脚本
bendis script vsim > compile.tcl

# 生成 VCS 脚本
bendis script vcs > compile.sh

# 生成文件列表
bendis script flist > files.f
```

#### 查看依赖关系 (View Dependencies)

```bash
# 列出所有包
bendis packages

# 查看依赖关系图
bendis packages -g
```

#### 其他 Bender 命令 (Other Bender Commands)

```bash
# 所有 Bender 命令都可以通过 bendis 调用
bendis checkout
bendis path common_cells
bendis sources
```

## 文件结构说明 (File Structure)

```
your_project/
├── .bendis/                 # Bendis 工作目录
│   ├── Bender.yml          # 原始配置（你编辑这个）
│   ├── .bender.yml         # 原始覆盖配置（可选）
│   └── Bender.lock         # 生成的锁文件
├── Bender.yml              # 自动生成（从 .bendis/ 复制）
├── .bender.yml             # 自动生成（带转换后的 URL）
├── Bender.lock             # 最终锁文件
├── .bender/                # 依赖下载目录
└── rtl/                    # 你的源代码
```

## 重要提示 (Important Notes)

### ⚠️ 永远编辑 `.bendis/Bender.yml`，不要编辑根目录的 `Bender.yml`
根目录的 `Bender.yml` 和 `.bender.yml` 是自动生成的，会被覆盖！

### ✓ Git 版本控制
建议提交到 git：
- `.bendis/Bender.yml`
- `.bendis/.bender.yml`
- `.bendis/Bender.lock`
- `Bender.yml`
- `.bender.yml`

建议添加到 `.gitignore`：
- `.bender/`
- `Bender.lock`

### 🔄 URL 自动转换
Bendis 会自动将 GitHub URL 转换为 IHEP 内部 URL：

**输入** (`.bendis/Bender.yml`)：
```yaml
dependencies:
  common_cells: { git: "https://github.com/pulp-platform/common_cells.git", version: 1.21.0 }
```

**输出** (`.bender.yml`)：
```yaml
overrides:
  common_cells: { git: "git@code.ihep.ac.cn:heris/heris-platform/common_cells.git", version: 1.21.0 }
```

## 常见问题 (Troubleshooting)

### 问题：bendis: command not found

**解决**：确保已经 source settings.sh

```bash
source /path/to/bendis/settings.sh
```

### 问题：bender: command not found

**解决**：安装 Bender

```bash
curl --proto '=https' --tlsv1.2 https://pulp-platform.github.io/bender/init -sSf | sh
```

### 问题：.bendis directory not found

**解决**：先运行 `bendis init`

```bash
bendis init
```

### 问题：网络连接问题

**检查**：
1. 确保可以访问 `code.ihep.ac.cn`
2. SSH 密钥已配置
3. Git 权限正确

## 完整示例 (Complete Example)

```bash
# 创建新项目
mkdir my_chip && cd my_chip

# 创建源文件目录
mkdir -p rtl

# 初始化 Bendis
bendis init

# 编辑配置
cat > .bendis/Bender.yml << 'EOF'
package:
  name: my_chip
  authors: ["Zhang San <zhangsan@ihep.ac.cn>"]

dependencies:
  common_cells: { git: "https://github.com/pulp-platform/common_cells.git", version: 1.21.0 }
  axi: { git: "https://github.com/pulp-platform/axi.git", version: 0.39.0 }
  register_interface: { git: "https://github.com/pulp-platform/register_interface.git", version: 0.4.0 }

export_include_dirs:
  - rtl/include

sources:
  - rtl/my_chip_pkg.sv
  - rtl/my_chip_top.sv
EOF

# 更新依赖
bendis update

# 生成仿真脚本
bendis script vsim > sim/compile.tcl

# 查看依赖
bendis packages

# 完成！
```

## 获取帮助 (Get Help)

```bash
# 查看帮助
bendis --help

# 查看版本
bendis --version
```

详细文档：
- [USAGE.md](USAGE.md) - 详细使用指南
- [BUILD.md](BUILD.md) - 构建说明
- [README.md](README.md) - 项目说明

## 总结 (Summary)

Bendis 的核心价值：
1. ✅ 自动转换 GitHub URL → IHEP 内部 URL
2. ✅ 智能管理依赖版本
3. ✅ 无缝集成 Bender 所有功能
4. ✅ 团队协作友好，可重现构建

**基本工作流程**：
1. 编辑 `.bendis/Bender.yml`
2. 运行 `bendis update`
3. 使用 `bendis <command>` 执行所有 Bender 操作
