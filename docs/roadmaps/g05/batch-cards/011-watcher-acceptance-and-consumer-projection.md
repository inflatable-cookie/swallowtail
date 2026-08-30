# 011 Watcher Acceptance And Consumer Projection

Status: complete; live evidence stop after one authorized Haiku turn
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-30
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed card 010; operator authorization recorded 2026-08-30 for one existing-access Claude turn using the cheap model

## Goal

Close the first Contract 059 route with deterministic lifecycle, failure,
consumer activity, and integration guidance.

## Scope

Start by re-probing the exact installed and official Claude identity. Run one
bounded live turn that attempts early completion with an active watcher and
must return active-watcher state to the same model turn before waiting or
stopping. Then prove multiple bounded watchers, explicit wait, both stop paths,
completion races, hook rejection, cancellation, deadline, provider failure,
summary redaction, joined cleanup, and unchanged omission. Update shared route
and feature documentation only after the exact live proof and deterministic
acceptance pass.

The first watcher claim is exact to the live-proved Claude point unless the
evidence independently supports a wider watcher segment. Base-route
qualification through `2.1.251` does not make watcher support available on the
whole route range. Provider-neutral Contract 060 fixtures do not substitute
for the route proof.

This card may publish watcher activity and existing matrix/guide truth. It does
not promote or implement the open cohesive consumer route-feature projection
facade.

## Readiness Reassessment

Card 010 landed the exact `2.1.251` credential-free binding and deterministic
provider fixtures. The provider-neutral registry, host supervision, HTTP/MCP
bridge, private material, Stop continuation, terminal barrier, and cleanup
prerequisites are closed. The remaining gate is external authority, not another
technical prerequisite.

On 2026-08-30 the operator explicitly authorized one bounded live Claude Code
turn using existing local provider access and requested a cheap model. This
card is ready for that single attempt. The worker must:

- re-probe the installed path, exact `2.1.251` identity, and frozen binary
  digest before any prompt; stop on drift
- require one dedicated live-probe gate and use existing local Claude state
  without reading, copying, logging, or changing credentials
- select exact `claude-haiku-4-5`; do not use a moving alias and do not fall
  back to Sonnet, Opus, or another model when the selection is unavailable
- run at most one provider turn; a second provider attempt after success or
  failure requires fresh operator authorization
- bind a `90`-second operation deadline; do not advertise that probe bound as
  a watcher capability
- use one host-approved, local, bounded watcher operation with no external
  network need and no public command, argument, path, environment, PID, or raw
  output exposure
- avoid provider install, update, login, ambient settings mutation, and shared
  project material
- add one explicit Effigy live-probe selector so ordinary QA remains
  credential-free and cannot contact Claude

Research 241 rejects `--max-budget-usd` for the selected local-subscription
route because its API-catalog estimate is not subscription allowance. Do not
reopen or bind that feature here. The separately qualified maximum-turns
feature also excludes unprobed `2.1.251`. Exact cheap-model selection, one
provider session, and the 90-second deadline are this probe's cost bounds.

## Review Oracle

- **Invariant:** one opted-in Claude turn cannot report successful terminal
  completion while any turn-owned watcher is active or unjoined. If the model
  tries to finish early, the exact Stop hook blocks completion, returns bounded
  active-watcher state to that same provider conversation, and success occurs
  only after the model waits or stops and the registry reports joined state.
- **Smallest counterexample:** the model starts one approved watcher and emits
  final text before it is joined; Claude exits successfully, or Swallowtail can
  only replace that already-terminal success with a local failure, without one
  same-conversation Stop continuation.
- **Expected failure/stop:** withhold every watcher capability, matrix, and
  guide claim; record the exact sanitized evidence; make no second live attempt
  without fresh authorization. Do not weaken the invariant into automatic
  waiting or terminal-only rejection.
- **Required proof:** sanitized event ordering for one exact `2.1.251` live
  turn shows watcher start, pre-terminal Stop block, same-conversation
  continuation, explicit wait or stop, zero active or unjoined watchers, clean
  provider completion, and joined bridge/process cleanup. Deterministic
  fixtures separately cover multiple watchers, operator and model stop,
  completion races, cancellation, deadline, provider/hook failure, redaction,
  omission, and every cleanup path.

## Acceptance Criteria

- [ ] no successful turn with active or unjoined watchers
- [ ] early completion returns bounded active-watcher state to the same live
      model turn before successful terminal admission
- [ ] consumer activity is ordered, bounded, and truthful
- [ ] raw logs, commands, paths, environment, and PIDs stay private
- [ ] failure classification and cleanup remain exact
- [ ] guide and matrix claims match the route fixture

## Stop Conditions

- the installed Claude identity is not exact `2.1.251` or its frozen binary
  digest no longer matches
- authentication requires login, credential inspection, provider setup, or
  ambient configuration mutation
- exact `claude-haiku-4-5` is unavailable or the route would need a pricier
  fallback
- the one authorized live turn does not prove the review-oracle ordering
- same-turn proof requires a second provider attempt without fresh authority
- the provider can only fail after irreversible successful completion
- proving the claim requires raw provider payload, process content, secret
  material, or a wider watcher version segment
- deterministic acceptance exposes a Contract 059 or 060 gap rather than an
  implementation miss

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-claude-agent swallowtail-host-local swallowtail-testkit`
- one explicit opt-in `effigy probe:claude-code-watcher-live` selector, added
  by this card and run only inside the authorized provider turn
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. The authorized live turn ran once. Host registry never observed a
turn-owned watcher, so the review oracle is unproved. Fresh operator
authorization is required before another provider session.
