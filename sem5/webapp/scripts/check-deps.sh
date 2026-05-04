#!/usr/bin/env bash
set -euo pipefail

printf "Checking for existence of \u001b[36;1mpackage.json\u001b[0m...\n"
if [[ ! -f package.json ]]; then
  printf "\u001b[31;1mNo\u001b[0m \u001b[36;1mpackage.json\u001b[0m \u001b[31;1mfound. Are you in the project root\u001b[0m?\n"
  exit 1
fi

printf "Checking existence of dependencies inside \u001b[36;1mnode_modules\u001b[0m...\n"
if [[ ! -d node_modules ]] || npm ls --depth=0 >/dev/null 2>&1; then
  printf "\u001b[31;1mDependencies missing or invalid. Running npm install\u001b[0m...\n"
  npm install
else
  printf "\u001b[32;1mDependencies are installed\u001b[0m.\n"
fi

outdated_json="$(npm outdated --json --depth=0 2>/dev/null || true)"

echo "Checking whether dependencies are outdated..."
if [[ -n "$outdated_json" && "$outdated_json" != "()" ]]; then
  printf "\u001b[33;1mSome dependencies are outdated. Running npm update\u001b[0m...\n"
  npm update
else
  printf "\u001b[32;1mDependencies are up to date\u001b[0m.\n"
fi

kitty npm run dev &
