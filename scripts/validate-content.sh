#!/usr/bin/env bash
set -euo pipefail

echo "Content validation placeholder."
echo "Future: run schema checks + cargo test on all YAML under content/"
find content -name '*.yaml' -print
