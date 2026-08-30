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

## Acceptance Criteria

- [ ] no successful turn with active or unjoined watchers
- [ ] early completion returns bounded active-watcher state to the same live
      model turn before successful terminal admission
- [ ] consumer activity is ordered, bounded, and truthful
- [ ] raw logs, commands, paths, environment, and PIDs stay private
- [ ] failure classification and cleanup remain exact
- [ ] guide and matrix claims match the route fixture

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-claude-agent swallowtail-host-local swallowtail-testkit`
- `effigy qa:docs`
- `git diff --check`

## Auto-Continuation

No. Remains planned until card 010 lands and live provider work is explicitly
authorized.
