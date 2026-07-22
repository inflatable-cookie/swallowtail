# 2026-07-22 Alibaba Conversation Records And Fixtures

## Changed

- added a session-only provider-state policy: prohibited by default, or one
  durable provider conversation with item-and-conversation delete-on-close
- bound that policy through operation requirements, immutable preflight, open
  requests, exact capability constraints, and pure runtime compatibility
- added generic conversation and aggregate conversation-item deletion kinds;
  terminal evidence cannot use conversation deletion as item-deletion proof
- added a reusable provider-conversation assertion pack for preflight mismatch,
  one active turn, two-turn maximum, deletion truth, and unconfirmed local
  cancellation/deadline races
- added fixture-only `swallowtail-adapter-alibaba-model-studio` with the exact
  2026-07-22 Singapore workspace-dedicated Conversations and Responses corpus
- completed card 088 and advanced card 089 to ready

## Frozen Boundary

- exact region `ap-southeast-1`, workspace-dedicated audience, general Model
  Studio API key, pay-as-you-go metering, provider support authority, and model
  `qwen3.7-plus-2026-05-26`
- conversation creation on open; maximum two serial Responses turns with only
  `model`, `input`, `conversation`, `stream=true`, `store=false`, and
  `reasoning.effort=none`
- one complete bounded ascending item inventory, every item deletion, then
  separate conversation deletion
- fragmented SSE sequence, text deltas/done, completed output/model/usage,
  request correlation, safe provider failure, structural unknown, disconnect,
  deletion mismatch, and late-remote-mutation fixtures
- no Coding Plan, Token Plan, alias, other region, response storage, cache,
  tool, output bound, background, retry, reattachment, resume, or fallback

## Boundary

The shared policy does not reuse structured-run `OperationPolicy`. It contains
no Alibaba, workspace, region, Qwen, or Responses identity. Conversation,
conversation item, provider response, runtime session, runtime turn, request,
stream, route, model, configured instance, access profile, and execution host
remain separate. The new adapter crate has protocol and selection fixtures but
no production network driver.

## Validation

- full repository QA passes with 396 tests; three installed or live probes
  remain separately gated
- focused core, runtime, testkit, and Alibaba tests pass
- focused warnings-denied clippy passes
- dependency audit finds no Alibaba identity in core, runtime, or testkit
- every new Rust file remains below 250 lines after formatting
- doctor remains at the inherited 19 findings: 12 warnings and seven errors
- no account, workspace, key, external request, conversation, or paid inference
  was used

## Continuation

Card 089 is the sole ready task. Implement the exact production driver against
the frozen corpus, then run local and remote-authoritative conformance and close
roadmap 029. Live authentication remains separately gated.
