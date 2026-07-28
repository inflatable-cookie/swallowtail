# 090 Input And Callback Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../027-input-and-callback-feature-closure.md`
Depends on: card 089

## Objective

Implement the contract-ready input/callback tranche through existing prepared
route identities.

## Scope

1. Implement exactly six audited cells:
   - `pi.rpc`: attachments
   - `opencode.http`: attachments and approval-or-question exchange
   - `anthropic.messages`: attachments, consumer-tool exchange, and external
     search
2. Keep each input or callback on its exact operation shape.
3. Add the adjacent Anthropic Messages interactive role required by Contract
   030 client-tool continuation. Do not widen its existing one-attempt
   structured role.
4. Bind request, plan, attachment lease, dispatch, callback admission,
   response, continuation attempt, search evidence, and cleanup.
5. Cover Pi structured and interactive prompts, and OpenCode structured and
   interactive turns, wherever the frozen route uses the same qualified wire
   feature.
6. Reject undeclared, mismatched, late, duplicate, or unsupported exchanges
   before they can become authority.
7. Preserve cancellation, deadlines, topology, version posture, and redacted
   diagnostics.
8. Update matrix cells only after public prepared paths and conformance exist.

## Acceptance Criteria

- [x] every changed matrix cell has a realized prepared operation
- [x] input and callback authority agree from request through completion
- [x] malformed and unsupported inputs fail deterministically
- [x] callbacks remain consumer-executed and exactly correlated
- [x] topology, cleanup, and version posture remain unchanged
- [x] package examples compile without live access

## Progress

- [x] `pi.rpc` attachments: prepared structured runs and opt-in interactive
  sessions bind one PNG up to one MiB, require attachment plus blocking-work
  services, encode exact inline RPC image input, and share one cleanup lease
  across terminal, close, rejection, and session cleanup.
- [x] Pi prepared coverage proves structured and interactive dispatch,
  redacted path handling, exact base64 wire content, and one release.
- [x] `opencode.http` attachments and approval-or-question exchange:
  prepared structured runs and opt-in interactive sessions bind one PNG up to
  one MiB plus exact `opencode/permission` and `opencode/question` exchanges.
- [x] OpenCode permission replies expose only `once` or `reject`; ordered
  question answers, run/turn correlation, duplicate and late rejection,
  cancellation abandonment, and one attachment release are fixture-proven.
- [x] `anthropic.messages` attachments, consumer-tool continuation, and
  provider-owned external search.
- [x] Anthropic structured inference binds one operation-scoped PNG or exact
  `web_search_20250305`; search retains provider execution, two-use bounds,
  allowed domains, explicit provider network policy, and progress evidence.
- [x] Anthropic consumer tools use a separate resource-free direct session:
  one provider call pauses for one exact consumer result, that submission
  authorizes one further request, and one later user turn remains bounded.

## Evidence

- `cargo test -p swallowtail-adapter-pi --no-fail-fast`: 36 passed.
- `cargo test -p swallowtail-adapter-opencode --no-fail-fast`: 74 passed,
  one installed-server probe ignored.
- `cargo test -p swallowtail-adapter-anthropic --no-fail-fast`: 50 passed.
- `effigy lint:rust`: passed across the workspace.
- `effigy check:examples`: passed.
- `effigy qa:routes`: passed with 437 remaining `No` cells and 68 remaining
  input/callback `No` cells.
- No installed Pi, live credential, or provider request was used.
- No installed OpenCode server, live credential, or provider request was used.
- No live Anthropic credential or provider request was used.

## Auto-Continuation

Satisfied. Continue to card 091.
