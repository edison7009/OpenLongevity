# Open Longevity 出厂知识库

这是 Open Longevity 首次启动时创建的独立、本地优先知识库。出厂内容提供相对完整的阅读与 AI 上下文基础，同时不包含开发者或任何真实用户的健康参数。

目录约定：

- `catalog/`：延寿策略与内容索引；
- `dossiers/`：运动、饮食、补充剂等策略档案；
- `cases/`：公开人物与方案案例；
- `stories/`：地区、文化与历史中的延寿轶事；新增 Markdown 会被自动发现；
- `guides/`：对用户有独立价值的精选科学与购买指南；
- `inbox/`：快速收录的待整理内容；
- `profile/`：用户主动填写的个人背景；
- `plans/`：用户自己的当前方案；
- `records/`：检测、饮食和训练记录。

出厂库包含策略档案、人物案例、延寿轶事和少量精选指南。品牌排名直接位于各补剂详情页，不再另存重复文章。`profile/`、`plans/` 与 `records/` 中的个人内容由用户在本机主动填写。

## 文章之间的站内链接

正文中出现其他文章的完整中英文标题时，Open Longevity 会自动把首次出现的标题显示为站内链接。也可以在 Markdown 中明确指定目标：

- `[力量训练](#/supplement/strength-training)`
- `[Bryan Johnson](#/person/bryan-johnson)`
- `[日本冲绳的延寿文化](#/story/okinawa-longevity)`

链接末尾使用相应文章 frontmatter 中的 `id`。站内链接只在 Open Longevity 内切换文章；外部参考网站仍由系统默认浏览器打开。
