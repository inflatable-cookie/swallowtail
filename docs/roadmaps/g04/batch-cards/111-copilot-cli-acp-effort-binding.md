# 111 Copilot CLI ACP Effort Binding

Status: ready after 110
Owner: Tom
Created: 2026-08-22
Milestone: [g04.040 Copilot CLI ACP Session Effort](../040-copilot-cli-acp-session-effort.md)
Depends on: card 110; promoted Research 188

## Goal

Bind only the Research 188 deliver-now Copilot CLI ACP effort values through
prepared session input, immutable plan/evidence, request policy, configured
driver, and exact child argv.

## Scope

1. Add one optional typed reasoning input to
   `CopilotCliSessionProfileInput`; preserve the existing constructor and absent
   behavior.
2. Admit only Research 188 values on exact package `1.0.80`. Reject raw strings,
   upstream aliases, unqualified values, and incompatible package evidence.
3. Add exact `ReasoningSelection` capability constraints to the prepared
   instance, requirements, and plan only when selected.
4. Bind the same mode into `OpenSessionRequest`, prepared evidence, and an
   explicitly configured low-level driver.
5. Emit only the canonical Research 188 startup argv form. Keep
   `--reasoning-effort` as an upstream alias and expose no raw argv choice.
6. Preserve exact `copilot --acp --stdio` argv when no effort is selected.
7. Keep the setting fixed for the owned process and fresh context-losing
   replacement. Do not add per-turn mutation.
8. Validate request, plan, evidence, package, and driver agreement before task
   or process work. Add deterministic preparation and low-level rejection tests.
9. Preserve current permissions, denied host callbacks, working-resource,
   cancellation, terminal, and cleanup behavior.

## Acceptance Criteria

- [ ] only Research 188 deliver-now values prepare
- [ ] input, request, plan, evidence, driver, and argv agree exactly
- [ ] absent effort preserves current argv and public behavior
- [ ] no generic argv/configuration map or provider string enters public API
- [ ] tool filters, permissions, model selection, and other controls remain
      independent
- [ ] known mismatch and unsupported failures occur before process work

## Validation

```sh
cargo fmt -p swallowtail-adapter-copilot-cli
effigy validate:focused swallowtail-adapter-copilot-cli
effigy package:verify-affected swallowtail-adapter-copilot-cli
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 112 when exact binding, absent-path, and zero-process
failure tests pass.

## Stop Conditions

- portable reasoning cannot represent the exact Research 188 semantics
- prepared evidence and driver cannot remain in exact agreement
- fresh replacement can lose or change the prepared selection
- compatibility requires a new contract, unresolved behavior segment, or
  breaking public change

## Out Of Scope

- route guide, matrices, architecture, programme, changelog, or shared closeout
- provider acceptance or effective-effort claims
- tool filters, permission broadening, TCP, login, model selection, or live work
