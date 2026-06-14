import { describe, expect, it } from "vitest";
import {
  GAME_WINDOW_HEADER,
  mountGameWindow,
  renderGameWindowState,
  type GameWindowSnapshot,
} from "./mount-game-window";

const scrapingSnapshot: GameWindowSnapshot = {
  activeSkill: "scraping",
  skills: [
    {
      id: "scraping",
      level: 2,
      xp: 150,
      isActive: true,
    },
  ],
  tokens: 0,
  totalLevel: 2,
  lastTickAt: 1_000,
};

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

  it("renders scraping level, xp, and active-skill indicator from snapshot", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    renderGameWindowState(root, scrapingSnapshot);

    expect(root.querySelector('[data-testid="skill-level-scraping"]')?.textContent).toBe(
      "Lv 2",
    );
    expect(root.querySelector('[data-testid="skill-xp-scraping"]')?.textContent).toBe(
      "150 XP",
    );
    expect(root.querySelector('[data-testid="active-skill-indicator"]')).not.toBeNull();
    expect(root.querySelector('[data-testid="total-level"]')?.textContent).toBe("2");
  });
});
