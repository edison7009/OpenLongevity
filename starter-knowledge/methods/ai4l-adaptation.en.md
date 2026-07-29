---
locale: en
translation_of: methods/ai4l-adaptation.md
---

# Localized AI4L Audit Method

Status: `working-standard`  
Local version: `0.1`  
Upstream baseline: AI4L `1.2.0` / `bfe6083a1ca04223a245b9a5a0a8be41f021998f`  
Date: `2026-07-20`

## Why It Was Introduced

Open Longevity already has a source policy, Tier rules, and a dossier format for general readers. This method adds a reproducible **second-pass review**. The most valuable part of AI4L is not any ready-made review, but its separation of evidence review into creation, independent audit, correction, and re-audit, with line-by-line verification of links and conclusions.

This method condenses AI4L’s 404 criteria into 47 core checks suited to Open Longevity. The condensation does not lower the standard. It removes inapplicable assumptions about formatting, commercial databases, brands, and populations, and concentrates review on whether conclusions are traceable and their boundaries are clear.

## Two-Layer Documents

Each dossier has two reading layers:

1. **Reader layer**: 30-second conclusion, plain-language mechanism, human evidence, food, supplements, and safety boundaries;
2. **Research layer**: population and outcomes, effect sizes, risks, effect modifiers, interactions, monitoring, conflicts of interest, ongoing trials, and audit records.

The research layer may be more comprehensive, but it must not turn the reader layer back into a difficult long-form report.

## Audit Cycle

```text
Define question → Search and archive → Write dossier → Independent audit
                                                   ↓
                                        Correct remediable issues
                                                   ↓
                                        Re-audit and preserve history
```

### Separation of roles

- Authors cannot treat their own writing process as audit evidence;
- The audit starts again from the dossier, paper records, and raw snapshots;
- When evidence is missing, the auditor marks the gap rather than filling it from general knowledge;
- Formatting, omissions, and citation errors may be corrected in the same round, but new medical conclusions must go through the source workflow again.

### Audit status

| Status | Meaning |
|---|---|
| `pending` | Not yet audited |
| `partial` | Audited, but failed items or unresolved sources affect completeness |
| `passed` | Core items passed and no issue was found that blocks the current conclusion |
| `stale` | The evidence cutoff is too old, or new material may change the conclusion |

`passed` means only that the local checklist was passed; it does not mean the medical conclusion is absolutely correct.

## Ten Quality Gates

The complete 47-item template is in [intervention-audit.md](../templates/intervention-audit.md).

| Quality gate | Core question |
|---|---|
| A. Scope and metadata | Are the population, intervention, dose, duration, outcomes, and evidence cutoff explicit? |
| B. Search and source identity | Is the search recorded, and do links, titles, PMID/DOI/NCT identifiers match one another? |
| C. Benefits | Are human functional/clinical outcomes prioritized; are effect sizes and uncertainty reported; are negative findings prominent? |
| D. Risks | Are common and serious risks, long-term unknowns, special populations, and stopping conditions covered? |
| E. Applicability | Could age, sex, baseline status, disease, or medications change the conclusion? |
| F. Practical use | Are study formulation, label, exposure, quality, cost, and opportunity cost distinguished? |
| G. Monitoring and future evidence | Are success/failure, monitoring, ongoing trials, and new evidence that would change the conclusion described? |
| H. Interests and bias | Are funding, author interests, commercial sources, and selective reporting recorded? |
| I. Readability and conclusion | Is plain language used first; does the conclusion only summarize the body while retaining uncertainty? |
| J. Completeness and traceability | Are Tier, catalog, body text, paper library, links, and audit record consistent? |

## Differences from AI4L

### 1. General readers first

AI4L’s intended readers lean toward health-optimization users willing to execute complex protocols. Open Longevity retains general readers as the entry point, puts specialist information in research appendices, and does not require clinical terminology throughout.

### 2. No default “treatment regimen”

Doses in research describe evidence; they do not automatically become personal regimens. Without guidelines or clear clinical boundaries, describe only how the research was conducted and what evidence is missing, rather than assembling a purported “standard longevity protocol.”

### 3. No unsupported optimal ranges

Monitoring should prioritize clinical guidelines, regulators, or validated reference ranges. Purported “functional-medicine optimal ranges” may appear as disputed views only after their sources and evidence are reviewed separately.

### 4. No brand recommendations

Labels, formulation, purity, and third-party testing may be explained, but brands are not listed by default. ConsumerLab, Examine, Grokipedia, expert videos, and similar materials are leads or background only and cannot replace primary evidence.

### 5. No pursuit of a false perfect score

When links are inaccessible, full text is missing, funding is unknown, or long-term data do not exist, retain `partial` or `fail`. An unknown is itself a result and must not be rewritten into an affirmative conclusion merely to achieve a 100% pass rate.

## Relationship Between Tier and Audit

- Tier represents the current evidence and conditions of applicability;
- Audit status indicates whether the dossier has completed quality checks;
- An audit failure does not automatically lower the Tier, but may expose an evidence omission sufficient to change it;
- Any Tier change must state which new evidence or error caused it;
- Use by an individual, product sales, and mechanistic popularity still cannot raise a Tier.

## Upstream Limitations

AI4L explicitly notes that a clean audit may still miss errors, 100% means only that the checklist was passed, tool availability and model quality affect results, and evidence reviews cannot replace medical care. Open Longevity retains these limitations and additionally requires local snapshots and human-traceable records.
