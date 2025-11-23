# Linux 环境下运行 Tauri 应用

## 系统依赖

在 Linux (Debian/Ubuntu) 环境下运行 Tauri 需要安装以下系统依赖：

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 运行开发服务器

安装依赖后:

```bash
npm run dev
```

## 仅前端开发

如果只想测试 Vue 前端界面 (不需要 Tauri API):

```bash
npm run dev:web
```

然后访问 http://localhost:1420/
