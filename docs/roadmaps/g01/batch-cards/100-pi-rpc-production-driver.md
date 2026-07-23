# 100 Pi RPC Production Driver

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../035-pi-rpc-harness-proof.md`

## Objective

Implement the separately registered Pi `0.80.10` RPC driver against the frozen
corpus and Contract 028 boundary.

## Readiness Gate

Card 099 is complete with stable shared records, exact runtime binding,
maintained compatibility-window classification, pure preflight, unchanged
profile posture, and a frozen corpus.

## Scope

- production driver inside the fixture-only `swallowtail-adapter-pi` scaffold
- exact executable, version, argv, environment, downstream provider, model,
  working directory, delegated-auth, and `AmbientHost` binding
- supervised piped process with strict-LF bounded JSONL
- prompt, steering, follow-up, abort, state, event, and UI request projection
- disabled session persistence, customization, update, telemetry, package,
  retry, write, edit, and bash behavior
- bounded queues, one active operation, two completed prompts
- cancellation, deadline, disconnect, safe error, redaction, and joined cleanup
- deterministic process fixture only; no installed or live provider call

## Acceptance Criteria

- [x] the descriptor is separate from Codex, ACP, and direct-inference drivers
- [x] exact downstream provider and model reach argv without fallback
- [x] no process or credential work occurs before successful preflight
- [x] command acknowledgement and model terminal events remain separate
- [x] native abort cannot overclaim provider stop
- [x] ambient read-intent behavior makes no containment claim
- [x] excluded sources and retry stay disabled
- [x] all process, reader, callback, resource, and auth work joins before close

## Validation

- 8 Pi adapter tests pass against the frozen corpus and supervised process
  fixture
- 47 runtime tests pass with the optional scheduling seam
- focused runtime and Pi warnings-denied clippy passes
- workspace all-target check, docs QA, format check, and diff check pass
- `effigy doctor` remains at the inherited 19 findings: 7 errors and 12
  warnings

## Evidence

The production driver owns a separate `swallowtail.pi.rpc` descriptor, exact
`0.80.10` compatibility point, supervised strict-LF process connection,
restrictive startup state validation, prompt and scheduled-message transport,
UI callback relay, native abort, deadline projection, safe failures, and
owner-ordered cleanup. Deterministic fixtures prove exact provider/model argv,
disabled customization and retry, acknowledgement-before-terminal behavior,
preflight-before-effects, startup drift cleanup, cancellation, timeout, and
credential-last release without installing Pi or contacting a provider.

Card 101 remains the cross-topology assertion-pack, callback-timeout, late-
response, disconnect, and full closeout gate.

## Open Risks

- callback deadlines are projected but expiry and late-response rejection need
  the card 101 deterministic timer matrix
- remote-authoritative host identity and shared RPC-profile conformance remain
  unproven against the production driver
- disconnect, provider failure, retry drift, malformed correlation, and cleanup
  failure need the card 101 cross-product before roadmap 035 closes

## Stop Conditions

- production behavior diverges from the frozen package/protocol version
- exact model or provider cannot be validated
- process startup needs package installation or credential mutation
- callback relay needs raw payload exposure
- cleanup requires detached work

## Auto-Continuation

No. Confirm production behavior before cross-topology conformance.
