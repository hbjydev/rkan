#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if [[ ${1:-} == --session ]]; then
  export PATH="$PWD/target/debug:$PATH"
  export RUST_LOG=info
  export TERM=xterm-256color
  unset NO_COLOR

  run() {
    local command="$1" character
    printf '\033[1;32m❯\033[0m '
    while IFS= read -r -n 1 character; do
      printf '%s' "$character"
      sleep 0.025
    done <<< "$command"
    printf '\n'
    bash -c "$command"
    sleep 1.5
  }

  printf '\033[2J\033[H'
  run 'ls tests/e2e'
  run 'rkan generate --configs-dir tests/e2e --out-dir target/demo/ckan'
  run 'find target/demo/ckan -name "*.ckan" | sort'
  exit 0
fi

# Requires asciinema 3, agg, and Cargo. Run from any directory.
# The E2E configurations download release assets from GitHub.
for tool in cargo asciinema agg; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    printf 'Required command is missing: %s\n' "$tool" >&2
    exit 1
  fi
done

cargo build --locked --target-dir target
mkdir -p docs/demo
asciinema rec --headless --return --overwrite \
  --output-format asciicast-v2 --window-size 80x30 \
  --title 'RKAN: generate CKAN metadata from the E2E configurations' \
  --command 'bash scripts/record-demo.sh --session' \
  docs/demo/rkan-e2e.cast
agg --quiet --theme github-dark --font-size 16 --fps-cap 15 \
  --idle-time-limit 2 --last-frame-duration 5 \
  docs/demo/rkan-e2e.cast docs/demo/rkan-e2e.gif

printf '\nCreated docs/demo/rkan-e2e.cast and docs/demo/rkan-e2e.gif\n'
