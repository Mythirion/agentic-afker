import { mountAgentOverlay } from "./overlay/agent-renderer";
import { mountInteractControls } from "./overlay/interact-controls";

window.addEventListener("DOMContentLoaded", () => {
  const shell = document.querySelector<HTMLElement>("#agent-overlay-shell");
  const root = document.querySelector<HTMLElement>("#agent-overlay");

  if (!shell || !root) {
    throw new Error("Agent Overlay root elements are missing");
  }

  void mountAgentOverlay(root);
  void mountInteractControls(shell);
});
