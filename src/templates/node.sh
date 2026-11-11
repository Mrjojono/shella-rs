#!/usr/bin/env bash
# description: Projet Node (npm init)
set -euo pipefail

name="${1:?usage: node.sh <nom-du-projet> <dossier-cible>}"
target="${2:?usage: node.sh <nom-du-projet> <dossier-cible>}"

mkdir -p "$target"
cd "$target"

npm init -y >/dev/null
npm pkg set name="$name" >/dev/null   # nécessite npm >= 7

printf "node_modules/\n" > .gitignore
