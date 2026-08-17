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
- `guides/`: selected scientific and purchasing guides with independent reader value;
- `inbox/`: newly captured material awaiting organization;
- `profile/`: personal background entered voluntarily by the user;
- `plans/`: the user's own current protocol;
- `records/`: laboratory, diet, and training records.

The starter library includes strategy dossiers, public-figure cases, longevity stories, and a small set of selected guides. Brand rankings live directly on each supplement page rather than in duplicate articles. Personal content in `profile/`, `plans/`, and `records/` is entered voluntarily by the user and stored locally.

## Internal Links Between Articles

When the full Chinese or English title of another article appears in the body text, Open Longevity automatically displays the first occurrence as an internal link. A target may also be specified explicitly in Markdown:

- `[Strength Training](#/supplement/strength-training)`
- `[Bryan Johnson](#/person/bryan-johnson)`
- `[Okinawa's Longevity Culture](#/story/okinawa-longevity)`

The final part of the link uses the `id` from the target article's frontmatter. Internal links switch articles only within Open Longevity; external reference sites continue to open in the system's default browser.
