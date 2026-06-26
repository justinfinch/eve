import type { SelfId } from "./contract.gen";

/** Parse a raw WebSocket frame into the generated SelfId type. */
export function parseSelfId(raw: string): SelfId {
  return JSON.parse(raw) as SelfId;
}
