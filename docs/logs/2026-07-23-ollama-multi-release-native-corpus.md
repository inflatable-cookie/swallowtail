# 2026-07-23 Ollama Multi-Release Native Corpus

## Changed

- Added `swallowtail-adapter-ollama` with no production transport yet.
- Published one maintained `ollama.runtime` compatibility segment from
  `0.14.0` through `0.32.1`.
- Froze tagged commits and one shared text corpus at `0.14.0`, `0.18.0`,
  `0.30.0`, and `0.32.1`.
- Added bounded pure codecs for `/api/version`, `/api/tags`, `/api/ps`,
  `/api/show`, and streaming `/api/chat`.
- Normalized the runtime digest into safe SHA-256 manifest evidence without
  exposing the raw manifest, template, license, model info, endpoint, or path.
- Kept installed, running, and selected-detail observations source-scoped.
  None constructs or selects a model route.
- Added one separately gated loopback version/catalogue probe. Default QA does
  not need Ollama, a model, a credential, a container, or network access.

## Failure Truth

- Below-range, above-range, prerelease, malformed, and fixture-claim drift fail
  closed.
- Cloud model fields and thinking output are outside the qualified local text
  subset.
- A native NDJSON error after output is a provider failure even though the HTTP
  response already succeeded.
- Malformed records and streams without a terminal record remain distinct safe
  protocol and disconnect failures.

## Validation

- 11 focused adapter tests pass
- one installed-runtime probe is ignored unless explicitly gated
- focused warnings-denied clippy passes
- `git diff --check` passes
- doctor remains at the inherited 19 findings

## Next

Card 108 maps the pure corpus onto host-approved network work. It retains
external runtime ownership, local unauthenticated access, one inference
attempt, and runtime-managed residency.
