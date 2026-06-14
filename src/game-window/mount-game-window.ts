export interface GameWindowHeader {
  tokenBalanceLabel: string;
  totalLevelLabel: string;
}

export interface SkillSnapshot {
  id: string;
  level: number;
  xp: number;
  isActive: boolean;
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
};

function formatSkillLabel(skillId: string): string {
  return SKILL_LABELS[skillId] ?? skillId;
}

function renderSkillRow(skill: SkillSnapshot): string {
  const activeClass = skill.isActive ? " skill-row--active" : "";
  const activeAttr = skill.isActive ? ' data-active="true"' : "";

  return `
    <li class="skill-row${activeClass}" data-testid="skill-${skill.id}"${activeAttr}>
      <span class="skill-row__name">${formatSkillLabel(skill.id)}</span>
      <span class="skill-row__level" data-testid="skill-level-${skill.id}">Lv ${skill.level}</span>
      <span class="skill-row__xp" data-testid="skill-xp-${skill.id}">${skill.xp} XP</span>
      ${
        skill.isActive
          ? '<span class="skill-row__badge" data-testid="active-skill-indicator">Active</span>'
          : ""
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

  if (!tokenBalance || !totalLevel || !skillList) {
    throw new Error("Game Window layout is missing expected elements");
  }

  tokenBalance.textContent = String(snapshot.tokens);
  totalLevel.textContent = String(snapshot.totalLevel);
  skillList.innerHTML = snapshot.skills.map(renderSkillRow).join("");
}

export function mountGameWindow(root: HTMLElement): void {
  root.classList.add("game-window");
  renderGameWindowStub(root);
}
