# 011 Watcher Acceptance And Consumer Projection

Status: planned; separately authorized live gate
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-30
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: completed card 010; explicit operator authorization for provider access, credentials, and any paid work

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

This card stays planned until the operator explicitly authorizes one bounded
live Claude Code turn using the existing local provider access and accepts any
normal paid-provider work. A bare continuation does not supply that authority.
After authorization, the worker must:

- re-probe the installed path, exact `2.1.251` identity, and frozen binary
  digest before any prompt; stop on drift
- require one dedicated live-probe gate and use existing local Claude state
  without reading, copying, logging, or changing credentials
- run at most one provider turn; a second provider attempt after success or
  failure requires fresh operator authorization
- use one host-approved, local, bounded watcher operation with no external
  network need and no public command, argument, path, environment, PID, or raw
  output exposure
- avoid provider install, update, login, ambient settings mutation, and shared
  project material
- add one explicit Effigy live-probe selector so ordinary QA remains
  credential-free and cannot contact Claude

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

No. Return one reviewable PR after an authorized live proof, or one sanitized
stop report after the single live attempt fails. The worker never retries live
provider work or starts another card automatically.
