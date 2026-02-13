#!/usr/bin/env bash
# deploy.sh — start swe-compliance behind agent-serv
# Usage: ./ui/deploy.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
AGENT_SERV_DIR="$(cd "$ROOT_DIR/../agent-serv" && pwd)"

export SWE_PORT="${SWE_PORT:-8081}"

# ── 1. Build compliance server ───────────────────────────────────────────────
echo "==> Building swe-compliance-server..."
cargo build --release --manifest-path "$ROOT_DIR/ui/server/Cargo.toml"

# ── 2. Start compliance server in background ─────────────────────────────────
echo "==> Starting compliance server on port $SWE_PORT..."
"$ROOT_DIR/target/release/swe-compliance-server" &
COMPLIANCE_PID=$!
echo "    PID: $COMPLIANCE_PID"

cleanup() {
    echo "==> Stopping compliance server (PID $COMPLIANCE_PID)..."
    kill "$COMPLIANCE_PID" 2>/dev/null || true
    wait "$COMPLIANCE_PID" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

# Wait briefly for the compliance server to bind
sleep 1

# ── 3. Start agent-serv ──────────────────────────────────────────────────────
echo "==> Starting agent-serv on port 8080 (config: swe-compliance.toml)..."
cd "$AGENT_SERV_DIR"
cargo run -- --config config/swe-compliance.toml
