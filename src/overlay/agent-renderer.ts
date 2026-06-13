import { Application, Graphics } from "pixi.js";
import { AGENT_PLACEHOLDER } from "./constants";

export function createAgentPlaceholder(): Graphics {
  const color = Number.parseInt(AGENT_PLACEHOLDER.color.slice(1), 16);
  const agent = new Graphics()
    .rect(0, 0, AGENT_PLACEHOLDER.width, AGENT_PLACEHOLDER.height)
    .fill(color);

  agent.pivot.set(
    AGENT_PLACEHOLDER.width / 2,
    AGENT_PLACEHOLDER.height / 2,
  );

  return agent;
}

export async function mountAgentOverlay(root: HTMLElement): Promise<void> {
  const app = new Application();

  await app.init({
    backgroundAlpha: 0,
    width: root.clientWidth,
    height: root.clientHeight,
    resizeTo: root,
  });

  root.replaceChildren(app.canvas);

  const agent = createAgentPlaceholder();
  agent.position.set(root.clientWidth / 2, root.clientHeight / 2);
  app.stage.addChild(agent);
}
