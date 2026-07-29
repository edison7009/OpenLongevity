---
locale: en
translation_of: methods/research-workflow.md
---

# Standard Research Workflow

## 0. Define the Question

Use PICO or an equivalent structure to specify:

- Population;
- Intervention and dose;
- Comparator;
- Duration;
- Primary clinical or functional outcomes;
- Safety outcomes.

If the question is only “Is this ingredient anti-aging?”, first split it into testable questions.

## 1. Search the Local Knowledge Base

Search first:

```powershell
rg -n "<Chinese name>|<English name>|<alias>" knowledge
```

Confirm whether a candidate, dossier, paper record, or raw snapshot already exists.

## 2. Register the Candidate

For a new item:

- Add it to `catalog/supplements.csv` first;
- Set `evidence_status=candidate`;
- Set `tier=pending`;
- Record the candidate source and Bryan status;
- Do not assign a Tier immediately.

## 3. Search External Evidence

Prioritize:

1. Guidelines and government/professional-organization materials;
2. Systematic reviews and meta-analyses;
3. Key RCTs;
4. Trial registrations and regulatory information;
5. Safety, drug-interaction, and product-quality materials.

Record the database, date, search string, and screening rationale.

## 4. Preserve Source Material

- Use XML or a stable abstract record for PubMed;
- Prefer XML/HTML/PDF for open full text;
- Save dated snapshots of dynamic webpages;
- Calculate SHA-256;
- Record the item in `sources/source-manifest.md`.

## 5. Create a Paper Record

Use `knowledge/templates/paper-record.md`. At minimum, verify:

- Title and persistent identifier;
- Preregistered primary endpoint;
- Sample, dose, and duration;
- Between-group effect size and uncertainty;
- Adverse events;
- Funding and conflicts of interest;
- Risk of bias.

## 6. Create or Update the Dossier

Use `knowledge/templates/intervention-dossier.md` and synthesize all included evidence rather than stacking paper-by-paper summaries.

## 7. Independent Audit

Use `knowledge/templates/intervention-audit.md`:

- The auditor starts again from the dossier, paper records, and raw snapshots;
- Verify source identity, link semantics, effect sizes, negative results, risks, interactions, conflicts of interest, and ongoing trials;
- Save results to `knowledge/audits/`;
- First correct problems that existing sources can support;
- When a new medical conclusion is needed, return to steps 3–6 rather than adding it directly to the audit form.

For the complete method, see `methods/ai4l-adaptation.md`. A `100% pass` does not mean the medical conclusion is absolutely correct, and unknown items must not be rewritten merely to obtain a full score.

## 8. Assign a Tier

Follow `methods/evidence-grading.md`. Items whose review is incomplete remain `pending`; do not force a grade merely to fill the list.

Record Tier and audit status separately: Tier describes the evidence position, whereas audit status describes dossier completeness.

## 9. Synchronize the Website

Update the website only after the knowledge base is complete. Ideally, generate it from CSV and Markdown rather than copy and paste.

## 10. Write the Research Log

Record:

- What this batch completed;
- Changes to key judgments;
- Sources added or removed;
- Questions that remain uncertain;
- The next priority queue.
