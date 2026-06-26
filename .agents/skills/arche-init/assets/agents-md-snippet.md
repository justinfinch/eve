<!-- arche-context-source -->
## Institutional context: consult the Arche first

This repo has an **Arche** at `./.arche/` — a curated knowledge base of *institutional context the code does not carry*: business domain, SME knowledge, architectural decisions (ARDs / SADs / ADRs), and research. It follows Karpathy's LLM-wiki pattern and is maintained by the `arche-*` skills. Treat it as a first-class context source, not an optional lookup.

**Before** planning, designing, scoping, specifying a feature, or making a setup decision (dev environment, tooling, dependencies), orient in the Arche first so the work descends from what the team already knows:

1. Read `./.arche/index.md` — the catalog and entry point.
2. Walk to the relevant `entities/` and `concepts/` (including ARDs / SADs / ADRs) pages.
3. Surface the relevant decisions, constraints, and prior research *before* proposing an approach, and cite the pages you used.

If the `arche-query` skill is available (e.g. `/arche-query` in Claude Code), invoke it to do this rather than reading pages ad hoc — it walks the index, cites provenance, and flags gaps. The other `arche-*` skills cover the rest of the institutional-context workflow: `arche-ingest` (add a source), `arche-architect` (design / ADRs), `arche-discover` (ideation), `arche-tell` (communicate), `arche-lint` (health check).

**This grounding applies to your existing dev-methodology skills too.** When a spec-writing, planning, or implementation skill from another library (spec-kit, superpowers, your own) runs in this repo, it should consult the Arche first by the same steps above. The Arche does not own spec or plan artifacts — it supplies the grounded *why/what/decisions* those skills build on, then gets out of the way.

**The Arche is institutional context, not code documentation.** If a question is answered by *reading the code*, the Arche is the wrong place. Do not dump in-flight TODOs or debugging notes into it — those stay in PRs and commit messages.
