# 075 Pi RPC And OpenCode Structured Runs

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../023-installed-and-attached-harness-structured-coverage.md`

## Objective

Add bounded structured execution to the exact Pi RPC and OpenCode attached HTTP
routes.

## Scope

1. Pi: start one RPC process, submit one prompt, relay qualified UI callbacks,
   await `agent_end`, then close and join.
2. OpenCode: create one session, submit one prompt, consume exact SSE terminal
   evidence, close, then delete the created session.
3. Add independent prepared structured operations and requirements.
4. Preserve ambient isolation, provider configuration, model identity,
   retention, and cancellation truth.
5. Run local and remote-authoritative conformance plus existing catalogue and
   interactive regression.

## Acceptance Criteria

- [x] one prompt per run
- [x] Pi retains no resume authority and reports exact transcript retention
- [x] OpenCode deletes only its operation-created session
- [x] callback, deadline, disconnect, and cleanup failures remain distinct
- [x] no attached OpenCode server lifecycle authority is claimed
- [x] model catalogue behavior remains separate

## Evidence

- Pi registers an independent structured role and typed prepared operation. One
  run starts one exact `--no-session` RPC child, submits one prompt, relays the
  qualified bounded UI callback exchange, awaits `agent_end`, then joins the
  turn, process, working-resource, and credential work.
- Pi requires `ProviderRetentionPolicy::Prohibited`, exposes no provider run,
  reusable session, resume binding, or management binding, and preserves the
  configured restrictive RPC policy without misclassifying interactive
  scheduling as a structured-run requirement.
- OpenCode registers an independent structured role and typed prepared
  operation against the unchanged attached-server range. One run creates one
  private provider session, subscribes to exact SSE terminal evidence, prompts
  once, closes the turn, deletes that session, then releases access.
- OpenCode requires temporary retention and reports confirmed or unconfirmed
  operation-owned session deletion on the terminal outcome. Delete rejection,
  malformed confirmation, disconnect, provider status, and cleanup stay
  separate.
- The OpenCode run returns no provider run or session-management binding and
  never starts, stops, or otherwise claims lifecycle authority over the
  external server.
- The provider-neutral structured-harness boundary pack now covers prohibited,
  temporary-with-deletion, and durable-without-deletion retention truth.
- The solution matrix changes only Pi RPC and OpenCode HTTP structured
  execution from `No` to `Yes`.

## Validation Evidence

- `cargo test -p swallowtail-testkit -p swallowtail-adapter-pi -p swallowtail-adapter-opencode`
- `cargo clippy -p swallowtail-testkit -p swallowtail-adapter-pi -p swallowtail-adapter-opencode --all-targets -- -D warnings`
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

## Validation

- Pi and OpenCode adapter suites
- focused projection conformance
- strict Clippy, docs, routes, and `git diff --check`

## Stop Conditions

- Pi events cannot be correlated to the single prompt
- OpenCode deletion can occur before terminal or close
- either route requires implicit approval or fallback

## Auto-Continuation

Yes. Continue to card 076.
