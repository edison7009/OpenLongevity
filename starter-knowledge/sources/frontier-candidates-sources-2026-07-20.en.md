---
locale: en
translation_of: sources/frontier-candidates-sources-2026-07-20.md
---

# Frontier Longevity Candidate Source Batch

Research date: `2026-07-20`  
Evidence cutoff: `2026-07-20`  
Scope: NMN, NR, spermidine, PQQ, and Ca-AKG  
Method: targeted evidence verification; not claimed to be a complete systematic review.

## Search Method

### PubMed

Targeted searches covered candidate names, human trials, functional outcomes, and aging outcomes:

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

Priority for inclusion was given to systematic reviews and meta-analyses, randomized controlled trials, negative primary endpoints, surrogate-endpoint studies directly relevant to marketing claims, and ongoing trials that could change the conclusion. Cell and animal studies were used only to explain mechanisms and research origins, not to infer efficacy for the general population.

### ClinicalTrials.gov

Interventional studies were searched by candidate name, and registrations most relevant to the current judgment were preserved:

- `NCT04691986`: NR for function and muscle physiology in older veterans; `ACTIVE_NOT_RECRUITING` at this snapshot, with results not yet posted;
- `NCT05706389`: ABLE, sustained-release Ca-AKG 1 g, 120 participants, six months, with change in DNA methylation age as the primary outcome; `ACTIVE_NOT_RECRUITING` at this snapshot;
- `NCT07114536`: Ca-AKG 2 g/day, 30 participants, 12 weeks, with PhenoAge as the primary outcome; `ACTIVE_NOT_RECRUITING` at this snapshot, with no results posted on the registration page.

Registration status changes over time; an “estimated completion date” must not be written as an “available result.”

## Local Snapshots

| Content | Local path | Bytes | SHA-256 |
|---|---|---:|---|
| 21 PubMed XML records | `work/references/pubmed/2026-07-20-frontier-longevity-candidates.xml` | 536528 | `6ca604e674a414a5c91cc91fbbc742642e112de367b7ff8ab709b430a6e4d8ac` |
| 10 open-full-text XML records | `work/references/fulltext/2026-07-20-frontier-longevity-open-fulltext.xml` | 1041844 | `f0fba520f84e3f209619e051428e781cc2fae74fbca0094a807c041e6fe29929` |
| NR trial registration | `work/references/clinicaltrials/2026-07-20-NCT04691986.json` | 14791 | `359c77e86cd38981365a2bef7aff35861b3110d1ba5127325a2b04a9c4ccad2f` |
| ABLE Ca-AKG registration | `work/references/clinicaltrials/2026-07-20-NCT05706389.json` | 16388 | `88d4e9e11e8f3e6d186c8fee2abfdae6f21b49b8a0413a82e180773381b99e32` |
| 30-person Ca-AKG registration | `work/references/clinicaltrials/2026-07-20-NCT07114536.json` | 18473 | `d22a426cd0f244b7d5351974ff4ba301ef5c4d6c504ddeaae9ce8bec5d02851d` |

## Key Judgments in This Round

| Candidate | Most consequential human information | Conclusion that cannot currently be drawn |
|---|---|---|
| NMN / NR | NAD-related markers generally rise; the muscle-function meta-analysis does not support stable benefit, and a small NR MCI trial did not improve cognition | Cannot infer life extension, cognitive protection, or universal physical-performance improvement |
| Spermidine | The primary endpoint was negative in a one-year cognition RCT of 100 participants; a 2026 vaccine-response trial in 40 participants produced an immune signal | A small trial in a specific context cannot establish general anti-aging or life extension |
| PQQ | A single-ingredient cognition RCT completed by 58 participants reported multiple positive results; small sample, many outcomes, and product relationships are labeled | Mitochondrial function, long-term cognition, and healthspan outcomes require expansion |
| Ca-AKG | Human evidence consists mainly of a combination-product before-after comparison, cross-sectional associations, and methylation clocks; key RCTs are progressing | Changes in “biological age” should be interpreted together with functional outcomes |

## Boundaries of Use

- Personal protocols and commercial products are used only to discover candidates, not in efficacy grading;
- “Wealthy people are using it” does not equal completed clinical validation;
- Research doses record trial exposure only and do not automatically become personal doses;
- Where data on long-term safety, drug interactions, cancer contexts, pregnancy and breastfeeding, or complex disease remain incomplete, mark them uniformly as requiring an update.
