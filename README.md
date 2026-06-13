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

You do **not** need Node/npm on Windows to test builds. CI produces a portable folder (no installer wizard):

1. Run **Actions → Staging build** on the branch you want (defaults to `staging`).
2. Open the completed workflow run on GitHub → **Artifacts** → download the artifact (GitHub always wraps it in **one** `.zip`).
3. Unzip once — you should see `agentic-afker-portable/agentic-afker.exe` (plus DLLs/resources). **Not** a zip inside a zip.
4. Run `agentic-afker.exe` (requires [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) — preinstalled on Windows 10/11).

If you get a zip containing another zip with the same name, the workflow was an older revision that pre-compressed before upload. Re-run **Staging build** on current `staging` (or any branch with the directory-upload fix).

Use Windows only as a playtest target; keep coding on your Linux box.

### Windows portable build from Linux (scp)

Cross-compile locally and copy the folder to a Windows machine:

```bash
# One-time prerequisites (Ubuntu/Debian)
sudo apt install nsis lld llvm clang
cargo install --locked cargo-xwin
rustup target add x86_64-pc-windows-msvc

npm run build:windows-portable
# → dist/agentic-afker-portable/agentic-afker.exe

scp -r dist/agentic-afker-portable you@windows-pc:~/Desktop/
```

Experimental — [Tauri cross-compile notes](https://v2.tauri.app/distribute/windows-installer/). GitHub Actions Windows builds are the supported path if local cross-compile fails.

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

**Staging build (`staging-build.yml`)** — manual only, never on push. **Actions → Staging build → Run workflow** builds a release binary with `--no-bundle` (no NSIS wizard), stages `agentic-afker-portable/`, and uploads it (GitHub delivers one zip on download). Artifact retained 30 days.

After the first merge to `main`, create `staging` and set it as the default branch in GitHub (**Settings → General → Default branch**):

```bash
git checkout main && git pull
git checkout -b staging && git push -u origin staging
```

## Docs

- [CONTEXT.md](./CONTEXT.md) — domain glossary
- [docs/prd/](./docs/prd/) — component PRDs and Alpha issue breakdown
- [docs/adr/](./docs/adr/) — architecture decisions
