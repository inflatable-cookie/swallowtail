# Nucleus Native Proof Readiness

Date: 2026-07-25

## Outcome

The Nucleus companion lane now implements every deterministic prerequisite
from Swallowtail card 040:

- one isolated Nucleus-owned desktop root
- the unchanged normal `~/.nucleus` default
- a positive proof deadline capped at the 180-second production default
- normal Agent Chat cancellation outside the serialized chat mutex
- typed cancelled, timed-out, cleanup-failed, and other failure mapping
- distinct durable completed, cancelled, timed-out, and failed truth
- explicit native launch and query-only evidence selectors
- count-only evidence that excludes prompts, output, provider ids, errors,
  credentials, project ids, and paths

## Evidence

The consumer batch passes focused protocol, adapter, storage, server, desktop,
client, and redaction tests. Nucleus desktop checking reports zero errors and
all 20 client tests pass. Missing or relative proof roots stop before desktop
launch.

The consumer used clean Swallowtail source
`2959810f2da3cc64b28cf979094e0166a34c3ff8`. Its own implementation is not yet
checkpointed, so no exact resulting Nucleus commit is claimed.

No Codex process, provider request, credential lookup, workspace write,
publication, push, tag, release, or registry mutation occurred.

## Gate

Nucleus card 010 can freeze the exact pilot tuple only after the operator
reviews and checkpoints the proof-readiness changes. Swallowtail card 041
still requires separate acceptance of the 15-attempt and 60-minute live
ceilings.
