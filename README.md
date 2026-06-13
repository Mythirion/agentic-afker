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

| Branch | CI on push/PR | Desktop builds |
|--------|---------------|----------------|
| `main` | Yes — `npm test`, `npm run build`, `cargo test` | No |
| `staging` | No automatic builds | Manual via **Actions → Staging build** |

**Main (`ci.yml`)** runs on every push to `main` and on pull requests. Fast checks only — no Windows bundle.

**Staging (`staging-build.yml`)** is manual. In GitHub: **Actions → Staging build → Run workflow**. Defaults to the `staging` branch; you can pass any branch, tag, or SHA. Produces a Windows `.zip` artifact (retained 30 days).

Create the staging branch when ready:

```bash
git checkout main
git pull
git checkout -b staging
git push -u origin staging
```

## Docs

- [CONTEXT.md](./CONTEXT.md) — domain glossary
- [docs/prd/](./docs/prd/) — component PRDs and Alpha issue breakdown
- [docs/adr/](./docs/adr/) — architecture decisions
