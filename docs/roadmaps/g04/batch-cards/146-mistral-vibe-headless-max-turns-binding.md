# 146 Mistral Vibe Headless Maximum-Turn Binding

Status: conditional
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.052 Mistral Vibe Headless Maximum Turns](../052-mistral-vibe-headless-max-turns.md)
Depends on: card 145; promoted Research 199 with a non-empty deliver-now set

## Goal

Bind only Research 199's exact Mistral Vibe caller-decreasing maximum-turn
subset through typed adapter-local input, immutable prepared state, driver
validation, and command construction.

## Scope

1. Add the smallest typed adapter-local selection admitted by Research 199.
   Preserve current constructors and caller-omission behavior.
2. Keep the control inside `swallowtail-adapter-mistral-vibe`. Do not add a
   shared `Capability`, Contract 040 control, generic provider-settings map, or
   sibling-route behavior.
3. Bind selected values through preparation input, immutable plan/evidence,
   driver state, and command construction. Reject input/plan/evidence/driver/
   version drift before process start or prompt exposure.
4. Emit only exact Research 199 values on exact release `2.24.2`. Caller
   omission must preserve current `--max-turns 8`; upstream flag omission must
   remain impossible.
5. Preserve exact `--prompt`, `--output streaming`, `--trust`, `--agent plan`,
   `--workdir`, stdin-close, stream decoding, and one-child lifecycle behavior.
6. Preserve local unauthenticated access, required host services and deadline,
   working-resource authority, cancellation, failure, diagnostics, and cleanup.
7. Map limit terminal behavior only to Research 199's proved boundary. Do not
   convert stderr, exit status, or partial output into successful completion or
   an effective-work claim.
8. Advance only exact feature-local revisions selected by Research 199.
   Preserve the Contract 029 release claim and currentness posture.

## Acceptance Criteria

- [ ] only Research 199 deliver-now values prepare
- [ ] selection, plan/evidence, driver, and argv agree exactly
- [ ] caller omission remains byte- and behavior-stable at `--max-turns 8`
- [ ] invalid, raised, zero/negative when withheld, unbounded, aliased, and
      mismatched values reject before effects
- [ ] prompt, output, plan agent, trust, workdir, access, deadline,
      cancellation, failure, diagnostics, and cleanup remain unchanged
- [ ] no shared runtime, portable capability, sibling route, retry, fallback,
      provider-acceptance, effective-work, quality, latency, cost, or billing
      claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-mistral-vibe
effigy validate:focused swallowtail-adapter-mistral-vibe
effigy package:verify-affected swallowtail-adapter-mistral-vibe
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 147 when exact preparation, command, terminal,
rejection, and lifecycle preservation pass.

## Stop Conditions

- implementation needs a shared capability, contract/currentness change, or
  breaking public API
- admitted values cannot remain exact across input, plan/evidence, driver, and
  command
- terminal truth, omission, or any fixed route boundary changes

## Out Of Scope

- route guide, shared closeout, live provider work, another version/profile,
  release, publication, or merge
