# 001 Working Rules

Status: active
Owner: Tom
Updated: 2026-07-19

## Scope

These rules apply to all Swallowtail work before v1.0.

## Rules

- Use Effigy for task routing and validation when available.
- Use this repository's Northstar docs as project authority.
- Keep implementation behind contracts clear enough to test.
- Prefer small Rust crates and focused modules.
- Do not add compatibility aliases, silent fallbacks, or speculative extension
  layers without operator approval.
- Do not flatten provider differences into a fake uniform interface.
- Do not import consumer product concepts into portable crates.
- Keep external source repositories as evidence, not hidden build inputs.
- Keep each roadmap generation as a long-lived container for roughly 30-50
  numbered roadmaps. Phase changes alone do not authorize rollover; batch cards
  do not count toward the generation range.

## Closeout

State what changed, current state, failed or material validation, and the next
move. Keep one active Next Task pointer in `docs/roadmaps/README.md`.
