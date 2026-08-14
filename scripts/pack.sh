#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DATE=$(date +%Y%m%d)
OUT="${ROOT}/../lift-ops-trainer-${DATE}.tar.gz"

echo "Packing lift-ops-trainer → ${OUT}"
tar -czf "${OUT}" \
  --exclude='target' \
  --exclude='.git' \
  --exclude='*.tar.gz' \
  -C "${ROOT}/.." \
  lift-ops-trainer

echo "Done: ${OUT}"
ls -lh "${OUT}"
