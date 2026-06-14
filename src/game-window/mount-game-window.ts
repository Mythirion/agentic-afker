export interface GameWindowHeader {
  tokenBalanceLabel: string;
  totalLevelLabel: string;
}

export interface SkillSnapshot {
  id: string;
  level: number;
  xp: number;
  xpIntoLevel: number;
  xpToNextLevel: number;
  levelProgress: number;
  isActive: boolean;
  isLocked: boolean;
  prerequisite?: string;
}

export interface GameWindowSnapshot {
  activeSkill: string;
  skills: SkillSnapshot[];
  tokens: number;
  totalLevel: number;
  lastTickAt: number;
}

export const GAME_WINDOW_HEADER: GameWindowHeader = {
  tokenBalanceLabel: "Tokens",
  totalLevelLabel: "Total Level",
};

const SKILL_LABELS: Record<string, string> = {
  scraping: "Scraping",
  labelling: "Labelling",
};

function formatSkillLabel(skillId: string): string {
  return SKILL_LABELS[skillId] ?? skillId;
}

export function formatActiveSkillLabel(activeSkill: string): string {
  if (!activeSkill) {
    return "Idle";
  }

  return formatSkillLabel(activeSkill);
}

export function hasActiveSkill(activeSkill: string): boolean {
  return activeSkill.length > 0;
}

function renderCurrentActionSection(activeSkillLabel: string): string {
  return `
    <section class="current-action" aria-label="Current action" data-testid="current-action">
      <div class="current-action__header">
        <span class="current-action__label">Current action</span>
        <span class="current-action__name" data-testid="current-action-name">${activeSkillLabel}</span>
      </div>
      <div
        class="current-action__progress"
        role="progressbar"
        aria-valuemin="0"
        aria-valuemax="100"
        aria-valuenow="0"
        aria-label="${activeSkillLabel} action progress"
        data-testid="current-action-progress"
      >
        <div
          class="current-action__progress-fill"
          style="width: 0%"
          data-testid="current-action-fill"
        ></div>
      </div>
    </section>
  `;
}

function formatProgressPercent(progress: number): string {
  return `${Math.round(progress * 100)}`;
}

function renderSkillRow(skill: SkillSnapshot): string {
  if (skill.isLocked) {
    return `
    <li class="skill-row skill-row--locked" data-testid="skill-${skill.id}">
      <div class="skill-row__header">
        <span class="skill-row__name">${formatSkillLabel(skill.id)}</span>
        <span class="skill-row__badge skill-row__badge--locked" data-testid="skill-locked-${skill.id}">
          Locked
        </span>
      </div>
      <p class="skill-row__prerequisite" data-testid="skill-prerequisite-${skill.id}">
        ${skill.prerequisite ?? "Locked"}
      </p>
    </li>
  `;
  }

  const activeClass = skill.isActive ? " skill-row--active" : "";
  const activeAttr = skill.isActive ? ' data-active="true"' : "";
  const progressPercent = formatProgressPercent(skill.levelProgress);
  const progressWidth = `${Math.round(skill.levelProgress * 100)}%`;

  return `
    <li class="skill-row${activeClass}" data-testid="skill-${skill.id}"${activeAttr}>
      <div class="skill-row__header">
        <span class="skill-row__name">${formatSkillLabel(skill.id)}</span>
        <span class="skill-row__level" data-testid="skill-level-${skill.id}">Lv ${skill.level}</span>
        ${
          skill.isActive
            ? '<span class="skill-row__badge" data-testid="active-skill-indicator">Active</span>'
            : ""
        }
      </div>
      <div class="skill-row__progress-wrap">
        <div
          class="skill-row__progress"
          role="progressbar"
          aria-valuenow="${progressPercent}"
          aria-valuemin="0"
          aria-valuemax="100"
          aria-label="${formatSkillLabel(skill.id)} level progress"
          data-testid="skill-progress-${skill.id}"
        >
          <div
            class="skill-row__progress-fill"
            style="width: ${progressWidth}"
            data-testid="skill-progress-fill-${skill.id}"
          ></div>
        </div>
        <span class="skill-row__xp" data-testid="skill-xp-${skill.id}">
          ${skill.xpIntoLevel} / ${skill.xpToNextLevel} XP
        </span>
      </div>
      ${
        skill.isActive
          ? ""
          : `<button
          type="button"
          class="skill-row__start"
          data-action="start-skill"
          data-skill-id="${skill.id}"
          data-testid="start-skill-${skill.id}"
        >
          Start ${formatSkillLabel(skill.id)}
        </button>`
      }
    </li>
  `;
}

export function renderGameWindowStub(root: HTMLElement): void {
  root.innerHTML = `
    <header class="game-header" aria-label="Game Window header">
      <div class="game-header__stat">
        <span class="game-header__label">${GAME_WINDOW_HEADER.tokenBalanceLabel}</span>
        <span class="game-header__value" data-testid="token-balance">—</span>
      </div>
      <div class="game-header__stat">
        <span class="game-header__label">${GAME_WINDOW_HEADER.totalLevelLabel}</span>
        <span class="game-header__value" data-testid="total-level">—</span>
      </div>
    </header>
    ${renderCurrentActionSection("—")}
    <main class="game-main">
      <section class="skill-list" aria-label="Skills">
        <h2 class="skill-list__title">Data Pipeline</h2>
        <ul class="skill-list__items" data-testid="skill-list">
          <li class="skill-row skill-row--placeholder">Waiting for simulation…</li>
        </ul>
      </section>
    </main>
  `;
}

export function renderGameWindowState(
  root: HTMLElement,
  snapshot: GameWindowSnapshot,
): void {
  const tokenBalance = root.querySelector('[data-testid="token-balance"]');
  const totalLevel = root.querySelector('[data-testid="total-level"]');
  const skillList = root.querySelector('[data-testid="skill-list"]');
  const actionName = root.querySelector('[data-testid="current-action-name"]');
  const actionBar = root.querySelector('[data-testid="current-action-progress"]');
  const currentAction = root.querySelector('[data-testid="current-action"]');

  if (!tokenBalance || !totalLevel || !skillList || !actionName || !actionBar || !currentAction) {
    throw new Error("Game Window layout is missing expected elements");
  }

  tokenBalance.textContent = String(snapshot.tokens);
  totalLevel.textContent = String(snapshot.totalLevel);
  const activeSkillLabel = formatActiveSkillLabel(snapshot.activeSkill);
  const idle = !hasActiveSkill(snapshot.activeSkill);
  actionName.textContent = activeSkillLabel;
  actionBar.setAttribute("aria-label", `${activeSkillLabel} action progress`);
  currentAction.classList.toggle("current-action--idle", idle);
  skillList.innerHTML = snapshot.skills.map(renderSkillRow).join("");
}

export function bindSkillListActions(
  root: HTMLElement,
  onStartSkill: (skillId: string) => Promise<void>,
): void {
  root.querySelectorAll<HTMLButtonElement>('[data-action="start-skill"]').forEach((button) => {
    button.addEventListener("click", () => {
      const skillId = button.dataset.skillId;
      if (!skillId) {
        return;
      }

      void onStartSkill(skillId).catch((error: unknown) => {
        console.error("failed to start skill", error);
      });
    });
  });
}

export function mountGameWindow(root: HTMLElement): void {
  root.classList.add("game-window");
  renderGameWindowStub(root);
}
