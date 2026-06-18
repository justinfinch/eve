---
type: story
title: {{TITLE}}
created: {{DATE}}
updated: {{DATE}}
tags: []
sources: []
audience: {{AUDIENCE}}
audience_depth: {{AUDIENCE_DEPTH}}    # arb | staff-eng | exec | board | all-hands | customer | internal | other
action_ask: {{ACTION_ASK}}
framework: {{FRAMEWORK}}              # pyramid | scqa | story-arc | before-after-bridge | pas
format: {{FORMAT}}                    # deck | narrative
time_budget: {{TIME_BUDGET}}          # e.g. "20min live" or "1500 words async"
html: assets/stories/{{SLUG}}.html
---

# {{TITLE}}

> **Audience.** {{AUDIENCE}} — {{AUDIENCE_DEPTH}}
> **The ask.** {{ACTION_ASK}}
> **Framework.** {{FRAMEWORK}}.  **Format.** {{FORMAT}}.  **Budget.** {{TIME_BUDGET}}.

## Outline

The story's spine, before expansion. Each section / slide gets a one-line claim and the Arche page it leans on. Replace with the actual framework skeleton from references/FRAMEWORKS.md.

1. Section title — one-line claim. ([cited page](../concepts/example.md))
2. ...

## Style

Name the choices that won't be obvious from the body:

- **Deck framework** (if `format: deck`) — reveal.js / impress.js / plain-CSS slides. Why.
- **Narrative shape** (if `format: narrative`) — memo / long-form / landing / report. Why.
- **Diagram tools used** — list per section: Mermaid, inline SVG, CSS+HTML, Chart.js, D3, ASCII, embedded image.
- **Tone / accent color / typography** — any deliberate departure from DESIGN.md defaults.

## Sections

### 1. {{SECTION_TITLE}}

The narrative for this section. Inline-cite Arche pages at the point of claim: per [Concept](../concepts/example.md), citing [Source](../sources/example.md).

**Speaker notes** (deck format only): what the presenter says that the slide does not.

Diagram (if any) — embed the source here in whatever form the chosen tool needs. Mermaid block, inline `<svg>`, ASCII `<pre>`, CSS-diagram HTML, or `embed: <image path>`. See DESIGN.md.

### 2. {{SECTION_TITLE}}

...

## The ask (closing)

Restate the action ask as a concrete verb with a date / owner / next step.

## See also

- [Pages cited inline in this story](../concepts/example.md)
- [Prior story for the same audience, if any](./previous-story.md)
