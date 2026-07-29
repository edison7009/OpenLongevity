# 前沿长寿候选来源批次

研究日期：`2026-07-20`  
证据截止：`2026-07-20`  
范围：NMN、NR、亚精胺、PQQ、Ca-AKG  
方法：定向证据核对，不声称为完整系统综述。

## 检索方法

### PubMed

围绕候选名称、人体试验、功能结局和衰老结局进行定向检索：

```text
("nicotinamide mononucleotide" OR "nicotinamide riboside") AND
(randomized OR trial OR meta-analysis) AND (aging OR older OR muscle OR cognition)

spermidine AND (randomized OR trial OR systematic review) AND
(older OR cognition OR immune OR aging)

("pyrroloquinoline quinone" OR PQQ) AND
(randomized OR trial OR systematic review) AND human

("calcium alpha-ketoglutarate" OR Ca-AKG OR alpha-ketoglutarate) AND
(trial OR human OR biological age OR lifespan)
```

纳入时优先保留：系统综述与荟萃分析、随机对照试验、阴性主要终点、与市场宣传直接相关的替代终点研究，以及可能改变结论的在研试验。细胞和动物研究仅用于解释机制与研究来源，不用于推导普通人疗效。

### ClinicalTrials.gov

以候选名称检索干预性研究，并保存与当前判断最相关的注册：

- `NCT04691986`：NR 对老年退伍军人的功能与肌肉生理；截至本次快照为 `ACTIVE_NOT_RECRUITING`，结果记录待发布；
- `NCT05706389`：ABLE，1 g 缓释 Ca-AKG、120 人、6 个月，主要结局为 DNA 甲基化年龄变化；截至本次快照为 `ACTIVE_NOT_RECRUITING`；
- `NCT07114536`：Ca-AKG 2 g/日、30 人、12 周，主要结局为 PhenoAge；截至本次快照为 `ACTIVE_NOT_RECRUITING`，注册页未发布结果。

注册状态会改变，不应把“预计完成日期”写成“已有结果”。

## 本地快照

| 内容 | 本地路径 | 字节 | SHA-256 |
|---|---|---:|---|
| 21 条 PubMed XML | `work/references/pubmed/2026-07-20-frontier-longevity-candidates.xml` | 536528 | `6ca604e674a414a5c91cc91fbbc742642e112de367b7ff8ab709b430a6e4d8ac` |
| 10 篇开放全文 XML | `work/references/fulltext/2026-07-20-frontier-longevity-open-fulltext.xml` | 1041844 | `f0fba520f84e3f209619e051428e781cc2fae74fbca0094a807c041e6fe29929` |
| NR 试验注册 | `work/references/clinicaltrials/2026-07-20-NCT04691986.json` | 14791 | `359c77e86cd38981365a2bef7aff35861b3110d1ba5127325a2b04a9c4ccad2f` |
| ABLE Ca-AKG 注册 | `work/references/clinicaltrials/2026-07-20-NCT05706389.json` | 16388 | `88d4e9e11e8f3e6d186c8fee2abfdae6f21b49b8a0413a82e180773381b99e32` |
| 30 人 Ca-AKG 注册 | `work/references/clinicaltrials/2026-07-20-NCT07114536.json` | 18473 | `d22a426cd0f244b7d5351974ff4ba301ef5c4d6c504ddeaae9ce8bec5d02851d` |

## 本轮关键判断

| 候选 | 最有分量的人体信息 | 当前不能推出的结论 |
|---|---|---|
| NMN / NR | NAD 相关指标通常升高；肌肉功能荟萃结果不支持稳定获益，NR 小型 MCI 试验未改善认知 | 不能据此推断延寿、认知保护或普遍改善体能 |
| 亚精胺 | 100 人一年期认知 RCT 主要终点阴性；2026 年 40 人疫苗反应试验出现免疫信号 | 不能由小型特定情境试验推断一般抗衰或延寿 |
| PQQ | 58 人完成的单方认知 RCT 报告多项阳性结果；样本小、结局多且产品相关项已标注 | 线粒体功能、长期认知与健康寿命结局待扩展 |
| Ca-AKG | 人体资料以复方前后比较、横断面关联和甲基化时钟为主；关键 RCT 推进中 | “生物年龄”变化与功能结局结合解读 |

## 使用边界

- 人物方案与商业产品只用于发现候选，不进入疗效判级；
- “有钱人正在用”不等于完成临床验证；
- 研究剂量只记录试验暴露，不自动成为个人用量；
- 对长期安全、药物相互作用、肿瘤情境、妊娠哺乳和复杂疾病资料待补的部分，统一标为待更新。

