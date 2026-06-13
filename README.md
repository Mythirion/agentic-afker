# Agentic Afker

Desktop companion idle game — train an Agent through the AI boom.

## Development

Dev on Linux (or any machine with Node + Rust). [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```bash
npm install
npm test
npm run tauri dev
```

### Windows playtest (no npm required)

You do **not** need Node/npm on Windows to test builds. CI produces a ready-to-run bundle:

1. Merge to `staging` (or run **Actions → Staging build** on the branch you want).
2. Open the completed workflow run on GitHub → **Artifacts** → download the `.zip`.
3. Unzip and run the `.exe` inside.

Use Windows only as a playtest target; keep coding on your Linux box.

### Windows local dev (optional)

Only if you want `npm run tauri dev` on Windows: install [Node.js LTS](https://nodejs.org/) (includes npm), [Rust](https://rustup.rs/), and the [Tauri Windows prerequisites](https://tauri.app/start/prerequisites/).

## Branches and CI

```
feature/* ──PR──▶ staging ──PR (occasional)──▶ main
                  │
                  └── manual Windows build (Actions → Staging build)
```

| Branch | Role | CI | Desktop builds |
|--------|------|----|----------------|
| `feature/*` | Day-to-day work | On PR → `staging` | No |
| `staging` | Integration / playtest | On push and on PR → `main` | Manual |
| `main` | Stable / release | On push (after promotion) | No |

**CI (`ci.yml`)** runs fast checks only — `npm test`, `npm run build`, `cargo test`. Triggers on:

- PRs targeting `staging` (feature work)
- PRs targeting `main` (promoting staging)
- Pushes to `staging` or `main`

**Staging build (`staging-build.yml`)** is manual. When `staging` has something you want to playtest on Windows: **Actions → Staging build → Run workflow** (defaults to the `staging` branch). Produces a `.zip` artifact retained 30 days.

After the first merge to `main`, create `staging` and set it as the default branch in GitHub (**Settings → General → Default branch**):

```bash
git checkout main && git pull
git checkout -b staging && git push -u origin staging
```

## Docs

- [CONTEXT.md](./CONTEXT.md) — domain glossary
- [docs/prd/](./docs/prd/) — component PRDs and Alpha issue breakdown
- [docs/adr/](./docs/adr/) — architecture decisions
