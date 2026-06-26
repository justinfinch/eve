# Lesson 5 — The Web Workbench

## What you'll learn

How the browser app connects to the simulator, receives the `SelfId` JSON, and renders
it — all while using the **generated** TypeScript type so the compiler checks that the
browser and the board agree.

## The concept

The `web/` folder is a standard front-end app built with three common tools:

- **Vite** — the build tool and dev server. It serves your app in development with
  instant reloads, and bundles it for production.
- **React** — the UI library. You write components (functions that return markup) and
  React keeps the screen in sync with your data.
- **TypeScript** — JavaScript with types. The types catch mistakes (like reading a field
  that doesn't exist) before you ever run the code.

The workbench's job in the Foundation is tiny: connect to `ws://127.0.0.1:8765`, wait for
the one `SelfId` frame, and show its `id` and `name`. But it does so **type-safely**,
using the `SelfId` type generated in [Lesson 4](04-codegen.md).

## Walk the code

The app is mostly the standard Vite React-TS scaffold. The two files that matter for the
Foundation are `selfId.ts` (parse the message) and `App.tsx` (display it).

### `web/src/selfId.ts` — parsing with the generated type

```ts
import type { SelfId } from "./contract.gen";

/** Parse a raw WebSocket frame into the generated SelfId type. */
export function parseSelfId(raw: string): SelfId {
  return JSON.parse(raw) as SelfId;
}
```

This is the join point between Rust and TypeScript. `import type { SelfId } from
"./contract.gen"` pulls in the **generated** type from [Lesson 4](04-codegen.md). The
`import type` form means "I only need this for type-checking" — it disappears at runtime.

`parseSelfId` takes the raw JSON string off the wire and returns it typed as `SelfId`. If
the Rust contract changes the shape, this file is type-checked against the regenerated
`contract.gen.ts`, so a mismatch becomes a compile error here.

### `web/src/App.tsx` — connect and render

```tsx
import { useEffect, useState } from "react";
import type { SelfId } from "./contract.gen";
import { parseSelfId } from "./selfId";

const SIM_URL = "ws://127.0.0.1:8765";   // matches the simulator from Lesson 3
```

The URL is exactly what the simulator binds to in [Lesson 3](03-the-simulator.md).

```tsx
export default function App() {
  const [selfId, setSelfId] = useState<SelfId | null>(null);
  const [status, setStatus] = useState("connecting…");
```

Two pieces of React **state**: the `selfId` we receive (starts `null`), and a `status`
string. `useState` gives you a current value plus a setter; calling the setter re-renders
the component with the new value.

```tsx
  useEffect(() => {
    const ws = new WebSocket(SIM_URL);
    ws.onmessage = (ev) => {
      setSelfId(parseSelfId(ev.data as string));
      setStatus("connected");
    };
    ws.onerror = () => setStatus("disconnected");
    ws.onclose = () => setStatus((s) => (s === "connected" ? s : "disconnected"));
    return () => ws.close();
  }, []);
```

`useEffect(..., [])` runs once when the component first appears. It opens a **browser
WebSocket** to the simulator. When a message arrives (`onmessage`), it parses the frame
with `parseSelfId` and stores it, flipping status to `"connected"`. The returned
`() => ws.close()` is cleanup — React calls it if the component goes away, so we don't
leak the connection.

Note this is the **same WebSocket protocol** the simulator speaks — but here it's the
browser's built-in `WebSocket`, no library needed.

```tsx
  return (
    <main style={{ fontFamily: "system-ui", padding: "2rem" }}>
      <h1>Mini-Molecule Workbench</h1>
      <p>status: {status}</p>
      {selfId ? (
        <dl>
          <dt>id</dt>   <dd data-testid="id">{selfId.id}</dd>
          <dt>name</dt> <dd data-testid="name">{selfId.name}</dd>
        </dl>
      ) : (
        <p>waiting for the molecule to announce itself…</p>
      )}
    </main>
  );
}
```

The markup: a heading, the current status, and — once `selfId` is set — its `id` and
`name`. Until then it shows "waiting…". Because `selfId` is typed as `SelfId`,
TypeScript guarantees `selfId.id` and `selfId.name` exist; a typo like `selfId.nme`
would fail to compile.

### `web/src/selfId.test.ts` — a unit test

```ts
import { describe, it, expect } from "vitest";
import { parseSelfId } from "./selfId";

describe("parseSelfId", () => {
  it("parses a SelfId frame into a typed object", () => {
    const raw = '{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[]}';
    const parsed = parseSelfId(raw);
    expect(parsed.id).toBe("mol-001");
    expect(parsed.name).toBe("Mini-Molecule");
    expect(parsed.capabilities).toEqual([]);
  });
});
```

This uses **vitest** (a test runner that pairs with Vite). It feeds `parseSelfId` the
exact JSON the simulator produces and checks the fields come out right. Note the raw
string here is identical to the JSON the Rust test pins in [Lesson 2](02-the-contract.md)
— both ends are testing against the same wire format.

## Run it yourself

Run the web unit tests:

```bash
devbox run -- npm --prefix web test
```

(`--prefix web` tells npm to operate inside the `web/` folder.)

Type-check and build the production bundle:

```bash
devbox run -- npm --prefix web run build
```

To see it live, you need the simulator running too — that's what `just dev` does in one
shot:

```bash
devbox run -- just dev
```

Open the URL Vite prints (typically `http://localhost:5173`) in **Chrome**. You'll see
`status: connected`, `id: mol-001`, `name: Mini-Molecule`. That's the byte the simulator
sent, displayed in the browser. [Lesson 8](08-running-everything.md) walks this through
in full.

## Recap

- The `web/` app is a standard **Vite + React + TypeScript** project.
- **`selfId.ts`** parses the wire JSON into the **generated** `SelfId` type — the bridge
  back to the Rust contract.
- **`App.tsx`** opens a browser WebSocket to the simulator, parses the frame, and renders
  `id` + `name`, all type-checked.
- Because the type is generated, the browser literally can't disagree with the board
  about the message shape.

**Next:** [Lesson 6 — The firmware](06-the-firmware.md): the real embedded Rust that
proves this same contract compiles for an actual microcontroller.
</content>
