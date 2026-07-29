# 7 种补剂购买指南研究日志

日期：2026-07-21

## 起因

用户提出实际需求：肌酸、可溶性膳食纤维、维生素 D3、镁、NMN/NR、麦角硫因、Ca-AKG 这 7 种补剂，现有 dossier 只有科学证据介绍，缺少"买什么品牌、怎么吃、Bryan Johnson 早期用什么牌子"的实操信息。

用户进一步要求：除 Bryan Johnson 外，补充其他名人的使用记录和品牌调研。

## 本轮处理

### 第一版（v1）

1. 检查了 7 个 dossier 中的用量、剂型和安全边界信息；
2. 检查了 Bryan 当前协议（2026-07-20 快照），确认当前全部为 Blueprint 自有产品；
3. 尝试检索 Bryan 早期（2021–2022）使用的第三方品牌；
4. 网络访问受限（Wayback Machine 超时、多数外部站点超时），未能完成早期协议页面的一手归档；
5. 基于已有 dossier 证据、产品质量判级框架和公开报道中的品牌信息，编写了购买与服用实操指南；
6. 所有 Bryan 早期品牌信息标记为 `needs-source-check`。

### 第二版（v2）

1. 检查了现有案例（陈传多、Edson Brandão），确认均无已确认的补剂品牌信息；
2. 新增其他公开人物的使用记录：Peter Attia、Andrew Huberman、David Sinclair、Joe Rogan、Bruce Ames；
3. 每个补剂的品牌对比从 2–3 个扩展到 5–8 个，覆盖不同价位和认证水平；
4. 新增"名人使用总览"表，标注商业关系和来源类型；
5. 新增"品牌快速指南"，按品牌定位和强项分类；
6. 新增中国市场购买渠道说明；
7. 所有名人使用记录按项目规则标注来源类型（本人协议/赞助/媒体转述/needs-source-check）。

## 决策

- 品牌推荐基于产品透明度、认证和剂型，不基于名人使用或品牌名气；
- 名人使用记录单独标注，不进入产品质量或疗效判级；
- Bryan 早期品牌（Thorne、Nordic Naturals 等）来自公开报道，尚未完成一手来源归档；
- Peter Attia、Huberman 与 Thorne/Momentous 有品牌合作关系，已标注 `sponsored`；
- David Sinclair 与 NMN 公司有商业关系，已标注；
- 不改变任何现有 Tier 判级。

## 待完成

- 归档 Bryan 2021–2022 年早期协议页面，完成品牌一手来源核对；
- 归档 Peter Attia《Outlive》和播客中的具体品牌推荐；
- 归档 Andrew Huberman 播客中的补剂协议和品牌合作披露；
- 归档 David Sinclair《Lifespan》和采访中的 NMN 品牌与商业关系；
- 为肌酸、镁、维生素 D3、NMN/NR 建立独立产品质量梯队（P1–P3）；
- 补充中国市场实际价格和可用渠道；
- 可溶性纤维拆分后更新指南。

### 第三轮：产品质量梯队

1. 在线核对 NSF 膳食补充剂认证目录（Thorne 和 Momentous），确认以下产品当前在列：
   - Thorne: Creatine (Powder 5g), Magnesium Bisglycinate (Powder), Magnesium Glycinate (Capsule), Vitamin D/K2 Liquid, Vitamin D Liquid 等；
   - Momentous: Creatine Monohydrate (Powder 5g), Magnesium Bisglycinate (Capsule), Magnesium Threonate (Capsule), Vitamin D3 2000IU (Capsule) 等；
2. 为肌酸、镁、维生素 D3、NMN/NR 四个品类建立独立 P1–P3 产品质量梯队文档；
3. 更新 products.csv 新增 23 条产品记录；
4. 更新 products/index.md 索引。

## 最终决策

- 肌酸 P1：Thorne Creatine、Momentous Creatine Monohydrate（NSF 认证可核对）；
- 镁 P1：Thorne Magnesium Bisglycinate/Glycinate、Momentous Magnesium Threonate/Bisglycinate（NSF 认证可核对）；
- 维生素 D3 P1：Thorne Vitamin D/K2 Liquid、Momentous Vitamin D3 2000 IU（NSF 认证可核对）；
- NR P1：Tru Niagen（人体试验标准原料）；
- NMN 最高 P2：ProHealth NMN Pro、DoNotAge NMN（有 COA 但无产品级独立认证）；
- 所有名人使用记录标注来源类型和商业关系。
