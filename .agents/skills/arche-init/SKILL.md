---
name: arche-init
description: Bootstrap an Arche at ./.arche/ in the current project using Karpathy's wiki pattern. The Arche captures **institutional context** (business domain, SME knowledge, ARB-style architectural decisions, research) that doesn't live in the code — adjacent to the codebase, never derived from it. Creates SCHEMA.md (conventions and operations), index.md (catalog), log.md (append-only changelog), and the standard subdirectories, and registers the Arche in the repo's agent context file(s) (AGENTS.md / CLAUDE.md / .cursorrules) so coding agents treat it as a first-class context source picked up automatically. If ./.arche/ already exists, runs in migration mode — additively brings the Arche's system files up to the current schema without rewriting content pages. Use when the user wants to start an Arche, set up an LLM-maintained knowledge base, upgrade an existing Arche, or says "init Arche", "bootstrap Arche", "set up an Arche here", or "migrate the Arche".
---

# arche-init

Bootstrap or migrate an Arche at `./.arche/`.

This skill owns only the Arche's **system files** (`SCHEMA.md`, `index.md`, `log.md`) and the directory tree. Each operation skill (`/arche-ingest`, `/arche-query`, `/arche-discover`, `/arche-architect`, `/arche-tell`) ships its own page templates and reads them from its own skill directory at runtime — `arche-init` does not copy templates into the Arche and does not need updating when a new Arche operation skill is added.

The path is **dotted** (`./.arche/` not `./arche/`) by convention with other agent-tooling directories (`.claude/`, `.cursor/`, `.vscode/`) and to avoid collision with project content folders. The Arche is curated content but has substantial machine-maintained scaffolding (index, log, frontmatter, lint) — the dot signals that.

## Workflow

1. Resolve today's date once (YYYY-MM-DD from the environment) — reuse it everywhere below.
2. Check whether `./.arche/` already exists.
   - **Does not exist** → fresh bootstrap (step 3).
   - **Exists** → migration mode (step 4).

### Step 3: Fresh bootstrap

1. Create the directory tree:
   ```
   .arche/
     SCHEMA.md
     index.md
     log.md
     raw/             # drop zone — immutable source files (PDFs, transcripts, snapshots)
     sources/         # LLM-written summaries that cite raw/ or external URLs
     entities/
     concepts/
     queries/
     discoveries/     # captured discovery / ideation sessions (see /arche-discover)
     stories/         # communication artifacts (see /arche-tell) — the .md source pages
     assets/stories/  # rendered HTML artifacts paired with stories/<slug>.md
   ```
   Add `.gitkeep` to each empty subdir so git tracks the structure.
2. Copy these files from this skill's `assets/` directory, replacing every `{{DATE}}` token with today's date:
   - `assets/SCHEMA.template.md` → `.arche/SCHEMA.md`
   - `assets/index.template.md` → `.arche/index.md`
   - `assets/log.template.md` → `.arche/log.md`
3. **Register the Arche in the repo's agent context file(s)** so coding agents treat it as a first-class context source — picked up automatically, not something the user must explicitly invoke. The snippet to write is [assets/agents-md-snippet.md](assets/agents-md-snippet.md), marked with `<!-- arche-context-source -->`. Do **not** detect or branch on which coding agent is in use — the rule below is agent-neutral and covers all of them:
   1. **Canonical home — `AGENTS.md`** (the cross-agent convention; Codex CLI, Cursor ≥recent, and others read it). Append the snippet if `AGENTS.md` exists; create `AGENTS.md` with the snippet if it doesn't. This is the single source of truth — the snippet lives here in full and nowhere else in full.
   2. **Claude Code bridge — `CLAUDE.md`.** Claude Code reads `CLAUDE.md`, **not** `AGENTS.md` ([docs](https://code.claude.com/docs/en/memory)), so the canonical home alone won't be picked up by it. Bridge it without duplicating content: if `CLAUDE.md` exists and carries neither the marker nor an `@AGENTS.md` import, add a line `@AGENTS.md` (Claude Code's import syntax) near the top; if `CLAUDE.md` doesn't exist, create it containing `@AGENTS.md`. (If for some reason `AGENTS.md` isn't the home — e.g. the user keeps everything in `CLAUDE.md` — put the snippet inline in `CLAUDE.md` instead of an import.)
   3. **Other tool files that already exist** — `.cursorrules` / `.cursor/rules/*.md`, `.windsurfrules`, `.github/copilot-instructions.md`. These don't support import syntax, so append the snippet inline. Only touch them if they already exist; don't create them from scratch (creating `AGENTS.md` already covers those tools where they honor it).
   Idempotent: a file is "done" if it contains the marker `<!-- arche-context-source -->`, and `CLAUDE.md` is also "done" if it contains an `@AGENTS.md` import. Skip done files. Don't edit `README.md` — that's user-facing, not agent-facing.
4. Tell the user the Arche is ready, list the agent context file(s) you created or updated (call out the `AGENTS.md` source + `CLAUDE.md` bridge), and point them at `/arche-ingest <source>` to add the first source.

### Step 4: Migration mode (Arche already exists)

The Arche already has content. The job is **additive** — never rewrite a content page (`sources/`, `entities/`, `concepts/`, `queries/`, `discoveries/`). Existing content normalizes opportunistically when future ingests touch it.

1. **Detect drift.** Compare the existing Arche against the current templates in this skill's directory. The principle: anything present in the current `SCHEMA.template.md` that is missing or stale in the existing `SCHEMA.md` is a candidate patch. Check at minimum for:
   - **Agent-context registration** (applies to every Arche, independent of schema era) — grep the repo's agent context files (`AGENTS.md`, `CLAUDE.md`, `.cursorrules` / `.cursor/rules/*.md`, `.windsurfrules`, `.github/copilot-instructions.md`) for the marker `<!-- arche-context-source -->`. Two distinct gaps to offer to fix, both via fresh-bootstrap step 3's agent-neutral rule:
     - **Not registered at all** — no context file carries the marker. The Arche isn't wired in as a first-class context source. This is the common case for Arches created before agent-context registration existed.
     - **Registered but Claude Code can't see it** — `AGENTS.md` carries the marker, but `CLAUDE.md` neither carries it nor has an `@AGENTS.md` import. Claude Code reads only `CLAUDE.md`, so add the bridge (step 3 sub-step 2). Easy to miss, because other agents will already be picking the Arche up fine.
   - **Framing block** — the "What belongs here (and what doesn't)" section that scopes the Arche to institutional context (business / SME / ADRs / research) and rules out code documentation
   - **Architecture pages section** — the "Architecture pages (ARD, SAD, ADR)" section under Page types: three concept-page conventions (`ard-<system>`, `sad-<system>`, `adr-<name>`) with their body-section tables, pairing rules, and shared status/supersession behavior. If only the older "ADRs (Architecture Decision Records)" subsection is present, the SAD and ARD conventions are missing — patch.
   - **Frontmatter fields** — `status:` and `superseded_by:` apply to ARD, SAD, and ADR concept pages (older Arches may scope these to ADR only)
   - **`architect` log op** — if SCHEMA's ops list does not include `architect`, the `/arche-architect` skill cannot file sessions; patch.
   - **Architect operation summary** — if SCHEMA's operations summary lacks the `architect` entry alongside `discovery`, patch.
   - **Retired spec/plan page types** — if the existing SCHEMA still defines `spec` or `plan` page types, `specs/`/`plans/` rows, the `specify`/`plan` log ops, or the `## Specs` / `## Plans` index sections, flag them for the user as **removed conventions**. These were retired when feature-spec and implementation-plan work moved to the team's own dev-methodology skills (which read the Arche for grounding). Do not auto-delete — existing `specs/`/`plans/` content pages need the user's manual attention (migrate them out, or leave them as legacy). Offer to drop the now-unused schema definitions and index stubs.
   - **Story page type** — if SCHEMA's page-types table does not include a `story` row pointing at `stories/<slug>.md`, the `/arche-tell` skill cannot file artifacts; patch.
   - **Story frontmatter fields** — if the frontmatter spec does not document `audience`, `action_ask`, `framework`, `format`, and `html` (story-page-only fields), patch.
   - **`story` log op** — if SCHEMA's ops list does not include `story`, patch.
   - **Story operation summary** — if SCHEMA's operations summary lacks the `story` entry alongside `architect`, patch.
   - **Index section** — if `index.md` lacks a `## Stories` section, add it (with the "None yet. Run /arche-tell..." stub).
   - **`stories/` subdir** — create with a `.gitkeep` if missing.
   - **`assets/stories/` subdir** — create with a `.gitkeep` if missing (this is where rendered HTML artifacts live).
   - **Page-type rename** — if SCHEMA still has `brainstorm` instead of `discovery` in the `type:` enum, the page-types table, the frontmatter spec, the sources-bidirectional clause, or the operations summary
   - **Log-op rename** — if SCHEMA's ops list still has `brainstorm` instead of `discovery`
   - **Index sections** — if SCHEMA's index.md description still says "Brainstorms" instead of "Discoveries"
   - **Earlier-era checks (still relevant for very old Arches):**
     - The expanded `type:` enum including `schema | index | log`
     - The "Slug derivation" subsection
     - The "Contradiction marker" subsection (log notes prefix `contradiction —`)
     - `migrate` in the ops list
     - The `.gitkeep` one-liner
   - Are any expected subdirs missing (`discoveries/`, `queries/`, etc.)? Note their `.gitkeep` if so. If a `brainstorms/` directory exists but no `discoveries/`, flag for the user — directory rename and any `type: brainstorm` content pages need their manual attention (this migration does not rewrite content pages).
2. **Present a migration plan** in one message. Example:
   ```
   Migration plan for ./.arche/:

   Additive (safe — new files only):
   - create .arche/discoveries/ (missing)
   - register the Arche: write snippet to AGENTS.md (source of truth) + add @AGENTS.md bridge to CLAUDE.md so Claude Code picks it up (marker <!-- arche-context-source --> absent)

   Schema patches (overlay .arche/SCHEMA.md):
   - add "What belongs here (and what doesn't)" framing block
   - add ADRs (Architecture Decision Records) subsection under Page types
   - add status / superseded_by frontmatter fields
   - rename brainstorm → discovery in type enum, page-types table, log ops list, operations summary
   - update index.md section name "Brainstorms" → "Discoveries"

   Needs your manual attention (out of scope for migration):
   - .arche/brainstorms/ directory exists with N pages — rename directory to discoveries/ and update each page's frontmatter `type: brainstorm` → `type: discovery`

   Untouched: all sources/, entities/, concepts/, queries/ pages.

   Apply all schema patches, pick selectively, or cancel?
   ```
3. **Apply** what the user accepts. For schema patches: overlay the relevant sections of the current `SCHEMA.template.md` into the existing `SCHEMA.md`, preserving any human edits the user has made in the "Conventions the human controls" section. For agent-context registration: append the snippet to the context files (idempotent via the marker). Do not rewrite content pages or rename directories — those stay in the user's hands.
4. **Append a `migrate` log entry** to `.arche/log.md` listing what changed:
   ```
   ## [{{DATE}}] migrate | Arche upgraded to current schema
   - pages touched: SCHEMA.md
   - notes: patched SCHEMA (added framing block, ADR section, status/superseded_by, renamed brainstorm→discovery)
   ```
5. Tell the user migration is done. Mention that existing content pages were left as-is and will normalize as future ingests touch them. If you flagged a brainstorms/ directory or `type: brainstorm` pages for manual attention, restate that. Suggest running `/arche-lint` to confirm the schema/index/log false positives are gone.

## Notes

- The schema is the source of truth for conventions — `/arche-ingest`, `/arche-query`, `/arche-lint`, `/arche-discover`, `/arche-architect`, and `/arche-tell` all read `.arche/SCHEMA.md` before acting. If the user later changes conventions, they edit the schema; the operation skills follow.
- Page templates (for `sources/`, `entities/`, `concepts/`, `queries/`, `discoveries/`, `stories/`) live next to the skills that write those page types. `arche-init` deliberately does not own them — that keeps init decoupled from the set of operation skills.
- Do not embed Arche content in the schema. The schema describes *how* pages are written, not what they contain.
- **Agent-context registration is what makes the Arche first-class.** A coding agent won't reliably consult `./.arche/` unless its always-loaded context file says to. Bootstrap (and migration) writes the `<!-- arche-context-source -->` snippet into the repo's context file(s) so the Arche is picked up automatically — the user shouldn't have to remember to invoke `/arche-query`. The snippet is agent-neutral (same approach as `devbox-add`'s source-of-truth snippet). This is `arche-init`'s own policy snippet about the artifact it bootstraps — not a page template owned by another operation skill.
- **No agent detection — bridge instead.** `AGENTS.md` is the single source of truth; we never branch on "which coding agent is this". The one wrinkle is that **Claude Code reads `CLAUDE.md`, not `AGENTS.md`** ([memory docs](https://code.claude.com/docs/en/memory)), so a repo with only `AGENTS.md` would be invisible to it. Rather than detect Claude Code, we always add a `CLAUDE.md` that imports `AGENTS.md` via `@AGENTS.md` — the Anthropic-documented bridge. One import line, no duplicated content (so no drift), and non-Claude agents harmlessly ignore the extra file. Tool files without import syntax (`.cursorrules`, `.windsurfrules`, Copilot) get the snippet inline, but only if they already exist.

## Templates

System-file templates in this skill's directory:
- [SCHEMA.template.md](assets/SCHEMA.template.md) — full Arche conventions
- [index.template.md](assets/index.template.md) — catalog stub
- [log.template.md](assets/log.template.md) — changelog stub
- [agents-md-snippet.md](assets/agents-md-snippet.md) — the "consult the Arche first" snippet appended to the repo's agent context file(s) (`AGENTS.md`, `CLAUDE.md`, `.cursorrules`, …) so the Arche is a first-class, auto-loaded context source
