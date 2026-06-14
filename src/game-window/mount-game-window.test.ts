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
      xpIntoLevel: 50,
      xpToNextLevel: 300,
      levelProgress: 50 / 300,
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

    it("renders scraping level, xp, active indicator, and progress bar from snapshot", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    const snapshot: GameWindowSnapshot = {
      ...scrapingSnapshot,
      skills: [
        {
          id: "scraping",
          level: 2,
          xp: 175,
          xpIntoLevel: 92,
          xpToNextLevel: 92,
          levelProgress: 0.5,
          isActive: true,
        },
      ],
    };

    renderGameWindowState(root, snapshot);

    expect(root.querySelector('[data-testid="current-action-name"]')?.textContent).toBe(
      "Scraping",
    );
    expect(root.querySelector('[data-testid="current-action-progress"]')).not.toBeNull();

    expect(root.querySelector('[data-testid="skill-level-scraping"]')?.textContent).toBe(
      "Lv 2",
    );
    expect(root.querySelector('[data-testid="skill-xp-scraping"]')?.textContent?.trim()).toBe(
      "92 / 92 XP",
    );
    expect(root.querySelector('[data-testid="active-skill-indicator"]')).not.toBeNull();
    expect(root.querySelector('[data-testid="total-level"]')?.textContent).toBe("2");

    const progressBar = root.querySelector('[data-testid="skill-progress-scraping"]');
    expect(progressBar?.getAttribute("aria-valuenow")).toBe("50");

    const progressFill = root.querySelector(
      '[data-testid="skill-progress-fill-scraping"]',
    ) as HTMLElement | null;
    expect(progressFill?.style.width).toBe("50%");
  });
});
