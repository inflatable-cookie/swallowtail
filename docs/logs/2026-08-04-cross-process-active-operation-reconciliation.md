# 2026-08-04 Cross-Process Active Operation Reconciliation

## Outcome

Swallowtail now exposes a read-only reconciliation role for consumer turns
whose live runtime handle was lost across a crash or process exit. It is
separate from provider-session import, load, resume, stream reattachment,
retry, cancellation, callback handling, management, and child control.

The portable boundary adds exact interrupted runtime-turn correlation,
optional provider-turn attribution, bounded replacement replay, explicit
snapshot completeness, cancellation and deadline control for the observation,
joined cleanup, prepared evidence, runtime registration, and one capability.
Session-scoped evidence cannot claim a terminal result.

## Route Acceptance

Codex app-server is the exact-turn mapping. Across the qualified thread-read
range `0.105.0..=0.146.0`, one `thread/read(includeTurns: true)` revalidates
thread, cwd, source, optional turn id, status, and bounded history. Exact
active, completed, failed, and cancelled states are available. A missing
requested turn fails closed. The observer issues no turn start, interrupt,
resume, or lifecycle request.

OpenCode HTTP is the session-scoped mapping. Across qualified
`1.14.48..=1.18.10` segments, it revalidates health, version, directory,
session, status, and bounded retained messages. Because `prompt_async` exposes
no exact prompt or turn id, idle becomes `InactiveUnresolved`, never completed.
The observer issues no prompt, abort, delete, load, resume, import, or callback
request.

Research 099 classifies every other route by the evidence needed to qualify.
The classification remains separate from the main provider feature CSV so
non-actionable negative columns do not dilute its conversion purpose.

Controlled application shutdown remains separate. Reconciliation repairs
crash/restart observation; it does not change ordinary handle-close or add a
detach-and-preserve disposition.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-opencode swallowtail-adapter-codex`
  — 437 tests passed across nine binaries; checks and clippy passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-opencode swallowtail-adapter-codex`
  — all four extracted packages compiled
- `effigy qa:docs`
- `cargo fmt --all -- --check`
- `git diff --check`
- no authenticated provider work or live provider operation

## Current State

Card 070 and roadmap g03.027 are complete. The sole Next Task has returned to
the g03 evidence gate. Kimi local-server cursor checkpoints and explicit
controlled-shutdown detach semantics remain separate promotion candidates.
