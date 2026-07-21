# 2026-07-21 Kimi ACP Production Driver

## Changed

- added the distinct `swallowtail-adapter-kimi` production crate
- pinned Kimi Code `0.28.1` over the existing provider-neutral ACP v1 framing
- added first-class load-with-replay beside replay-free resume
- extended durable session bindings with working resource and expanded access
  policy
- added explicit replay and working-resource text-write capability bounds
- added local-host bounded UTF-8 create-or-replace with traversal, symlink,
  lease, representation, regular-file, and size rejection
- held Kimi delegated authentication as an opaque scoped lease; no login or
  credential extraction occurs
- repaired Kimi fixture authority metadata from the abandoned mandatory App
  Sandbox helper to explicit `AmbientHost`

## Current State

Kimi is Swallowtail's second production ACP agent and its eighth production
transport route. New, load, and resume remain separate. Load returns ordered,
bounded historical replay before the ready session. Resume rejects historical
replay. Every persistent attachment must match the provider session,
configured instance, execution host, model route, model, working resource, and
full ambient read-write policy before credentials, resources, processes, or
wire work begin.

The working directory is location and callback scope only. The Kimi process
and descendants retain ambient host authority. The consumer decides whether to
offer that route. Enforced isolation remains an optional, separate configured
route and never receives silent fallback.

## Validation

- deterministic Kimi driver tests cover new, load, resume, replay, prompt,
  text write, cancellation, exact binding rejection, and joined cleanup
- focused runtime, local-host, ACP, Codex, OpenCode, and Kimi tests pass
- focused warnings-denied clippy and `git diff --check` pass
- one existing Codex callback-deadline test failed once during a combined run,
  then passed in isolation and in the complete focused rerun
- full repository QA passes with 269 tests; Gemini and OpenCode installed
  probes remain gated and ignored by default
- doctor remains at the inherited 19 findings: 12 warnings and 7 errors

## Continuation

- card 059 is ready: cross-agent ACP, persistent-session, topology, redaction,
  disconnect, and cleanup conformance
- any installed Kimi probe remains explicitly operator-gated
- provider- or host-enforced Kimi isolation remains outside this ambient lane
