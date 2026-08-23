# 150 Qoder Headless Maximum-Turn Acceptance

Status: conditional on card 149
Owner: Tom
Created: 2026-08-24
Milestone: [g04.053 Qoder Headless Maximum Turns](../053-qoder-headless-max-turns.md)
Depends on: cards 148-149; promoted Research 200

## Goal

Prove the admitted exact Qoder maximum-turn boundary across preparation,
dispatch, terminal classification, cancellation, deadline, and cleanup, then
close the route-local lane without closing g04.

## Method

1. Prove every admitted boundary value and representative interior value.
2. Prove omission retains exact current argv and behavior.
3. Prove invalid values and prepared/driver drift reject before effects.
4. Preserve exact `error_max_turns` provider-failed truth and distinguish it
   from success, cancellation, and host deadline.
5. Prove fixed permission, output, persistence, workdir, access, and cleanup
   boundaries remain unchanged.
6. Update Qoder guide, example, route-local fixtures, public API baseline, cards,
   milestone, Research 200, and reserved closeout delta. Leave shared surfaces
   to the orchestrator.

## Acceptance Criteria

- [ ] admitted, omitted, invalid, and drifted selections are deterministic
- [ ] native limit remains provider-failed with exact safe diagnostic truth
- [ ] cancellation and deadline remain distinct and cleanup joins
- [ ] no provider acceptance, effective work, token, cost, latency, or quality
      claim is introduced
- [ ] route guide and example show typed selection and omission semantics
- [ ] default validation performs no install, login, prompt, or paid work
- [ ] g04.053 closes only this family; g04 remains open

## Validation

- `effigy validate:focused swallowtail-adapter-qoder`
- `effigy package:verify-affected swallowtail-adapter-qoder`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy package:api`
- `git diff --check`

## Stop Conditions

- Stop if Research 200 or implementation invalidates the planned domain.
- Stop if deterministic proof needs credentials, a live prompt, or a provider
  acceptance claim.
- Stop on shared-contract, currentness, sibling-route, or breaking-API need.
