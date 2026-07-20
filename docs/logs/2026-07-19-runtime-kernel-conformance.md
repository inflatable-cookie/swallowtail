# Runtime Kernel Conformance

Date: 2026-07-19
Status: recorded

## Result

g01 cards 015-016 and roadmap 005 are complete.

- Testkit exposes public runners for one-shot structured CLI, long-lived RPC,
  hosted direct API, attached self-hosted, and owned self-hosted profiles.
- Each report covers the 14 common Contract 011 assertions. Profile-specific
  assertions prove process, session, hosted no-process, attached no-stop, and
  owned cleanup boundaries.
- Deliberate failures identify route, access, ownership, and topology before
  recorded effects.
- Profile composition uses public core and runtime APIs. No provider process,
  network, credential, executor, or consumer implementation entered the
  conformance layer.

## Evidence

- All 38 tests pass.
- Full Effigy QA and rustdoc warnings-as-errors pass.
- Runtime depends only on core and `futures-core`; core remains dependency-free.
- Source scans find no provider, consumer, UI framework, executor,
  transport-client, or credential-store coupling.
- Effigy reports no new oversized-file warning from the conformance batch.

## Next Lane

Roadmap 006 is active. Card 017 implements the first real local process host
behind approved opaque executable and working-resource references. Codex
protocol behavior remains out of scope until that host boundary passes.
