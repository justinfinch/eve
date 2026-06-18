---
name: arche-ingest
description: Ingest a source into the project's Arche at ./.arche/. The Arche captures **institutional context** (business domain, SME knowledge, ARB-style architectural decisions, research) — not code documentation. Accepts a URL, a file path, pasted text, OR — with no argument — processes every file in ./.arche/raw/ not yet referenced by a source page (batch mode). Snapshots the raw input to ./.arche/raw/, writes a source-summary page, updates index.md, revises affected entity and concept pages, and appends a log entry — following the conventions in ./.arche/SCHEMA.md. Use when the user provides a source to add to an Arche, says "ingest", "add to Arche", "remember this article", "process the raw folder", or shares an article/paper/PDF/transcript/SME-interview/ADR they want filed.
---

# arche-ingest

Add one or more sources to the project Arche.

## Preflight

1. Verify `./.arche/SCHEMA.md` exists. If not, tell the user to run `/arche-init` first and stop.
2. Read `./.arche/SCHEMA.md` end to end. The schema is authoritative — follow its slug rules (including the "Slug derivation" subsection), page-type definitions, frontmatter shape, cross-linking rules, contradiction marker convention, and log format.
3. Read `./.arche/index.md` so you know what entities and concepts already exist (you'll be linking into them and possibly extending them).
4. Read this skill's own page templates so new pages follow the canonical layout: [source.template.md](assets/source.template.md), [entity.template.md](assets/entity.template.md), [concept.template.md](assets/concept.template.md).

## Dispatch

- **Explicit argument** (URL, file path, pasted text): single-source ingest. Continue to Workflow.
- **No argument**: batch mode. List every file in `./.arche/raw/` that no existing `sources/*.md` page references via its `raw:` frontmatter field. Show the user the list and ask which to ingest (default: all). Then run the Workflow once per file, in order.

## Workflow (per source)

1. **Land the raw file in `./.arche/raw/`.** Derive the slug per SCHEMA's "Slug derivation" rules.
   - File path → copy (don't move; leave the original alone) to `raw/<slug>.<ext>`.
   - URL → fetch the page, convert to markdown, save as `raw/<slug>.md`. Record the canonical URL for frontmatter.
   - Pasted text → save verbatim to `raw/<slug>.md` with a header noting the date and source description.
   - On slug collision: if the incoming file is byte-identical to the existing one, skip ingest and tell the user it's already captured. Otherwise append `-2`/`-3` to the slug (do not silently overwrite).
2. **Discuss with the user first** (one short message): name the 3–5 key points you saw and the entities/concepts you plan to touch. Stop and let them redirect before writing anything. This is the "discuss key points with you" step from Karpathy's pattern. Skip the discussion step in batch mode unless the user asked for it explicitly.
3. **Write the source summary** at `.arche/sources/<slug>.md` using this skill's [source.template.md](assets/source.template.md) as the layout (frontmatter including `raw:` and/or `url:`, summary within SCHEMA.md's length cap, `## Key claims`, `## See also` listing every entity/concept page touched).
4. **Update or create entity/concept pages.** For each entity/concept the source touches:
   - If the page exists: add new claims with inline citations to the source page, update the `sources:` frontmatter list, bump `updated:`.
   - If the page is new: create it using this skill's [entity.template.md](assets/entity.template.md) or [concept.template.md](assets/concept.template.md) as the layout, with full frontmatter and at least the claims this source supports.
   - Do not duplicate facts already present — extend, don't restate.
5. **Update `index.md`.** Add the new source under Sources. Add any new entities/concepts under their sections. Each entry: title, one-line gloss, link, tags.
6. **Append to `log.md`** in the format from SCHEMA.md. List every page touched (including the new `raw/` file). Notes line: one sentence on what the source contributed.

## Discipline

- Touch as many pages as the source warrants. A rich source legitimately updates 10–15 pages; a thin one might update 2.
- Cite at the point of claim, not just in frontmatter.
- If the source contradicts an existing claim, do NOT silently rewrite. Use `~~strikethrough~~` on the old claim, add the new one with an inline `[source link](...)` citation in the same paragraph (this is what marks the strikethrough as resolved), and write the log entry's `notes:` line with the prefix `contradiction —` (per SCHEMA's contradiction marker convention) so `/arche-lint` can find it.
- Bump `updated:` on every page you edit. Never on pages you only read.
- If you're unsure whether to create a new entity or extend an existing one, ask the user — slug churn is expensive.

## Output

End with a one-line summary: `Ingested <title> → N pages updated (list them).` Nothing longer.
