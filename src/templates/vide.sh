#!/usr/bin/env bash
# description: Dossier vide (aucun scaffolding)
set -euo pipefail

target="${2:?usage: vide.sh <nom-du-projet> <dossier-cible>}"

mkdir -p "$target"
