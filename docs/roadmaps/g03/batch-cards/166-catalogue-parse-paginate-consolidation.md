# 166 Catalogue Parse/Paginate Consolidation

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../054-remaining-duplication-tranches.md`
Depends on: card 165

## Goal

Extract one shared catalogue parse/paginate core and migrate the HTTP/SSE
catalogue adapters onto it.

## Scope

1. Extract the shared page-parse and pagination core in `swallowtail-runtime`
   covering the bounded JSON page shape, next-page continuation token, and
   entry projection used by the hosted catalogue adapters (deepseek,
   alibaba-model-studio, gemini, kimi-platform, llama-cpp, anthropic, openai,
   xai, bedrock, ollama, opencode, and peers).
2. Migrate the adapters whose page shapes fit; keep provider-specific
   decoders and envelope parsing adapter-local.
3. Record adapters with genuinely different page semantics as intentional.

## Out Of Scope

- public API, diagnostic, or behavior changes
- provider payload decoding

## Acceptance

- [x] one shared parse/paginate core exists with focused tests
- [x] migrated adapters pass with identical catalogue behavior
- [x] non-fitting adapters are recorded with reasons

## Closeout

The tranche premise (052 audit estimate of ~2,320 duplicated catalogue
lines) was superseded by card 159's measured completion evidence, which
already recorded the disposition for this family. Card 166 re-measured and
confirms the state is unchanged:

- the shared slice is the small validation family only: `bounded_text` in
  eight files, `optional_bounded_text` in five, `optional_u64` in five
  (exactly card 159's counts)
- the actual model-envelope parsing is provider-specific in every adapter
  (typed vs Value-based deserialization, entry metadata, modality
  observations, provider-identity checks, pagination cursors, health
  envelopes)
- extraction would require a `serde_json` dependency in runtime (violating
  the recorded "core, futures-core, zeroize" dependency posture) or plumbing
  each adapter's exact failure code through every call; the migration risk
  outweighs the ~60-line gain

Disposition (card 159, confirmed): the catalogue parse/paginate family stays
adapter-local behind the shared transport wrapper consolidation; no public
API, dependency, or behavior change. The remaining net-positive duplication
tranche is the ACP event projection (card 167).

## Stop Conditions

- stop if a catalogue observation or pagination behavior changes

## Auto-Continuation

Yes, to card 167 after acceptance.

## Validation

- `effigy validate:focused swallowtail-runtime`
- focused validation for every touched adapter
- `effigy package:api`
