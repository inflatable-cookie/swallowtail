# 107 Ollama Multi-Release Native Corpus

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../038-ollama-native-attached-runtime-proof.md`

## Objective

Freeze the exact native Ollama subset and prove its maintained
`0.14.0`-through-`0.32.1` behavior offline.

## Scope

- descriptor compatibility claim with one maintained semantic-version segment
- tagged-source evidence at `0.14.0`, `0.18.0`, `0.30.0`, and `0.32.1`
- exact `/api/version`, bounded `/api/tags`, `/api/ps`, and `/api/show`
- streaming `/api/chat` request and NDJSON output, terminal usage, error,
  malformed-record, and disconnect fixtures
- text-only selected model with exact tag and safe digest
- rejection of tools, thinking, vision, format, cloud, mutation,
  administration, retry, and fallback
- separately gated installed-runtime version/catalogue probe
- no production network driver or live model inference

## Acceptance Criteria

- [x] every claimed qualification point has frozen source and corpus evidence
- [x] one text behavior segment spans only the unchanged selected subset
- [x] versions below, above, prerelease, malformed, and stale fail closed
- [x] catalogue categories remain source-scoped and cannot create a route
- [x] native NDJSON errors after HTTP success remain provider failures
- [x] corpus needs no installed Ollama, model, credential, container, or network

## Validation

- focused adapter corpus and selection tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Evidence

- the adapter descriptor publishes one maintained semantic segment from
  `0.14.0` through `0.32.1` under behavior revision
  `ollama.native-text-v1`.
- the corpus records exact tagged commits for `0.14.0`, `0.18.0`, `0.30.0`,
  and `0.32.1`; each retains the selected version, tags, ps, show, chat, and
  native NDJSON source shapes.
- pure codecs bind exact version, bounded installed/running observations,
  selected detail with a safe normalized digest, and a single text chat
  request with positive `num_predict`.
- fixtures reject below/above range, prerelease, malformed version, cloud
  model fields, thinking semantics, malformed records, missing terminal state,
  and unbounded input.
- an error object after output remains an ordered provider failure despite HTTP
  success.
- 11 focused tests pass; the installed loopback version/catalogue probe is
  separately gated and ignored by default. Focused warnings-denied clippy and
  `git diff --check` pass.
- `effigy doctor` remains at seven inherited errors and twelve warnings.

## Auto-Continuation

Yes. Continue to card 108 when the corpus and pure selection boundary pass.
