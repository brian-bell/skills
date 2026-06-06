#!/bin/bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
exec "$REPO_DIR/scripts/install-skills.sh" "$@"
