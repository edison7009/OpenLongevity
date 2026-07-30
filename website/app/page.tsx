"use client";

import { useEffect, useRef, useState } from "react";

type Locale = "zh" | "en";

const githubRepository = "https://github.com/edison7009/OpenLongevity";
const latestRelease = `${githubRepository}/releases/latest`;
const installCommand =
  "irm https://edison7009.github.io/OpenLongevity/install.txt | iex";

const assetPath = (path: string) =>
  `${import.meta.env.BASE_URL}${path.replace(/^\//, "")}`;

const copy = {
  zh: {
    brand: "Open Longevity（开源长寿）",
    metaTitle: "Open Longevity · 让 AI 与科学滋养你的生命之树",
    metaDescription:
      "本地优先、由科学依据支持的长寿知识与 AI 桌面应用。独立资料库，中英双语，支持 Windows、macOS 与 Linux。",
    nav: ["理念", "产品", "证据", "AI 模型"],
    heroLead: "让 AI 与科学，",
    heroAccent: "滋养你的生命之树",
    heroBody:
      "Open Longevity 以 Bryan Johnson 的延寿计划为蓝本，融入 AI 与科学依据，让普通人也能拥有富豪级的延寿策略。",
    install: "Install Open Longevity",
    star: "Star on GitHub",
    copyCommand: "复制 PowerShell 安装命令",
    copied: "已复制",
    copy: "复制",
    specimen: "标本 OL—001",
    treeLabel: "生命之树 / 持续生长的知识",
    statementEyebrow: "OPEN LONGEVITY / 开放宣言",
    statement: "科学长寿，不应该是富豪专属。",
    statementBody:
      "昂贵的私人团队、封闭的数据和难以验证的建议，不该成为通往更长生命的门槛。Open Longevity 希望把知识、证据与工具打开，让每个人都能理解、检查并建立自己的延寿计划。",
    openPrinciples: [
      ["01", "开放知识", "OPEN KNOWLEDGE", "中英文出厂资料公开组织，让专业信息变得可读、可检索、可继续生长。"],
      ["02", "可验证证据", "TRACEABLE EVIDENCE", "结论始终保留来源、证据边界与适用条件，欢迎追问，而不是要求盲信。"],
      ["03", "开源工具", "OPEN SOURCE", "代码与数据结构可以被检查、改进和扩展；模型可替换，个人资料默认属于你。"],
    ],
    productEyebrow: "三条生长路径",
    productTitle: "AI + 科学的时代",
    products: [
      {
        no: "01",
        title: "最新科学进展",
        en: "DISCOVER",
        body: "补剂、运动、饮食与人物案例，以普通人能读懂的方式组织；专业证据仍然随时可追溯。",
      },
      {
        no: "02",
        title: "AI 快速收录",
        en: "CAPTURE",
        body: "把论文链接、摘要或临时想法直接交给 AI，提炼研究对象、结果、局限与来源，再由你决定是否保存。",
      },
      {
        no: "03",
        title: "AI 长寿助力",
        en: "DIALOGUE",
        body: "围绕你的资料持续对话。它记得当前上下文，也清楚地区分个人方案、一般信息与尚未证实的推测。",
      },
    ],
    interfaceEyebrow: "一间属于你的研究室",
    interfaceTitle: "人人都能看得懂的界面。",
    interfaceBody:
      "左侧是知识地图，中间是阅读与证据，右侧是计划与收藏。无需打开开发工具，也无需把私人资料上传到陌生平台。",
    evidenceEyebrow: "优先级，不是假装精确的分数",
    evidenceTitle: "先做确定性更高的事。",
    evidenceBody:
      "Open Longevity 默认参考公开方案与证据成熟度给出起始排序，但保留适用条件与不确定性。你可以按自己的理解调整，也可以让 AI 协助重排。",
    tiers: [
      ["T1", "力量训练 · 有氧运动 · 健康饮食", "FOUNDATION"],
      ["T2", "肌酸 · 可溶性膳食纤维 · Omega-3", "HIGH VALUE"],
      ["T3", "维生素 D3 · 镁 · 维生素 C", "CONTEXTUAL"],
      ["T4", "辅酶 Q10 · NAD+ · 亚精胺", "EMERGING"],
      ["T5", "麦角硫因 · PQQ · Ca-AKG", "FRONTIER"],
    ],
    openEyebrow: "选择你的 AI",
    openTitle: ["模型由你选择，", "AI 真正为你运行。"],
    openBody:
      "OpenAI、DeepSeek、OpenRouter，或任何兼容接口都可以接入。选择模型、API 地址与本地知识库后，AI 即可参与资料收录、证据理解和持续对话；密钥仅保留在本次运行的内存中。",
    modelFeatures: ["OPENAI", "DEEPSEEK", "OPENROUTER", "CUSTOM API", "LOCAL LIBRARY"],
    modelAlt: "Open Longevity 模型与知识库配置界面",
    modelCaption: "模型与知识库 · 密钥仅存本次运行",
    closing: ["AI 和科学，", "改变人类命运。"],
    footerNote: "科学长寿知识与 AI 桌面应用",
  },
  en: {
    brand: "Open Longevity",
    metaTitle: "Open Longevity · Let AI and science nurture your Tree of Life",
    metaDescription:
      "A local-first longevity knowledge and AI desktop app grounded in scientific evidence, with an independent bilingual library for Windows, macOS, and Linux.",
    nav: ["Principles", "Product", "Evidence", "AI models"],
    heroLead: "Let AI and science",
    heroAccent: "nurture your Tree of Life",
    heroBody:
      "Open Longevity takes Bryan Johnson’s longevity plan as a starting blueprint, then adds AI and scientific evidence so ordinary people can access a level of strategy once reserved for the wealthy.",
    install: "Install Open Longevity",
    star: "Star on GitHub",
    copyCommand: "Copy the PowerShell install command",
    copied: "Copied",
    copy: "Copy",
    specimen: "SPECIMEN OL—001",
    treeLabel: "TREE OF LIFE / KNOWLEDGE IN GROWTH",
    statementEyebrow: "OPEN LONGEVITY / AN OPEN MANIFESTO",
    statement: "Longevity science should not belong only to the wealthy.",
    statementBody:
      "Expensive private teams, closed data, and advice that cannot be examined should not stand between people and a longer life. Open Longevity opens the knowledge, evidence, and tools so anyone can understand, inspect, and build a plan of their own.",
    openPrinciples: [
      ["01", "Open knowledge", "OPEN KNOWLEDGE", "Bilingual starter material makes specialist information readable, searchable, and ready to grow."],
      ["02", "Traceable evidence", "TRACEABLE EVIDENCE", "Every conclusion keeps its sources, boundaries, and conditions—inviting questions instead of blind trust."],
      ["03", "Open-source tools", "OPEN SOURCE", "Code and data structures can be inspected, improved, and extended. Models are replaceable; your data stays yours."],
    ],
    productEyebrow: "Three paths of growth",
    productTitle: "The age of AI + science",
    products: [
      {
        no: "01",
        title: "Latest science",
        en: "DISCOVER",
        body: "Supplements, movement, nutrition, and public cases are written for clarity, while the underlying evidence remains traceable.",
      },
      {
        no: "02",
        title: "AI capture",
        en: "CAPTURE",
        body: "Give AI a paper, abstract, link, or rough thought. It extracts populations, outcomes, limits, and sources before you decide what to keep.",
      },
      {
        no: "03",
        title: "AI dialogue",
        en: "UNDERSTAND",
        body: "Keep asking questions against your own library. The assistant separates personal protocols, general information, and unproven hypotheses.",
      },
    ],
    interfaceEyebrow: "A research room of your own",
    interfaceTitle: "An interface anyone can read.",
    interfaceBody:
      "A knowledge map on the left, reading and evidence in the center, plans and favorites on the right. No developer tool and no need to surrender private notes to an unfamiliar platform.",
    evidenceEyebrow: "Priorities, not pretend precision",
    evidenceTitle: "Start with what is more certain.",
    evidenceBody:
      "The default order reflects public protocols and evidence maturity while preserving conditions and uncertainty. Adjust it yourself or ask AI to help reorder it.",
    tiers: [
      ["T1", "Strength · Aerobic exercise · Healthy diet", "FOUNDATION"],
      ["T2", "Creatine · Soluble fiber · Omega-3", "HIGH VALUE"],
      ["T3", "Vitamin D3 · Magnesium · Vitamin C", "CONTEXTUAL"],
      ["T4", "CoQ10 · NAD+ · Spermidine", "EMERGING"],
      ["T5", "Ergothioneine · PQQ · Ca-AKG", "FRONTIER"],
    ],
    openEyebrow: "Choose your AI",
    openTitle: ["Choose a model.", "Run AI your way."],
    openBody:
      "Connect OpenAI, DeepSeek, OpenRouter, or any compatible endpoint. Once you choose a model, API address, and local library, AI can capture research, interpret evidence, and continue the conversation. Your key stays only in memory for the current run.",
    modelFeatures: ["OPENAI", "DEEPSEEK", "OPENROUTER", "CUSTOM API", "LOCAL LIBRARY"],
    modelAlt: "Open Longevity model and knowledge-library configuration",
    modelCaption: "MODEL & LIBRARY · KEYS KEPT IN MEMORY",
    closing: ["AI and science", "change humanity’s destiny."],
    footerNote: "Scientific longevity knowledge and AI desktop app",
  },
} as const;

export default function Home() {
  const [locale, setLocale] = useState<Locale>("zh");
  const [commandCopied, setCommandCopied] = useState(false);
  const copyResetTimer = useRef<number | undefined>(undefined);
  const t = copy[locale];

  const sectionLinks = ["#principles", "#product", "#evidence", "#open-source"];

  useEffect(() => {
    document.documentElement.lang = locale === "zh" ? "zh-CN" : "en";
    document.title = t.metaTitle;

    const setMetaContent = (selector: string, content: string) => {
      document.querySelector(selector)?.setAttribute("content", content);
    };

    setMetaContent('meta[name="description"]', t.metaDescription);
    setMetaContent('meta[property="og:title"]', t.metaTitle);
    setMetaContent('meta[property="og:description"]', t.heroBody);
  }, [locale, t.heroBody, t.metaDescription, t.metaTitle]);

  useEffect(
    () => () => {
      window.clearTimeout(copyResetTimer.current);
    },
    [],
  );

  const copyInstallCommand = async () => {
    try {
      await navigator.clipboard.writeText(installCommand);
    } catch {
      const field = document.createElement("textarea");
      field.value = installCommand;
      field.setAttribute("readonly", "");
      field.style.position = "fixed";
      field.style.opacity = "0";
      document.body.appendChild(field);
      field.select();
      document.execCommand("copy");
      field.remove();
    }

    setCommandCopied(true);
    window.clearTimeout(copyResetTimer.current);
    copyResetTimer.current = window.setTimeout(
      () => setCommandCopied(false),
      2200,
    );
  };

  return (
    <main className="site-shell">
      <header className="site-header">
        <a className="brand" href="#top" aria-label="Open Longevity home">
          <span>{t.brand}</span>
        </a>
        <nav aria-label={locale === "zh" ? "主要导航" : "Primary navigation"}>
          {t.nav.map((label, index) => (
            <a href={sectionLinks[index]} key={label}>
              {label}
            </a>
          ))}
        </nav>
        <button
          className="language-switch"
          type="button"
          onClick={() => setLocale(locale === "zh" ? "en" : "zh")}
          aria-label={locale === "zh" ? "Switch to English" : "切换至中文"}
        >
          <span className={locale === "zh" ? "active" : ""}>中</span>
          <i />
          <span className={locale === "en" ? "active" : ""}>EN</span>
        </button>
      </header>

      <section className="hero" id="top">
        <div className="hero-copy">
          <h1>
            {t.heroLead}
            <em>{t.heroAccent}</em>
          </h1>
          <p className="hero-body">{t.heroBody}</p>
          <div className="hero-actions">
            <a
              className="button button-primary"
              href={latestRelease}
              target="_blank"
              rel="noreferrer"
            >
              {t.install}
              <span className="button-icon" aria-hidden="true">↗</span>
            </a>
            <a
              className="button button-ghost"
              href={githubRepository}
              target="_blank"
              rel="noreferrer"
            >
              {t.star}
              <span className="button-icon star-icon" aria-hidden="true">☆</span>
            </a>
          </div>
          <button
            className="install-command"
            type="button"
            onClick={copyInstallCommand}
            aria-label={t.copyCommand}
          >
            <span className="terminal-prompt" aria-hidden="true">PS&gt;</span>
            <code>{installCommand}</code>
            <span
              className={`copy-state ${commandCopied ? "is-copied" : ""}`}
              aria-live="polite"
            >
              {commandCopied ? t.copied : t.copy}
            </span>
          </button>
        </div>

        <div className="hero-visual" role="img" aria-label={t.treeLabel} />
        <div className="hero-index">01 / THE LIVING INDEX</div>
      </section>

      <div className="signal-strip" aria-hidden="true">
        <div className="signal-track">
          {[0, 1].flatMap((round) =>
            t.tiers.map(([tier, items, label]) => (
              <span className="signal-item" key={`${round}-${tier}`}>
                <strong>{tier}</strong>
                <span>{items}</span>
                <small>{label}</small>
                <i>✦</i>
              </span>
            )),
          )}
        </div>
      </div>

      <section className="manifesto paper-section" id="principles">
        <div className="manifesto-mark" aria-hidden="true">
          <strong>01</strong>
          <span>OPEN / FOR ALL</span>
        </div>
        <div className="manifesto-copy">
          <p className="section-eyebrow">{t.statementEyebrow}</p>
          <h2>{t.statement}</h2>
          <p>{t.statementBody}</p>
        </div>
        <div className="open-principles">
          {t.openPrinciples.map(([number, title, label, body]) => (
            <article className="open-principle" key={number}>
              <div className="open-principle-heading">
                <span>{number}</span>
                <small>{label}</small>
              </div>
              <h3>{title}</h3>
              <p>{body}</p>
            </article>
          ))}
        </div>
      </section>

      <section className="product-section" id="product">
        <div className="section-heading">
          <p className="section-eyebrow">{t.productEyebrow}</p>
          <h2>{t.productTitle}</h2>
        </div>
        <div className="product-grid">
          {t.products.map((product) => (
            <article className="product-card" key={product.no}>
              <div className="card-topline">
                <span>{product.no}</span>
                <span>{product.en}</span>
              </div>
              <div className={`process-mark mark-${product.no}`}>
                <i />
                <i />
                <i />
              </div>
              <h3>{product.title}</h3>
              <p>{product.body}</p>
            </article>
          ))}
        </div>
      </section>

      <section className="interface-section">
        <div className="interface-copy">
          <p className="section-eyebrow">{t.interfaceEyebrow}</p>
          <h2>{t.interfaceTitle}</h2>
          <p>{t.interfaceBody}</p>
          <div className="material-list" aria-label="Key product qualities">
            <span>KNOWLEDGE MAP</span>
            <span>TRACEABLE EVIDENCE</span>
            <span>PLANS & FAVORITES</span>
          </div>
        </div>

        <div className="interface-gallery" aria-label={locale === "zh" ? "主界面预览" : "Main interface preview"}>
          <figure className="product-shot product-shot-home">
            <img
              src={assetPath(
                locale === "zh"
                  ? "/product-ui/home-zh.png"
                  : "/product-ui/home-en.png",
              )}
              alt={locale === "zh" ? "Open Longevity 中文首页与策略地图" : "Open Longevity home and strategy map"}
            />
            <figcaption>
              {locale === "zh" ? "首页 · 长寿策略地图" : "HOME · LONGEVITY STRATEGY MAP"}
            </figcaption>
          </figure>
        </div>
      </section>

      <section className="evidence-section" id="evidence">
        <div className="evidence-intro">
          <p className="section-eyebrow">{t.evidenceEyebrow}</p>
          <h2>{t.evidenceTitle}</h2>
          <p>{t.evidenceBody}</p>
        </div>
        <div className="tier-table">
          {t.tiers.map(([tier, items, label], index) => (
            <div className="tier-row" key={tier}>
              <strong style={{ "--tier": index } as React.CSSProperties}>{tier}</strong>
              <span>{items}</span>
              <small>{label}</small>
            </div>
          ))}
        </div>
      </section>

      <section className="open-section model-section" id="open-source">
        <div className="open-copy">
          <p className="section-eyebrow">{t.openEyebrow}</p>
          <h2>
            {t.openTitle.map((line) => (
              <span key={line}>{line}</span>
            ))}
          </h2>
          <p>{t.openBody}</p>
          <div className="model-features">
            {t.modelFeatures.map((feature) => (
              <span key={feature}>{feature}</span>
            ))}
          </div>
        </div>
        <figure className="model-stage">
          <img
            src={assetPath(
              locale === "zh"
                ? "/product-ui/settings-zh.png"
                : "/product-ui/settings-en.png",
            )}
            alt={t.modelAlt}
          />
          <figcaption>{t.modelCaption}</figcaption>
        </figure>
      </section>

      <section className="closing-section">
        <div className="closing-tree" aria-hidden="true">
          <span>│</span>
          <span>╱╲</span>
          <span>╱┼╲</span>
        </div>
        <p>OPEN LONGEVITY · SCIENTIA VITAE</p>
        <h2>
          {t.closing.map((line) => (
            <span key={line}>{line}</span>
          ))}
        </h2>
        <a href="#top">
          {locale === "zh" ? "回到生命之树" : "Return to the Tree of Life"} ↑
        </a>
      </section>

      <footer>
        <div className="brand footer-brand">
          <span>{t.brand}</span>
        </div>
        <p>{t.footerNote}</p>
        <p>© 2026 · OPEN SOURCE · v0.0.1</p>
      </footer>
    </main>
  );
}
