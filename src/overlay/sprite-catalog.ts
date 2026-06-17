export const FORM_STAGE_2_TOTAL_LEVEL = 30;

export const EQUIPMENT_MILESTONES = [10, 20, 30] as const;

export const BODY_SPRITES = {
  1: { id: "body-stage-1", color: "#6c5ce7", width: 80, height: 96 },
  2: { id: "body-stage-2", color: "#00b894", width: 88, height: 104 },
} as const;

export const EQUIPMENT_SPRITES: Record<
  string,
  Record<number, { id: string; color: string; width: number; height: number }>
> = {
  scraping: {
    1: { id: "scraping-equipment-1", color: "#fdcb6e", width: 24, height: 20 },
    2: { id: "scraping-equipment-2", color: "#e17055", width: 28, height: 24 },
    3: { id: "scraping-equipment-3", color: "#d63031", width: 32, height: 28 },
  },
  labelling: {
    1: { id: "labelling-equipment-1", color: "#74b9ff", width: 22, height: 18 },
    2: { id: "labelling-equipment-2", color: "#0984e3", width: 26, height: 22 },
    3: { id: "labelling-equipment-3", color: "#2d3436", width: 30, height: 26 },
  },
  "fine-tuning": {
    1: { id: "fine-tuning-equipment-1", color: "#a29bfe", width: 20, height: 20 },
    2: { id: "fine-tuning-equipment-2", color: "#6c5ce7", width: 24, height: 24 },
    3: { id: "fine-tuning-equipment-3", color: "#5f27cd", width: 28, height: 28 },
  },
};

export const ACTIVITY_SPRITES: Record<
  string,
  { id: string; color: string; width: number; height: number; animation: string }
> = {
  scraping: {
    id: "activity-scraping",
    color: "#ffeaa7",
    width: 36,
    height: 12,
    animation: "scrape",
  },
  labelling: {
    id: "activity-labelling",
    color: "#81ecec",
    width: 32,
    height: 14,
    animation: "label",
  },
  "fine-tuning": {
    id: "activity-fine-tuning",
    color: "#dfe6e9",
    width: 40,
    height: 16,
    animation: "tune",
  },
};

export function equipmentTierForLevel(level: number): number | null {
  if (level < EQUIPMENT_MILESTONES[0]) {
    return null;
  }

  if (level < EQUIPMENT_MILESTONES[1]) {
    return 1;
  }

  if (level < EQUIPMENT_MILESTONES[2]) {
    return 2;
  }

  return 3;
}

export function resolveFormStage(formStage: number, totalLevel: number): 1 | 2 {
  if (formStage >= 2 || totalLevel >= FORM_STAGE_2_TOTAL_LEVEL) {
    return 2;
  }

  return 1;
}
