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
};

const devMenuCallbacks = {
  onSetLevel: vi.fn().mockResolvedValue(undefined),
  onSetTokens: vi.fn().mockResolvedValue(undefined),
  onAddTokens: vi.fn().mockResolvedValue(undefined),
  onResetSave: vi.fn().mockResolvedValue(undefined),
};

describe("dev menu", () => {
  it("renders skill level controls for each skill", () => {
    const html = renderDevMenuPanel([scrapingSkill], 0);

    expect(html).toContain("Scraping");
    expect(html).toContain('data-testid="dev-level-input-scraping"');
    expect(html).toContain('value="3"');
    expect(html).toContain('data-testid="dev-apply-scraping"');
  });

  it("renders token controls with the current balance", () => {
    const html = renderDevMenuPanel([scrapingSkill], 250);

    expect(html).toContain('data-testid="dev-token-controls"');
    expect(html).toContain('data-testid="dev-set-tokens-input"');
    expect(html).toContain('value="250"');
    expect(html).toContain('data-testid="dev-add-tokens-apply"');
  });

  it("renders reset save control", () => {
    const html = renderDevMenuPanel([scrapingSkill], 0);

    expect(html).toContain('data-testid="dev-reset-save"');
    expect(html).toContain("Reset Save");
  });

  it("mounts dev menu shell into the game window root", () => {
    const root = document.createElement("div");
    const shell = mountDevMenuShell(root);

    expect(root.querySelector('[data-testid="dev-menu"]')).toBe(shell);
  });

  it("calls onSetLevel when apply is clicked", async () => {
    const container = document.createElement("div");
    const callbacks = {
      ...devMenuCallbacks,
      onSetLevel: vi.fn().mockResolvedValue(undefined),
    };

    renderDevMenu(container, [scrapingSkill], 0, callbacks);

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

    expect(callbacks.onSetLevel).toHaveBeenCalledWith("scraping", 12);
  });

  it("calls onSetTokens when set tokens is clicked", () => {
    const container = document.createElement("div");
    const callbacks = {
      ...devMenuCallbacks,
      onSetTokens: vi.fn().mockResolvedValue(undefined),
    };

    renderDevMenu(container, [scrapingSkill], 50, callbacks);

    const input = container.querySelector<HTMLInputElement>(
      '[data-testid="dev-set-tokens-input"]',
    );
    const button = container.querySelector<HTMLButtonElement>(
      '[data-testid="dev-set-tokens-apply"]',
    );

    input!.value = "420";
    button!.click();

    expect(callbacks.onSetTokens).toHaveBeenCalledWith(420);
  });

  it("calls onAddTokens when add tokens is clicked", () => {
    const container = document.createElement("div");
    const callbacks = {
      ...devMenuCallbacks,
      onAddTokens: vi.fn().mockResolvedValue(undefined),
    };

    renderDevMenu(container, [scrapingSkill], 50, callbacks);

    const input = container.querySelector<HTMLInputElement>(
      '[data-testid="dev-add-tokens-input"]',
    );
    const button = container.querySelector<HTMLButtonElement>(
      '[data-testid="dev-add-tokens-apply"]',
    );

    input!.value = "25";
    button!.click();

    expect(callbacks.onAddTokens).toHaveBeenCalledWith(25);
  });

  it("requires confirmation before reset save", () => {
    const container = document.createElement("div");
    const callbacks = {
      ...devMenuCallbacks,
      onResetSave: vi.fn().mockResolvedValue(undefined),
    };
    const originalConfirm = window.confirm;
    const confirmMock = vi.fn();
    window.confirm = confirmMock;

    renderDevMenu(container, [scrapingSkill], 0, callbacks);

    const button = container.querySelector<HTMLButtonElement>(
      '[data-testid="dev-reset-save"]',
    );

    confirmMock.mockReturnValue(false);
    button!.click();
    expect(callbacks.onResetSave).not.toHaveBeenCalled();

    confirmMock.mockReturnValue(true);
    button!.click();
    expect(callbacks.onResetSave).toHaveBeenCalledTimes(1);

    window.confirm = originalConfirm;
  });

  it("rebinds controls after rerender", () => {
    const container = document.createElement("div");
    const callbacks = {
      ...devMenuCallbacks,
      onSetLevel: vi.fn().mockResolvedValue(undefined),
    };

    container.innerHTML = renderDevMenuPanel([scrapingSkill], 0);
    bindDevMenuControls(container, callbacks);

    container
      .querySelector<HTMLButtonElement>('[data-testid="dev-apply-scraping"]')
      ?.click();

    expect(callbacks.onSetLevel).toHaveBeenCalledWith("scraping", 3);
  });
});
