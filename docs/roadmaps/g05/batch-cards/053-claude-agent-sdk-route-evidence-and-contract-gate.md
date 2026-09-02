# 053 Claude Agent SDK Route Evidence And Contract Gate

Status: complete; evidence and contract gate returned; contract promotion pending orchestrator review with card 054
Owner: Tom
Created: 2026-09-02
Milestone: `../022-claude-agent-dual-route-parity.md`
Depends on: Research 277; official Anthropic Agent SDK documentation

## Goal

Produce the exact evidence and contract gate for a distinct
`claude-agent.sdk` route using the official TypeScript SDK through a bounded
Node sidecar and the user's own Claude subscription.

## Scope

1. Recheck the official subscription-use article immediately before evidence
   freeze. Record retrieval time, update text, eligible plan/user shape, usage
   source, and announced-change boundary. Stop if third-party subscription use
   is no longer explicitly supported.
2. Select one current official stable `@anthropic-ai/claude-agent-sdk` artifact
   without executing shipped code. Freeze npm/GitHub identity, license,
   supported Node range, package tree, executable/sidecar dependencies, public
   TypeScript declarations, and upstream schema or message types.
3. Inventory the exact APIs needed for persistent streaming sessions,
   read/write tools, `canUseTool`, permission changes, interrupt, model/effort/
   thinking controls, resume/fork, MCP, checkpoints, hooks, subagents, plugins,
   commands, account state, usage, and authentication readiness. Separate
   present, conditional, missing, and undocumented behavior.
4. Trace credentials from official login through SDK launch and requests.
   Prove Swallowtail can remain outside token storage, copying, logging,
   transport, pooling, and refresh. Define typed readiness/failure observations
   without exposing credential material.
5. Design the smallest bounded Rust-to-Node sidecar protocol and lifecycle:
   exact launch authority, cwd/environment, framing, backpressure, callbacks,
   cancellation, crash/disconnect, cleanup, session identity, restart, and
   packaging. Prefer existing runtime/host contracts; name every true gap.
6. Compare the proposed native route with existing `claude-agent.acp`, Claude
   Code headless, and response-only routes. Prove why it is additive and cannot
   reuse their route or behavior revision identities.
7. Write Research 278 plus a reviewable contract/spec gate. Do not implement
   production Rust, TypeScript, manifests, claims, routes, or package pins.

## Acceptance Criteria

- exact stable SDK identity and complete selected public API inventory
- explicit subscription-policy evidence and policy-currentness trigger
- credential non-custody proof with no secret-bearing fixture or log
- bounded sidecar lifecycle and failure model tied to existing contracts
- route identity and provider-specific/shared vocabulary partition
- a smallest implementation foundation with later parity layers clearly split
- every unsupported or undocumented target remains withheld

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:links`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the proposed native route can use one user's subscription without
Swallowtail possessing the credential and can stop every session through an
owned, joined lifecycle.

Smallest counterexample: a token appears in a sidecar request/fixture/log, a
detached Node process survives Rust session cleanup, a resumed session binds a
different cwd or user, or an SDK declaration is treated as runtime proof.

Required proof: official policy capture, artifact/tree/type inventories,
credential-flow diagram with negative searches, lifecycle state machine,
failure table, exact feature ledger, and adversarial counterexamples.

## Auto-Continuation

No. Contract promotion and implementation compilation follow joint review with
card 054.

## Stop Conditions

Stop on policy withdrawal, ambiguous artifact authority, license conflict,
token custody, missing bounded framing/cancellation, provider contact need, or
a shared public API choice not fixed by current contracts and Research 277.

