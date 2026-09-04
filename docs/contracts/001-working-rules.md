# 001 Working Rules

Status: active
Owner: Tom
Updated: 2026-09-04

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
- Run all-route version currentness as a named Contract 029 checkpoint. It is
  a standing lane, not a generation runway goal. Do not extend a
  compatibility claim from registry `latest` or local `--version` alone.

## Roles And Authority

- The Chatterbox thread is the planning authority. It explores problems with
  the operator, reconciles `docs/triage/`, and promotes canonical planning on
  `main` only after explicit operator confirmation. It writes no runtime code.
- The coordinator consumes the dispatch manifest published in ready planning,
  launches the whole approved frontier, places independent review, owns the
  merge gate, and performs closeout on the reserved shared surfaces. It does
  not design lanes or invent concurrency.
- A worker exists only through a coordinator-dispatched handoff under
  `docs/handoffs/` whose frontmatter names worker mode and orchestrator
  dispatch authority. Do not infer worker mode from a branch, path, or
  harness. A worker edits only the paths its card and manifest own.
- Triage notes are intake, never execution authority. Papercuts in
  `PAPERCUTS.md` are observations for later triage, never an automatic
  backlog.
- Release mutations (tag, push, publication, GitHub Release, consumer or
  provider changes) need explicit operator authority under Contract 036. A
  green gate, changelog, or closeout commit grants none.

## Closeout

State what changed, current state, failed or material validation, and the next
move. Keep one active Next Task pointer in `docs/roadmaps/README.md`.
