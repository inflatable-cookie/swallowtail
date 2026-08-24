# 163 Antigravity Headless Agent Profile Acceptance

Status: blocked
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.058 Antigravity Headless Agent Profile Selection](../058-antigravity-headless-agent-profile-selection.md)
Depends on: card 162
Blocked by: Research 205 empty deliver-now set; card 162 blocked

## Goal

Close the route-local agent-profile family with deterministic exact-version,
operation, dispatch, confirmation, failure, and documentation proof.

## Work

1. Add or extend frozen fixtures for every delivered exact version/operation
   row, including selected and omitted init envelopes.
2. Prove prepared profile id, plan, request, command argv, parser expectation,
   and `init.agent` confirmation agree exactly.
3. Prove invalid/unavailable id, missing/foreign confirmation, duplicate argv,
   malformed init, provider failure, cancellation, timeout, incomplete stream,
   and cleanup paths without raw provider leakage or fallback.
4. Prove composition with every Research 205-admitted model/effort/schema,
   access, isolation, structured-run, and continuation shape.
5. Prove omission retains the existing command and decoder corpus.
6. Update the Antigravity guide, feature matrix notes/cells only when warranted,
   package API baseline, Research 205, milestone/cards, and route-local closeout.
7. Run the named package, route, docs, API, example, doctor, and diff gates.

## Acceptance Criteria

- [ ] every delivered row has deterministic exact-version fixtures
- [ ] selected and omitted argv are exact and non-duplicated
- [ ] selected output cannot be accepted before exact `init.agent`
      confirmation
- [ ] invalid, missing, mismatched, malformed, and drift cases fail closed
- [ ] continuation reassertion/replacement truth is proved only where admitted
- [ ] model, effort, schema, access, isolation, permission, deadline,
      cancellation, provider-state, and cleanup behavior remain exact
- [ ] stable diagnostics disclose no prompt, output, raw payload, profile body,
      tools, account identity, credential, endpoint, or host path
- [ ] guide and feature matrix describe dispatch/effect truth without claiming
      agent content, capabilities, safety, or quality
- [ ] no other route, currentness claim, contract, release, or generation state
      changes
- [ ] `cargo fmt -p swallowtail-adapter-antigravity` passes
- [ ] `effigy validate:focused swallowtail-adapter-antigravity` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-antigravity` passes
- [ ] `effigy check:examples`, `effigy qa:routes`, `effigy qa:northstar`,
      relevant index gates, `effigy package:api`, and `git diff --check` pass
- [ ] `effigy doctor` does not worsen the inherited baseline

## Stop Conditions

- any admitted row lacks deterministic dispatch and confirmation proof
- selected profiles can exceed the prepared route's authority or silently
  fall back
- validation reveals a contract/currentness dependency or a breaking API

## Out Of Scope

- another feature family, provider prompt, release, publication, merge,
  generation rollover, or g04 closure
