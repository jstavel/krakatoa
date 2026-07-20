#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

REPO_ROOT=$(get_repo_root) || exit 1
cd "$REPO_ROOT"

CREATE_SCRIPT="$SCRIPT_DIR/create-new-feature.sh"

DRY_RUN_OUTPUT=$("$CREATE_SCRIPT" --json --dry-run "$@" 2>/dev/null) || {
    echo '{"error": "Failed to compute branch name"}' >&2
    exit 1
}

BRANCH_NAME=$(echo "$DRY_RUN_OUTPUT" | jq -r '.BRANCH_NAME')
FEATURE_NUM=$(echo "$DRY_RUN_OUTPUT" | jq -r '.FEATURE_NUM')

if [ -z "$BRANCH_NAME" ] || [ "$BRANCH_NAME" = "null" ]; then
    echo '{"error": "Empty branch name"}' >&2
    exit 1
fi

if git rev-parse --verify "$BRANCH_NAME" >/dev/null 2>&1; then
    echo '{"error": "Branch already exists: '"$BRANCH_NAME"'"}' >&2
    exit 1
fi

git checkout -b "$BRANCH_NAME" >/dev/null 2>&1
echo "{\"BRANCH_NAME\":\"$BRANCH_NAME\",\"FEATURE_NUM\":\"$FEATURE_NUM\"}"
