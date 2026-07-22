# 084 Compatible Chat Codec And Kimi K3 Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../028-kimi-platform-k3-direct-inference-proof.md`

## Objective

Realize Contract 024's provider-neutral structural codec and freeze the exact
Kimi K3 provider corpus without network or credential use.

## Governing References

- Research 018
- Contracts 005, 006, 009, 014, 020, and 024
- roadmap 028
- existing llama.cpp build-9910 compatible Chat Completions corpus

## Scope

- new `swallowtail-protocol-openai-chat` package with no provider, runtime,
  host, or consumer dependency
- bounded SSE fragmentation, comments, data fields, `[DONE]`, disconnect, and
  malformed-frame handling
- bounded common request, chunk, choice, delta, finish, model, usage, and error
  structures
- extraction of llama.cpp's common framing and envelope parsing without
  changing its request or semantic behavior
- dated Kimi Open Platform K3 manifest and offline catalogue, request, success,
  reasoning, usage, error, unknown, mismatch, and disconnect fixtures
- provider-specific reasoning, fixed parameters, errors, catalogue flags, and
  exclusions remain inside the Kimi fixture layer

## Acceptance Criteria

- [x] the common codec contains no Kimi, Moonshot, llama.cpp, provider, endpoint,
      credential, retry, capability, lifecycle, or runtime-event branch
- [x] both independent corpora pass the same bounded structural decoder
- [x] unknown semantic fields are returned as bounded unknowns or rejected
- [x] llama.cpp retains exact build, endpoint, request, text-only rejection,
      usage, cancellation, and serving-ownership behavior
- [x] Kimi fixtures pin `kimi-k3`, `/v1/models`, `/v1/chat/completions`, bearer
      access shape, reasoning efforts, output bound, returned-model agreement,
      usage, `[DONE]`, and safe error categories
- [x] no Kimi Membership, Kimi Code, regional key, tool, multimodal, state,
      retry, reconnect, or fallback claim appears
- [x] fixture tests use no credential, external request, paid inference, or live
      catalogue

## Completion Evidence

- `swallowtail-protocol-openai-chat` depends only on `serde_json` and owns
  bounded request JSON, SSE fragmentation/comments/data/`[DONE]`, common
  envelopes, and structural unknowns
- llama.cpp build-9910 request, stream, text-only rejection, usage, cancellation,
  attached ownership, and owned-serving suites pass through the shared codec
- the dated K3 corpus freezes platform access, authenticated catalogue shape,
  exact request, reasoning/output/usage, safe errors, unknowns, model mismatch,
  disconnect, fixed-parameter omissions, and exclusions offline
- the dedicated K3 guide and current schema settle `low`, `high`, and `max`; the
  fixture records the contradictory stale paragraph without widening behavior
- 37 focused protocol and llama.cpp tests pass; no credential, external request,
  paid inference, live catalogue, or new oversized-file finding is present

## Validation

- focused protocol, llama.cpp, and Kimi fixture tests
- focused warnings-denied clippy
- all-target workspace compile
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- sharing requires a provider identity or semantic fallback inside the codec
- extraction changes realized llama.cpp behavior
- Kimi documentation does not settle a field needed by the first subset
- the new package or fixtures create oversized-file debt

## Auto-Continuation

Yes. Continue to card 085 only when the codec passes both corpora and the Kimi
boundary is exact.
