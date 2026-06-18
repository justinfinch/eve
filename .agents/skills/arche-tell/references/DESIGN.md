# Design

This skill **designs** HTML; it doesn't fill in templates. The output is one file, self-contained, opens by double-click, survives being emailed.

Pick the simplest tool that fits the content. Default to taste — match the audience, not a house style.

## Self-contained discipline

- One `.html` file. No build step. No bundler. No external CSS/JS files in the same directory.
- CDN allowed for libraries (reveal.js, Chart.js, Mermaid, Tailwind play CDN). Pin a major version.
- If the recipient may be offline, say so up front and vendor what you'd otherwise CDN.
- Images: prefer linking external URLs over inlining; for designer-owned brand assets, data-URI base64 is acceptable but bloats the file.
- The HTML must work when opened as `file://` — no fetch-relative dependencies that fail without a server.

## Two output shapes

`format: deck` and `format: narrative` are the only top-level shapes. The *style* inside each is the skill's choice per story — describe it in the story page body, not in frontmatter.

### Deck — paginated, presenter-controlled

Pick the underlying framework based on the deck's job:

| Framework        | Strengths                                                         | Pick when                                                   |
| :--------------- | :---------------------------------------------------------------- | :---------------------------------------------------------- |
| reveal.js (CDN)  | Mature; speaker notes, fragments, themes, hash nav, presenter mode | Default. Most decks. Hybrid live-then-circulated.           |
| impress.js (CDN) | 3D transforms, non-linear paths, bold spatial concept             | Vision / transformation pitches where the visual *is* the message |
| Plain CSS slides | Zero JS; full control; tiny file                                  | ≤8 slides, no presenter mode needed, hostile email filters   |

reveal.js example shape:
```html
<div class="reveal"><div class="slides">
  <section>...title...</section>
  <section>...content...<aside class="notes">speaker note</aside></section>
</div></div>
```

Plain CSS slides example shape (each `<section>` is one viewport):
```css
section { min-height: 100vh; padding: 4rem; page-break-after: always; }
```

Slidev, Marp, MDX-deck and similar are **not** in scope — they require a build step.

### Narrative — scrollable, reader-controlled

Pick the shape based on audience expectation:

| Shape         | Length         | Typography density | Best for                                        |
| :------------ | :------------- | :----------------- | :---------------------------------------------- |
| Memo          | 300–800 words  | Tight, single column, no headings until §2 | Exec / board single-pager (Bezos-memo feel)     |
| Long-form post | 800–2500 words | Headings every 2–3 paras; serif body OK   | Technical audience reading async                |
| Landing page  | 1500–4000 words | Hero + sections + CTA; image-friendly | Mixed audience needing emotional + content beat |
| Report        | 2500+ words    | Numbered sections, exec summary, formal | Compliance, board pack, archive                 |

No framework needed for narrative — plain HTML + CSS is enough. Tailwind via the play CDN is fine if the design wants utility classes; otherwise hand-write the CSS.

## Typography

- Default to a system font stack for sans: `-apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif`.
- For editorial narratives (memo, long-form post, report), a serif body reads as more considered. Use the system serif stack: `ui-serif, Georgia, Cambria, "Times New Roman", serif`.
- Code: `ui-monospace, SFMono-Regular, Menlo, monospace`.
- Headings: tight letter-spacing (`-0.01em`), line-height `1.2`–`1.3`.
- Body: line-height `1.55`–`1.7`, base size `16`–`18px` for narrative, larger for decks.
- Cap narrative line length at `60`–`75ch`. Let decks breathe wider.
- Two type sizes minimum, four maximum, across the whole document.

## Color

- Respect `prefers-color-scheme`. Provide both a light and dark variable set unless you have a deliberate reason not to.
- Avoid pure black and pure white. `#111` / `#fafafa` look more designed.
- One accent color per story, picked to match emotional valence: cool blue for professional/neutral, warm amber/red for urgency, green for success/progress, purple/teal for novelty.
- Don't rely on color alone for signal. Pair with weight, size, or rule lines.
- Contrast ratio ≥ 4.5:1 for body text. ≥ 3:1 for large text.

## Visual rhythm

- Generous whitespace beats density. A deck with breathing room communicates more than one crammed full.
- One `<h1>` per page or slide.
- Rule lines (`border-top`/`border-bottom`) for section breaks in narratives.
- Slide breaks in decks — one idea per slide; nesting via reveal.js vertical-slide only when a section earns it.

## Hero / opening pattern

The first thing the audience sees. Used at the top of any story:

- Audience tag — small, letter-spaced, dim (e.g. `FOR PLATFORM ARB · 2026-06-09`)
- Title — large, tight letter-spacing, one line if possible
- Subtitle — one sentence; the "so what"
- Optional: presenter / author, date, story-version

## Closing / ask pattern

Every story ends with the action ask, visually distinct from the body:

- Single sentence; concrete verb + date + owner
- Accent color, larger type, framed by rule lines or background tint
- For decks: dedicated final slide
- For narratives: a `<p class="ask">` or `<aside>` near the end

## Diagrams — the tool palette

**No tool is mandatory.** Pick by fit.

| Tool                  | Strengths                                          | Best for                                                   | Tradeoffs                                |
| :-------------------- | :------------------------------------------------- | :--------------------------------------------------------- | :--------------------------------------- |
| Mermaid (CDN)         | Declarative; many diagram types                    | Process / sequence / state / ER — standard diagram shapes  | Auto-layout; defaults look generic       |
| Inline SVG            | Full control; lightweight; no JS                   | Branded visuals, architecture topologies, custom shapes    | More authoring effort                    |
| CSS + HTML semantic   | Tiny; accessible; themeable                        | Layered architecture stacks, before/after grids, comparisons | Limited to box-and-line                  |
| Chart.js (CDN)        | Easy data viz with sensible defaults               | Metrics, growth curves, bar/line comparisons               | Adds a JS dep                            |
| D3.js (CDN)           | Bespoke data viz                                   | Custom data shapes (sankey, force-directed, treemap)       | Author overhead                          |
| ASCII in `<pre>`      | Zero dependencies; terminal-native feel            | CLI stories, protocol diagrams, file format layouts        | Limited expressive range                 |
| Embedded image        | Use when canonical brand visuals already exist      | Designer-made architecture diagrams, marketing visuals     | Bloats file; risks staleness vs. source  |

### Picking a diagram tool

- Already exists in a cited SAD/ADR? **Copy through verbatim** in whatever form it's in. Story stays aligned with the canonical artifact. Don't re-author.
- Process / sequence / state / ER → **Mermaid** unless the visual identity matters.
- Architecture layers / before-vs-after / quadrant / matrix → **CSS + HTML** semantic markup.
- Topology with non-rectangular shapes, branded colors, specific spatial relationships → **inline SVG**.
- Quantitative — metrics, deltas, distributions → **Chart.js** (or D3 if Chart.js's defaults don't fit).
- "Imagine a terminal" / "this is the wire format" → **ASCII in `<pre>`**.
- The Arche has a canonical image already → **embed it**.

### Diagram patterns

**Mermaid flowchart:**
```html
<pre class="mermaid">
flowchart LR
  A[Client] --> B[API] --> C[(DB)]
</pre>
```

**CSS layered stack** (no diagram library):
```html
<div class="stack">
  <div class="layer" data-label="UI">React</div>
  <div class="layer" data-label="API">Node + Express</div>
  <div class="layer" data-label="Store">Postgres</div>
</div>
```

**ASCII protocol diagram:**
```html
<pre>
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Source Port             |     Destination Port            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
</pre>
```

**Inline SVG topology** (skeleton):
```html
<svg viewBox="0 0 800 300" role="img" aria-label="Billing topology">
  <rect x="20" y="20" width="160" height="80" rx="8" fill="#0a4d8c"/>
  <text x="100" y="65" fill="white" text-anchor="middle">Edge</text>
  <line x1="180" y1="60" x2="320" y2="60" stroke="#888" stroke-width="2"/>
  <!-- ... -->
</svg>
```

**Chart.js bar:**
```html
<canvas id="latency"></canvas>
<script>
  new Chart(document.getElementById('latency'), {
    type: 'bar',
    data: { labels: ['p50','p95','p99'], datasets: [{ data: [12, 84, 210] }] }
  });
</script>
```

### Diagram anti-patterns

- Defaulting to Mermaid because it's familiar. Pick for the content.
- Diagram-per-slide / diagram-per-section. Noise. Typically 1–3 per deck, 2–5 per narrative.
- Recreating a diagram that already lives in a cited SAD/ADR. Copy through; stay aligned with the canonical source.
- Branded marketing-polish visuals. The Arche's role is correctness, not pixels — hand off to a designer when polish is the requirement.

## Citation treatment

- Inline links are sufficient. Do not burden every claim with a footnote number.
- Decks: small dim footer per slide with the cited slug — e.g. `sad-billing · adr-event-driven-billing`.
- Narratives: inline links — `as committed in [sad-billing](../concepts/sad-billing.md)`.
- Quote sparingly; paraphrase and cite.

## Accessibility minimums

- Color contrast: 4.5:1 body, 3:1 large text.
- Real semantic headings (`<h1>`/`<h2>`), not styled paragraphs.
- Diagrams: `aria-label` or adjacent caption when the visual carries the claim.
- Decks: slide titles are real headings.
- Tested at default font size; respects user zoom.

## Examples worth reading

- **Tufte CSS** (`https://edwardtufte.github.io/tufte-css/`) — editorial narrative typography.
- **reveal.js demo deck** (`https://revealjs.com/demo`) — what a clean deck looks like.
- **Stripe's docs** — restrained, content-first, accent-color discipline.
- **Bret Victor essays** — narrative + interactive diagrams, single self-contained pages.

The skill does not load these. They're calibration references for taste.
