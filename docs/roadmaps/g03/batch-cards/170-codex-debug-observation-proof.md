# 170 Codex Debug Observation Proof

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../055-opt-in-debug-observation-seam.md`
Depends on: card 169

## Goal

Prove the first production adapter emits correlated debug observations on the
Codex malformed-inbound failure path while preserving exact safe diagnostics
and poisoned-session behavior.

## Scope

1. Emit structured debug observations at the Codex app-server RPC pump
   boundaries that matter for the known malformed-inbound class: inbound wire
   reject, protocol parse/map failure, and stderr-ring snapshot when retained.
2. Correlate observations to the existing exact safe diagnostic codes.
3. Extend the scripted malformed-notification fixture (or sibling) to register
   a recording observer and assert observation kinds/codes without asserting
   raw secret-bearing payloads.
4. Keep safe diagnostic message policy from g03.047 unchanged.

## Out Of Scope

- other adapters
- changing diagnostic codes, classification, or session poison rules
- consumer-repo host wiring
- live Codex runs

## Acceptance

- [x] malformed-inbound fixture still returns the exact safe code and bounded
      safe excerpt behavior from g03.047
- [x] with an observer registered, at least one correlated debug observation
      is recorded for the failure
- [x] without an observer, behavior matches today's public path
- [x] focused Codex and affected-package validation passes

## Closeout

`RpcConnection` now carries `HostServices` and emits `WireInbound`,
`ProtocolParse`, and `StderrRing` observations on the malformed-inbound path,
correlated to the exact safe codes. Detail uses the existing sanitizer; raw
padding stays out. The original no-observer fixture remains; a sibling proves
observer capture.

## Validation

- `effigy validate:focused swallowtail-adapter-codex`: 164 passed
- `effigy package:verify-affected swallowtail-adapter-codex`: extracted
  package proof passed
- `effigy package:api`: unchanged at the v0.3.0 candidate baseline
