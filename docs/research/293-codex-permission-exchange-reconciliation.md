# Research 293 — Codex Permission Exchange Reconciliation

Status: complete; provider limitation; no live/provider claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 / 138

## Question

On the qualified Codex app-server range, can a consumer answer an approval
request and have that answer reach Codex, or can it only observe the request?
Does the combined `codex.app-server; codex.exec` row hide a route difference?

## Decision

No answerable Codex approval exchange is shipped. `codex.app-server` exposes
approval-shaped requests through the bounded namespaced extension, but the
adapter observes them, rejects the provider request, interrupts the turn, and
finishes with `ProviderRequestObserved`. It never queues an approval response.

The same app-server route does support answerable typed user-input exchange
when explicitly selected. The consumer response is converted to Codex's
`answers` result and sent with the original provider request id. That is
`question_exchange`, not `permission_exchange`.

`codex.exec` is a separate one-shot `StructuredRunDriver`. It has no
`InteractiveSessionDriver`, provider callback exchange, or approval response
path. The combined permission cell therefore remains `No` /
`provider_limitation`; its reason must name app-server observation-only
approval handling and exec's non-applicable callback surface.

## Frozen Corpus

- The frozen app-server activity corpus contains approval server requests for
  command execution, file changes, and permissions at
  `crates/swallowtail-adapter-codex/tests/fixtures/activity/app-server.jsonl#L23-L24`.
  It contains a separate `item/tool/requestUserInput` request-id case at
  `crates/swallowtail-adapter-codex/tests/fixtures/activity/app-server.jsonl#L29`.
- The frozen route ledger keeps the surfaces separate:
  `docs/research/281-v0-4-0-compatibility-and-freeze-audit/route-behavior-ledger.tsv#L15`
  describes app-server interactive sessions, while
  `docs/research/281-v0-4-0-compatibility-and-freeze-audit/route-behavior-ledger.tsv#L16`
  describes exec as a structured run with no persistent session.
- The checked-in Codex protocol identity records the selected app-server and
  exec source/schema digests as frozen and never executed:
  `crates/swallowtail-adapter-codex/tests/fixtures/codex-cli-0.152.1/protocol.json#L39-L74`.

## Shipped Adapter Path

- `crates/swallowtail-adapter-codex/src/session_access.rs#L13-L45` names the
  approval and user-input namespaces and maps the three approval methods plus
  `item/tool/requestUserInput`.
- `crates/swallowtail-adapter-codex/src/prepared_profile/session_capabilities.rs#L66-L84`
  gives read-only sessions
  only user-input exchange; bounded-workspace sessions observe approvals and
  exchange user input.
- `crates/swallowtail-adapter-codex/src/turn_state/provider_requests.rs#L148-L186`
  permits exchange only for the
  user-input namespace. The approval path falls through to
  `crates/swallowtail-adapter-codex/src/turn_state/provider_requests.rs#L188-L222`,
  which constructs an extension observation and closes it.
- `crates/swallowtail-adapter-codex/src/callback_exchange.rs#L131-L145` makes
  an observation terminal. Its `CallbackResponder::respond` implementation at
  `crates/swallowtail-adapter-codex/src/callback_exchange.rs#L193-L218` can only
  claim pending dynamic-tool or user-input callbacks;
  `crates/swallowtail-adapter-codex/src/callback_exchange.rs#L249-L277` has no
  approval result variant.
- The RPC path at
  `crates/swallowtail-adapter-codex/src/rpc/pump.rs#L193-L220` sends the observed request an error,
  issues `turn/interrupt`, and records `ProviderRequestObserved`. By contrast,
  `crates/swallowtail-adapter-codex/src/callback_exchange.rs#L203-L207` calls
  `respond_server_request`, whose wire form is
  `crates/swallowtail-adapter-codex/src/rpc.rs#L155-L162`.
- Typed user-input conversion is explicit at
  `crates/swallowtail-adapter-codex/src/user_input.rs#L82-L100`, and
  `CodexSessionProfileInput::with_user_input_exchange()` is the opt-in at
  `crates/swallowtail-adapter-codex/src/prepared_profile/input.rs#L298-L325`.

## Provider-Free Fixtures

- `crates/swallowtail-adapter-codex/tests/app_server_workspace.rs#L238-L318`
  proves both approval and user-input observations terminate the turn and send
  no approval answer.
- `crates/swallowtail-adapter-codex/tests/prepared_profile_cases/session/user_input.rs#L1-L33`
  proves the profile opt-in rejects approval exchange, while
  `crates/swallowtail-adapter-codex/tests/prepared_profile_cases/session/user_input.rs#L85-L125`
  proves a typed answer reaches the fake provider as `result.answers`.
- `crates/swallowtail-adapter-codex/tests/support/app_server/respond.rs#L245-L310`
  emits the approval and
  user-input requests; only the user-input response completes the fake turn at
  `crates/swallowtail-adapter-codex/tests/support/app_server/respond.rs#L14-L18`.
- The exec driver is one-shot structured-run only at
  `crates/swallowtail-adapter-codex/src/exec.rs#L22-L86`, and writes one prompt
  before closing stdin at
  `crates/swallowtail-adapter-codex/src/exec.rs#L219-L228`;
  `crates/swallowtail-adapter-codex/tests/fixtures/compatibility/exec-core.jsonl`
  contains no callback request path.

## Bounded Chatterbox/Desktop Correction Capsule

Correction to the 2026-09-07 Codex reconciliation: the producer does not
provide answerable Codex approval exchange. `codex_approval_request_extension()`
is an observable, namespaced approval request; the adapter rejects the provider
request, interrupts the turn, and reports `ProviderRequestObserved`. A consumer
cannot use `CallbackResponder::respond` to send an approval decision to Codex.

`CodexSessionProfileInput::with_user_input_exchange()` is narrower and
answerable for `item/tool/requestUserInput` only. Its typed response does reach
Codex. Therefore Desktop's `ObservedOnly` posture matches the producer ceiling
for Codex approvals, but remains a consumer-side ceiling if Desktop chooses not
to use the separately qualified question exchange. The combined
`codex.app-server; codex.exec` `permission_exchange` cell remains `No` /
`provider_limitation`; `question_exchange=Yes` is the distinct answerable row.

## Non-Claims

No live Codex, credentials, config mutation, provider install, Desktop action,
runtime change, release, tag, or approval-exchange redesign was performed.
