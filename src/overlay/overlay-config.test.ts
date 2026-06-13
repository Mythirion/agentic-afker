import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

describe("tauri overlay window config", () => {
  it("declares a transparent always-on-top Agent Overlay window", () => {
    const configPath = resolve(process.cwd(), "src-tauri/tauri.conf.json");
    const config = JSON.parse(readFileSync(configPath, "utf8"));
    const overlay = config.app.windows.find(
      (window: { label: string }) => window.label === "overlay",
    );

    expect(overlay).toMatchObject({
      label: "overlay",
      width: 300,
      height: 200,
      transparent: true,
      alwaysOnTop: true,
      decorations: false,
      resizable: false,
    });
  });
});
