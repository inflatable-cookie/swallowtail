# 2026-07-28 Structured-Run Provider-Wide Closeout

## Changed

- Closed roadmap g02.024 and cards 077-079.
- Reconciled 25 production routes into 21 provider solutions.
- Closed `structured_run` at 18 `Yes`, two realtime `No`, and one
  owned-serving `Not applicable`.
- Required every `Yes` to map to a realized public prepared facade.
- Kept Kimi ACP, headless, and local-server identities separate. The installed
  facade selects ACP or headless explicitly; the endpoint-backed local facade
  remains independent.
- Preserved durable Kimi local-server thread retention without archive or
  deletion claims.
- Extended extracted-package verification to execute the closure-tranche
  structured suites.
- Refreshed the public API baseline for the intentional additive surfaces from
  cards 071-078.

## Evidence

- Full workspace tests passed across all 23 crates.
- Strict workspace Clippy and Kimi Rustdoc passed.
- Route, docs, examples, public API, formatting, and diff gates passed.
- All 23 crates assembled locally. The extracted workspace compiled all
  targets and executed packaged structured suites for Alibaba Model Studio,
  Claude Agent, DeepSeek, Gemini CLI headless, Kimi headless, Kimi local
  server, OpenCode, Pi, and xAI.
- Packaged Kimi lifecycle, binding-import, and interactive suites also passed.
- No live authentication or provider call ran.
- `effigy doctor` remains red on the file-size scan: 66 findings, with 50
  warnings and 16 errors. This cumulative tranche added oversized production
  and test files; no runtime, package, or conformance failure is hidden by that
  result.

## Current State

Gemini Live and OpenAI Realtime remain realtime-media interfaces, not bounded
structured-run routes. llama.cpp owned remains an ephemeral serving mechanism,
not an inference operation. Those classifications are deliberate.

No package was published. Candidate replacement and publication remain held
during working-application soak.

## Next

Return to operator-held card 060. Nucleus provider-session lifecycle adoption
requires an explicit authorize, defer, or rescope decision.
