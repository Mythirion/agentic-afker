import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { INTERACT_MODE_CHANGED_EVENT } from "../shell/events";

export const OPEN_GAME_WINDOW_BUTTON_ID = "open-game-window";

export function interactButtonVisible(interactModeEnabled: boolean): boolean {
  return interactModeEnabled;
}

export function createOpenGameWindowButton(): HTMLButtonElement {
  const button = document.createElement("button");
  button.type = "button";
  button.id = OPEN_GAME_WINDOW_BUTTON_ID;
  button.className = "overlay-interact-button";
  button.textContent = "Game Window";
  button.hidden = true;
  button.addEventListener("click", () => {
    void invoke("open_game_window");
  });
  return button;
}

export async function mountInteractControls(
  root: HTMLElement,
): Promise<UnlistenFn> {
  const button = createOpenGameWindowButton();
  root.appendChild(button);

  return listen<{ enabled: boolean }>(
    INTERACT_MODE_CHANGED_EVENT,
    (event) => {
      button.hidden = !interactButtonVisible(event.payload.enabled);
    },
  );
}
