import { describe, expect, it } from "vitest";
import {
  actionProgress,
  formatActionProgressPercent,
  updateCurrentActionProgress,
} from "./action-progress";

describe("action progress", () => {
  it("returns zero at cycle start", () => {
    expect(actionProgress(0)).toBe(0);
  });

  it("returns one when a full tick has elapsed", () => {
    expect(actionProgress(1_000)).toBe(1);
  });

  it("clamps progress between zero and one", () => {
    expect(actionProgress(-50)).toBe(0);
    expect(actionProgress(2_000)).toBe(1);
    expect(actionProgress(500)).toBe(0.5);
  });

  it("formats progress as a whole-number percent", () => {
    expect(formatActionProgressPercent(0.456)).toBe(46);
  });

  it("updates the current action progress bar in the game window", () => {
    const root = document.createElement("div");
    root.innerHTML = `
      <div data-testid="current-action-progress" aria-valuenow="0">
        <div data-testid="current-action-fill" style="width: 0%"></div>
      </div>
    `;

    updateCurrentActionProgress(root, 0.25);

    const bar = root.querySelector('[data-testid="current-action-progress"]');
    const fill = root.querySelector<HTMLElement>(
      '[data-testid="current-action-fill"]',
    );

    expect(bar?.getAttribute("aria-valuenow")).toBe("25");
    expect(fill?.style.width).toBe("25%");
  });
});
