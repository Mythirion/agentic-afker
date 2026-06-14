import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  mountGameWindow,
  renderGameWindowState,
  type GameWindowSnapshot,
} from "./game-window/mount-game-window";

async function bootstrapGameWindow(root: HTMLElement): Promise<void> {
  mountGameWindow(root);

  const snapshot = await invoke<GameWindowSnapshot>("get_game_state");
  renderGameWindowState(root, snapshot);

  await listen<GameWindowSnapshot>("game-state", (event) => {
    renderGameWindowState(root, event.payload);
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
