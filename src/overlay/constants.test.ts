import { describe, expect, it } from "vitest";
import { AGENT_PLACEHOLDER, OVERLAY_SIZE } from "./constants";

describe("overlay constants", () => {
  it("defines the Agent Overlay viewport size", () => {
    expect(OVERLAY_SIZE).toEqual({ width: 300, height: 200 });
  });

  it("defines a visible placeholder Agent appearance", () => {
    expect(AGENT_PLACEHOLDER.width).toBeGreaterThan(0);
    expect(AGENT_PLACEHOLDER.height).toBeGreaterThan(0);
    expect(AGENT_PLACEHOLDER.color).toMatch(/^#[0-9a-f]{6}$/i);
  });
});
