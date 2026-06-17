import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Application, Container, Graphics } from "pixi.js";
import {
  activityAnimationForSkill,
  activityOffset,
  type ActivityAnimation,
} from "./animation-controller";
import {
  composeAgentLayers,
  type AgentAppearanceInput,
  type AgentSpriteLayer,
} from "./equipment-compositor";

export interface OverlayGameSnapshot {
  activeSkill: string;
  formStage: number;
  totalLevel: number;
  skills: Array<{ id: string; level: number }>;
}

function toAppearanceInput(snapshot: OverlayGameSnapshot): AgentAppearanceInput {
  return {
    formStage: snapshot.formStage,
    totalLevel: snapshot.totalLevel,
    activeSkill: snapshot.activeSkill,
    skills: snapshot.skills,
  };
}

export function createLayerGraphic(layer: AgentSpriteLayer): Graphics {
  const color = Number.parseInt(layer.color.slice(1), 16);
  const graphic = new Graphics()
    .rect(0, 0, layer.width, layer.height)
    .fill(color);

  graphic.pivot.set(layer.width / 2, layer.height / 2);
  graphic.label = layer.id;

  return graphic;
}

export function layerPosition(layer: AgentSpriteLayer): { x: number; y: number } {
  if (layer.kind === "activity") {
    return { x: 0, y: -52 };
  }

  if (layer.kind === "equipment") {
    const offsets: Record<string, { x: number; y: number }> = {
      scraping: { x: -18, y: -8 },
      labelling: { x: 18, y: -12 },
      "fine-tuning": { x: 0, y: 20 },
    };

    return offsets[layer.skillId ?? ""] ?? { x: 0, y: 0 };
  }

  return { x: 0, y: 0 };
}

export class AgentAppearanceRenderer {
  private readonly container = new Container();
  private readonly layers = new Map<string, Graphics>();
  private currentLayers: AgentSpriteLayer[] = [];
  private animationStartedAt = Date.now();
  private activeAnimation: ActivityAnimation = "idle";
  private frameId = 0;

  constructor(app: Application, root: HTMLElement) {
    this.container.sortableChildren = true;
    this.container.position.set(root.clientWidth / 2, root.clientHeight / 2);
    app.stage.addChild(this.container);
  }

  applySnapshot(snapshot: OverlayGameSnapshot): void {
    const next = composeAgentLayers(toAppearanceInput(snapshot));
    const nextIds = new Set(next.map((layer) => layer.id));

    for (const [id, graphic] of this.layers) {
      if (!nextIds.has(id)) {
        this.container.removeChild(graphic);
        graphic.destroy();
        this.layers.delete(id);
      }
    }

    for (const layer of next) {
      let graphic = this.layers.get(layer.id);
      if (!graphic) {
        graphic = createLayerGraphic(layer);
        this.layers.set(layer.id, graphic);
        this.container.addChild(graphic);
      } else {
        graphic.clear();
        const color = Number.parseInt(layer.color.slice(1), 16);
        graphic.rect(0, 0, layer.width, layer.height).fill(color);
        graphic.pivot.set(layer.width / 2, layer.height / 2);
      }

      const position = layerPosition(layer);
      graphic.position.set(position.x, position.y);
      graphic.zIndex = layer.zIndex;
    }

    this.container.sortChildren();
    this.currentLayers = next;
    this.activeAnimation = activityAnimationForSkill(snapshot.activeSkill);
    this.animationStartedAt = Date.now();
  }

  startAnimationLoop(): void {
    const tick = () => {
      this.tickActivity();
      this.frameId = requestAnimationFrame(tick);
    };

    this.frameId = requestAnimationFrame(tick);
  }

  stopAnimationLoop(): void {
    cancelAnimationFrame(this.frameId);
  }

  private tickActivity(): void {
    const activityLayer = this.currentLayers.find((layer) => layer.kind === "activity");
    if (!activityLayer) {
      return;
    }

    const graphic = this.layers.get(activityLayer.id);
    if (!graphic) {
      return;
    }

    const base = layerPosition(activityLayer);
    const offset = activityOffset(
      this.activeAnimation,
      Date.now() - this.animationStartedAt,
    );

    graphic.position.set(base.x + offset.x, base.y + offset.y);
  }
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

  const renderer = new AgentAppearanceRenderer(app, root);
  renderer.startAnimationLoop();

  const applySnapshot = (snapshot: OverlayGameSnapshot) => {
    renderer.applySnapshot(snapshot);
  };

  const initial = await invoke<OverlayGameSnapshot & Record<string, unknown>>("get_game_state");
  applySnapshot({
    activeSkill: String(initial.activeSkill ?? ""),
    formStage: Number(initial.formStage ?? 1),
    totalLevel: Number(initial.totalLevel ?? 1),
    skills: Array.isArray(initial.skills)
      ? initial.skills.map((skill) => ({
          id: String((skill as { id: string }).id),
          level: Number((skill as { level: number }).level),
        }))
      : [],
  });

  await listen<OverlayGameSnapshot & Record<string, unknown>>("game-state", (event) => {
    const snapshot = event.payload;
    applySnapshot({
      activeSkill: String(snapshot.activeSkill ?? ""),
      formStage: Number(snapshot.formStage ?? 1),
      totalLevel: Number(snapshot.totalLevel ?? 1),
      skills: Array.isArray(snapshot.skills)
        ? snapshot.skills.map((skill) => ({
            id: String((skill as { id: string }).id),
            level: Number((skill as { level: number }).level),
          }))
        : [],
    });
  });
}
