#!/usr/bin/env bash
# Default pycache prefix must not write bytecode into the checked tree.

collision_pycache_root=$collision_scratch/pycache-root
mkdir -p "$collision_pycache_root/scripts"
cp "$collision_checker_root/scripts/check-roadmap-number-collision.py" \
  "$collision_pycache_root/scripts/"
cp "$collision_checker_root/scripts/roadmap_number_authority.py" \
  "$collision_pycache_root/scripts/"
collision_git "$collision_scratch" clone -q "$collision_canonical" \
  "$collision_pycache_root/work"

collision_run_default_pycache() {
  env -u PYTHONDONTWRITEBYTECODE -u PYTHONPYCACHEPREFIX \
    python3 -c '
import os
import runpy
import sys

sys.pycache_prefix = None
script = sys.argv[1]
sys.path.insert(0, os.path.dirname(os.path.abspath(script)))
sys.argv = sys.argv[1:]
runpy.run_path(script, run_name="__main__")
' "$@"
}

if ! collision_run_default_pycache \
  "$collision_pycache_root/scripts/check-roadmap-number-collision.py" \
  --root "$collision_pycache_root/work" \
  --authority "$collision_canonical" >/dev/null; then
  printf 'default-pycache production analogue failed\n' >&2
  exit 1
fi
if collision_run_default_pycache \
  "$collision_pycache_root/scripts/check-roadmap-number-collision.py" \
  --root "$collision_worker" \
  --authority "$collision_canonical" >/dev/null 2>&1; then
  printf 'default-pycache suite analogue unexpectedly passed\n' >&2
  exit 1
fi

collision_pycache_hits=$(
  find "$collision_pycache_root" \
    \( -name __pycache__ -o -name '*.pyc' -o -name '*.pyo' \) -print
  find "$collision_checker_root/scripts" \
    \( -name 'roadmap_number_authority*.pyc' \
       -o -name 'check-roadmap-number-collision*.pyc' \
       -o -path '*/__pycache__/roadmap_number_authority*' \
       -o -path '*/__pycache__/check-roadmap-number-collision*' \) -print
)
if [[ -n "$collision_pycache_hits" ]]; then
  printf 'bytecode written under checked tree:\n%s\n' "$collision_pycache_hits" >&2
  exit 1
fi
