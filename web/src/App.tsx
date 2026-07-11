import { useEffect, useRef, useState } from "react";
import type { SelfId } from "./contract.gen";
import { parseDeviceMsg, encodeSetGpio, gpioCapability } from "./protocol";

const ENDPOINTS = {
  simulator: "ws://127.0.0.1:8765",
  device: "ws://127.0.0.1:8766",
} as const;
type Target = keyof typeof ENDPOINTS;

export default function App() {
  const [target, setTarget] = useState<Target>("simulator");
  const [selfId, setSelfId] = useState<SelfId | null>(null);
  const [status, setStatus] = useState("connecting…");
  const [ack, setAck] = useState<string>("");
  const [on, setOn] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    setSelfId(null);
    setStatus("connecting…");
    const ws = new WebSocket(ENDPOINTS[target]);
    wsRef.current = ws;
    ws.onopen = () => setStatus("connected");
    ws.onmessage = (ev) => {
      const msg = parseDeviceMsg(ev.data as string);
      if (msg.kind === "selfid") setSelfId(msg.selfId);
      else if (msg.kind === "ack") setAck(msg.ok ? "ok ✓" : `error: ${msg.error}`);
    };
    ws.onerror = () => setStatus("disconnected");
    ws.onclose = () => setStatus((s) => (s === "connected" ? "disconnected" : s));
    return () => ws.close();
  }, [target]);

  const gpio = selfId ? gpioCapability(selfId) : null;

  function toggle() {
    const next = !on;
    setOn(next);
    wsRef.current?.send(encodeSetGpio(0, next));
  }

  return (
    <main style={{ fontFamily: "system-ui", padding: "2rem" }}>
      <h1>Mini-Molecule Workbench</h1>
      <label>
        source:{" "}
        <select value={target} onChange={(e) => setTarget(e.target.value as Target)}>
          <option value="simulator">simulator (no hardware)</option>
          <option value="device">device (bridge)</option>
        </select>
      </label>
      <p>status: {status}</p>
      {selfId ? (
        <>
          <dl>
            <dt>id</dt>
            <dd data-testid="id">{selfId.id}</dd>
            <dt>name</dt>
            <dd data-testid="name">{selfId.name}</dd>
          </dl>
          {gpio ? (
            <button data-testid="led-toggle" onClick={toggle}>
              LED (ch0): turn {on ? "off" : "on"}
            </button>
          ) : (
            <p>no gpio capability advertised</p>
          )}
          {ack && <p data-testid="ack">last command: {ack}</p>}
        </>
      ) : (
        <p>waiting for the molecule to announce itself…</p>
      )}
    </main>
  );
}
