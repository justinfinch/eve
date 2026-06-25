import { useEffect, useState } from "react";
import type { SelfId } from "./contract.gen";
import { parseSelfId } from "./selfId";

const SIM_URL = "ws://127.0.0.1:8765";

export default function App() {
  const [selfId, setSelfId] = useState<SelfId | null>(null);
  const [status, setStatus] = useState("connecting…");

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

  return (
    <main style={{ fontFamily: "system-ui", padding: "2rem" }}>
      <h1>Mini-Molecule Workbench</h1>
      <p>status: {status}</p>
      {selfId ? (
        <dl>
          <dt>id</dt>
          <dd data-testid="id">{selfId.id}</dd>
          <dt>name</dt>
          <dd data-testid="name">{selfId.name}</dd>
        </dl>
      ) : (
        <p>waiting for the molecule to announce itself…</p>
      )}
    </main>
  );
}
