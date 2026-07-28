# 2026-07-28 Generation-Control Tranche And Matrix Continuation

Status: complete

## Changed

- Added provider-neutral structured-output enforcement evidence:
  provider-native or harness-validated.
- Added exact generation-control preparation and dispatch for:
  - OpenAI background reasoning and inline JSON Schema
  - OpenAI Realtime output maximum
  - Ollama attached reasoning and inline JSON Schema
  - OpenCode model-variant reasoning and zero-retry harness schema validation
- Bound each request to matching capability constraints in its configured
  instance, model route, preflight plan, and driver validation.
- Added selected-model capability evidence for Ollama and reasoning plus tool
  capability evidence to OpenCode catalogue entries.
- Updated seven matrix cells from `No` to `Yes`.

## Matrix State

The 48-cell generation-control audit closes at:

- 13 output-token-limit `No`
- 11 reasoning-selection `No`
- 17 structured-output `No`
- 41 generation-control `No` total

The complete audited feature range now has 444 `No` and 29 `Not applicable`
cells. Exact identities and classification counts are machine-enforced.

Roadmap 027 keeps the broader matrix programme active. Its 74 starting cells
cover attachments, consumer tools, approval or question exchange, and
external search.

## Validation

- all OpenAI, Ollama, and OpenCode package tests pass
- workspace inventory: 954 tests, 950 pass, four separately gated installed
  probes ignored
- workspace examples compile
- workspace Clippy passes with warnings denied
- all 23 public API declaration fingerprints pass
- all 23 local crate archives assemble; the extracted workspace and selected
  packaged facade suites pass
- route-matrix checks pass
- no live account, credential, provider request, container, or model server
  was used

## Retained Risks

- OpenCode generation controls require exact selected-model catalogue
  evidence. They do not fall back when variants or tool capability are absent.
- Ollama reasoning remains model-dependent and requires the selected detail to
  advertise thinking support.
- OpenAI controls remain exact to the selected GPT-5.6 background route and
  Realtime session surface.
- Twenty starting cells remain upstream-unsupported, three xAI cells remain
  operator-held, and three managed-agent cells remain operation-shape
  mismatches.

## Next

Execute card 088. Revalidate all 74 input/callback `No` cells before promoting
contracts or implementation.
