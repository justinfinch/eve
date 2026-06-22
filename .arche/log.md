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
