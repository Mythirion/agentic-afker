import { describe, expect, it } from "vitest";
import {
  GAME_WINDOW_HEADER,
  mountGameWindow,
} from "./mount-game-window";

describe("mountGameWindow", () => {
  it("renders stub header with Token balance and Total Level", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    expect(root.querySelector(".game-header")).not.toBeNull();
    expect(root.textContent).toContain(GAME_WINDOW_HEADER.tokenBalanceLabel);
    expect(root.textContent).toContain(GAME_WINDOW_HEADER.totalLevelLabel);
    expect(root.querySelector('[data-testid="token-balance"]')?.textContent).toBe(
      "—",
    );
    expect(root.querySelector('[data-testid="total-level"]')?.textContent).toBe(
      "—",
    );
  });
});
