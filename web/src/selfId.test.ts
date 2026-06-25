import { describe, it, expect } from "vitest";
import { parseSelfId } from "./selfId";

describe("parseSelfId", () => {
  it("parses a SelfId frame into a typed object", () => {
    const raw =
      '{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[]}';
    const parsed = parseSelfId(raw);
    expect(parsed.id).toBe("mol-001");
    expect(parsed.name).toBe("Mini-Molecule");
    expect(parsed.capabilities).toEqual([]);
  });
});
