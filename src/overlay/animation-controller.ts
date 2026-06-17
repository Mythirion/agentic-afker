export type ActivityAnimation = "idle" | "scrape" | "label" | "tune";

export function activityAnimationForSkill(activeSkill: string): ActivityAnimation {
  switch (activeSkill) {
    case "scraping":
      return "scrape";
    case "labelling":
      return "label";
    case "fine-tuning":
      return "tune";
    default:
      return "idle";
  }
}

export function activityOffset(animation: ActivityAnimation, elapsedMs: number): { x: number; y: number } {
  if (animation === "idle") {
    return { x: 0, y: 0 };
  }

  const phase = (elapsedMs % 1_000) / 1_000;
  const bob = Math.sin(phase * Math.PI * 2) * 4;

  switch (animation) {
    case "scrape":
      return { x: bob, y: 0 };
    case "label":
      return { x: 0, y: bob };
    case "tune":
      return { x: bob * 0.5, y: -bob * 0.5 };
    default:
      return { x: 0, y: 0 };
  }
}
