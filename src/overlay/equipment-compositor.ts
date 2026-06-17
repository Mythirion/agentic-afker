import {
  ACTIVITY_SPRITES,
  BODY_SPRITES,
  EQUIPMENT_SPRITES,
  equipmentTierForLevel,
  resolveFormStage,
} from "./sprite-catalog";

export interface AgentSkillAppearance {
  id: string;
  level: number;
}

export interface AgentAppearanceInput {
  formStage: number;
  totalLevel: number;
  activeSkill: string;
  skills: AgentSkillAppearance[];
}

export interface AgentSpriteLayer {
  id: string;
  kind: "body" | "equipment" | "activity";
  spriteId: string;
  color: string;
  width: number;
  height: number;
  zIndex: number;
  skillId?: string;
  animation?: string;
}

const SKILL_Z_OFFSET: Record<string, number> = {
  scraping: 0,
  labelling: 1,
  "fine-tuning": 2,
};

export function composeAgentLayers(input: AgentAppearanceInput): AgentSpriteLayer[] {
  const layers: AgentSpriteLayer[] = [];
  const stage = resolveFormStage(input.formStage, input.totalLevel);
  const body = BODY_SPRITES[stage];

  layers.push({
    id: body.id,
    kind: "body",
    spriteId: body.id,
    color: body.color,
    width: body.width,
    height: body.height,
    zIndex: 0,
  });

  for (const skill of input.skills) {
    const tier = equipmentTierForLevel(skill.level);
    if (tier === null) {
      continue;
    }

    const equipment = EQUIPMENT_SPRITES[skill.id]?.[tier];
    if (!equipment) {
      continue;
    }

    layers.push({
      id: equipment.id,
      kind: "equipment",
      spriteId: equipment.id,
      color: equipment.color,
      width: equipment.width,
      height: equipment.height,
      zIndex: 10 + (SKILL_Z_OFFSET[skill.id] ?? 0),
      skillId: skill.id,
    });
  }

  if (input.activeSkill) {
    const activity = ACTIVITY_SPRITES[input.activeSkill];
    if (activity) {
      layers.push({
        id: activity.id,
        kind: "activity",
        spriteId: activity.id,
        color: activity.color,
        width: activity.width,
        height: activity.height,
        zIndex: 100,
        skillId: input.activeSkill,
        animation: activity.animation,
      });
    }
  }

  return layers.sort((left, right) => left.zIndex - right.zIndex);
}
