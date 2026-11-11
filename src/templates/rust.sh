#!/usr/bin/env bash
# description: Projet Rust (cargo init)
set -euo pipefail

name="${1:?usage: rust.sh <nom-du-projet> <dossier-cible>}"
target="${2:?usage: rust.sh <nom-du-projet> <dossier-cible>}"

mkdir -p "$target"
cd "$target"

# --vcs none : c'est le CLI qui fait le `git init`, pas cargo.
cargo init --name "$name" --vcs none .

echo "/target" > .gitignore
