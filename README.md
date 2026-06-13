# Agentic Afker

Desktop companion idle game — train an Agent through the AI boom.

## Development

Prerequisites: [Tauri prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```bash
npm install
npm test
npm run tauri dev
```

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
