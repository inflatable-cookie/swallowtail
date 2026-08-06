# Muse Code Headless Driver Core

Date: 2026-08-06
Roadmap: g03.045
Card: 136

## Outcome

Added `swallowtail-adapter-muse` as the exact low-level Muse Code route:

- opaque qualified-only `0.1.0-R708.1` compatibility claim
- target-only discovery requiring basename `muse-bin-0.1.0-R708.1`
- explicit `meta`, `muse-spark-1.2`, reasoning, and bounded `exec --json`
  command
- provider sandbox retained; writes, shell, web, foreign context, session log,
  parallel tools, and implicit question waits disabled
- strict session, command, run, task, sequence, model, output, and terminal
  correlation
- 1 MiB record, 8 MiB stream, 4,096-record, 64 KiB unknown-payload, and 256
  KiB output bounds
- task lifecycle projected as activity without task-list or subagent authority
- bounded unknown payloads projected under a Muse-only namespace
- distinct cancellation, deadline, provider failure, harness exit, host failure,
  runtime failure, event-delivery failure, and cleanup outcomes

The mutable `muse` launcher is rejected before probing or execution even when
it could report the same version after updating. The selected configured target
must be the signed versioned payload.

## Validation

- `python3 scripts/check-muse-code-corpus.py`: 5 passed
- `effigy validate:focused swallowtail-adapter-muse`: 14 passed; warnings-denied
  check passed
- `effigy package:verify-affected swallowtail-adapter-muse`: passed from the
  extracted source package

No authenticated provider work ran.

## Next

Execute card 137. Add local Meta account posture, configured instance and model
route, exact discovery-backed preparation, and the prepared read-only run.
