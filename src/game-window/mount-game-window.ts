export interface GameWindowHeader {
  tokenBalanceLabel: string;
  totalLevelLabel: string;
}

export const GAME_WINDOW_HEADER: GameWindowHeader = {
  tokenBalanceLabel: "Tokens",
  totalLevelLabel: "Total Level",
};

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
      <p class="game-main__placeholder">Skill management coming soon.</p>
    </main>
  `;
}

export function mountGameWindow(root: HTMLElement): void {
  root.classList.add("game-window");
  renderGameWindowStub(root);
}
