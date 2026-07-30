# Grok ACP Driver And Prepared Facade

Date: 2026-07-30
Status: completed

## Changed

- added exact `--no-auto-update agent stdio` ACP process dispatch
- validated wire v1, exact agent version, required capabilities,
  `cached_token` default access, and preflight-bound `grok-4.5`
- acquired one delegated credential lease and sent one headless
  `authenticate` request; discarded the complete response value
- added durable local session allocation, bounded text turns, native
  cancellation, disconnect handling, and explicit attachment close
- projected assistant text, reasoning summaries, plans, provider tools,
  unknown activity, provider requests, and terminal outcomes
- added a typed prepared model selection, session input, operation evidence,
  plan, request, and session opener beside the low-level driver
- joined protocol, turn, process, working-resource, and credential work

## Authority

The route remains an ambient harness relay. It binds read-write workspace
authority and durable provider-session preservation explicitly. Client file
callbacks are read-only; provider-owned tools retain their native harness
authority. Permission requests stop the turn and are never silently approved.

Close releases the Swallowtail attachment and host leases. It does not delete,
archive, restore, load, or resume the durable Grok session. No sandbox,
read-only harness, login, API-key, model fallback, executable search, or
provider-state cleanup claim was added.

## Privacy

The authentication response is treated as opaque success and dropped in full.
Deterministic tests return a private sentinel and prove it does not enter
writes, events, outcomes, or diagnostics. Prompts and provider payloads remain
outside stable diagnostics.

## Validation

- 12 focused `swallowtail-adapter-grok` tests pass
- deterministic ACP tests cover success, provider permission, and active-turn
  cancellation
- prepared discovery and session construction pass on local and
  remote-authoritative fixture topologies
- focused all-target warnings-denied clippy passes
- no live Grok prompt or account mutation ran

## Closeout

Card 145 completed the independent structured projection, broader failure and
cross-host conformance, package assembly, public route and matrix truth, and
roadmap closeout.

## Next

See the Grok structured conformance closeout. Hold at the g02 stabilization
checkpoint.
