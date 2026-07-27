# 2026-07-27 Model Catalogue No-Closure

## Changed

- Rechecked every former `model_catalog = No` against exact harness sources
  and official provider APIs.
- Corrected Research 042: Qwen Code exposes a stream-JSON
  `get_available_models` control request, and Alibaba Model Studio exposes an
  official deployable-model control-plane API.
- Added Kimi local-server attached catalogue discovery.
- Added bounded negotiated model-option evidence to already-authorized Gemini
  and Kimi ACP sessions.
- Added separate hosted catalogue drivers and typed prepared operations for
  OpenAI, Gemini, and xAI.
- Added separate Alibaba base/custom deployable-model catalogue discovery.
- Added Qwen safe-mode ephemeral process discovery with capability
  negotiation, bounded projection, and joined cleanup.
- Updated Contract 020, architecture, route guidance, and the solution matrix
  without relabeling caller-supplied, session-negotiated, managed-agent, or
  serving-only semantics.

## Evidence

- Qwen exact `0.19.11` and current `0.21.0` tagged control-plane source.
- Kimi exact `0.28.1` and `0.29.0` model-catalogue corpus.
- Official OpenAI, Gemini, xAI, and Alibaba response-shape decoders.
- Prepared Qwen process execution and ACP/local-server lifecycle fixtures.
- Provider-specific immutable catalogue plan and access bindings.
- In-flight hosted catalogue deadline fixtures prove transport stop is
  requested and joined before timeout returns.
- The sorted 21-row CSV now contains 16 `Yes`, two
  `Session-negotiated`, two `Not applicable`, one `Caller-supplied`, and zero
  `No`.

## Validation

- focused core, runtime, Pi, Qwen, Kimi, OpenAI, Gemini, xAI, and Alibaba
  suites: 334 passed; two installed live probes ignored
- `effigy check:rust`, `effigy lint:rust`, `effigy check:examples`,
  `effigy format:check`, `effigy qa:docs`, and `effigy qa:routes`: passed
- CSV sort, row-count, and model-catalogue disposition assertions: passed
- `git diff --check`: passed

## Current State

Roadmap g02.021 and cards 066-070 are complete. Every selected solution with a
qualified machine-readable model source now exposes it through Swallowtail.
Catalogue presence still proves neither entitlement nor compatibility with an
inference route.

## Next

Card 060 remains planned and operator-held: decide whether to issue the
documentation-only Nucleus thread-lifecycle adoption handoff.
