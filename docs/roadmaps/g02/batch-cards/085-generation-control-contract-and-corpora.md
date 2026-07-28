# 085 Generation-Control Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../026-generation-control-feature-closure.md`
Depends on: card 084

## Objective

Promote the exact shared generation-control distinctions and freeze the
selected route corpora.

## Scope

1. Promote the generation-control application and enforcement boundary from
   Research 049.
2. Keep requested, planned, dispatched, provider-accepted, and effective
   controls separate.
3. Preserve provider/model/version-specific option domains and schema
   dialects.
4. Distinguish provider-native schema enforcement from harness-owned
   validation and retry.
5. Freeze deterministic corpora for:
   - OpenAI background reasoning and structured output
   - OpenAI Realtime output limit
   - Ollama attached reasoning and structured output
   - OpenCode HTTP reasoning and structured output
6. Freeze supported, rejected, ignored, drifted, and unverified-newer records.
7. Make card 086's prepared evidence and conformance expectations exact.

## Acceptance Criteria

- [x] contracts make the selected tranche deterministic
- [x] fixtures require no live access
- [x] unsupported and best-effort behavior remains explicit
- [x] card 086 names exact routes and cells

## Result

- Contract 040 defines independent controls, exact application states,
  model/version qualification, and structured-output enforcement source.
- OpenAI background and Realtime, Ollama attached, and OpenCode HTTP now have
  synthetic request corpora.
- Four focused corpus tests pass without accounts, credentials, external
  requests, containers, or model servers.
- OpenCode schema retry is fixed at zero for the first implementation.

## Auto-Continuation

Satisfied. Continue to card 086.
