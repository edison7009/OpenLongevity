# Open Longevity 架构说明

## 产品边界

Open Longevity 不是一个换皮的代码 Agent。它只暴露长寿知识工作所需的安全能力：

1. 读取用户明确选择的知识库；
2. 检索长寿策略、人物案例、长寿轶事、个人计划与论文笔记；
3. 在用户明确触发时写入 `inbox/`；
4. 将命中的本地笔记作为模型上下文；
5. 后续通过可审计的领域工具访问论文与科学数据库。

应用不会给模型开放任意 Shell，也不会默认允许任意文件写入。

## 技术栈

- Tauri 2：Windows、macOS、Linux 桌面壳与本地权限边界；
- React + TypeScript + Vite：三栏知识阅读和对话界面；
- Rust：路径安全、本地 Markdown 读取、收录写入、轻量检索与模型请求；
- Markdown/CSV：开放、可迁移的产品资料格式。

## 独立资料库

Open Longevity 不读取或绑定开发者的私人 `Life extension` 目录。默认资料库位于各平台的应用数据目录：

```text
Windows  %LOCALAPPDATA%/OpenLongevity/library
macOS    ~/Library/Application Support/OpenLongevity/library
Linux    $XDG_DATA_HOME/OpenLongevity/library
```

首次启动由应用创建目录和入门内容。`starter-knowledge/` 只包含公开的结构示例、用户资料空模板和通用安全边界，不包含开发者的个人方案、检测结果或私人研究记录。

外部 Markdown 资料以后通过显式“导入”流程复制或转换到产品资料库，不会成为产品运行依赖。

正文会把其他策略、人物或轶事的完整标题识别为站内链接；Markdown 作者也可以使用 `#/supplement/{id}`、`#/person/{id}` 或 `#/story/{id}` 明确指定目标。站内跳转保留页面历史，外部参考网址仍交给系统默认浏览器。

## Agent 选择：轻量自研核心，保留 Pi 适配器

首版不直接嵌入 Pi。

Pi 的 `pi-ai` 与 `pi-agent-core` 很适合统一模型调用、工具循环和状态管理；但 Pi 的主要运行时是 TypeScript/Node/Bun，而 Tauri WebView 本身不是 Node 环境。直接嵌入通常需要额外 sidecar、跨平台二进制打包和更大的权限面。Pi 官方说明也明确指出它本身不提供文件系统、进程、网络或凭证的内建权限系统。

因此首版使用小型 Rust 核心，只实现 Open Longevity 必需的能力。接口应保持可替换：

```text
React UI
   │
   ▼
LongevityAgent API
   ├── NativeRuntime（首版：Rust，安全范围小）
   └── PiRuntime（未来：可选 sidecar，用于高级研究任务）
```

当产品需要多步研究计划、多个科学数据库和长任务恢复时，再增加 Pi sidecar 适配器，而不更换 UI 与知识库模型。

参考：[Pi Agent Harness](https://github.com/earendil-works/pi)

## 科学技能：精选接入，不整包内置

OpenScience 提供大量科学技能、专业 Agent、数据库工具，以及分子、结构、基因组和图表的内联渲染。这些能力值得借鉴，但整包嵌入会带来明显的体积、依赖、权限与维护成本。

建议分三层接入：

1. 首版：PubMed/DOI/OpenAlex 元数据收录、来源去重、结构化笔记；
2. 第二阶段：PDB、UniProt、Ensembl、ChEMBL、PubChem 等只读连接器；
3. 第三阶段：3Dmol.js/NGL 等确定性查看器，用真实 PDB/AlphaFold 结构展示蛋白与分子。3D 展示属于查看器能力，不应伪装成“AI 生成的真实 DNA/细胞”。

技能必须有清单、版本、许可证、输入输出 Schema 和网络权限声明。默认只读；会写文件、运行代码或使用外部计算的技能需要单独授权。

参考：[OpenScience](https://github.com/synthetic-sciences/openscience)

## 本地知识与个人记忆

模型上下文优先级：

1. 当前打开的笔记；
2. `profile/about-me.md`、`plans/current-protocol.md`、`records/lab-results.md`；
3. 与问题关键词命中的长寿策略、人物、论文和来源笔记；
4. 模型通用知识。

当前使用内置的零外部依赖“本地知识地图”检索：

- 按当前界面语言扫描并缓存 Markdown，文件大小或修改时间变化时自动重建；
- 综合标题、路径、章节标题和正文词频排序；
- 解析 Markdown 内链，并从高相关笔记扩展一层出边与入边邻居；
- 只截取命中问题的少量段落进入模型上下文，个人资料、个人方案和当前页面保持最高优先级；
- 自动上下文注入与 Agent 的 `search_library` 工具复用同一检索器。

这一设计借鉴 Microsoft GraphRAG 的“知识图 + 原始文本片段”查询方式，以及本地 CodeGraph
工具的“预解析关系图、按需返回少量相关内容”方式，但没有复制或嵌入其运行时。完整 GraphRAG
索引需要额外的 LLM 抽取成本，现阶段不适合只有几十到几百篇 Markdown 的本地桌面应用。
当前方案不调用嵌入 API，不需要 Python、向量数据库或额外 Token。知识库规模进一步增长后，
再评估 SQLite FTS5、可选本地嵌入与重排序。

参考：

- [Microsoft GraphRAG](https://github.com/microsoft/graphrag)
- [LightRAG](https://github.com/HKUDS/LightRAG)
- [CodeGraph](https://github.com/colbymchenry/codegraph)

## 安全与隐私

- 所有读操作限制在用户选择的知识目录；
- 路径 canonicalize 后检查，阻止 `../` 越界；
- AI 收录只写入 `knowledge/inbox/`，且不会覆盖同名文件；
- AI 服务商配置（包括 API Key）以明文 JSON 保存在当前用户的应用数据
  目录 `OpenLongevity/config.json`，不写入仓库或知识库；
- 个人资料只在用户发起模型请求时发送给其配置的模型服务商；
- 医疗相关输出保留交互作用、过敏、妊娠和器官功能等基本安全边界。
