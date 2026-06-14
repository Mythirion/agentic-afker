import type { SkillSnapshot } from "./mount-game-window";

const SKILL_LABELS: Record<string, string> = {
  scraping: "Scraping",
};

export interface DevMenuCallbacks {
  onSetLevel: (skillId: string, level: number) => Promise<void>;
  onSetTokens: (amount: number) => Promise<void>;
  onAddTokens: (amount: number) => Promise<void>;
  onResetSave: () => Promise<void>;
}

function formatSkillLabel(skillId: string): string {
  return SKILL_LABELS[skillId] ?? skillId;
}

function renderSkillLevelControl(skill: SkillSnapshot): string {
  return `
    <div class="dev-menu__row" data-testid="dev-skill-${skill.id}">
      <label class="dev-menu__label" for="dev-level-${skill.id}">
        ${formatSkillLabel(skill.id)}
      </label>
      <input
        id="dev-level-${skill.id}"
        class="dev-menu__input"
        type="number"
        min="1"
        value="${skill.level}"
        data-testid="dev-level-input-${skill.id}"
      />
      <button
        type="button"
        class="dev-menu__apply"
        data-skill-id="${skill.id}"
        data-testid="dev-apply-${skill.id}"
      >
        Set level
      </button>
    </div>
  `;
}

function renderTokenControls(tokens: number): string {
  return `
    <div class="dev-menu__section" data-testid="dev-token-controls">
      <h3 class="dev-menu__section-title">Tokens</h3>
      <div class="dev-menu__row" data-testid="dev-set-tokens-row">
        <label class="dev-menu__label" for="dev-set-tokens-input">Set balance</label>
        <input
          id="dev-set-tokens-input"
          class="dev-menu__input"
          type="number"
          min="0"
          value="${tokens}"
          data-testid="dev-set-tokens-input"
        />
        <button
          type="button"
          class="dev-menu__apply"
          data-testid="dev-set-tokens-apply"
        >
          Set Tokens
        </button>
      </div>
      <div class="dev-menu__row" data-testid="dev-add-tokens-row">
        <label class="dev-menu__label" for="dev-add-tokens-input">Add amount</label>
        <input
          id="dev-add-tokens-input"
          class="dev-menu__input"
          type="number"
          min="0"
          value="100"
          data-testid="dev-add-tokens-input"
        />
        <button
          type="button"
          class="dev-menu__apply"
          data-testid="dev-add-tokens-apply"
        >
          Add Tokens
        </button>
      </div>
    </div>
  `;
}

function renderResetSaveControl(): string {
  return `
    <div class="dev-menu__section" data-testid="dev-reset-controls">
      <button
        type="button"
        class="dev-menu__danger"
        data-testid="dev-reset-save"
      >
        Reset Save
      </button>
    </div>
  `;
}

export function renderDevMenuPanel(skills: SkillSnapshot[], tokens: number): string {
  return `
    <details class="dev-menu__details" open>
      <summary class="dev-menu__summary">Dev</summary>
      <p class="dev-menu__hint">Requires <code>AGENTIC_AFKER_DEV=1</code></p>
      <div class="dev-menu__controls">
        ${skills.map(renderSkillLevelControl).join("")}
        ${renderTokenControls(tokens)}
        ${renderResetSaveControl()}
      </div>
    </details>
  `;
}

export function mountDevMenuShell(root: HTMLElement): HTMLElement {
  const section = document.createElement("section");
  section.className = "dev-menu";
  section.dataset.testid = "dev-menu";
  root.appendChild(section);
  return section;
}

export const mountDevMenu = mountDevMenuShell;

function parseNonNegativeInt(value: string): number | null {
  const parsed = Number.parseInt(value, 10);
  if (!Number.isFinite(parsed) || parsed < 0) {
    return null;
  }

  return parsed;
}

export function bindDevMenuControls(
  container: HTMLElement,
  callbacks: DevMenuCallbacks,
): void {
  container.querySelectorAll<HTMLButtonElement>(".dev-menu__apply").forEach((button) => {
    button.addEventListener("click", () => {
      const skillId = button.dataset.skillId;
      if (!skillId) {
        return;
      }

      const input = container.querySelector<HTMLInputElement>(
        `#dev-level-${skillId}`,
      );
      if (!input) {
        return;
      }

      const level = Number.parseInt(input.value, 10);
      if (!Number.isFinite(level) || level < 1) {
        return;
      }

      void callbacks.onSetLevel(skillId, level).catch((error: unknown) => {
        console.error("failed to set skill level", error);
      });
    });
  });

  const setTokensInput = container.querySelector<HTMLInputElement>(
    '[data-testid="dev-set-tokens-input"]',
  );
  const setTokensButton = container.querySelector<HTMLButtonElement>(
    '[data-testid="dev-set-tokens-apply"]',
  );
  setTokensButton?.addEventListener("click", () => {
    if (!setTokensInput) {
      return;
    }

    const amount = parseNonNegativeInt(setTokensInput.value);
    if (amount === null) {
      return;
    }

    void callbacks.onSetTokens(amount).catch((error: unknown) => {
      console.error("failed to set tokens", error);
    });
  });

  const addTokensInput = container.querySelector<HTMLInputElement>(
    '[data-testid="dev-add-tokens-input"]',
  );
  const addTokensButton = container.querySelector<HTMLButtonElement>(
    '[data-testid="dev-add-tokens-apply"]',
  );
  addTokensButton?.addEventListener("click", () => {
    if (!addTokensInput) {
      return;
    }

    const amount = parseNonNegativeInt(addTokensInput.value);
    if (amount === null) {
      return;
    }

    void callbacks.onAddTokens(amount).catch((error: unknown) => {
      console.error("failed to add tokens", error);
    });
  });

  const resetButton = container.querySelector<HTMLButtonElement>(
    '[data-testid="dev-reset-save"]',
  );
  resetButton?.addEventListener("click", () => {
    if (!window.confirm("Reset save to a fresh Alpha start? This cannot be undone.")) {
      return;
    }

    void callbacks.onResetSave().catch((error: unknown) => {
      console.error("failed to reset save", error);
    });
  });
}

function isDevMenuInputEditing(input: HTMLInputElement): boolean {
  return input.dataset.devMenuEditing === "true";
}

function trackDevMenuInputEditing(container: HTMLElement): void {
  container.querySelectorAll<HTMLInputElement>(".dev-menu__input").forEach((input) => {
    input.addEventListener("focus", () => {
      input.dataset.devMenuEditing = "true";
    });
    input.addEventListener("blur", () => {
      delete input.dataset.devMenuEditing;
    });
  });
}

export function syncDevMenuFromSnapshot(
  container: HTMLElement,
  skills: SkillSnapshot[],
  tokens: number,
): void {
  for (const skill of skills) {
    const input = container.querySelector<HTMLInputElement>(
      `[data-testid="dev-level-input-${skill.id}"]`,
    );
    if (input && !isDevMenuInputEditing(input)) {
      input.value = String(skill.level);
    }
  }

  const setTokensInput = container.querySelector<HTMLInputElement>(
    '[data-testid="dev-set-tokens-input"]',
  );
  if (setTokensInput && !isDevMenuInputEditing(setTokensInput)) {
    setTokensInput.value = String(tokens);
  }
}

export function renderDevMenu(
  container: HTMLElement,
  skills: SkillSnapshot[],
  tokens: number,
  callbacks: DevMenuCallbacks,
): void {
  container.innerHTML = renderDevMenuPanel(skills, tokens);
  trackDevMenuInputEditing(container);
  bindDevMenuControls(container, callbacks);
}
