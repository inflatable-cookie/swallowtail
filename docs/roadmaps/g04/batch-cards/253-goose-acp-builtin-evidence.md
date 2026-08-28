# 253 Goose ACP Builtin Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.089 Sixth Parallel Per-Route Feature Qualification](../089-sixth-parallel-per-route-feature-qualification.md)
Depends on: g03.088-g03.092; g04.088 closeout
Research: [250 Goose ACP Builtin Evidence](../../../research/250-goose-acp-builtin-evidence.md)

## Goal

Freeze exact Goose ACP builtin name, host-extension authority, spawn,
application, advertisement, failure, lifecycle, and omission truth. Promote
Research 250 with a closed deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `goose.acp`, exact `1.46.0`, host-approved executable and
       environment, provider-owned local config, current ACP lifecycle, and
       current permission posture unchanged.
2. [ ] Freeze official `--with-builtin` syntax plus exact tagged parser/source,
       builtin registry, name membership, repeat/composition rules,
       initialization, session exposure, and failures.
3. [ ] Separate builtins from MCP/extensions, provider/model configuration,
       tool permissions, host capabilities, and prompt text.
4. [ ] Build a closed builtin/host-config/lifecycle table. A non-empty row must
       prove the host already authorizes every dependency and the session
       advertises or otherwise confirms the selected builtin.
5. [ ] Prove unknown and unavailable builtins reject before prompt effects;
       prove omission retains exact `goose acp` argv and behavior.
6. [ ] Audit prepared sessions, spawn plan/evidence, ACP fixtures, guide,
       matrices, and API baseline without production changes.
7. [ ] Promote Research 250 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact builtin/authority/lifecycle table or honest empty set exists
- [ ] a non-empty row closes membership, dependency authority, application,
      confirmation, cleanup, and omission
- [ ] unknown or unavailable builtins reject before prompt effects
- [ ] no provider setup, extension install, permission widening, or ambient mutation
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-goose
effigy qa:northstar
git diff --check
```

## Stop Conditions

- membership or dependency authority is host-local, mutable, or live-only
- application cannot be confirmed before provider effects
- proof needs login, credentials, provider prompts, paid work, install/update,
  host configuration, or shared-contract change

## Out Of Scope

Goose mode, provider/model setup, MCP management, new extension installation,
production binding, live provider work, currentness, release, shared closeout,
rollover, or g04 closure.
