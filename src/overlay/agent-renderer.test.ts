import { describe, expect, it } from "vitest";
import {
  createLayerGraphic,
  layerPosition,
} from "./agent-renderer";
import { composeAgentLayers } from "./equipment-compositor";

describe("createLayerGraphic", () => {
  it("returns a PixiJS graphics object sized for the layer descriptor", () => {
    const [body] = composeAgentLayers({
      formStage: 1,
      totalLevel: 1,
      activeSkill: "",
      skills: [{ id: "scraping", level: 1 }],
    });
    const graphic = createLayerGraphic(body);

    expect(graphic.width).toBe(80);
    expect(graphic.height).toBe(96);
  });
});

describe("layerPosition", () => {
  it("offsets equipment layers by skill", () => {
    const layers = composeAgentLayers({
      formStage: 1,
      totalLevel: 12,
      activeSkill: "scraping",
      skills: [{ id: "scraping", level: 12 }],
    });
    const equipment = layers.find((layer) => layer.kind === "equipment");

    expect(equipment).toBeDefined();
    expect(layerPosition(equipment!)).toEqual({ x: -18, y: -8 });
  });
});
