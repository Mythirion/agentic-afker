import { describe, expect, it } from "vitest";
import { createAgentPlaceholder } from "./agent-renderer";

describe("createAgentPlaceholder", () => {
  it("returns a PixiJS graphics object sized for the placeholder Agent", () => {
    const agent = createAgentPlaceholder();

    expect(agent.width).toBe(80);
    expect(agent.height).toBe(96);
  });
});
