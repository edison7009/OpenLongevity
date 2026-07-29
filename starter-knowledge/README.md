# Open Longevity 出厂知识库

这是 Open Longevity 首次启动时创建的独立、本地优先知识库。出厂内容提供相对完整的阅读与 AI 上下文基础，同时不包含开发者或任何真实用户的健康参数。

目录约定：

- `catalog/`：长寿策略与内容索引；
- `dossiers/`：运动、饮食、补充剂等策略档案；
- `cases/`：公开人物与方案案例；
- `stories/`：地区、文化与历史中的长寿轶事；新增 Markdown 会被自动发现；
- `papers/`：论文记录；
- `sources/`：来源登记；
- `products/`：产品与品牌质量记录；
- `audits/`：档案审计；
- `methods/`：研究与证据整理方法；
- `topics/`：跨策略专题；
- `research-log/`：研究过程记录；
- `inbox/`：快速收录的待整理内容；
- `profile/`：用户主动填写的个人背景；
- `plans/`：用户自己的当前方案；
- `records/`：检测、饮食和训练记录。

出厂库包含策略档案、人物案例、论文、来源、产品质量记录、研究方法和必要的研究日志。`profile/`、`plans/` 与 `records/` 只提供空白模板；年龄、疾病、用药、剂量、化验结果、饮食和训练记录均由用户在本机主动填写。用户可自由修改或删除全部资料。

## 文章之间的站内链接

正文中出现其他文章的完整中英文标题时，Open Longevity 会自动把首次出现的标题显示为站内链接。也可以在 Markdown 中明确指定目标：

- `[力量训练](#/supplement/strength-training)`
- `[Bryan Johnson](#/person/bryan-johnson)`
- `[日本冲绳的长寿文化](#/story/okinawa-longevity)`

链接末尾使用相应文章 frontmatter 中的 `id`。站内链接只在 Open Longevity 内切换文章；外部参考网站仍由系统默认浏览器打开。
