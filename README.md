# Open Longevity / 科学长寿

Open Longevity 是一个本地优先的科学长寿桌面应用，支持 Windows、macOS 与 Linux。它把个人长寿知识库、快速资料收录和 AI 对话放进一个无需开发工具的垂直工作台。

当前首版包括：

- 中英文界面；
- 中英文成对的出厂资料，缺少译文时自动回退原文；
- 首次启动自动创建产品自己的本地 Markdown 资料库；
- 以“长寿策略”和“人物案例”为核心的三栏阅读界面；
- T1–T5 长寿策略优先级地图；
- 本地 `inbox/` 快速收录；
- OpenAI-compatible 模型配置与基于本地笔记的轻量检索问答；
- API Key 仅保留在当前运行内存中，不写入磁盘。

Open Longevity 不依赖开发者的私人笔记目录。首次启动会在系统应用数据目录创建独立资料库和不含私人内容的入门档案：

- Windows：`%LOCALAPPDATA%\OpenLongevity\library`
- macOS：`~/Library/Application Support/OpenLongevity/library`
- Linux：`$XDG_DATA_HOME/OpenLongevity/library`，通常为 `~/.local/share/OpenLongevity/library`

用户可以从设置中主动选择其他存储目录；外部笔记导入将作为独立功能提供，而不是默认绑定。

## 本地开发

```powershell
npm install
npm run dev
```

启动 Tauri 桌面窗口：

```powershell
npm run tauri:dev
```

生产构建：

```powershell
npm run tauri:build
```

## 跨平台发行

窗口会按平台适配：

- Windows 与 Linux 使用 Open Longevity 自绘标题栏和右侧窗口控制；
- macOS 使用系统原生红黄绿窗口按钮，自绘标题栏只保留右侧设置入口；
- macOS GitHub 构建目前使用 ad-hoc 签名，正式面向普通用户发布时应配置 Apple Developer ID 与公证。

仓库包含 `.github/workflows/release.yml`。推送与应用版本一致的 `v*` 标签后，GitHub Actions 会并行生成：

- Windows x64 安装包；
- Linux x64 安装包；
- macOS Apple Silicon 安装包；
- macOS Intel 安装包。

发布 `0.0.1` 的示例：

```bash
git tag v0.0.1
git push origin v0.0.1
```

工作流会创建对应的 GitHub Release 并上传各平台安装文件。创建标签前，应同步更新 `package.json`、`src-tauri/tauri.conf.json` 和 `src-tauri/Cargo.toml` 中的版本号。

## 验证

```powershell
npm run typecheck
npm run build
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
```

技术取舍与后续演进见 [架构说明](docs/ARCHITECTURE.md)，双语资料维护规则见
[Bilingual starter library](docs/BILINGUAL_LIBRARY.md)。
