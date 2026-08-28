# 256 Goose ACP Mode Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.090 Residual Per-Route Feature Qualification](../090-residual-per-route-feature-qualification.md)
Depends on: g04.089; Research 250
Research: 253 reserved

## Goal

Freeze exact `goose.acp` `1.46.0` mode membership, request, application,
confirmation, failure, permission, lifecycle, and omission truth. Promote
Research 253 with a closed deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `goose.acp`, exact `1.46.0`, host-local provider/model
       configuration, read-only working resource, current permission exchange,
       and current lifecycle unchanged.
2. [ ] Freeze official docs and exact tagged source for `auto|approve|chat|smart_approve`,
       `session/new` mode advertisement, the supported ACP selection method,
       validation order, stored state, updates, and failure behavior.
3. [ ] Separate mode behavior from host isolation, resource access, tool
       membership, and permission reply authority. Exclude automatic approval
       or durable approval widening.
4. [ ] Test whether any exact provider mode has a safe adapter-local meaning.
       Do not map `chat`, `approve`, or any other Goose label to portable
       `HarnessMode::Plan` without exact semantic equivalence.
5. [ ] Build a closed value/lifecycle table. Require membership, pre-prompt
       selection, application, selected-value confirmation, fail-closed unknown
       handling, and unchanged omission.
6. [ ] Audit prepared input/evidence, session options, driver request order,
       decoder, fixtures, guide, matrices, and API baseline without production
       changes.
7. [ ] Promote Research 253 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact mode/resource/tool/permission/lifecycle table or honest empty set exists
- [ ] any non-empty row closes application and confirmation before first prompt
- [ ] unknown, missing, duplicate, malformed, or drifted modes fail closed
- [ ] omission retains exact current host-owned mode posture and `goose acp` argv
- [ ] no mode widens approval or persistence authority by default
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-goose
effigy qa:northstar
git diff --check
```

## Stop Conditions

- exact mode application or confirmation requires a provider prompt or login
- mode semantics cannot be separated from ambient host configuration
- only unsafe automatic or durable approval rows survive
- proof needs install/update, credential use, account inspection, host mutation,
  or shared-contract change

## Out Of Scope

Goose builtins, MCP management, extension installation, provider/model
selection, `goose serve`, production binding, live provider work, currentness,
release, shared closeout, rollover, or g04 closure.
