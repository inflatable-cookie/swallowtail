# 019 Watcher Proof Oracle And Activity Delivery Repair

Status: ready
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-30
Milestone: `../006-watcher-proof-repair.md`
Depends on: card 011 live evidence stop; Contracts 044, 059, and 060

## Goal

Repair the three reviewed defects in prototype head `49f2692f` and return a
credential-free PR whose future live selector can prove the intended Stop
counterexample rather than merely observe a clean final result.

## Scope

Start from current pushed `main`. Inspect `49f2692f` as salvage evidence, not
as a merge base. Preserve only contract-valid deterministic acceptance,
Claude watcher binding, opt-in selector, and safe closeout work.

Deliver one coherent runtime, local-host, Claude adapter, test, and planning
tranche:

1. Add the smallest provider-neutral watcher lifecycle delivery seam required
   by Contract 059. It must retain bounded ordered accepted, running, and
   terminal transitions independently of provider stdout and expose enough
   monotonic identity to deliver each transition once across multiple
   watchers. Latest-only polling tied to provider output is not acceptable.
2. Drive that seam concurrently with the Claude process pump from before
   provider spawn through terminal-barrier and cleanup settlement. Use
   `project_watcher_activity`; remove the adapter-local duplicate mapping.
   Advertise `HostWatcher` complete lifecycle only on watcher opt-in. Omission
   remains bit-for-bit equivalent and advertises no watcher activity.
3. Fail closed on feed, projection, event delivery, identity, ordering, or
   capacity errors. Stop and join watcher work, provider work, feed work, and
   bridge work before returning. Joined state must not emit a second completed
   activity.
4. Replace the live probe oracle with a test-local recording bridge or
   equivalent interception seam. It must observe safe ordered facts for exact
   MCP initialization and reserved tool discovery, watcher start invocation,
   a completion-gate response while work is active, the configured Stop hook
   lifecycle, post-block activity in the same Claude session, explicit wait or
   stop, zero active or unjoined watchers, joined cleanup, and provider
   terminal success. A model that proactively waits, directly calls the gate,
   or reaches only adapter terminal rejection must not satisfy the oracle.
5. Retain only enums, counts, bounded identities/revisions, and ordering facts
   needed by the assertion. Never retain raw HTTP bodies or headers, provider
   envelopes, prompt text, endpoint, bearer, path, command, environment, PID,
   watcher output, or credential material.
6. Establish temporary-workspace cleanup with a drop guard or equivalent
   finally-style owner before provider contact and before any assertion that
   can fail. Credential-free tests must prove cleanup on assertion, setup,
   provider, hook, cancellation, and deadline failure.
7. Reconcile the branch closeout honestly. Keep card 011 and g05.003 stopped,
   preserve the consumed-attempt log, and keep every capability/matrix/guide
   claim unpublished. This card may complete g05.006 as repair infrastructure;
   it cannot authorize or perform a provider retry.

Do not add containers, a generic event bus, generic MCP middleware, arbitrary
process authority, or a consumer route-feature facade.

## Review Oracle

- **Lifecycle invariant:** every accepted watcher publishes one started
  observation, every running transition publishes in-progress activity, and
  exactly one terminal observation precedes joined cleanup on the existing
  ordered turn stream.
- **Lifecycle counterexample:** a watcher moves accepted → running → terminal
  while Claude emits no stdout. A latest-snapshot or provider-output-triggered
  poll emits only terminal activity, duplicates completion at join, or emits
  no running state.
- **Required lifecycle proof:** deterministic silent-provider fixtures for one
  fast watcher and interleaved multiple watchers show ordered, non-regressing,
  exactly-once start/update/completion through `project_watcher_activity`, plus
  fail-closed feed overflow/closure, projection failure, event backpressure,
  cancellation, deadline, and cleanup.
- **Proof invariant:** future live acceptance must show the private Stop hook
  called the completion gate while one watcher was active, blocked completion,
  and returned control to the same provider session before the model waited or
  stopped and the turn completed successfully.
- **Proof counterexample:** the model waits proactively, calls the completion
  gate as an ordinary tool, or Claude finishes and the adapter rejects the
  already-terminal success. Registry presence plus final `WATCHER_LIVE_OK`
  would still pass the old selector without proving Stop re-entry.
- **Required proof design:** credential-free fake-provider and recording-bridge
  fixtures prove that only the ordered conjunction of tool discovery, start,
  active gate response, exact Stop hook lifecycle, same-session post-hook
  activity, explicit wait/stop, joined zero state, and provider success can
  pass. Missing, reordered, cross-session, proactive-wait, direct-gate, and
  terminal-only traces fail.

## Acceptance Criteria

- [ ] no adapter-local watcher lifecycle projector remains
- [ ] watcher activity has complete lifecycle fidelity only when opted in
- [ ] silent-provider and fast-watcher cases still expose running activity
- [ ] multiple watchers preserve host order without duplicates or regressions
- [ ] live-probe assertions require direct active Stop and same-session re-entry
- [ ] the recorder and default formatting retain no private or raw material
- [ ] live workspace cleanup is panic-safe before contact
- [ ] ordinary QA remains credential-free and the live selector remains opt-in
- [ ] no live selector is run and no watcher support claim is published

## Validation

- `cargo fmt -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-adapter-claude-agent -p swallowtail-testkit`
- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

Do not run `effigy probe:claude-code-watcher-live` or any direct Claude command
that supplies a prompt.

## Auto-Continuation

No. Return one reviewable PR and stop. After merge, the orchestrator reassesses
whether the repaired oracle is strong enough to ask the operator for fresh
provider authorization.
