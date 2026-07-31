(() => {
  "use strict";

  const installCommands = {
    windows: { prompt: "PS>", command: "irm https://openlongevity.life/install.ps1 | iex" },
    macos: { prompt: "$", command: "curl -fsSL https://openlongevity.life/install.sh | sh" },
    linux: { prompt: "$", command: "curl -fsSL https://openlongevity.life/install.sh | sh" },
  };

  const copy = {
    zh: {
      metaTitle: "Open Longevity · 让 AI 与科学滋养你的生命之树",
      metaDescription: "本地优先、由科学依据支持的长寿知识与 AI 桌面应用。独立资料库，中英双语，支持 Windows、macOS 与 Linux。",
      brand: "Open Longevity（开源长寿）", navPrinciples: "理念", navProduct: "产品", navEvidence: "证据", navModels: "AI 模型",
      heroLead: "让 AI 与科学，", heroAccent: "滋养你的生命之树",
      heroBody: "富豪花费百万美元享受科技带来的长寿，而 Open Longevity 希望把生命之光同样带给普通家庭",
      heroDetail: "（数据以 Bryan Johnson 的延寿计划为蓝本，融入 AI 与科学依据，让普通人也能拥有富豪级的延寿策略）",
      install: "Install Open Longevity", star: "Star on GitHub", copy: "复制", copied: "已复制", copyCommand: "复制安装命令",
      macosNote: "macOS：首次打开若无反应或提示「已损坏」，在「终端」运行",
      statementEyebrow: "OPEN LONGEVITY / 开放宣言", statement: "科学长寿，不应该是富豪专属。",
      statementBody: "昂贵的私人团队、封闭的数据和难以验证的建议，不该成为通往更长生命的门槛。Open Longevity 希望把知识、证据与工具打开，让每个人都能理解、检查并建立自己的延寿计划。",
      principle1Title: "开放知识", principle1Body: "中英文出厂资料公开组织，让专业信息变得可读、可检索、可继续生长。",
      principle2Title: "可验证证据", principle2Body: "结论始终保留来源、证据边界与适用条件，欢迎追问，而不是要求盲信。",
      principle3Title: "开源工具", principle3Body: "代码与数据结构可以被检查、改进和扩展；模型可替换，个人资料默认属于你。",
      productEyebrow: "三条生长路径", productTitle: "AI + 科学的时代",
      product1Title: "最新科学进展", product1Body: "补剂、运动、饮食与人物案例，以普通人能读懂的方式组织；专业证据仍然随时可追溯。",
      product2Title: "AI 快速收录", product2Body: "把论文链接、摘要或临时想法直接交给 AI，提炼研究对象、结果、局限与来源，再由你决定是否保存。",
      product3Title: "AI 长寿助力", product3Body: "围绕你的资料持续对话。它记得当前上下文，也清楚地区分个人方案、一般信息与尚未证实的推测。",
      interfaceEyebrow: "一间属于你的研究室", interfaceTitle: "人人都能看得懂的界面。",
      interfaceBody: "左侧是知识地图，中间是阅读与证据，右侧是计划与收藏。无需打开开发工具，也无需把私人资料上传到陌生平台。",
      homeCaption: "首页 · 长寿策略地图", homeAlt: "Open Longevity 中文首页与策略地图",
      evidenceEyebrow: "优先级，不是假装精确的分数", evidenceTitle: "先做确定性更高的事。",
      evidenceBody: "Open Longevity 默认参考公开方案与证据成熟度给出起始排序，但保留适用条件与不确定性。你可以按自己的理解调整，也可以让 AI 协助重排。",
      tiers: ["力量训练 · 有氧运动 · 健康饮食", "肌酸 · 可溶性膳食纤维 · Omega-3", "维生素 D3 · 镁 · 维生素 C", "辅酶 Q10 · NAD+ · 亚精胺", "麦角硫因 · PQQ · Ca-AKG"],
      openEyebrow: "选择你的 AI", openTitle1: "模型由你选择，", openTitle2: "AI 真正为你运行。",
      openBody: "OpenAI、Anthropic，或任何兼容接口都可以接入。选择模型、API 地址与本地知识库后，AI 即可参与资料收录、证据理解和持续对话；密钥只保存在当前用户的本地配置中。",
      modelCaption: "模型与知识库 · 配置保存在本地", modelAlt: "Open Longevity 模型与知识库配置界面",
      closing1: "AI 和科学，", closing2: "改变人类命运。", returnTop: "回到生命之树", footerNote: "科学长寿知识与 AI 桌面应用",
    },
    en: {
      metaTitle: "Open Longevity · Let AI and science nurture your Tree of Life",
      metaDescription: "A local-first longevity knowledge and AI desktop app grounded in scientific evidence, with an independent bilingual library for Windows, macOS, and Linux.",
      brand: "Open Longevity", navPrinciples: "Principles", navProduct: "Product", navEvidence: "Evidence", navModels: "AI models",
      heroLead: "Let AI and science", heroAccent: "nurture your Tree of Life",
      heroBody: "The wealthy spend millions on longevity technology. Open Longevity aims to bring that same light of life to ordinary families.",
      heroDetail: "(Built on data from Bryan Johnson's longevity plan, enriched with AI and scientific evidence, so ordinary people can access longevity strategies once reserved for the wealthy.)",
      install: "Install Open Longevity", star: "Star on GitHub", copy: "Copy", copied: "Copied", copyCommand: "Copy the install command",
      macosNote: "macOS: If the app does not open or is reported as damaged, run this in Terminal:",
      statementEyebrow: "OPEN LONGEVITY / AN OPEN MANIFESTO", statement: "Longevity science should not belong only to the wealthy.",
      statementBody: "Expensive private teams, closed data, and advice that cannot be examined should not stand between people and a longer life. Open Longevity opens the knowledge, evidence, and tools so anyone can understand, inspect, and build a plan of their own.",
      principle1Title: "Open knowledge", principle1Body: "Bilingual starter material makes specialist information readable, searchable, and ready to grow.",
      principle2Title: "Traceable evidence", principle2Body: "Every conclusion keeps its sources, boundaries, and conditions, inviting questions instead of blind trust.",
      principle3Title: "Open-source tools", principle3Body: "Code and data structures can be inspected, improved, and extended. Models are replaceable; your data stays yours.",
      productEyebrow: "Three paths of growth", productTitle: "The age of AI + science",
      product1Title: "Latest science", product1Body: "Supplements, movement, nutrition, and public cases are written for clarity, while the underlying evidence remains traceable.",
      product2Title: "AI capture", product2Body: "Give AI a paper, abstract, link, or rough thought. It extracts populations, outcomes, limits, and sources before you decide what to keep.",
      product3Title: "AI dialogue", product3Body: "Keep asking questions against your own library. The assistant separates personal protocols, general information, and unproven hypotheses.",
      interfaceEyebrow: "A research room of your own", interfaceTitle: "An interface anyone can read.",
      interfaceBody: "A knowledge map on the left, reading and evidence in the center, plans and favorites on the right. No developer tool and no need to surrender private notes to an unfamiliar platform.",
      homeCaption: "HOME · LONGEVITY STRATEGY MAP", homeAlt: "Open Longevity home and strategy map",
      evidenceEyebrow: "Priorities, not pretend precision", evidenceTitle: "Start with what is more certain.",
      evidenceBody: "The default order reflects public protocols and evidence maturity while preserving conditions and uncertainty. Adjust it yourself or ask AI to help reorder it.",
      tiers: ["Strength · Aerobic exercise · Healthy diet", "Creatine · Soluble fiber · Omega-3", "Vitamin D3 · Magnesium · Vitamin C", "CoQ10 · NAD+ · Spermidine", "Ergothioneine · PQQ · Ca-AKG"],
      openEyebrow: "Choose your AI", openTitle1: "Choose a model.", openTitle2: "Run AI your way.",
      openBody: "Connect OpenAI, Anthropic, or any compatible endpoint. Once you choose a model, API address, and local library, AI can capture research, interpret evidence, and continue the conversation. Your key stays in local configuration for the current user.",
      modelCaption: "MODEL & LIBRARY · CONFIGURATION STAYS LOCAL", modelAlt: "Open Longevity model and knowledge-library configuration",
      closing1: "AI and science", closing2: "change humanity's destiny.", returnTop: "Return to the Tree of Life", footerNote: "Scientific longevity knowledge and AI desktop app",
    },
  };

  const tierLabels = ["FOUNDATION", "HIGH VALUE", "CONTEXTUAL", "EMERGING", "FRONTIER"];
  let locale = "zh";
  const platformHint = [navigator.userAgentData?.platform, navigator.platform, navigator.userAgent].filter(Boolean).join(" ").toLowerCase();
  let platform = /mac|iphone|ipad|ipod/.test(platformHint) ? "macos" : /win/.test(platformHint) ? "windows" : "linux";
  let copyResetTimer;
  const languageButton = document.querySelector(".language-switch");
  const installButton = document.querySelector(".install-command");
  const commandCode = installButton.querySelector("code");
  const terminalPrompt = installButton.querySelector(".terminal-prompt");
  const copyState = installButton.querySelector(".copy-state");
  const platformNotes = document.querySelectorAll("[data-platform-note]");
  const homePreview = document.querySelector("#home-preview");
  const settingsPreview = document.querySelector("#settings-preview");

  function renderTicker() {
    const items = copy[locale].tiers.map((item, index) => `<span class="signal-item"><strong>T${index + 1}</strong><span>${item}</span><small>${tierLabels[index]}</small><i>✦</i></span>`).join("");
    document.querySelector(".signal-track").innerHTML = items + items;
  }

  function applyLocale(nextLocale) {
    locale = nextLocale;
    const text = copy[locale];
    document.documentElement.lang = locale === "zh" ? "zh-CN" : "en";
    document.title = text.metaTitle;
    document.querySelector('meta[name="description"]').content = text.metaDescription;
    document.querySelector('meta[property="og:title"]').content = text.metaTitle;
    document.querySelector('meta[property="og:description"]').content = text.heroBody;
    document.querySelectorAll("[data-i18n]").forEach((element) => {
      const value = text[element.dataset.i18n];
      if (typeof value === "string") element.textContent = value;
    });
    document.querySelectorAll("[data-tier]").forEach((element) => { element.textContent = text.tiers[Number(element.dataset.tier)]; });
    const languageLabels = languageButton.querySelectorAll("span");
    languageLabels[0].classList.toggle("active", locale === "zh");
    languageLabels[1].classList.toggle("active", locale === "en");
    languageButton.setAttribute("aria-label", locale === "zh" ? "Switch to English" : "切换至中文");
    document.querySelector(".site-header nav").setAttribute("aria-label", locale === "zh" ? "主要导航" : "Primary navigation");
    installButton.setAttribute("aria-label", text.copyCommand);
    homePreview.src = `./product-ui/home-${locale}.png`;
    homePreview.alt = text.homeAlt;
    settingsPreview.src = `./product-ui/settings-${locale}.png`;
    settingsPreview.alt = text.modelAlt;
    copyState.textContent = text.copy;
    renderTicker();
  }

  function applyPlatform(nextPlatform) {
    platform = nextPlatform;
    document.querySelectorAll("[data-platform]").forEach((button) => button.classList.toggle("active", button.dataset.platform === platform));
    terminalPrompt.textContent = installCommands[platform].prompt;
    commandCode.textContent = installCommands[platform].command;
    copyState.textContent = copy[locale].copy;
    platformNotes.forEach((note) => { note.hidden = note.dataset.platformNote !== platform; });
  }

  async function copyInstallCommand() {
    const command = installCommands[platform].command;
    try {
      await navigator.clipboard.writeText(command);
    } catch {
      const field = document.createElement("textarea");
      field.value = command;
      field.setAttribute("readonly", "");
      field.style.position = "fixed";
      field.style.opacity = "0";
      document.body.appendChild(field);
      field.select();
      document.execCommand("copy");
      field.remove();
    }
    copyState.textContent = copy[locale].copied;
    copyState.classList.add("is-copied");
    window.clearTimeout(copyResetTimer);
    copyResetTimer = window.setTimeout(() => {
      copyState.textContent = copy[locale].copy;
      copyState.classList.remove("is-copied");
    }, 2200);
  }

  async function loadVersion() {
    try {
      const response = await fetch(`./version.json?time=${Date.now()}`, { cache: "no-store" });
      if (!response.ok) return;
      const data = await response.json();
      if (!/^\d+\.\d+\.\d+$/.test(data.version)) return;
      document.querySelectorAll(".release-version").forEach((element) => { element.textContent = `v${data.version}`; });
    } catch { /* Keep the HTML fallback version when offline. */ }
  }

  languageButton.addEventListener("click", () => applyLocale(locale === "zh" ? "en" : "zh"));
  document.querySelectorAll("[data-platform]").forEach((button) => button.addEventListener("click", () => applyPlatform(button.dataset.platform)));
  installButton.addEventListener("click", copyInstallCommand);
  applyLocale(locale);
  applyPlatform(platform);
  loadVersion();
})();
