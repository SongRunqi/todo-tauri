#!/bin/bash

# Tauri Todo App 开发模式启动脚本

echo "🚀 启动 Tauri Todo App 开发模式..."
echo ""

# 检查是否设置了 API_KEY
if [ -z "$API_KEY" ]; then
    echo "⚠️  警告: 未检测到 API_KEY 环境变量"
    echo "请设置你的 DeepSeek API 密钥:"
    echo "  export API_KEY=\"your-api-key-here\""
    echo ""
    read -p "是否要现在设置? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        read -p "请输入你的 API 密钥: " api_key
        export API_KEY="$api_key"
        echo "✅ API_KEY 已设置"
    else
        echo "⚠️  继续运行，但 AI 功能可能无法使用"
    fi
fi

# 设置默认环境变量
export TODO_LANG="${TODO_LANG:-zh}"
export LOG_LEVEL="${LOG_LEVEL:-info}"

echo ""
echo "📋 环境配置:"
echo "  - 语言: $TODO_LANG"
echo "  - 日志级别: $LOG_LEVEL"
echo "  - API_KEY: ${API_KEY:+已设置}"
echo ""

# 检查 Rust 是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 错误: 未找到 cargo (Rust)"
    echo "请先安装 Rust: https://rustup.rs/"
    exit 1
fi

# 检查 go-todo 二进制文件
if [ ! -f "src-tauri/binaries/todo-x86_64-unknown-linux-gnu" ]; then
    echo "❌ 错误: 未找到 go-todo 二进制文件"
    echo "请确保 src-tauri/binaries/todo-x86_64-unknown-linux-gnu 存在"
    exit 1
fi

echo "🔧 检查依赖..."

# 检查是否需要安装 npm 依赖
if [ -f "package.json" ] && [ ! -d "node_modules" ]; then
    echo "📦 安装 npm 依赖..."
    npm install
fi

echo ""
echo "🎯 启动应用..."
echo ""

# 运行 Tauri 开发模式
npm run tauri dev
