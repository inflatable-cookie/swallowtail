# 150 Claude SDK MCP Courier Startup Determinism

Status: complete; Desktop PR 178 merged at `dfc9c6a6`
Owner: Desktop live-evidence harness owner
Created: 2026-09-08
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: Card 149 return; Research 299; Card 139 artifact-race evidence

## Goal

Explain and remove the `mcp_server_failed` variance that blocked Card 149,
without contacting Claude or weakening the registered-MCP release gate.

## Scope

1. Reconcile Card 312's successful registered-MCP open with Card 314's failed
   open at the same route tuple. Trace courier build, artifact acquisition,
   executable approval, fixed args/env, one-shot rendezvous, authenticated
   bridge connect, SDK spawn, and MCP-status projection.
2. Reproduce or falsify the leading mutable-artifact race provider-free by
   repeatedly opening the fake-SDK registered route while concurrent builds
   churn the shared `target/debug` courier path.
3. If Desktop's runner owns the defect, publish a per-run immutable courier
   artifact after the build and pass that exact path into the runner. Never
   spawn a Cargo-owned mutable target path. Preserve executable identity and
   clean it after the bounded run.
4. Make any provider-free startup failure name the bounded stage and cause
   without persisting paths, environment values, stderr, provider content, or
   secrets in a live capsule.
5. Run at least 24 churned opens plus the focused Card 132/314 fixture suite.
   Preserve every historical capsule byte-identically.

Owned mutable paths: Desktop's Card 132/314 runner and wrapper, focused tests,
one factual log, the Desktop successor card/result, and additive handoff docs.
Queue closeout owns shared Desktop indexes.

Forbidden: Claude/native provider execution; prompt or turn; live MCP; retry;
account mutation or top-up; changing prior capsules; removing MCP from the
qualification route; Swallowtail runtime changes without returning a precise
owner handoff; qualification, matrix, candidate, tag, release, or dependency
pin changes.

## Acceptance Criteria

- [ ] Card 312 success and Card 314 failure are reconciled without inventing a billing or provider cause
- [ ] the mutable-target race is reproduced or falsified with retained provider-free evidence
- [ ] no provider-spawned courier path can be removed or replaced by a later Cargo build
- [ ] 24+ churned fake-SDK registered opens pass with exact authenticated-connect and cleanup assertions
- [ ] startup failures carry a bounded actionable cause outside immutable live capsules
- [ ] all historical capsules remain byte-identical and no provider process runs

## Validation

- Desktop focused Card 132/314 checks
- 24+ provider-free registered opens under concurrent courier build churn
- Swallowtail `effigy validate:card116-mediated-stdio`
- Desktop docs and Northstar QA
- `git diff --check`
- exact-head independent review

## Review Oracle

Smallest counterexample: the runner still hands the provider a Cargo-owned
mutable target path, or a startup failure still collapses to
`mcp_server_failed` when a safe local cause was already observed.

## Stop Conditions

- evidence identifies a Swallowtail production carrier/host/sidecar defect;
- immutable artifact publication cannot preserve exact executable identity; or
- any proposed fix weakens MCP admission, cleanup, redaction, or the live gate.

## Auto-Continuation

No. A later live attempt requires its own explicit operator authority after
this provider-free repair is accepted.

## Result

Desktop Card 315 reproduced the mutable Cargo target-path race provider-free:
24 churned opens produced zero passes and included Card 149's exact
`open_rejected` / `sidecar_rejected` / `mcp_server_failed` shape. Desktop now
publishes the completed courier to a read-only, content-addressed per-run path,
re-verifies its digest, and passes that exact path to the runner. The repaired
path passed 24/24 churned registered opens, 28/28 focused fixtures, and 15/15
Swallowtail mediated-stdio checks at exact source pin `0d120067`.

PR 178 merged at `dfc9c6a6e6f29c56751278d39fd025febdd8d917`
after accepted independent review comment `5593574653`; Desktop canonical
closeout is `d2b23bbca0c0c602a188478a722a3763cd0df656`. Reproduction evidence SHA-256:
`0e006e3a195443e52373810e873dbe3e13771e6cabbbcb2634694d7de3d442b2`.
Passing proof SHA-256:
`8191d143eb2620844f10c5c81651b0238a9dbf600b7f8f4099f4298359b366ec`.
No provider ran and both Contract 061 cells remain unqualified. Card 151 owns
the separately authorized zero-credit first-turn rerun.
