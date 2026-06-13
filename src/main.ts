import { mountAgentOverlay } from "./overlay/agent-renderer";

window.addEventListener("DOMContentLoaded", () => {
  const root = document.querySelector<HTMLElement>("#agent-overlay");

  if (!root) {
    throw new Error("Agent Overlay root element is missing");
  }

  void mountAgentOverlay(root);
});
