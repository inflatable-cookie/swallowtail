# 074 ACP Structured Projection And Claude Agent

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../023-installed-and-attached-harness-structured-coverage.md`

## Objective

Add the shared single-turn projection assertion pack and prove it through the
qualified Claude Agent ACP route.

## Scope

1. Add reusable testkit assertions without a generic production adapter.
2. Register a separate Claude structured role and prepared operation.
3. Create one ACP session, execute one prompt, relay qualified callbacks, and
   close natively where supported.
4. Preserve exact version segments, deletion truth, ambient isolation,
   configuration, retention, and remote-ACP portability.
5. Prove cancellation, deadline, callback abandonment, disconnect, and joined
   cleanup.

## Acceptance Criteria

- [x] one terminal ACP prompt becomes one run outcome
- [x] no reusable session or management binding escapes
- [x] native close does not imply history deletion
- [x] callbacks remain exact and consumer-owned
- [x] stdio and explicit remote ACP retain separate transport composition
- [x] the full Claude interactive and lifecycle range still passes

## Evidence

- Testkit exposes a provider-neutral ACP single-turn projection assertion pack
  over the existing long-lived ACP profile. It proves explicit durable
  retention, structured cancellation, exactly-once run callback correlation,
  and no deletion claim.
- Claude Agent registers an independent structured role and prepared
  operation across the unchanged maintained range and visible
  unverified-newer posture.
- One run creates one operation-private ACP session, sends one prompt, awaits
  one terminal result, closes natively only at qualified versions, then joins
  turn, deadline, process, resource, and credential work.
- The run returns no provider run, reusable session, resume, or management
  binding. Its policy explicitly accepts durable transcript retention.
- Claude's selected subset still exposes no consumer callback exchange.
  Filesystem reads stay host-service callbacks; permission requests are
  rejected and surfaced as provider-request observations without auto-approval.
- The existing explicit remote-ACP portability tests pass without adding a
  stdio fallback or claiming a production remote Claude driver.

## Validation Evidence

- `cargo test -p swallowtail-testkit -p swallowtail-adapter-claude-agent`
- `cargo clippy -p swallowtail-testkit -p swallowtail-adapter-claude-agent --all-targets -- -D warnings`
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

## Validation

- testkit projection assertions
- Claude Agent full adapter suite
- remote ACP focused regression
- strict Clippy and `git diff --check`

## Stop Conditions

- ACP prompt completion cannot map without losing stop or callback truth
- provider data retention cannot be expressed by the operation policy
- remote transport would require fallback

## Auto-Continuation

Yes. Continue to card 075.
