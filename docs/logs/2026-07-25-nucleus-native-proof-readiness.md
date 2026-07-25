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
- explicit disposable Git fixture binding instead of the Nucleus source tree
- count-only evidence that excludes prompts, output, provider ids, errors,
  credentials, project ids, and paths

## Evidence

The consumer batch passes focused protocol, adapter, storage, server, desktop,
client, and redaction tests. Nucleus desktop checking reports zero errors and
all 20 client tests pass. Missing or relative proof roots stop before desktop
launch.

The completed Nucleus runtime is
`2a6d72a8d3326cc70c6852f8fa86ff7f8ca995f2`. The exact pilot handoff is
recorded separately in `2026-07-25-nucleus-native-pilot-handoff.md`.

No Codex process, provider request, credential lookup, workspace write,
publication, push, tag, release, or registry mutation occurred.

## Gate

Nucleus card 010 is complete. Swallowtail card 041 is ready but still requires
separate acceptance of the ChatGPT-backed 15-attempt and 60-minute live
ceilings.
