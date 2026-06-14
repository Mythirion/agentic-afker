export const ACTION_TICK_MS = 1_000;

export function actionProgress(elapsedMs: number): number {
  return Math.min(1, Math.max(0, elapsedMs / ACTION_TICK_MS));
}

export function formatActionProgressPercent(progress: number): number {
  return Math.round(progress * 100);
}

export function updateCurrentActionProgress(
  root: HTMLElement,
  progress: number,
): void {
  const fill = root.querySelector<HTMLElement>(
    '[data-testid="current-action-fill"]',
  );
  const bar = root.querySelector<HTMLElement>(
    '[data-testid="current-action-progress"]',
  );

  if (!fill || !bar) {
    return;
  }

  const percent = formatActionProgressPercent(progress);
  fill.style.width = `${percent}%`;
  bar.setAttribute("aria-valuenow", String(percent));
}

export function startCurrentActionProgressLoop(
  root: HTMLElement,
  getCycleStartedAt: () => number,
  hasActiveAction: () => boolean,
): () => void {
  let frameId = 0;

  const tick = () => {
    const progress = hasActiveAction()
      ? actionProgress(Date.now() - getCycleStartedAt())
      : 0;
    updateCurrentActionProgress(root, progress);
    frameId = requestAnimationFrame(tick);
  };

  frameId = requestAnimationFrame(tick);

  return () => cancelAnimationFrame(frameId);
}
