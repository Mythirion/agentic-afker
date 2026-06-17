import { describe, expect, it } from "vitest";
import {
  activityAnimationForSkill,
  activityOffset,
} from "./animation-controller";

describe("activityAnimationForSkill", () => {
  it("maps each active skill to a distinct activity animation", () => {
    expect(activityAnimationForSkill("scraping")).toBe("scrape");
    expect(activityAnimationForSkill("labelling")).toBe("label");
    expect(activityAnimationForSkill("fine-tuning")).toBe("tune");
    expect(activityAnimationForSkill("")).toBe("idle");
  });
});

describe("activityOffset", () => {
  it("returns no movement for idle", () => {
    expect(activityOffset("idle", 250)).toEqual({ x: 0, y: 0 });
  });

  it("oscillates scraping activity horizontally", () => {
    expect(activityOffset("scrape", 250).x).not.toBe(0);
    expect(activityOffset("scrape", 250).y).toBe(0);
  });
});
