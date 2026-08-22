# 114 Qwen Headless Reasoning Effort Binding

Status: ready after 113
Owner: Tom
Created: 2026-08-22
Milestone: [g04.041 Qwen Headless Reasoning Effort](../041-qwen-headless-reasoning-effort.md)
Depends on: card 113; promoted Research 189 with a non-empty deliver-now set

## Goal

Bind only Research 189 deliver-now Qwen model/value rows through prepared run
and session input, immutable plan/evidence, request policy, configured driver,
and exact operation-private child transport.

## Scope

1. Add one optional typed reasoning input to `QwenRunProfileInput` and
   `QwenSessionProfileInput`; preserve existing constructors and absent
   behavior.
2. Admit only Research 189 exact model/value/package rows. Reject raw strings,
   upstream aliases, unqualified models, and incompatible package evidence.
3. Add exact `ReasoningSelection` capability constraints to the model route,
   configured instance, requirements, and plan only when selected.
4. Bind the same selection into structured-run or open-session request policy,
   prepared evidence, and an explicitly configured low-level driver.
5. Emit only the Research 189 operation-private transport. Expose no raw
   setting key, argv, environment, or configuration choice.
6. Preserve exact current argv/environment when reasoning is absent.
7. Apply the same immutable selection to a structured-run child, first-turn
   child, every `--resume` child, and a fresh context-losing replacement.
8. Ensure ambient configuration cannot override the planned value. Do not
   create a synthetic home/config root or mutate user/project settings.
9. Validate request, plan, evidence, model, package, driver, and transport
   agreement before task or process work. Preserve existing lifecycle and
   resource behavior.

## Acceptance Criteria

- [ ] only Research 189 deliver-now rows prepare
- [ ] input, request, plan, evidence, driver, and child transport agree exactly
- [ ] absent reasoning preserves current command and public behavior
- [ ] no generic setting/argv/environment map enters the public API
- [ ] model selection and reasoning selection remain exact and independent
- [ ] ambient settings cannot override the prepared value
- [ ] known mismatch and unsupported failures occur before process work

## Validation

```sh
cargo fmt -p swallowtail-adapter-qwen
effigy validate:focused swallowtail-adapter-qwen
effigy package:verify-affected swallowtail-adapter-qwen
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 115 when exact run/session binding, absent-path, and
zero-process failure tests pass.

## Stop Conditions

- portable reasoning cannot represent Research 189 semantics exactly
- prepared evidence and driver cannot remain in exact agreement
- ambient configuration can override the planned value
- resumed turns or fresh replacement can lose or change the selection
- compatibility requires a new contract, unplanned version corpus, or breaking
  public change

## Out Of Scope

- route guide, matrices, architecture, programme, changelog, or shared closeout
- provider-effective reasoning claims not proved by Research 189
- other Qwen settings, model discovery, permissions, tools, or live work
