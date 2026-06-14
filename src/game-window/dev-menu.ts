import type { SkillSnapshot } from "./mount-game-window";

const SKILL_LABELS: Record<string, string> = {
  scraping: "Scraping",
};

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

export function renderDevMenuPanel(skills: SkillSnapshot[]): string {
  return `
    <details class="dev-menu__details" open>
      <summary class="dev-menu__summary">Dev</summary>
      <p class="dev-menu__hint">Requires <code>AGENTIC_AFKER_DEV=1</code></p>
      <div class="dev-menu__controls">
        ${skills.map(renderSkillLevelControl).join("")}
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

export function bindDevMenuControls(
  container: HTMLElement,
  onSetLevel: (skillId: string, level: number) => Promise<void>,
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

      void onSetLevel(skillId, level).catch((error: unknown) => {
        console.error("failed to set skill level", error);
      });
    });
  });
}

export function renderDevMenu(
  container: HTMLElement,
  skills: SkillSnapshot[],
  onSetLevel: (skillId: string, level: number) => Promise<void>,
): void {
  container.innerHTML = renderDevMenuPanel(skills);
  bindDevMenuControls(container, onSetLevel);
}
