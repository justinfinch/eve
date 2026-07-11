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
  // The level our last unacked command requested; `on` only commits once the
  // device acks it, so the UI never claims a change the device rejected/ignored.
  const pendingRef = useRef<boolean | null>(null);

  useEffect(() => {
    // Reset all device-derived state for the new source: a freshly-connected
    // molecule boots with its LED off and hasn't announced or acked anything yet.
    setSelfId(null);
    setStatus("connecting…");
    setAck("");
    setOn(false);
    pendingRef.current = null;

    // `disposed` guards against the source changing / component unmounting while a
    // reconnect timer is pending; `attempt` drives exponential backoff.
    let disposed = false;
    let retryTimer: ReturnType<typeof setTimeout> | undefined;
    let attempt = 0;

    const connect = () => {
      if (disposed) return;
      const ws = new WebSocket(ENDPOINTS[target]);
      wsRef.current = ws;
      // A stale socket (from a prior connection/source) can still deliver events
      // asynchronously; ignore anything not from the current socket.
      const isCurrent = () => wsRef.current === ws;
      // Socket open ≠ molecule present; "connected" is asserted only once we see a
      // SelfId announce, so it never contradicts "waiting to announce…".
      ws.onopen = () => {
        if (!isCurrent()) return;
        attempt = 0;
        setStatus("link up — waiting for announce…");
      };
      ws.onmessage = (ev) => {
        if (!isCurrent()) return;
        const msg = parseDeviceMsg(ev.data as string);
        if (msg.kind === "selfid") {
          setSelfId(msg.selfId);
          setStatus("connected");
        } else if (msg.kind === "ack") {
          setAck(msg.ok ? "ok ✓" : `error: ${msg.error}`);
          // Commit the requested level only on a successful ack.
          if (msg.ok && pendingRef.current !== null) setOn(pendingRef.current);
          pendingRef.current = null;
        }
      };
      // A failed connection fires onerror then onclose; let onclose drive retries.
      ws.onclose = () => {
        // `disposed` is the load-bearing guard: sockets are created serially, so a
        // live socket's onclose is always still current — only teardown stops retries.
        if (disposed) return;
        // The device/bridge is gone: drop its announce so the LED control
        // disappears and can't be clicked against a closed socket.
        setSelfId(null);
        pendingRef.current = null;
        // Auto-reconnect: a device reboot drops the bridge's serial port, which
        // closes us. The firmware re-announces every 1s, so a reconnect recovers
        // on its own once the device is back. Back off up to 5s between tries.
        const delay = Math.min(1000 * 2 ** attempt, 5000);
        attempt += 1;
        setStatus("disconnected — reconnecting…");
        retryTimer = setTimeout(connect, delay);
      };
    };

    connect();

    return () => {
      disposed = true;
      if (retryTimer) clearTimeout(retryTimer);
      wsRef.current?.close();
    };
  }, [target]);

  const gpio = selfId ? gpioCapability(selfId) : null;

  function toggle() {
    if (wsRef.current?.readyState !== WebSocket.OPEN) return;
    const next = !on;
    pendingRef.current = next;
    wsRef.current.send(encodeSetGpio(0, next));
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
