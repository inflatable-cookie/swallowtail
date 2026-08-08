# Codex Debug Observation Proof

Date: 2026-08-08
Roadmap: g03.055
Card: 170

## Outcome

Codex app-server emits correlated debug observations on the malformed-inbound
failure path.

`RpcConnection` carries `HostServices` and, on malformed notification or
message failures, emits sanitized `WireInbound` and `ProtocolParse`
observations plus a `StderrRing` observation when a stderr tail is retained.
Safe diagnostic codes, bounded safe excerpts, and poisoned-session behavior
from g03.047 remain unchanged. A sibling fixture proves observer capture;
the original no-observer fixture still passes.

## Local Validation

- `effigy validate:focused swallowtail-adapter-codex`: 164 passed
- `effigy package:verify-affected swallowtail-adapter-codex`: extracted
  package proof passed
- `effigy package:api`: unchanged at the v0.3.0 candidate baseline

## Boundaries

No other adapters, consumer-repo wiring, tag, or release.
