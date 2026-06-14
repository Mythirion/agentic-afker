import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { startCurrentActionProgressLoop } from "./game-window/action-progress";
import { mountDevMenu, renderDevMenu } from "./game-window/dev-menu";
import {
  mountGameWindow,
  renderGameWindowState,
  type GameWindowSnapshot,
} from "./game-window/mount-game-window";

async function bootstrapGameWindow(root: HTMLElement): Promise<void> {
  mountGameWindow(root);

  const devMenuEnabled = await invoke<boolean>("is_dev_menu_enabled");
  const devMenuRoot = devMenuEnabled ? mountDevMenu(root) : null;
  let actionCycleStartedAt = Date.now();

  const applySnapshot = (snapshot: GameWindowSnapshot) => {
    actionCycleStartedAt = Date.now();
    renderGameWindowState(root, snapshot);

    if (devMenuRoot) {
      renderDevMenu(devMenuRoot, snapshot.skills, async (skillId, level) => {
        await invoke("dev_set_skill_level", { skillId, level });
      });
    }
  };

  applySnapshot(await invoke<GameWindowSnapshot>("get_game_state"));
  startCurrentActionProgressLoop(root, () => actionCycleStartedAt);

  await listen<GameWindowSnapshot>("game-state", (event) => {
    applySnapshot(event.payload);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  const root = document.querySelector<HTMLElement>("#game-window");

  if (!root) {
    throw new Error("Game Window root element is missing");
  }

  void bootstrapGameWindow(root).catch((error: unknown) => {
    console.error("failed to bootstrap game window", error);
  });
});
