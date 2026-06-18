---
name: write-a-skill
description: Create new agent skills that conform to the agentskills.io specification — proper frontmatter, naming rules, progressive disclosure, and the standard scripts/references/assets layout. Use when user wants to create, write, scaffold, or validate a new skill.
---

# Writing Skills

Follows the [Agent Skills specification](https://agentskills.io/specification).

## Process

1. **Gather requirements** — ask the user:
   - What task or domain does the skill cover?
   - What specific triggers (keywords, file types, contexts) should activate it?
   - Does it need executable scripts, reference docs, or just instructions?
   - Any environment requirements (runtimes, tools, network)?

2. **Draft the skill** — create:
   - `SKILL.md` with valid frontmatter and concise body
   - `references/` files only when content would push `SKILL.md` past ~500 lines or covers a distinct domain
   - `scripts/` only when a deterministic operation is needed
   - `assets/` only when bundling templates or data files

3. **Validate** — run `skills-ref validate ./skill-name` (see [skills-ref](https://github.com/agentskills/agentskills/tree/main/skills-ref)) to check frontmatter and naming.

4. **Review with user** — present the draft and confirm coverage, tone, and triggers.

## Directory structure

Per the spec, a skill is a directory containing at minimum a `SKILL.md`:

```
skill-name/
├── SKILL.md          # Required: frontmatter + instructions
├── scripts/          # Optional: executable code
├── references/       # Optional: detailed docs loaded on demand
└── assets/           # Optional: templates, data files, images
```

Only create subdirectories when you actually have content for them — don't scaffold empty folders.

## Frontmatter

YAML frontmatter at the top of `SKILL.md`:

| Field           | Required | Constraint |
| --------------- | -------- | ---------- |
| `name`          | Yes      | 1–64 chars, `[a-z0-9-]`, no leading/trailing/consecutive hyphens, must match parent directory name |
| `description`   | Yes      | 1–1024 chars, says what it does and when to use it |
| `license`       | No       | License name or pointer to bundled `LICENSE` file |
| `compatibility` | No       | ≤500 chars; environment requirements (runtime, tools, network) |
| `metadata`      | No       | Arbitrary string→string map for client-specific data |
| `allowed-tools` | No       | Space-separated pre-approved tools (experimental) |

**Minimal example:**

```markdown
---
name: pdf-processing
description: Extract PDF text, fill forms, merge files. Use when handling PDFs.
---
```

**With optional fields:**

```markdown
---
name: pdf-processing
description: Extract PDF text, fill forms, merge files. Use when handling PDFs.
license: Apache-2.0
compatibility: Requires Python 3.14+ and uv
metadata:
  author: example-org
  version: "1.0"
---
```

## Writing the description

The description is the only thing an agent sees when deciding whether to load the skill. It must encode both **what** and **when**.

- Max 1024 characters
- Lead with the capability, then the triggers
- Include specific keywords, file types, or user-phrasings that should fire it

**Good:**

```
Extracts text and tables from PDF files, fills PDF forms, and merges multiple PDFs. Use when working with PDF documents or when the user mentions PDFs, forms, or document extraction.
```

**Poor:**

```
Helps with PDFs.
```

## Progressive disclosure

The spec defines three load tiers — structure the skill to match:

1. **Metadata (~100 tokens)** — `name` + `description`, loaded at startup for every installed skill. This is your activation surface.
2. **Instructions (<5000 tokens recommended)** — the `SKILL.md` body, loaded when the skill activates. Keep under ~500 lines.
3. **Resources** — files in `scripts/`, `references/`, `assets/`, loaded only when the body links to them.

Push detail down. The body should orient and dispatch; deep material lives in `references/`.

## When to add scripts

Use `scripts/` when:

- The operation is deterministic (validation, formatting, parsing)
- The same code would otherwise be regenerated on every invocation
- Error paths need explicit handling

Scripts save tokens and improve reliability versus model-generated code. Document dependencies inline or in a header comment.

## When to split into references

Move content into `references/` when:

- `SKILL.md` would exceed ~500 lines
- Content covers distinct domains that won't all be needed at once (e.g. `finance.md` vs. `legal.md`)
- Advanced features are rarely used

Keep references one level deep — no nested reference chains. Reference them with relative paths:

```markdown
See [the reference guide](references/REFERENCE.md) for details.
```

## Body content

No format is mandated by the spec. Useful sections:

- Step-by-step instructions or workflow
- Concrete input/output examples
- Edge cases and how to handle them
- Links to `scripts/`, `references/`, `assets/`

## Review checklist

Before shipping:

- [ ] `name` matches the parent directory name and follows the character rules
- [ ] `description` includes both capability and triggers, ≤1024 chars
- [ ] `SKILL.md` body is under ~500 lines / ~5000 tokens
- [ ] References go one level deep, not nested
- [ ] No empty scaffolding directories
- [ ] Concrete examples in the body, not just abstract guidance
- [ ] `skills-ref validate ./skill-name` passes

---

_Adapted from [mattpocock/skills](https://github.com/mattpocock/skills/blob/main/skills/productivity/write-a-skill/SKILL.md) (MIT-licensed), aligned to the [Agent Skills specification](https://agentskills.io/specification)._
