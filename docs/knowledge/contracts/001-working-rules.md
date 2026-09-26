# 001 Working Rules

Status: active
Owner: Tom
Updated: 2026-09-26

## Scope

These rules apply to all Swallowtail work before v1.0.

## Rules

- Use Effigy for task routing and validation when available.
- Use this repository's knowledge files (`docs/knowledge/`) as project
  authority.
- Keep implementation behind contracts clear enough to test.
- Prefer small Rust crates and focused modules.
- Do not add compatibility aliases, silent fallbacks, or speculative extension
  layers without operator approval.
- Do not flatten provider differences into a fake uniform interface.
- Do not import consumer product concepts into portable crates.
- Use the minimum sufficient acceptance oracle. Every restrictive condition
  must trace to a concrete consumer requirement, safety boundary, or named
  failure harm. Do not fail useful work on internal provider activity when the
  required boundary is narrower, such as no prompt, model session, tool use,
  or inference. Recheck that trace before spending a one-shot live allowance;
  if it is absent, correct the plan rather than hardening the probe.
- Keep external source repositories as evidence, not hidden build inputs.
- Run all-route version currentness as a named Contract 029 checkpoint. It is
  a standing plan item, never finished. Do not extend a compatibility claim
  from registry `latest` or local `--version` alone.

## Roles And Authority

- The planner (Chatterbox) explores problems with the operator, keeps
  `docs/plan.md` and the knowledge files current, writes task briefs, and
  requests dispatch approval. It writes no runtime code and does not merge.
- Tasks, briefs, status, review and closeout live in Queue, never in this
  repository. A worker implements one brief in the worktree Queue gives it,
  edits only what the brief owns, and stops at exact-head review; it never
  merges.
- Triage notes are intake, never execution authority. Papercuts in
  `PAPERCUTS.md` are observations for later triage, never an automatic
  triage note or task.
- Release mutations (tag, push, publication, GitHub Release, consumer or
  provider changes) need explicit operator authority under Contract 036. A
  green gate, changelog, or merge grants none.

## Validation Before Push

A push that touches `docs/**`, `PAPERCUTS.md`, or `CHANGELOG.md` runs
`effigy qa:docs` first. The git pre-push hook is the enforcement; this rule is
the authority. Install the hook once with `effigy hooks:install`
(`effigy bootstrap` and `effigy doctor` run that task).
