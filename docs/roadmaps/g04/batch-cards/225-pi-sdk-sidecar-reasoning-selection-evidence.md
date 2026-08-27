# 225 Pi SDK Sidecar Reasoning Selection Evidence

Status: ready
Owner: Tom
Created: 2026-08-27
Milestone: [g04.081 Pi SDK Sidecar Reasoning Selection](../081-pi-sdk-sidecar-reasoning-selection.md)
Depends on: g04.033; g04.080 closeout

## Goal

Freeze exact Pi SDK-sidecar thinking-level model, value, lifecycle, clamp,
persistence, and confirmation truth, then promote Research 228 with a
non-empty exact deliver-now table or an honest empty set.

## Work

1. [ ] Reuse and verify the exact route, driver, package, Node, wire,
   source-tag, access, resource, model, persistence, replay, and cleanup
   boundaries from Research 181 and g04.033.
2. [ ] Freeze exact `0.84.2` public types and tagged source for
   `ThinkingLevel`, `clampThinkingLevel`, model reasoning metadata,
   `createAgentSessionFromServices`, session persistence,
   `AgentSessionRuntime` replacement, and `session.thinkingLevel`.
3. [ ] Enumerate the exact Pi vocabulary and distinguish accepted input,
   clamped result, stored value, default, effective state, and emitted
   reasoning. Do not treat examples or a reasoning boolean as membership.
4. [ ] Build a closed provider/model/value table for only models already
   selectable through `pi.sdk-sidecar`. Prove how the adapter can reject every
   unsupported combination before process, environment, credential, resource,
   or provider work.
5. [ ] Classify new, load, resume, session replacement, and fresh
   context-losing restoration independently. Settle whether explicit
   `thinkingLevel` overrides stored state and is reapplied by the runtime
   factory on every attachment.
6. [ ] Freeze state confirmation and event ordering. Determine whether
   bootstrap/state `thinkingLevel` is enough to detect clamp, substitution,
   stale stored state, or model fallback before readiness and whether
   `thinking_level_changed` can appear during setup/rebind.
7. [ ] Audit the current private wire. It accepts `thinkingLevel` and fixtures
   already contain `medium`, while Rust omits and ignores it. Decide whether
   exact typed semantics require a wire/behavior/source-tag advance.
8. [ ] Audit preparation, `SessionOptions`, capability/profile construction,
   plan agreement, open/load/resume requests, resume binding, bootstrap/state
   validation, restoration, fixtures, guide, matrices, changelog, and API
   baseline.
9. [ ] Prove omission retains exact prior bytes and behavior. Empty options
   must not claim Pi's default or stored mode as caller-selected.
10. [ ] Keep mode changing, cycling, model switching, raw settings, `pi.rpc`,
    newer SDK currentness, and provider execution out.
11. [ ] Promote Research 228 with frozen sources, exact tables, and a non-empty
    deliver-now set or explicit empty set. Update milestone/card state and
    close out honestly.

## Acceptance Criteria

- [ ] exact vocabulary, model membership, clamp, persistence, replacement,
      and state-confirmation semantics are frozen
- [ ] every lifecycle has an explicit deliver-now or withheld disposition
- [ ] the pre-effect rejection boundary for unsupported rows is exact
- [ ] current Rust, sidecar, fixture, docs, and public seams are audited
- [ ] Research 228 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, currentness, release, merge, rollover,
      or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-pi
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 226 only when Research 228 admits a non-empty exact set
with static model/value membership, pre-effect rejection, and effective-state
confirmation for every claimed lifecycle.

## Stop Conditions

- exact membership depends on a mutable remote catalogue or account fact
- requested values can clamp or substitute without detectable mismatch
- load/resume semantics remain ambiguous or require mutation outside the
  caller-authorized attachment
- deterministic proof needs a provider prompt, credential, account inspection,
  package install, or ambient configuration mutation

## Out Of Scope

- production binding, dynamic mode change, model switch, raw Pi options,
  `pi.rpc`, currentness, live provider work, release, merge, rollover, or g04
  closure
