import { describe, expect, it } from "vitest";
import {
  createOpenGameWindowButton,
  interactButtonVisible,
  OPEN_GAME_WINDOW_BUTTON_ID,
} from "./interact-controls";

describe("interactButtonVisible", () => {
  it("is hidden when Interact Mode is off", () => {
    expect(interactButtonVisible(false)).toBe(false);
  });

  it("is visible when Interact Mode is on", () => {
    expect(interactButtonVisible(true)).toBe(true);
  });
});

describe("createOpenGameWindowButton", () => {
  it("creates a hidden open game window button", () => {
    const button = createOpenGameWindowButton();

    expect(button.id).toBe(OPEN_GAME_WINDOW_BUTTON_ID);
    expect(button.hidden).toBe(true);
  });
});
