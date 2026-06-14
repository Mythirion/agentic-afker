#!/usr/bin/env bash
# Cross-compile a portable Windows bundle from Linux (exe + DLLs + resources).
# Output: dist/agentic-afker-portable/ — scp this folder to your Windows machine.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

TARGET="${TARGET:-x86_64-pc-windows-msvc}"
RUNNER="${RUNNER:-cargo-xwin}"
OUT_DIR="$ROOT/dist/agentic-afker-portable"
RELEASE_DIR="$ROOT/src-tauri/target/$TARGET/release"

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

echo "==> Checking prerequisites"
require npm
require cargo
require rustup

if ! rustup target list --installed | grep -q "^${TARGET}$"; then
  echo "Installing Rust target ${TARGET}..."
  rustup target add "$TARGET"
fi

if [[ "$RUNNER" == "cargo-xwin" ]] && ! command -v cargo-xwin >/dev/null 2>&1; then
  echo "cargo-xwin not found. Install with: cargo install --locked cargo-xwin" >&2
  echo "Also install: sudo apt install nsis lld llvm clang" >&2
  exit 1
fi

echo "==> Building frontend"
npm ci
npm run build

echo "==> Cross-compiling Windows release binary (--no-bundle)"
npm run tauri build -- --runner "$RUNNER" --target "$TARGET" --no-bundle

EXE="$RELEASE_DIR/agentic-afker.exe"
if [[ ! -f "$EXE" ]]; then
  echo "Release binary not found at $EXE" >&2
  exit 1
fi

echo "==> Staging portable bundle at $OUT_DIR"
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"
cp "$EXE" "$OUT_DIR/"
shopt -s nullglob
for dll in "$RELEASE_DIR"/*.dll; do
  cp "$dll" "$OUT_DIR/"
done
if [[ -d "$RELEASE_DIR/resources" ]]; then
  cp -r "$RELEASE_DIR/resources" "$OUT_DIR/"
fi
cp "$ROOT/scripts/run-dev.bat" "$OUT_DIR/"

echo
echo "Done. Portable bundle:"
find "$OUT_DIR" -maxdepth 2 -type f | sort | sed "s|^|  |"
echo
echo "Copy to a Windows host, e.g.:"
echo "  scp -r \"$OUT_DIR\" you@192.168.0.1:~/projects/agentic-afker/"
echo "Attempting to copy to Windows host..."

scp -r "$OUT_DIR" aaron@192.168.0.201:~/projects/agentic-afker/