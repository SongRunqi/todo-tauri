# Tauri Todo App - 集成 go-todo

这是一个集成了 [go-todo](https://github.com/SongRunqi/go-todo) 的 Tauri 桌面应用，提供现代化的 GUI 界面和强大的 AI 驱动的任务管理功能。

## 功能特性

### 🎨 现代化 GUI
- 美观的渐变色界面
- 响应式设计
- 流畅的用户体验

### 🤖 AI 驱动的任务管理
- **自然语言输入**：直接用中文或英文描述任务，AI 自动解析
- **智能任务解析**：自动提取截止日期、紧急程度和任务详情
- **循环任务支持**：每日、每周、每月、每年的重复任务
- **详细描述生成**：AI 自动生成任务的详细描述和上下文

### 📋 核心功能
- ✅ 添加待办事项（支持自然语言）
- ✅ 标记完成/未完成
- ✅ 删除待办事项
- ✅ 过滤显示（全部/进行中/已完成）
- ✅ 数据持久化
- ✅ 任务计数统计

## 前置要求

### 必需
- **Rust**: 用于编译 Tauri 应用
- **Node.js**: 用于前端依赖管理（可选）
- **DeepSeek API 密钥**: 用于 AI 功能 ([获取 API 密钥](https://platform.deepseek.com/))

### 可选
- 其他 LLM API（OpenAI、Claude 等）也可以通过配置使用

## 安装和运行

### 1. 设置环境变量

创建一个环境变量文件或在终端中设置：

```bash
# 必需：AI API 密钥
export API_KEY="your-deepseek-api-key-here"

# 可选：自定义配置
export TODO_LANG="zh"  # 语言设置（zh=中文，en=英文）
export LLM_BASE_URL="https://api.deepseek.com/chat/completions"  # LLM API 端点
export LLM_MODEL="deepseek-chat"  # 使用的模型
export LOG_LEVEL="info"  # 日志级别
```

### 2. 安装依赖（可选）

如果需要安装 npm 依赖：

```bash
npm install
```

### 3. 开发模式运行

```bash
# 确保已设置 API_KEY
export API_KEY="your-api-key"

# 运行开发模式
npm run tauri dev
```

### 4. 构建生产版本

```bash
npm run tauri build
```

构建完成后，可执行文件将位于 `src-tauri/target/release/` 目录。

## 使用方法

### 添加任务（自然语言）

在输入框中直接用自然语言描述任务，AI 会自动解析：

- "明天下午之前完成项目报告"
- "每周一早上9点团队会议"
- "下周五之前给客户打电话"
- "每天晚上8点运动"

AI 会自动：
- 提取任务名称
- 设置截止日期
- 判断紧急程度
- 生成详细描述
- 识别循环模式

### 管理任务

- **完成任务**：点击复选框
- **删除任务**：点击删除按钮
- **过滤显示**：使用顶部的过滤按钮
- **清除已完成**：点击底部的"清除已完成"按钮

## 项目结构

```
todo-tauri/
├── index.html              # 主 HTML 文件
├── styles.css              # 样式文件
├── main.js                 # 前端 JavaScript
├── package.json            # Node.js 配置
├── src-tauri/              # Tauri 后端
│   ├── src/
│   │   └── main.rs        # Rust 主程序（调用 go-todo）
│   ├── binaries/          # go-todo 二进制文件
│   │   └── todo-x86_64-unknown-linux-gnu
│   ├── Cargo.toml         # Rust 依赖配置
│   ├── tauri.conf.json    # Tauri 配置
│   └── build.rs           # 构建脚本
└── README.md              # 本文件
```

## 技术栈

- **前端**: HTML + CSS + JavaScript
- **桌面框架**: Tauri 1.5
- **后端**: Rust（调用 go-todo）
- **任务管理**: go-todo（Go 语言，AI 驱动）
- **AI**: DeepSeek / OpenAI / 其他兼容的 LLM

## 集成说明

本应用通过 Tauri 的 sidecar 功能集成了 go-todo：

1. **go-todo 二进制文件**作为 Tauri 的外部二进制文件打包
2. **Rust 后端**通过 `std::process::Command` 调用 go-todo 命令
3. **前端**通过 Tauri 的 IPC 机制与 Rust 后端通信
4. **数据流**：前端 ↔ Rust ↔ go-todo ↔ JSON 文件

## 自定义 LLM 提供商

如果想使用其他 LLM（如 OpenAI、Claude）：

```bash
# OpenAI 示例
export API_KEY="your-openai-api-key"
export LLM_BASE_URL="https://api.openai.com/v1/chat/completions"
export LLM_MODEL="gpt-4"

# Anthropic Claude 示例（通过代理）
export API_KEY="your-claude-api-key"
export LLM_BASE_URL="https://your-claude-proxy.com/v1/chat/completions"
```

## 故障排除

### 应用无法启动

1. 检查是否设置了 `API_KEY` 环境变量
2. 确保 go-todo 二进制文件有执行权限
3. 查看终端输出的错误信息

### AI 功能不工作

1. 验证 API 密钥是否正确
2. 检查网络连接
3. 尝试设置不同的 LLM_BASE_URL

### 构建失败

```bash
# 清理并重新构建
cd src-tauri
cargo clean
cd ..
npm run tauri build
```

## 数据存储

- 任务数据存储在 `~/.todo/todos.json`
- 已完成任务存储在 `~/.todo/backup.json`
- 配置文件位于 `~/.todo/config.json`

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 相关链接

- [go-todo 项目](https://github.com/SongRunqi/go-todo)
- [Tauri 文档](https://tauri.app/)
- [DeepSeek API](https://platform.deepseek.com/)
