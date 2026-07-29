# 标准研究工作流

## 0. 定义问题

用 PICO 或等价结构明确：

- 人群；
- 干预和剂量；
- 对照；
- 周期；
- 主要临床或功能结局；
- 安全结局。

如果问题只是“这个成分抗衰吗”，先拆成可验证问题。

## 1. 检索本地知识库

先查：

```powershell
rg -n "<中文名>|<英文名>|<别名>" knowledge
```

确认是否已有候选、dossier、论文记录和原始快照。

## 2. 登记候选

如果是新项目：

- 先加入 `catalog/supplements.csv`；
- `evidence_status=candidate`；
- `tier=pending`；
- 记录候选来源和 Bryan 状态；
- 不立即给 Tier。

## 3. 检索外部证据

优先：

1. 指南与政府/专业机构资料；
2. 系统综述和荟萃分析；
3. 关键 RCT；
4. 试验注册和监管信息；
5. 安全性、药物相互作用和产品质量资料。

搜索时记录数据库、日期、检索式和筛选理由。

## 4. 保存原始资料

- PubMed 使用 XML 或稳定摘要记录；
- 开放全文优先保存 XML/HTML/PDF；
- 动态网页保存带日期快照；
- 计算 SHA-256；
- 写入 `sources/source-manifest.md`。

## 5. 创建论文记录

使用 `knowledge/templates/paper-record.md`。至少核对：

- 题目与永久标识；
- 预注册主要终点；
- 样本、剂量、周期；
- 组间效应量和不确定性；
- 不良事件；
- 资金与利益冲突；
- 风险偏倚。

## 6. 创建或更新 dossier

使用 `knowledge/templates/intervention-dossier.md`，综合所有纳入证据，避免逐篇堆叠。

## 7. 独立审计

使用 `knowledge/templates/intervention-audit.md`：

- 审计者从 dossier、论文记录和原始快照重新开始；
- 核对来源身份、链接语义、效应量、阴性结果、风险、相互作用、利益冲突和在研试验；
- 把结果保存到 `knowledge/audits/`；
- 先修正已有来源能够支持的问题；
- 需要新医学结论时返回第 3–6 步，不在审计表中直接补写。

完整方法见 `methods/ai4l-adaptation.md`。`100% pass` 不代表医学结论绝对正确，未知项不得为获得满分而改写。

## 8. 给出 Tier

依据 `methods/evidence-grading.md`。未完成评审的项目保持 `pending`，不要为了填满榜单强行定级。

Tier 与审计状态分开记录：Tier 描述证据位置，审计状态描述档案完整性。

## 9. 同步网页

只有知识库完成后才更新网页。理想状态是由 CSV 和 Markdown 自动生成，而不是复制粘贴。

## 10. 写研究日志

记录：

- 本批次完成内容；
- 关键判断变化；
- 新增或删除的来源；
- 仍不确定的问题；
- 下一步优先队列。
