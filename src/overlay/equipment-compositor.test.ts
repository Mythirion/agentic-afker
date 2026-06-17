import { describe, expect, it } from "vitest";
import {
  composeAgentLayers,
  type AgentAppearanceInput,
} from "./equipment-compositor";

const freshStart: AgentAppearanceInput = {
  formStage: 1,
  totalLevel: 1,
  activeSkill: "",
  skills: [{ id: "scraping", level: 1 }],
};

describe("composeAgentLayers", () => {
  it("includes a form stage body layer as the base sprite", () => {
    const layers = composeAgentLayers(freshStart);

    expect(layers[0]).toMatchObject({
      kind: "body",
      spriteId: "body-stage-1",
      zIndex: 0,
    });
  });

  it("uses the stage 2 body when total level crosses the configured threshold", () => {
    const layers = composeAgentLayers({
      ...freshStart,
      formStage: 1,
      totalLevel: 30,
    });

    expect(layers.find((layer) => layer.kind === "body")).toMatchObject({
      spriteId: "body-stage-2",
      color: "#00b894",
    });
  });

  it("adds equipment layers at skill level milestones", () => {
    const layers = composeAgentLayers({
      formStage: 1,
      totalLevel: 12,
      activeSkill: "scraping",
      skills: [
        { id: "scraping", level: 12 },
        { id: "labelling", level: 1 },
      ],
    });

    expect(layers.filter((layer) => layer.kind === "equipment")).toEqual([
      expect.objectContaining({
        spriteId: "scraping-equipment-1",
        skillId: "scraping",
      }),
    ]);
  });

  it("upgrades equipment tier at higher skill milestones", () => {
    const layers = composeAgentLayers({
      formStage: 1,
      totalLevel: 22,
      activeSkill: "labelling",
      skills: [{ id: "labelling", level: 22 }],
    });

    expect(layers.find((layer) => layer.kind === "equipment")).toMatchObject({
      spriteId: "labelling-equipment-2",
    });
  });

  it("adds an activity layer for the active skill", () => {
    const layers = composeAgentLayers({
      ...freshStart,
      activeSkill: "fine-tuning",
      skills: [
        { id: "scraping", level: 1 },
        { id: "labelling", level: 1 },
        { id: "fine-tuning", level: 1 },
      ],
    });

    expect(layers.find((layer) => layer.kind === "activity")).toMatchObject({
      spriteId: "activity-fine-tuning",
      animation: "tune",
      skillId: "fine-tuning",
    });
  });
});
