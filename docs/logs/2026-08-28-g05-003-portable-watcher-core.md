# 2026-08-28 g05.003 Portable Watcher Core

Status: complete
Owner: Tom
Card: 008
Contract: 059

## Result

Provider-neutral watcher identity, lifecycle, ownership, dual control roles,
optional host-service registration, activity projection, and testkit assertions
landed without process launch or route selection.

## Evidence

- `crates/swallowtail-core`: `WatcherId`, owning-turn keys, phases, terminal
  causes, revisions, summaries, requester identity; `HostServiceKind::Watcher`;
  `ActivityKindClass::HostWatcher`
- `crates/swallowtail-runtime`: pure `WatcherRegistry`, model/operator control
  roles, `WatcherHostService` registration on `HostServices`,
  `project_watcher_activity`
- `crates/swallowtail-testkit`: `assert_portable_watcher_lifecycle_contract`

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-testkit`
- `git diff --check`
