---
locale: en
translation_of: sources/ai4l-upstream.md
---

# AI4L Upstream Method Record

## Source Identity

| Field | Record |
|---|---|
| Project | AI4L — AI for Practical Longevity |
| Maintainer | Forever Healthy Foundation |
| Upstream repository | [forever-healthy/AI4L](https://github.com/forever-healthy/AI4L) |
| Local snapshot version | `1.2.0` |
| Git commit | `bfe6083a1ca04223a245b9a5a0a8be41f021998f` |
| Commit date | `2026-07-17T15:01:24+02:00` |
| Local path | `work/references/upstream/AI4L/` |
| Acquisition date | `2026-07-20` |
| License | MIT |

## Verified Files

| File | Use | SHA-256 |
|---|---|---|
| `prompts/AI4L.md` | 404-item quality-check standard for evidence reviews | `88369ec9678fa5f75d9672396153dd4b40962fed282cd53476879559ef4f8aaa` |
| `.codex/skills/er/SKILL.md` | Creation, audit, correction, and repeat-audit workflow | `583cc4cc3a30599f066b8337117245ec85a66e43cab6eaf3cb73dae5f3fe7628` |
| `docs/Limitations.md` | AI and audit limitations | `627ca322fd2f3f6344887dae09a75ab83bea442a0473e6432afc90f5e59b38e3` |
| `LICENSE` | Original MIT license | `ba7cefd3b80d4324725f76f9e723d3c0a7a100e44109b49a5849968799d0cada` |

## Role in Open Longevity

AI4L is a **source for research methods**, not a factual source for Open Longevity conclusions and not a web template to copy verbatim.

Open Longevity adopts:

- The create → independent audit → correct → re-audit cycle;
- Item-by-item verification of links, titles, identifiers, and link semantics;
- Completeness checks for benefits, risks, effect modifiers, interactions, monitoring, and ongoing trials;
- Separation of auditor and author to reduce confirmation bias from shared context;
- Preservation of unknowns, conflicts, competing interests, and failed endpoints in the body.

Open Longevity does not directly adopt:

- Experts, commercial databases, or brands as sources of efficacy conclusions;
- “Functional-medicine optimal ranges” unsupported by guidelines;
- Brand lists or purchase recommendations by default;
- Sacrificing readability for general readers in pursuit of a perfect checklist score;
- Interpreting `100% pass` as a guarantee of medical fact.

See the [localized AI4L method](../methods/ai4l-adaptation.en.md) for specific decisions.

## License and Attribution

AI4L uses the MIT license. Open Longevity’s localized method is a reorganized derivative checklist framework that preserves the upstream project name, link, version, and license record. See [AI4L-MIT.md](../licenses/AI4L-MIT.md) for the original MIT license.

## Update Rules

Do not overwrite old records when upstream changes:

1. Acquire the new version and record the new commit;
2. Compare `prompts/AI4L.md`, the skill file, and the limitations document;
3. Merge only changes compatible with Open Longevity’s source policy into the local framework;
4. Record added, rejected, and changed items in the research log;
5. Existing dossier Tiers do not change automatically when the upstream template changes.
