import { mountGameWindow } from "./game-window/mount-game-window";

window.addEventListener("DOMContentLoaded", () => {
  const root = document.querySelector<HTMLElement>("#game-window");

  if (!root) {
    throw new Error("Game Window root element is missing");
  }

  mountGameWindow(root);
});
