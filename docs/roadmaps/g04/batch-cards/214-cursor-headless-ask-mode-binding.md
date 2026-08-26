# 214 Cursor Headless Ask-Mode Binding

Status: blocked; Research 224 empty deliver-now set
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Milestone: [g04.077 Cursor Headless Ask Mode](../077-cursor-headless-ask-mode.md)
Depends on: card 213; promoted Research 224 with a non-empty deliver-now set

## Goal

Bind only Research 224's exact Ask rows through one closed Cursor-local
selection, immutable prepared state, exact-version validation, and canonical
`--mode ask` dispatch.

## Scope

1. Add one documented adapter-local closed type for the exact admitted read
   modes. Do not add `Ask` to portable `HarnessMode` or expose raw strings,
   generic modes, configuration maps, permissions, or tool policy.
2. Preserve `CursorHeadlessRunProfileInput::new` as the exact current path:
   `Read` selects Plan and `ReadWrite` selects no mode. Add only a fallible
   typed selection path admitted by Research 224.
3. Retain the selected Ask value immutably in the prepared result and exact
   low-level driver binding. Expose safe inspectable adapter-local evidence
   only when needed; do not pretend the core plan contains a portable mode.
4. Validate exact build/behavior revision, `ResourceAccess::Read`, prepared
   selection, driver state, and command intent before process work. Reject Ask
   with `ReadWrite`, missing qualification, stale evidence, or mismatched
   low-level use.
5. Dispatch exactly one canonical `--mode ask`. Never select Agent, infer Ask
   from access, accept caller strings, rely on persisted mode, or fall back to
   Plan/Agent after a rejected Ask selection.
6. Preserve exact Plan and no-mode argv for existing callers. Keep `--trust`,
   explicit model and Research 183 parameters, working resource, ambient
   configuration, `AmbientHost`, and durable provider-state posture unchanged.
7. Preserve activity, usage, cancellation, deadline, terminal, failure,
   process ownership, and joined cleanup. Advance an adapter-private behavior
   revision only when Research 224 requires it.

## Acceptance Criteria

- [ ] only Research 224 deliver-now rows prepare Ask
- [ ] the public seam is closed and Cursor-local; no portable or raw mode API
      appears
- [ ] prepared state, driver state, exact version, access, and argv agree
- [ ] existing `Read` Plan and `ReadWrite` no-mode paths remain exact
- [ ] unsupported, mismatched, stale, or wider rows reject before process work
- [ ] Ask does not imply isolation, containment, permissions, tools, approval,
      network, or configuration suppression
- [ ] model parameters, retention, lifecycle, and cleanup claims do not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-cursor
effigy validate:focused swallowtail-adapter-cursor
effigy package:verify-affected swallowtail-adapter-cursor
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 215 only when typed preparation, exact-version gating,
canonical argv, default compatibility, rejection, and lifecycle proof pass.

## Stop Conditions

- immutable adapter-local state cannot bind Ask without a shared-plan or
  breaking public change
- mode can drift, be overridden, or fall back after preparation
- implementation needs raw config, writable authority, approval exchange,
  sibling-route work, shared contract/runtime change, or authority widening

## Out Of Scope

- portable mode promotion, another Cursor feature/route, live provider work,
  currentness, release, merge, rollover, or g04 closure
