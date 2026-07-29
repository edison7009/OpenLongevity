---
locale: en
translation_of: methods/source-policy.md
---

# Source and Archiving Policy

Version: `0.1`  
Date: `2026-07-20`

## Source Levels

### A. Conclusion sources

Used to assess efficacy and safety:

- Clinical guidelines and materials from governments or professional organizations;
- PubMed-indexed systematic reviews, meta-analyses, and randomized trials;
- Trial registrations, regulatory documents, and formal corrections;
- Full papers when necessary to answer questions the abstract cannot resolve.

### B. Protocol sources

Used to record “what someone says they are doing”:

- Bryan Johnson’s official protocol, official videos, and his public posts;
- Official product formulations and change records;
- Dated interviews or podcasts.

These sources cannot establish efficacy.

### C. Lead sources

- Wikipedia;
- News reports;
- Community posts;
- User-compiled lists;
- Commercial promotional pages.

These sources are used only to discover candidate items and original references.

## Local Archiving Rules

- Place original files in `work/references/<source>/`.
- Begin filenames with the capture date, for example `2026-07-20-current-protocol.html`.
- Record the original URL, date, format, file size, and SHA-256 in `knowledge/sources/source-manifest.md`.
- For dynamic webpages, also record the page title and identifiable version information.
- Mark sources that cannot be downloaded as `remote-only`; preserve a structured summary and URL rather than fabricating a local copy.
- Archived files are for research verification only; citation and republication must comply with the original site’s license.

## Change Detection

When recapturing the same source:

1. Save it under a new dated filename without overwriting the old snapshot;
2. Calculate SHA-256;
3. Record the difference if the checksum changes;
4. Update `current-explicit` or an evidence conclusion only after human review.

## Conflict Handling

When an official protocol, product page, video, and older list conflict:

- Prefer the most recent first-party/official source with the clearest semantics;
- Preserve the conflict rather than forcing a merge;
- Set status to `historical-or-discussed` or `needs-source-check`;
- Record dose, formulation, frequency, and whether use is ongoing separately.

## Commercial Interests

Bryan Johnson’s official product pages are both biographical sources and sales pages. They may confirm a product name or formulation, but their wording such as “research-backed” cannot determine the evidence grade.
