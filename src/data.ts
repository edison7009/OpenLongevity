import type { LibrarySnapshot, Person, Story, Supplement } from './types';

export const fallbackSupplements: Supplement[] = [
  {
    id: 'strength-training',
    nameZh: '力量训练',
    nameEn: 'Strength training',
    category: '运动',
    tier: 'T1',
    summary: '维持力量、肌肉、骨骼负荷和老年独立生活能力。',
    filePath: 'dossiers/strength-training.md',
  },
  {
    id: 'aerobic-exercise',
    nameZh: '有氧运动',
    nameEn: 'Aerobic exercise',
    category: '运动',
    tier: 'T1',
    summary: '支持心肺适能、代谢健康和长期活动能力。',
    filePath: 'dossiers/aerobic-exercise.md',
  },
  {
    id: 'healthy-diet',
    nameZh: '健康饮食',
    nameEn: 'Healthy diet',
    category: '饮食',
    tier: 'T1',
    summary: '以可长期坚持的整体饮食模式支持健康衰老。',
    filePath: 'dossiers/healthy-diet.md',
  },
  {
    id: 'quality-protein',
    nameZh: '优质蛋白质',
    nameEn: 'Quality protein',
    category: '饮食',
    tier: 'T1',
    summary: '满足蛋白质需要，并与力量训练共同维护肌肉。',
    filePath: 'dossiers/quality-protein.md',
  },
  {
    id: 'creatine',
    nameZh: '肌酸',
    nameEn: 'Creatine',
    category: '运动营养',
    tier: 'T2',
    summary: '配合阻力训练，支持力量、瘦体重与训练适应。',
    filePath: 'dossiers/creatine.md',
  },
  {
    id: 'soluble-fiber',
    nameZh: '可溶性膳食纤维',
    nameEn: 'Soluble fiber',
    category: '肠道',
    tier: 'T2',
    summary: '按车前子、β-葡聚糖、菊粉与 GOS 等原料分别理解。',
    filePath: 'dossiers/soluble-fiber.md',
  },
  {
    id: 'omega3',
    nameZh: 'DHA / EPA',
    nameEn: 'DHA / EPA',
    category: '脂肪酸',
    tier: 'T2',
    summary: '围绕饮食摄入、心血管风险与产品纯度进行个体化判断。',
    filePath: 'dossiers/omega3.md',
  },
  {
    id: 'vitamin-d3',
    nameZh: '维生素 D3',
    nameEn: 'Vitamin D3',
    category: '维生素',
    tier: 'T3',
    summary: '结合检测水平、日照、饮食与特殊风险进行调整。',
    filePath: 'dossiers/vitamin-d3.md',
  },
  {
    id: 'magnesium',
    nameZh: '镁',
    nameEn: 'Magnesium',
    category: '矿物质',
    tier: 'T3',
    summary: '关注元素镁剂量、剂型、肾功能与药物相互作用。',
    filePath: 'dossiers/magnesium.md',
  },
  {
    id: 'vitamin-c',
    nameZh: '维生素 C',
    nameEn: 'Vitamin C',
    category: '维生素',
    tier: 'T3',
    summary: '优先从饮食来源、营养需要和缺乏风险理解。',
    filePath: 'dossiers/vitamin-c.md',
  },
  {
    id: 'coq10',
    nameZh: '辅酶 Q10',
    nameEn: 'CoQ10',
    category: '线粒体',
    tier: 'T4',
    summary: '结合健康状态、疾病情境和他汀使用情况拆分判断。',
    filePath: 'dossiers/coq10.md',
  },
  {
    id: 'nmn',
    nameZh: 'NAD+',
    nameEn: 'NAD+',
    category: 'NAD 相关',
    tier: 'T4',
    summary: '区分 NAD+、NADH、NMN、NR 及其生物标志物与人体结局。',
    filePath: 'dossiers/nmn.md',
  },
  {
    id: 'spermidine',
    nameZh: '亚精胺',
    nameEn: 'Spermidine',
    category: '细胞稳态',
    tier: 'T4',
    summary: '区分食物来源、提取物和不同人群的观察结果。',
    filePath: 'dossiers/spermidine.md',
  },
  {
    id: 'ergothioneine',
    nameZh: '麦角硫因',
    nameEn: 'Ergothioneine',
    category: '抗氧化',
    tier: 'T5',
    summary: '以饮食来源、人体标志物和正在发展的研究为主线。',
    filePath: 'dossiers/ergothioneine.md',
  },
  {
    id: 'pqq',
    nameZh: 'PQQ',
    nameEn: 'PQQ',
    category: '线粒体',
    tier: 'T5',
    summary: '追踪认知、疲劳与线粒体方向的小规模人体研究。',
    filePath: 'dossiers/pqq.md',
  },
  {
    id: 'ca-akg',
    nameZh: 'Ca-AKG',
    nameEn: 'Ca-AKG',
    category: '代谢',
    tier: 'T5',
    summary: '区分机制研究、复方产品研究与可迁移的人体结论。',
    filePath: 'dossiers/ca-akg.md',
  },
  {
    id: 'partial-reprogramming',
    nameZh: '部分细胞重编程（山中因子）',
    nameEn: 'Partial cellular reprogramming',
    category: '前沿生物技术',
    tier: 'T5',
    summary: '追踪 OSK、山中因子与细胞年轻化研究，目前不可自行实践。',
    filePath: 'dossiers/partial-reprogramming.md',
  },
];

export const fallbackPeople: Person[] = [
  {
    id: 'bryan-johnson',
    name: 'Bryan Johnson',
    nameZh: '布莱恩·约翰逊',
    summary: '高强度检测、补剂与医疗干预公开方案。',
    filePath: 'cases/bryan-johnson-daily.md',
    accent: '#dce8fb',
  },
  {
    id: 'peter-attia',
    name: 'Peter Attia',
    nameZh: '彼得·阿提亚',
    summary: '以运动、代谢健康和风险管理为核心的延寿医学方案。',
    filePath: 'cases/peter-attia-protocol.md',
    accent: '#e1eee8',
  },
  {
    id: 'andrew-huberman',
    name: 'Andrew Huberman',
    nameZh: '安德鲁·休伯曼',
    summary: '零成本工具优先，并按目标拆分补剂与生活方式。',
    filePath: 'cases/andrew-huberman-protocol.md',
    accent: '#eee8da',
  },
  {
    id: 'chuando-tan',
    name: 'Chuando Tan',
    nameZh: '陈传多',
    summary: '长期力量训练、简单饮食与体态管理案例。',
    filePath: 'cases/chuando-tan.md',
    accent: '#eadff1',
  },
  {
    id: 'edson-brandao',
    name: 'Edson Brandão',
    nameZh: '埃德森·布兰当',
    summary: '运动、少加工饮食、皮肤管理与公开商业关系核查。',
    filePath: 'cases/edson-brandao.md',
    accent: '#e4e9f3',
  },
  {
    id: 'leslie-kenny',
    name: 'Leslie Kenny',
    nameZh: '莱士里·肯尼',
    summary: '亚精胺、发酵食物与碎片化活动的公开实践。',
    filePath: 'cases/leslie-kenny.md',
    accent: '#dcebec',
  },
];

export const fallbackStories: Story[] = [
  {
    id: 'okinawa-longevity',
    title: '日本冲绳的延寿文化',
    titleEn: 'Longevity culture in Okinawa, Japan',
    summary: '从传统饮食、日常活动、社会联结与长期百岁老人研究理解冲绳案例。',
    summaryEn: 'A field note on traditional diet, daily movement, social ties, and long-running centenarian research.',
    filePath: 'stories/okinawa-longevity.md',
    accent: '#dcefe8',
  },
];

export const fallbackLibrary: LibrarySnapshot = {
  root: 'Open Longevity / library',
  connected: true,
  supplements: fallbackSupplements,
  people: fallbackPeople,
  stories: fallbackStories,
  noteCount: 84,
};

export const fallbackMarkdown: Record<string, string> = {
  'dossiers/strength-training.md': `# 力量训练

> **30 秒结论**
>
> 力量训练是 Open Longevity 的 T1 基础支柱：它直接训练力量与身体功能，并帮助维护肌肉、骨骼负荷和晚年的独立生活能力。

## 如何开始

- 每周至少安排 2 天覆盖主要肌群的训练；
- 从稳定动作、可控负荷和持续进步开始；
- 同时追踪力量、训练量、恢复和疼痛信号。

## 安全边界

有胸痛、晕厥、未控制的高血压、近期手术或明显关节损伤时，先由合格医疗或运动专业人员评估。`,
  'dossiers/aerobic-exercise.md': `# 有氧运动

> **30 秒结论**
>
> 有氧运动是 T1 基础支柱，重点是持续维护心肺适能、代谢健康和日常活动能力。

## 如何开始

- 先建立可持续的中等强度活动总量；
- 逐步增加时长、频率，再考虑强度；
- 用步行、骑行、游泳等自己能长期执行的形式完成。`,
  'dossiers/healthy-diet.md': `# 健康饮食

> **30 秒结论**
>
> 健康饮食是 Open Longevity 的 T1 基石。重点不是追逐唯一食谱，而是建立能够长期坚持、营养充足并适合个人健康状况的整体饮食模式。

## 核心原则

- 以蔬菜、水果、全谷物、豆类、坚果和其他营养密度高的食物为基础；
- 根据年龄、活动量和健康状况获得足够蛋白质与总能量；
- 优先选择不饱和脂肪，减少反式脂肪、含糖饮料和高度加工食品；
- 结合健康指标与实际执行情况持续调整。

## 安全边界

出现非计划性体重下降、吞咽或咀嚼困难、肾脏疾病或糖尿病用药调整等情况时，应由医生或注册营养师评估。`,
  'dossiers/quality-protein.md': `# 优质蛋白质

> **30 秒结论**
>
> 优质蛋白质是 T1 基础支柱。目标不是追逐单一补剂，而是满足总量、分配和质量需要，并与力量训练共同维护肌肉和功能。

## 实践原则

- 优先通过完整食物获得；
- 根据体重、活动、年龄、总能量和健康状况个体化；
- 肾脏疾病等特殊情况应由医生或注册营养师评估。`,
  'dossiers/creatine.md': `# 肌酸一水合物

> **30 秒结论**
>
> 肌酸像是肌肉的“短时充电宝”：在举重、冲刺等高强度活动时，帮助更快补回可立即使用的能量。成分单一的肌酸一水合物是研究最多的形式。

| 快速判断 | 当前答案 |
|---|---|
| 它是什么 | 人体会合成、也存在于肉和鱼中的含氮化合物 |
| 为什么受关注 | 帮助快速再生 ATP，让训练质量和适应更容易提高 |
| 食物还是补剂 | 肉和鱼可提供；研究最充分的补剂是肌酸一水合物 |
| 当前等级 | **T2：高价值补充策略** |

## 先说人话：它在身体里做什么？

肌肉收缩需要 ATP，但肌肉里随时可用的 ATP 很少。磷酸肌酸系统像一只短时充电宝，可以迅速把“用过的能量货币”重新充回 ATP。

## 安全边界

健康成人的中长期安全资料相对较多。肾病、妊娠或使用可能影响肾功能的药物时，应由医生评估。`,
  'dossiers/partial-reprogramming.md': `# 部分细胞重编程（山中因子）

部分细胞重编程试图短暂启动 OSK 或 OSKM 等因子，让部分年龄相关细胞状态向年轻方向移动，同时保留成熟细胞原有身份。

## 当前判断

- 动物研究已出现组织修复与延寿信号；
- Bryan Johnson 完成的是体外 iPSC 重编程，不是身体返老还童；
- 人体研究仍处于局部、早期安全性探索；
- 细胞身份丢失、异常增殖、肿瘤和递送控制仍是核心风险；
- 当前没有可供普通人自行实践的方案。

## 当前等级

T5：前沿探索。`,
  'stories/okinawa-longevity.md': `# 日本冲绳的延寿文化

> 冲绳案例真正值得记录的，不是某一种“延寿秘诀”，而是饮食、活动、社会关系、文化与遗传背景长期共同作用的观察窗口。

## 为什么冲绳受到关注

冲绳百岁老人研究始于 1975 年，是持续时间很长的百岁老人研究项目。研究者长期记录当地高龄人群的健康、生活方式、心理社会因素与家族背景。

## 可以观察到什么

- 传统饮食以营养密度较高的植物性食物为主，同时包含豆制品、海产品等本地食物；
- 日常生活包含自然活动，而不只依赖集中式锻炼；
- 家庭、邻里与稳定社交网络可能提供长期支持；
- 极高寿命同时受到遗传、时代和生活环境影响。

## 不应如何解读

这是一组人口与文化观察，不能证明照搬冲绳食谱或某个习惯就能延寿。可迁移的是整体模式与长期执行，不是被商业包装后的单一“秘诀”。`,
  'cases/bryan-johnson-daily.md': `# Bryan Johnson 的一天与一周

Bryan Johnson 的公开方案适合用作一个持续更新的观察窗口：它把睡眠、饮食、训练、检测和医疗干预放在同一个迭代系统中。

## 可以迁移的结构

- 先定义可追踪的目标，再选择干预；
- 用固定频率复测，而不是凭短期感受频繁改变；
- 把运动、睡眠和饮食放在补剂之前；
- 记录方案版本、时间和公开来源。

## 需要个体化的部分

处方药、激素、极端剂量和依赖密集检测的干预不能直接复制。`,
};
