# AI4L 上游方法记录

## 来源身份

| 字段 | 记录 |
|---|---|
| 项目 | AI4L — AI for Practical Longevity |
| 维护方 | Forever Healthy Foundation |
| 上游仓库 | [forever-healthy/AI4L](https://github.com/forever-healthy/AI4L) |
| 本地快照版本 | `1.2.0` |
| Git 提交 | `bfe6083a1ca04223a245b9a5a0a8be41f021998f` |
| 提交日期 | `2026-07-17T15:01:24+02:00` |
| 本地路径 | `work/references/upstream/AI4L/` |
| 取得日期 | `2026-07-20` |
| 许可 | MIT |

## 已核对文件

| 文件 | 用途 | SHA-256 |
|---|---|---|
| `prompts/AI4L.md` | 404 项证据综述质量检查规范 | `88369ec9678fa5f75d9672396153dd4b40962fed282cd53476879559ef4f8aaa` |
| `.codex/skills/er/SKILL.md` | 创建、审计、修正和重复审计流程 | `583cc4cc3a30599f066b8337117245ec85a66e43cab6eaf3cb73dae5f3fe7628` |
| `docs/Limitations.md` | AI 与审计局限 | `627ca322fd2f3f6344887dae09a75ab83bea442a0473e6432afc90f5e59b38e3` |
| `LICENSE` | MIT 许可原文 | `ba7cefd3b80d4324725f76f9e723d3c0a7a100e44109b49a5849968799d0cada` |

## 在 Open Longevity 中的角色

AI4L 是**研究方法来源**，不是 Open Longevity 结论的事实来源，也不是需要原样复制的网页模板。

Open Longevity 吸收：

- 创建 → 独立审计 → 修正 → 再审计的循环；
- 链接、标题、标识符和链接语义的逐项核验；
- 获益、风险、影响因素、相互作用、监测和在研试验的完整性检查；
- 审计者与作者分离，减少同一上下文的确认偏误；
- 把未知、冲突、利益关系和失败终点保留在正文中。

Open Longevity 不直接吸收：

- 将专家、商业数据库或品牌当作疗效结论来源；
- 未被指南支持的“功能医学最优范围”；
- 默认列出品牌或购买建议；
- 为追求检查表满分而牺牲普通读者可读性；
- 把 `100% pass` 解释成医学事实已经得到保证。

具体取舍见 [AI4L 本地化方法](../methods/ai4l-adaptation.md)。

## 许可与归属

AI4L 采用 MIT 许可。Open Longevity 的本地化方法是重新组织后的衍生检查框架，保留上游项目名称、链接、版本与许可记录。MIT 许可原文见 [AI4L-MIT.md](../licenses/AI4L-MIT.md)。

## 更新规则

上游更新时不覆盖旧记录：

1. 获取新版本并记录新提交；
2. 比较 `prompts/AI4L.md`、技能文件和局限性说明；
3. 只把适合 Open Longevity 来源政策的变化合并到本地框架；
4. 在研究日志中说明新增、拒绝和改变的条目；
5. 已有 dossier 不因上游模板变化自动改变 Tier。
