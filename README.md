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

You do **not** need Node/npm on Windows to test builds. CI produces a portable zip (no installer wizard):

1. Run **Actions → Staging build** on the branch you want (defaults to `staging`).
2. Open the completed workflow run on GitHub → **Artifacts** → download the `.zip`.
3. Unzip the folder and run `Agentic Afker.exe` inside.

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
| `feature/*` | Day-to-day work | On PR → `staging` (code paths only) | No |
| `staging` | Integration / playtest | On PR → `main` only | Manual |
| `main` | Stable / release | On PR → `main` when promoting | No |

**CI (`ci.yml`)** — one Ubuntu job per PR: `npm test`, `npm run build`, `cargo test`.

Runs only when:

- A PR targets `staging` or `main`
- Changed files are under `src/`, `src-tauri/`, or build config (docs-only PRs skip CI)

Does **not** run on push (merge already passed the PR checks).

**Staging build (`staging-build.yml`)** — manual only, never on push. **Actions → Staging build → Run workflow** produces a portable Windows `.zip` (no NSIS installer). Artifact retained 30 days.

After the first merge to `main`, create `staging` and set it as the default branch in GitHub (**Settings → General → Default branch**):

```bash
git checkout main && git pull
git checkout -b staging && git push -u origin staging
```

## Docs

- [CONTEXT.md](./CONTEXT.md) — domain glossary
- [docs/prd/](./docs/prd/) — component PRDs and Alpha issue breakdown
- [docs/adr/](./docs/adr/) — architecture decisions
