import { describe, expect, it, vi } from "vitest";
import {
  bindDevMenuControls,
  mountDevMenuShell,
  renderDevMenu,
  renderDevMenuPanel,
} from "./dev-menu";
import type { SkillSnapshot } from "./mount-game-window";

const scrapingSkill: SkillSnapshot = {
  id: "scraping",
  level: 3,
  xp: 174,
  xpIntoLevel: 0,
  xpToNextLevel: 102,
  levelProgress: 0,
  isActive: true,
  isLocked: false,
  rawData: 0,
};

describe("dev menu", () => {
  it("renders skill level controls for each skill", () => {
    const html = renderDevMenuPanel([scrapingSkill]);

    expect(html).toContain("Scraping");
    expect(html).toContain('data-testid="dev-level-input-scraping"');
    expect(html).toContain('value="3"');
    expect(html).toContain('data-testid="dev-apply-scraping"');
  });

  it("mounts dev menu shell into the game window root", () => {
    const root = document.createElement("div");
    const shell = mountDevMenuShell(root);

    expect(root.querySelector('[data-testid="dev-menu"]')).toBe(shell);
  });

  it("calls onSetLevel when apply is clicked", async () => {
    const container = document.createElement("div");
    const onSetLevel = vi.fn().mockResolvedValue(undefined);

    renderDevMenu(container, [scrapingSkill], onSetLevel);

    const input = container.querySelector<HTMLInputElement>(
      '[data-testid="dev-level-input-scraping"]',
    );
    const button = container.querySelector<HTMLButtonElement>(
      '[data-testid="dev-apply-scraping"]',
    );

    expect(input).not.toBeNull();
    expect(button).not.toBeNull();

    input!.value = "12";
    button!.click();

    expect(onSetLevel).toHaveBeenCalledWith("scraping", 12);
  });

  it("rebinds controls after rerender", () => {
    const container = document.createElement("div");
    const onSetLevel = vi.fn().mockResolvedValue(undefined);

    container.innerHTML = renderDevMenuPanel([scrapingSkill]);
    bindDevMenuControls(container, onSetLevel);

    container
      .querySelector<HTMLButtonElement>('[data-testid="dev-apply-scraping"]')
      ?.click();

    expect(onSetLevel).toHaveBeenCalledWith("scraping", 3);
  });
});
