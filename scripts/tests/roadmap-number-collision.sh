#!/usr/bin/env bash
# Hermetic mutation evidence for canonical-main roadmap card collisions.
# Throwaway repos and remotes only. Does not read the live checkout or its remotes.
set -euo pipefail

collision_tests_dir=$(cd "$(dirname "$0")" && pwd)
collision_checker_root=$(cd "$collision_tests_dir/../.." && pwd)
# Used by sourced case files.
# shellcheck disable=SC2034
collision_checker=(
  python3 "$collision_checker_root/scripts/check-roadmap-number-collision.py"
)
collision_cases=$collision_tests_dir/roadmap-number-collision
collision_exit_hooks=()

collision_on_exit() {
  local collision_hook
  for collision_hook in "${collision_exit_hooks[@]+"${collision_exit_hooks[@]}"}"; do
    eval "$collision_hook"
  done
  rm -rf "$collision_scratch"
}

# shellcheck source=scripts/tests/roadmap-number-collision/helpers.sh
source "$collision_cases/helpers.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/helpers-isolation.sh
source "$collision_cases/helpers-isolation.sh"

collision_scratch=$(mktemp -d)
trap collision_on_exit EXIT

# shellcheck source=scripts/tests/roadmap-number-collision/fixtures.sh
source "$collision_cases/fixtures.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/cases-stale.sh
source "$collision_cases/cases-stale.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/cases-reuse.sh
source "$collision_cases/cases-reuse.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/cases-unicode.sh
source "$collision_cases/cases-unicode.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/cases-containment.sh
source "$collision_cases/cases-containment.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/cases-transport.sh
source "$collision_cases/cases-transport.sh"
# shellcheck source=scripts/tests/roadmap-number-collision/cases-pycache.sh
source "$collision_cases/cases-pycache.sh"

printf 'roadmap number collision tests passed\n'
