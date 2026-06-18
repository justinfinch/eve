---
name: arche-lint
description: Audit the project's Arche at ./.arche/ for health. Finds contradictions flagged during ingest, stale dates, orphan pages with no inbound links, broken or missing cross-references, frontmatter drift, coverage gaps, and discovery-promotion drift (top ideas never filed back); suggests next investigations. Reports findings — does not auto-fix without confirmation. Use when the user says "lint the Arche", "Arche health check", "audit the Arche", or after a batch of ingests or discovery sessions when they want a tidy-up.
---

# arche-lint

Audit the project Arche and report issues. Do not fix without asking.

## Preflight

1. Verify `./.arche/SCHEMA.md`, `./.arche/index.md`, `./.arche/log.md` all exist. If not, instruct the user to run `/arche-init` first and stop.
2. Read `./.arche/SCHEMA.md` — it defines what "valid" means for this Arche.

## Checks

Run these against every page under `./.arche/`:

1. **Contradictions.** Grep `log.md` for entries whose `notes:` line starts with `contradiction —` (per SCHEMA's contradiction marker convention). For each, locate the strikethrough(s) on the pages listed in `pages touched:` — a strikethrough is **resolved** if the same paragraph contains a follow-up claim with an inline `[source link](...)` citation; otherwise flag it. Also scan all pages for stray `~~strikethrough~~` claims that have no corresponding `contradiction —` log entry — flag as untracked.
2. **Stale dates.** Any page whose `updated:` is older than 90 days AND that links to a source page updated more recently → flag as possibly stale.
3. **Orphan pages.** Any entity/concept/query/discovery page with zero inbound links from other Arche pages (excluding `index.md`) → flag. New pages get a grace pass — only flag if `created:` is older than 14 days. Discoveries are expected to be linked from at least the concept/entity pages that promoted their top ideas; a discovery with no inbound links after the grace period likely means no promotions stuck.
4. **Orphan raw files.** Any file in `./.arche/raw/` not referenced by any `sources/*.md` page's `raw:` frontmatter → flag as unprocessed. This is the "inbox not drained" signal; suggest `/arche-ingest` (batch mode) to handle them.
5. **Broken links.** Resolve every relative markdown link inside `./.arche/`. Flag any whose target file doesn't exist.
6. **Frontmatter drift.** For every page, check that required fields from SCHEMA.md's frontmatter spec are present and well-formed (valid date, valid `type:`, `sources:` is a list, etc.). Flag mismatches.
7. **Index/log integrity.** Every page in `./.arche/{sources,entities,concepts,queries,discoveries}/` should appear in `index.md`. Every `ingest`/`query`/`discovery` entry in `log.md` should reference pages that exist. Flag mismatches both directions.
8. **Coverage gaps.** Concepts that are referenced from other pages but have no page of their own → flag as candidates to create. Entities mentioned in ≥3 sources but with thin entity pages (<5 lines of body) → flag for expansion.
9. **Discovery promotion drift.** For every `discoveries/*.md` page older than 14 days, check that at least one concept or entity page lists it in their `sources:` frontmatter (i.e., a top idea was actually filed back). Discoveries with zero promoted pages → flag with note "promotion never filed — was that intentional, or did the session end before Phase 4 wrote back?". Skip this check if the user explicitly chose "self-contained discovery" in the session.

## Report format

Single response, sectioned by check:

```
## arche-lint report — {{TODAY}}

### Contradictions (N)
- [path](path) — one-line description

### Stale (N)
...

### Orphans (N)
...

(etc, omit sections with zero findings)

### Suggested next ingests
- topic — why
```

## After reporting

Ask: "Want me to fix any of these now?" Wait for the user to pick. Then handle one category at a time, confirming destructive edits (strikethrough resolutions, page deletions, slug renames).

Do not append a `lint` entry to `log.md` for the audit itself — only log when fixes are actually applied.

## Discipline

- Read-only by default. Never edit pages during the audit phase.
- Don't flood — if a check produces >20 findings, summarize and offer to drill in.
- Prefer suggesting next investigations over auto-creating empty stub pages.
