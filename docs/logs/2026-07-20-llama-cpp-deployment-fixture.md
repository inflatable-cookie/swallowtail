# llama.cpp Deployment Fixture

Date: 2026-07-20
Roadmap: 015
Card: 048

## Changed

- rechecked current official llama.cpp release, server, properties, catalogue,
  Chat Completions, server-test, and model-fixture evidence
- froze installed `llama-server` build `9910`, commit
  `f5525f7e7a7e7cbecd386144299493ea40499bd3`, rather than substituting current
  upstream `b10069`
- pinned an optional operator-supplied 1,185,376-byte Stories 260K GGUF by
  repository revision and SHA-256 without downloading or bundling it
- added a fixture-first llama.cpp adapter crate with bounded health,
  properties, single-model catalogue, request, SSE, failure, disconnect, and
  unobserved-semantics parsers

## Decisions

- attached single-model loopback deployment; endpoint and port remain
  host-approved
- exact facade: native `/health` and `/props`, plus `/v1/models` and streaming
  `/v1/chat/completions`
- explicit route alias; no provider identity inferred from the artifact
- ChatML/Jinja, reasoning disabled, text-only capability claim
- local cancellation may close owned connection work only; close never stops
  the server or touches the artifact
- Contracts 007 and 014 already cover the shared boundary; no contract delta
  required

## Evidence

- eight focused tests pass
- default QA uses synthetic payloads only
- no server was started, no model or credential was read, and no authenticated
  request was made

## Continuation

Card 049 is ready. Implement the attached b9910 driver against this exact
facade. Card 050 remains planned and in bounds.
