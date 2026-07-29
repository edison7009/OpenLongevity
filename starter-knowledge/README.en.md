---
locale: en
translation_of: README.md
---

# Open Longevity Starter Knowledge Base

This is the independent, local-first knowledge base created when Open Longevity is launched for the first time. The starter content provides a reasonably complete foundation for reading and AI context, while containing no health parameters from the developer or any real user.

Directory conventions:

- `catalog/`: longevity strategy and content indexes;
- `dossiers/`: strategy dossiers for exercise, diet, supplements, and other interventions;
- `cases/`: public figures and protocol case studies;
- `stories/`: longevity anecdotes from regions, cultures, and history; new Markdown files are discovered automatically;
- `papers/`: paper records;
- `sources/`: source registry;
- `products/`: product and brand quality records;
- `audits/`: dossier audits;
- `methods/`: research and evidence-synthesis methods;
- `topics/`: cross-strategy topics;
- `research-log/`: research process records;
- `inbox/`: newly captured material awaiting organization;
- `profile/`: personal background entered voluntarily by the user;
- `plans/`: the user's own current protocol;
- `records/`: laboratory, diet, and training records.

The starter library includes strategy dossiers, public-figure cases, papers, sources, product-quality records, research methods, and essential research logs. `profile/`, `plans/`, and `records/` contain blank templates only. Age, conditions, medications, dosages, laboratory results, diet, and training records are entered voluntarily by the user and stored locally. Users may freely modify or delete all materials.

## Internal Links Between Articles

When the full Chinese or English title of another article appears in the body text, Open Longevity automatically displays the first occurrence as an internal link. A target may also be specified explicitly in Markdown:

- `[Strength Training](#/supplement/strength-training)`
- `[Bryan Johnson](#/person/bryan-johnson)`
- `[Okinawa's Longevity Culture](#/story/okinawa-longevity)`

The final part of the link uses the `id` from the target article's frontmatter. Internal links switch articles only within Open Longevity; external reference sites continue to open in the system's default browser.
