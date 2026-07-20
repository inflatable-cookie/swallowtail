# Local Process Host

Date: 2026-07-19
Status: recorded

## Result

g01 card 017 is complete.

- `swallowtail-host-local` is the first concrete host crate.
- Its approval catalog maps opaque runtime references to local executables,
  environment bundles, and working resources. Requests cannot supply paths or
  raw environment values.
- Child environments start empty. Arguments and all three I/O directions are
  host-bounded.
- One supervisor owns each child. Independent bounded readers drain stdout and
  stderr. Wait resolves only after exit and reader joins.
- Graceful stop closes stdin. Force-stop is explicit and idempotent. Dropped
  unwaited handles request best-effort force cleanup.
- Safe failures and default formatting exclude paths, environment values,
  stdin, stdout, and stderr payloads.

## Evidence

- Full Effigy QA passes with 44 tests.
- Deterministic real-process fixtures cover exact approval failures, working
  resources, environment injection, bounded round-trip I/O, overflow,
  graceful cancellation, force-stop, exit, and joined cleanup.
- The new host crate adds no external dependency and no oversized-file warning.

## Next Lane

Card 018 establishes `swallowtail-adapter-codex` and implements the bounded
`codex exec` structured-run surface using only the shared runtime process port.
