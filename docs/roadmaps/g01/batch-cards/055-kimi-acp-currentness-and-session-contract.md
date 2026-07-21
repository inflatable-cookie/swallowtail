# 055 Kimi ACP Currentness And Session Contract

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Revalidate Kimi Code ACP and promote the smallest shared contract needed before
second-agent fixtures or implementation.

## Governing References

- `../../../research/005-post-tranche-coverage-evidence.md`
- `../../../research/006-kimi-code-acp-currentness-and-persistent-session-evidence.md`
- `../../../contracts/005-integration-identity-and-transport-diversity.md`
- `../../../contracts/006-execution-layer-and-access-boundary.md`
- `../../../contracts/009-async-operation-lifecycle.md`
- `../../../contracts/012-interactive-session-options-and-callback-exchange.md`
- `../../../contracts/013-interactive-session-access-policy.md`
- `../../../contracts/015-acp-v1-negotiation-and-client-callbacks.md`
- `../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `../018-kimi-code-acp-portability-proof.md`

## Scope

- current official or maintained Kimi Code repository, release, ACP SDK/schema,
  and capability evidence
- exact ACP launch, access, delegated-login, support, and topology authority
- new/load/resume identity, replay ordering, write callbacks, cancellation,
  disconnect, and cleanup semantics
- durable contract promotion and fixture subset selection

## Out Of Scope

- protocol fixture corpus
- production driver or installed binary probe
- reading, exporting, or mutating Kimi login state
- consumer write policy, model selection, prompts, or session persistence

## Ordered Steps

1. Recheck current Kimi Code and ACP primary sources.
2. Separate wire, SDK, CLI, provider, access, and support identities.
3. Compare Kimi load/resume/write behavior with Contract 015 and Gemini proof.
4. Promote only missing provider-neutral lifecycle and authority rules.
5. Select an exact deterministic subset and version boundary for card 056.
6. Mark card 056 ready only if no authority decision remains.

## Acceptance Criteria

- [x] current versions and source authority are pinned independently
- [x] load, resume, replay, and session identity are not flattened
- [x] filesystem writes require exact host-authorized writable access
- [x] delegated login grants no credential extraction authority
- [x] cancellation, disconnect, and cleanup ownership are testable
- [x] no Kimi identity branch enters shared core/runtime contracts

## Validation

- official-source link and version check
- docs and Northstar QA
- `git diff --check`

## Evidence Required

- source URLs, release dates, versions, and authority classification
- promoted contract delta or an explicit proof that none is needed
- exact included and excluded fixture subset
- provider-auth, replay, write, versioning, and topology risks

## Stop Conditions

- maintained sources contradict on load/resume or write semantics
- delegated login would require ambient credential-store inspection
- choosing writable behavior would establish consumer authorization policy
- current evidence only supports repeating Gemini's existing subset

## Auto-Continuation

No. Card 056 becomes ready only after the contract and fixture boundary are
settled.

## Evidence

- Kimi Code `0.28.1`, tag commit
  `0032545b65f95c139ecba5a48ba1b911844e1ffe`, replaced the earlier `0.28.0`
  planning pin on 2026-07-20.
- the tagged adapter package is `@moonshot-ai/acp-adapter` `0.3.4` and its
  lock selects `@agentclientprotocol/sdk` `0.23.0`
- ACP wire version `1` remains separate from current stable schema artifact
  `schema-v1.19.1`, tag commit
  `0331ddaea7814afa147194cd5e93495a2a0f8c82`
- Research 006 records the current release, method matrix, tagged load/resume
  source, filesystem bridge, terminal auth, and process cleanup evidence
- Contract 017 binds provider sessions to exact route, host, resource, and
  policy identity; defines load replay separately from resume; and separates
  write callbacks, permission approval, and process containment

## Fixture Boundary

Card 056 includes exact initialization, one bound new session, load replay
before response, resume without replay, text prompt/update, native turn cancel,
bounded UTF-8 replacement callback, disconnect, and process-owned close.

It excludes session list and arbitrary stored ids, mode/model/configuration
mutation, MCP forwarding, rich prompt content, provider permission approval,
terminal calls, native session close, logout, terminal-auth execution, and
configured non-OAuth provider credentials.

The write transcript proves protocol and host-callback authority only. It does
not approve a Kimi tool or claim that ACP callbacks sandbox the Kimi process.

## Remaining Risks

- Kimi load/resume does not enforce the request `cwd`; only a prior exact
  Swallowtail binding is eligible.
- Kimi can bypass text callbacks through local filesystem and child-process
  operations. Research 012 later makes containment optional; card 058 now
  proceeds under explicit ambient execution.
- Kimi `0.28.1` accepts OAuth and configured non-OAuth routes through one auth
  gate. The first route uses only pre-existing isolated delegated auth.
- Kimi has no stable native session close; owned-process EOF/stop and joined
  cleanup remain mandatory.
