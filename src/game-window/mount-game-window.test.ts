import { describe, expect, it, vi } from "vitest";
import {
  bindSkillListActions,
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
      isLocked: false,
      rawData: 0,
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

  it("renders idle current action on a fresh save", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    renderGameWindowState(root, {
      activeSkill: "",
      skills: [
        {
          id: "scraping",
          level: 1,
          xp: 0,
          xpIntoLevel: 0,
          xpToNextLevel: 83,
          levelProgress: 0,
          isActive: false,
          isLocked: false,
          rawData: 0,
        },
      ],
      tokens: 0,
      totalLevel: 1,
      lastTickAt: 0,
    });

    expect(root.querySelector('[data-testid="current-action-name"]')?.textContent).toBe(
      "Idle",
    );
    expect(root.querySelector(".current-action--idle")).not.toBeNull();
    expect(root.querySelector('[data-testid="active-skill-indicator"]')).toBeNull();
    expect(root.querySelector('[data-testid="start-skill-scraping"]')).not.toBeNull();
  });

  it("calls onStartSkill when start button is clicked", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    renderGameWindowState(root, {
      activeSkill: "",
      skills: [
        {
          id: "scraping",
          level: 1,
          xp: 0,
          xpIntoLevel: 0,
          xpToNextLevel: 83,
          levelProgress: 0,
          isActive: false,
          isLocked: false,
          rawData: 0,
        },
      ],
      tokens: 0,
      totalLevel: 1,
      lastTickAt: 0,
    });

    const onStartSkill = vi.fn().mockResolvedValue(undefined);
    bindSkillListActions(root, onStartSkill);

    root.querySelector<HTMLButtonElement>('[data-testid="start-skill-scraping"]')?.click();

    expect(onStartSkill).toHaveBeenCalledWith("scraping");
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
          isLocked: false,
          rawData: 0,
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
    expect(root.querySelector('[data-testid="start-skill-scraping"]')).toBeNull();
    expect(root.querySelector('[data-testid="total-level"]')?.textContent).toBe("2");

    const progressBar = root.querySelector('[data-testid="skill-progress-scraping"]');
    expect(progressBar?.getAttribute("aria-valuenow")).toBe("50");

    const progressFill = root.querySelector(
      '[data-testid="skill-progress-fill-scraping"]',
    ) as HTMLElement | null;
    expect(progressFill?.style.width).toBe("50%");
  });

  it("renders locked skills with prerequisite text", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    renderGameWindowState(root, {
      activeSkill: "scraping",
      skills: [
        {
          id: "scraping",
          level: 4,
          xp: 0,
          xpIntoLevel: 0,
          xpToNextLevel: 83,
          levelProgress: 0,
          isActive: true,
          isLocked: false,
          rawData: 17,
        },
        {
          id: "labelling",
          level: 1,
          xp: 0,
          xpIntoLevel: 0,
          xpToNextLevel: 83,
          levelProgress: 0,
          isActive: false,
          isLocked: true,
          prerequisite: "Requires Scraping Lv 5",
          rawData: 0,
        },
      ],
      tokens: 12,
      totalLevel: 4,
      lastTickAt: 0,
    });

    expect(root.querySelector('[data-testid="token-balance"]')?.textContent).toBe("12");
    expect(root.querySelector('[data-testid="skill-locked-labelling"]')).not.toBeNull();
    expect(
      root.querySelector('[data-testid="skill-prerequisite-labelling"]')?.textContent?.trim(),
    ).toBe("Requires Scraping Lv 5");
    expect(root.querySelector('[data-testid="start-skill-labelling"]')).toBeNull();
  });

  it("renders scraping raw data on the skill row", () => {
    const root = document.createElement("div");
    mountGameWindow(root);

    renderGameWindowState(root, {
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
          isLocked: false,
          rawData: 42,
        },
        {
          id: "labelling",
          level: 1,
          xp: 0,
          xpIntoLevel: 0,
          xpToNextLevel: 83,
          levelProgress: 0,
          isActive: false,
          isLocked: true,
          prerequisite: "Requires Scraping Lv 5",
          rawData: 0,
        },
      ],
      tokens: 0,
      totalLevel: 2,
      lastTickAt: 0,
    });

    expect(
      root.querySelector('[data-testid="skill-raw-data-scraping"]')?.textContent?.trim(),
    ).toBe("Raw data: 42");
    expect(root.querySelector('[data-testid="skill-raw-data-labelling"]')).toBeNull();
  });
});
