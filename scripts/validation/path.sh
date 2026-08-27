#!/usr/bin/env bash

# Resolve symlink-aliased paths to one spelling. macOS review worktrees under
# /var/... must match Cargo lockfile patch paths written as /private/var/... .
validation_canonical_path() {
  (cd "$1" && pwd -P)
}
