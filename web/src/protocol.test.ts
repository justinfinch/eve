import { describe, it, expect } from "vitest";
import { parseDeviceMsg, encodeSetGpio, gpioCapability } from "./protocol";

describe("parseDeviceMsg", () => {
  it("routes a selfid frame", () => {
    const raw =
      '{"type":"selfid","id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[{"kind":"gpio","channels":1,"ops":["set"]}]}';
    const msg = parseDeviceMsg(raw);
    expect(msg.kind).toBe("selfid");
    if (msg.kind === "selfid") {
      expect(msg.selfId.id).toBe("mol-001");
      expect(gpioCapability(msg.selfId)).toEqual({ channels: 1 });
    }
  });

  it("routes an ack frame", () => {
    const msg = parseDeviceMsg('{"type":"ack","ok":true,"error":null}');
    expect(msg).toEqual({ kind: "ack", ok: true, error: null });
  });

  it("returns unknown for a non-JSON frame instead of throwing", () => {
    // Serial lines are forwarded verbatim; garbage must not throw out of onmessage.
    expect(() => parseDeviceMsg("not json <<")).not.toThrow();
    expect(parseDeviceMsg("not json <<")).toEqual({ kind: "unknown" });
  });
});

describe("encodeSetGpio", () => {
  it("builds the addressed command line", () => {
    expect(encodeSetGpio(0, true)).toBe(
      '{"capability":"gpio","channel":0,"op":"set","args":[true]}'
    );
  });
});

describe("gpioCapability", () => {
  it("returns null when no gpio is advertised", () => {
    const selfId = { id: "x", name: "x", fw_version: "0", capabilities: [] };
    expect(gpioCapability(selfId as never)).toBeNull();
  });

  it("returns null (no throw) when capabilities is absent", () => {
    const selfId = { id: "x", name: "x", fw_version: "0" };
    expect(() => gpioCapability(selfId as never)).not.toThrow();
    expect(gpioCapability(selfId as never)).toBeNull();
  });
});
