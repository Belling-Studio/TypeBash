#!/usr/bin/env bash
set -euo pipefail

# ──────────────────────────────────────────────────────────
# Generated professionally by TypeBash Compiler
# ──────────────────────────────────────────────────────────

typebash_err_handler() {
    local exit_code=$1
    local line_no=$2
    local bash_command=$3
    echo "" >&2
    echo "==================================================" >&2
    echo "❌ [TypeBash Runtime Error] Script failed!" >&2
    echo "  • Exit Code : $exit_code" >&2
    echo "  • Line      : $line_no" >&2
    echo "  • Command   : $bash_command" >&2
    echo "==================================================" >&2
}
trap 'typebash_err_handler $? $LINENO "$BASH_COMMAND"' ERR

# good_script.tb
readonly BACKUP_DIR="./backups"
readonly TARGET_HOST="backup-server"
notify () {
    local MSG=$1
    echo "${MSG}"
}

notify "Starting automated backup process..."
declare -a FILES=("config.json" "database.sql" "assets.tar.gz")
for FILE in "${FILES[@]}"; do
    echo "${FILE}"
done
mkdir -p ./backups
echo 'All files synchronized successfully'
