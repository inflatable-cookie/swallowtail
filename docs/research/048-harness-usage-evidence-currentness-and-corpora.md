# 048 Harness Usage-Evidence Currentness And Corpora

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Can Claude Agent ACP, Pi RPC, and OpenCode emit one honest cumulative operation
usage observation under their existing qualified version claims?

## Method

Evidence was accessed 2026-07-28.

- checked every published Claude Agent ACP point in the qualified range
- checked Pi RPC exact qualified `0.80.10`
- checked `StepFinishPart` in all 45 qualified OpenCode OpenAPI documents
- froze synthetic safe records in each adapter crate
- separated cumulative records from disjoint components, context occupancy,
  and cost

No executable, credential, provider request, paid operation, account, or live
server was used.

## Claude Agent ACP

All nine published qualified points return prompt-response usage:

- `0.53.0`
- `0.54.0`
- `0.55.0`
- `0.56.0`
- `0.57.0`
- `0.58.1`
- `0.59.0`
- `0.60.0`
- `0.61.0`

The response is cumulative for the prompt and carries input, output,
cache-read, cache-write, and total tokens. `usage_update` remains context
occupancy plus optional cost. It is not the token breakdown.

The frozen corpus keeps both records so the implementation cannot confuse
them.

## Pi RPC

Exact `0.80.10` `message_end` records include complete assistant messages.
Each message carries disjoint input, output, cache-read, and cache-write usage.
A tool turn may contain several assistant messages.

The adapter must sum each message once and emit at `agent_settled`.
`get_session_stats` is not required. That command remains session-cumulative
and includes other work.

## OpenCode

All 45 qualified OpenCode releases from `1.14.48` through `1.18.4` retain one
identical required `StepFinishPart` schema:

- schema SHA-256:
  `f288773c3883f7acb8c53bd9f320a1bc965b3528bc17a059c09821fd8db552b9`
- input tokens
- output tokens
- reasoning tokens
- cache-read tokens
- cache-write tokens
- cost

The adapter must sum each disjoint step-finish part once and emit at session
idle.

Reasoning tokens cannot be discarded or folded into output by assumption.
`TokenUsage` therefore needs one optional `reasoning_tokens` field in card
082. The field remains an independently reported dimension; consumers cannot
infer whether a provider includes it in another total.

## Common Rule

Contract 011 now requires:

- `ProviderObservation::Usage` is cumulative for its operation at the emitted
  boundary
- provider-cumulative records replace earlier snapshots
- disjoint records may be summed once
- ambiguous composition, malformed values, or overflow fail closed
- context occupancy, billed cost, rate, quota, and token limits remain
  separate

The rule does not require every transport to emit at the same wire boundary.
Claude emits after its prompt response, Pi after settlement, and OpenCode at
idle. All three precede the terminal outcome.

## Corpora

- `swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-v0.53.0-v0.61.0/`
- `swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.80.10/usage-events.jsonl`
- `swallowtail-adapter-opencode/tests/fixtures/opencode-v1.14.48-v1.18.4/usage.sse`

Focused tests prove the qualified version set, disjoint component sums,
context separation, and terminal boundary records.

## Cost Disposition

All three upstream shapes expose some cost data, but card 082 does not promote
it:

- Claude context updates may carry currency-qualified cost
- Pi message cost is provider/model calculated
- OpenCode step cost has no currency in the selected schema

Those differences belong to the later billed-cost family. No cost cell changes
with usage evidence.

## Promotion

- Contract 011 owns cumulative replacement and disjoint aggregation.
- Architecture records the realized runtime rule.
- Card 082 can implement three exact adapter paths without new product policy.

