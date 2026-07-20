# Codex Dual-Driver Conformance

Date: 2026-07-19
Status: recorded

## Result

g01 card 020 is complete.

- Exec and app-server remain separate drivers under the same Codex integration
  family. Their adapter ids, transports, roles, and operation shapes differ.
- Dynamic registrations contain only the implemented roles. No common runtime
  switch branches on Codex identity.
- A driver rejects the other driver's preflight plan before process work.
- One-shot CLI and long-lived RPC profiles retain the same common Contract 011
  assertions while keeping distinct process/session lifecycle assertions.
- App-server structured output rejects before provider turn work; it does not
  inherit the exec surface's result-shape concept.

## Evidence

- Full Effigy QA passes with 61 tests.
- The adapter dependency tree contains core, runtime, and private JSON support;
  it does not depend on the concrete local host or a consumer.
- Core, runtime, testkit, and local-host source contain no Codex identity.
- Effigy doctor reports no errors.

## Next Lane

Card 021 compiles separate Soundcheck and Nucleus adoption roadmaps from the
proven public seams. It does not modify consumer repositories.
