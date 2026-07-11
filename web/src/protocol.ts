import type { SelfId } from "./contract.gen";

export type ParsedMsg =
  | { kind: "selfid"; selfId: SelfId }
  | { kind: "ack"; ok: boolean; error: string | null }
  | { kind: "unknown" };

/** Route a raw WebSocket frame by its "type" tag. */
export function parseDeviceMsg(raw: string): ParsedMsg {
  // Serial frames are forwarded verbatim, so a non-JSON line (truncated UTF-8,
  // line noise, a stray log line) can arrive. Never let it throw out of onmessage.
  let obj: Record<string, unknown>;
  try {
    obj = JSON.parse(raw) as Record<string, unknown>;
  } catch {
    return { kind: "unknown" };
  }
  if (obj.type === "selfid") {
    return { kind: "selfid", selfId: obj as unknown as SelfId };
  }
  if (obj.type === "ack") {
    return { kind: "ack", ok: Boolean(obj.ok), error: (obj.error as string | null) ?? null };
  }
  return { kind: "unknown" };
}

/** Build the addressed GPIO set command as a JSON line. */
export function encodeSetGpio(channel: number, on: boolean): string {
  return JSON.stringify({ capability: "gpio", channel, op: "set", args: [on] });
}

/** Derive the GPIO control descriptor from the announcement, if any. */
export function gpioCapability(selfId: SelfId): { channels: number } | null {
  // A partial/older announce may omit `capabilities`; don't iterate a non-array.
  const caps = selfId.capabilities;
  if (!Array.isArray(caps)) return null;
  for (const cap of caps as Array<Record<string, unknown>>) {
    if (cap.kind === "gpio") {
      return { channels: Number(cap.channels) };
    }
  }
  return null;
}
