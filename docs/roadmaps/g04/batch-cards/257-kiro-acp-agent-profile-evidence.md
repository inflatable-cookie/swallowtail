# 257 Kiro ACP Agent-Profile Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.090 Residual Per-Route Feature Qualification](../090-residual-per-route-feature-qualification.md)
Depends on: g04.089; Research 251
Research: 254 reserved

## Goal

Freeze exact `kiro.acp` `2.18.1` `--agent` profile membership, authority,
request, application, confirmation, failure, lifecycle, and omission truth.
Promote Research 254 with a closed deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `kiro.acp`, exact `2.18.1`, host-local account access,
       read-only working resource, current permission exchange, and current
       lifecycle unchanged.
2. [ ] Freeze the official ACP `--agent` documentation and recover exact
       qualified package/source parser evidence when available. Do not promote
       current stable or chat/TUI behavior onto the qualified ACP route.
3. [ ] Identify the exact profile namespace, membership source, listing or
       validation surface, built-in versus user-defined authority, precedence,
       invalid-name failure, and any ambient persistence.
4. [ ] Trace selection before `initialize` and `session/new`; identify a bounded
       applied-profile confirmation surface. Do not infer the selected profile
       from generic `agentInfo` identity or display prose.
5. [ ] Build a closed profile/lifecycle table. Require static or observable
       membership, pre-prompt rejection, application, confirmation, and
       unchanged omission.
6. [ ] Audit prepared input/evidence, command builder, decoder, fixtures,
       guide, matrices, and API baseline without production changes.
7. [ ] Promote Research 254 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact profile/authority/lifecycle table or honest empty set exists
- [ ] any non-empty row closes membership and applied-profile confirmation
- [ ] unknown, missing, malformed, or drifted profiles fail before prompt effects
- [ ] omission retains exact current `kiro-cli acp` argv and host-owned default
- [ ] no trust-all, cloud, login, profile mutation, or ambient persistence is added
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-kiro
effigy qa:northstar
git diff --check
```

## Stop Conditions

- exact `2.18.1` package/source evidence remains unrecoverable and docs cannot
  close failure or application truth
- membership or confirmation requires login, account inspection, or a provider prompt
- profile selection depends on unbounded user-owned ambient state
- proof needs install/update, credential use, host mutation, or shared-contract change

## Out Of Scope

Kiro chat/headless, effort, cloud sessions, model selection, trust-all tools,
production binding, live provider work, currentness, release, shared closeout,
rollover, or g04 closure.
