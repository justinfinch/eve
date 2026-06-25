---
type: log
created: 2026-06-16
---

# Arche Log

Append-only. Newest entries at the bottom. See [SCHEMA.md](SCHEMA.md) for entry format.

## [2026-06-16] init | Arche bootstrapped
- pages touched: SCHEMA.md, index.md, log.md
- notes: created by /arche-init

## [2026-06-17] ingest | Adom, Decoded — Problem Space + a POC of Their Actual App
- pages touched: raw/adom-decoded-and-poc-plan.md, sources/adom-decoded-and-poc-plan.md, entities/adom-industries.md, entities/john-lauer.md, concepts/molecules-and-workcells.md, concepts/adom-technical-architecture.md, concepts/remote-labs-prior-art.md, concepts/device-farms.md, concepts/instrument-control-standards.md, concepts/programmable-wiring.md, concepts/ai-native-eda.md, concepts/automated-remote-bring-up.md, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: first content ingest — established Adom company/founder entities, decoded technical architecture, prior-art landscape (5 concept pages), white space, and the recommended desk-scale POC.

## [2026-06-17] query | What Hardware Do I Need to Buy (POC BOM)
- pages touched: queries/poc-hardware-bom.md, index.md
- notes: filed concrete parts list for the Mini-Molecule POC (Pico 2, MCP2515, ADS1220 + supporting parts; CH446Q/Jumperless for optional Concept B). Exact SKUs/prices sourced from web vendor data (June 2026), not the Arche.

## [2026-06-17] story | What the POC Builds — A Visual Explainer
- pages touched: stories/poc-explainer-for-self.md, assets/stories/poc-explainer-for-self.html, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: POC explainer → narrative (Pyramid) for self/understanding. Inline-SVG end-to-end architecture diagram + CSS concept cards (A–D) + 5-pillar grid + v1→v2 path. Back-link added to the POC concept page.

## [2026-06-22] story | What the POC Builds — added "what it physically does" callout
- pages touched: stories/poc-explainer-for-self.md, assets/stories/poc-explainer-for-self.html
- notes: re-rendered story — added a callout under the system diagram listing concrete behaviors (GPIO LED, PWM dim/motor, ADC live read; +B/+D options). Cites POC page and the hardware BOM query.

## [2026-06-22] manual | Repo README mirrors the POC explainer story
- pages touched: /README.md (repo root, not an Arche page), stories/poc-explainer-for-self.md
- notes: created repo-root README.md mirroring stories/poc-explainer-for-self.md (one-liner, Mermaid architecture diagram, what-it-physically-does table, concept table, pillars, v1→v2, BOM links). Added a "Mirrored by /README.md" sync note to the story's See also.

## [2026-06-22] manual | Reframe README to external resume voice
- pages touched: /README.md (repo root), stories/poc-explainer-for-self.md
- notes: rewrote README voice from internal "our approach" to first-person, addressed to an Adom resume reviewer ("Why I built it this way", "I scoped...", scope column instead of internal status). README and story now diverge in voice by design; updated the story's companion note to keep facts in sync without flattening README voice.

## [2026-06-22] discovery | POC Unknown-Unknowns — Learning Hardware as a Software Dev
- pages touched: discoveries/poc-unknown-unknowns.md, concepts/learning-hardware-as-a-software-dev.md, queries/poc-hardware-bom.md, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: pre-mortem/assumption-reversal session on unknown-unknowns in the Mini-Molecule POC — 24 ideas across 5 themes, 3 promoted. Reframed the deliverable from "working rig" to hardware literacy. New concept page (learning-hardware-as-a-software-dev); promoted arrival/smoke-test ritual + learning tooling (logic analyzer, USB power meter, 2nd Waveshare reference node, ESD strap) into the BOM query; added a learning-sequenced build-order section to the POC concept.

## [2026-06-22] query | POC BOM — added vetted Amazon picks for learning tools
- pages touched: queries/poc-hardware-bom.md
- notes: vetted current Amazon SKUs (web, June 2026) for the learning-tool additions from the unknown-unknowns discovery: HiLetgo 24MHz 8ch logic analyzer (B077LSG5P2), Waveshare RP2350-CAN reference node (B0F4JH65HY), MakerHawk AT34 USB power meter (B07FMQZVW2), iFixit ESD strap (B00B2T9C8Y). ~$50 picks subtotal.

## [2026-06-22] story | POC Hardware Manual — Build It, Understand It
- pages touched: stories/poc-hardware-manual-for-self.md, assets/stories/poc-hardware-manual-for-self.html, stories/poc-explainer-for-self.md, index.md
- notes: hardware build + literacy manual → narrative (report) for self. Pyramid spine; rungs 0–6 with derived pin maps (verify on bench), per-part reference cards, arrival ritual + diagnosis flowcharts (Mermaid), inline-SVG CAN topology. Companion-linked to the prior explainer story.

## [2026-06-22] architect | Mini-Molecule POC — software solution architecture
- pages touched: concepts/ard-mini-molecule-poc.md, concepts/sad-mini-molecule-poc.md, concepts/adr-phased-control-plane.md, concepts/adr-message-contract-tsify.md, concepts/adr-capability-registry.md, concepts/adr-firmware-rust-embassy.md, concepts/adr-device-simulator.md, concepts/adr-ai-constrained-planner.md, concepts/adr-reproducible-run-artifacts.md, concepts/poc-mini-molecule-cloud-workbench.md, index.md
- notes: mini-molecule-poc — software-focused architect session (hardware literacy delegated to the build guide; success = works + Adom-faithful). Filed ARD + SAD + 7 ADRs: phased control plane (contract-as-invariant), tsify contract (JSON v1/postcard v2), self-describing capability registry, Rust+Embassy firmware, protocol-level device simulator, AI constrained sim-first planner, sim-replayable run artifacts. ADRs interlock: registry=AI guardrail, sim=AI sandbox+replay engine, contract=phasing reversibility. Forward links added to the POC concept page.

## [2026-06-22] migrate | Arche upgraded to current schema (spec page-type support)
- pages touched: SCHEMA.md, index.md
- notes: patched SCHEMA for /arche-specify — added Feature-specifications framing bullet + spec-vs-plan clause, spec row in page-types table, "Feature specs (spec)" section, `spec` in type enum, status/superseded_by extended to spec pages, `context_pages:` frontmatter field + spec frontmatter clause, `specify` log op + operations-summary entry, index description updated. Additive: created specs/ (.gitkeep) and added `## Specs` section to index.md. No content pages touched.

## [2026-06-22] specify | Project Foundation — POC repository skeleton
- pages touched: specs/spec-project-foundation.md, concepts/sad-mini-molecule-poc.md, index.md
- notes: spec-project-foundation — first POC increment as a trivial end-to-end vertical slice (contract → simulator advertises minimal self-ID → web displays it), single-source-of-truth contract banked day one (generated consumer + drift check), firmware build-only, hardware-free. 8 FRs, 6 SCs, 0 open clarifications (5 resolved). Grounded in SAD/ARD + 4 ADRs + hardware-literacy concept. SAD back-links to the spec. WHAT/WHY only — HOW deferred to existing SAD/ADRs.

## [2026-06-25] migrate | Arche upgraded to current schema (plan era + agent-context registration)
- pages touched: SCHEMA.md, index.md, AGENTS.md, CLAUDE.md, plans/.gitkeep
- notes: added plan page type (page-types row, Implementation-plans subsection, frontmatter type-enum + plan-only `spec:` field + status/superseded_by/context_pages coverage + plan frontmatter paragraph, framing-block bullet + revised execution-state carve-out, `plan` log op, plan operations-summary entry, index Plans section); created plans/ dir; registered Arche as first-class context source (snippet appended to AGENTS.md, @AGENTS.md bridge created in CLAUDE.md for Claude Code). Content pages left untouched.

## [2026-06-25] migrate | Arche schema reverted to current — spec/plan page types retired
- pages touched: SCHEMA.md, index.md, AGENTS.md
- notes: dropped the retired `spec` and `plan` conventions from SCHEMA (page-types rows, "Feature specs (spec)" + "Implementation plans (plan)" sections, framing-block bullets restored to dev-methodology carve-out, `spec`/`plan` removed from type enum + plan-only `spec:`/spec+plan frontmatter clauses, status/superseded_by/context_pages scoped back to architecture+discovery, `specify`/`plan` log ops + operations-summary entries removed, index section list trimmed); removed empty `## Plans` stub from index.md; refreshed stale `<!-- arche-context-source -->` snippet in AGENTS.md (removed arche-specify/arche-plan + specs/plans page references, added dev-methodology-grounding note). Manual attention left to user: specs/spec-project-foundation.md content page + its `## Specs` index entry, and the empty plans/ dir. No content pages touched.

## [2026-06-25] manual | Deleted retired spec page — spec-project-foundation
- pages touched: index.md, concepts/sad-mini-molecule-poc.md
- notes: deleted specs/spec-project-foundation.md (the lone legacy spec content page) at user request following the spec/plan retirement migration; removed its `## Specs` index entry and the SAD's `## See also` back-link (no broken links remain); removed the now-empty specs/ and plans/ directories. SAD `updated:` bumped.
