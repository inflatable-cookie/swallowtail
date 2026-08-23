# 140 DeepSeek Structured-Run Thinking-Mode Binding

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.050 DeepSeek Structured-Run Thinking Mode](../050-deepseek-structured-run-thinking-mode.md)
Depends on: card 139; promoted Research 197 with a non-empty deliver-now set

## Goal

Bind only Research 197's exact DeepSeek structured-run thinking-mode subset
through adapter-local typed prepared state, immutable evidence, driver
validation, and exact request encoding.

## Scope

1. Add one typed adapter-local selection only to the Research 197 admitted
   one-request structured-run profile. Preserve current constructors and exact
   enabled behavior.
2. Keep the control inside `swallowtail-adapter-deepseek`. Do not add a shared
   `Capability`, portable `ReasoningMode`, generic provider-settings map, or
   sibling-route behavior.
3. Bind selected mode through configured input, preflight plan/evidence,
   driver state, and request encoding. Reject input/plan/evidence/driver/model/
   facade drift before endpoint, credential, or provider work.
4. Encode only the exact Research 197 field combination. Do not emit an effort
   value for disabled mode unless Research 197 expressly admits it. Do not
   map disabled to `ReasoningMode("none")`.
5. Preserve exact enabled `low|high|max` plan constraints and request bytes.
   Existing callers must not silently change mode, effort, model, facade,
   cache acceptance, output bound, deadline, cancellation, or cleanup.
6. Keep session preparation and every direct-continuation attempt enabled-only.
   A disabled session request must be impossible or reject before effects.
7. Parse response fields only to Research 197's admitted boundary. Reject
   impossible or drifted private-reasoning fields where the exact selected
   mode makes that knowable; never expose private reasoning.
8. Advance only exact facade/private behavior/claim/model-route revisions
   selected by Research 197. Preserve prior evidence as historical proof.

## Acceptance Criteria

- [x] only Research 197 deliver-now structured-run states prepare
- [x] selected mode, plan/evidence, driver, and request bytes agree exactly
- [x] enabled `low|high|max` calls and all continuation paths are unchanged
- [x] disabled mode carries no false reasoning capability or effort selection
- [x] aliases, unknown values, unsupported profiles, and knowable drift reject
      before effects
- [x] no shared runtime, portable capability, sibling route, retry, fallback,
      quality, latency, price, or provider-acceptance claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-deepseek
effigy validate:focused swallowtail-adapter-deepseek
effigy package:verify-affected swallowtail-adapter-deepseek
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 141 when exact preparation, request, response,
composition, rejection, and enabled-continuation preservation pass.

## Stop Conditions

- implementation needs a shared capability, contract change, currentness
  change, or breaking public API
- disabled state cannot remain exact across input, plan/evidence, driver, and
  wire request
- enabled behavior or direct continuation changes

## Out Of Scope

- route guide, shared closeout, live provider work, another DeepSeek profile,
  release, publication, or merge

## Closeout

Card 140 bound the one Research 197 deliver-now state through the public
adapter-local `DeepSeekThinkingMode::disabled()` type and
`DeepSeekRunProfileInput::new_with_thinking_mode`. Existing `new` input keeps
the enabled `low|high|max` path. Disabled preparation removes
`ReasoningSelection` from the immutable plan and leaves the shared request
policy without a portable reasoning value; prepared evidence and the bound
driver retain the typed mode.

The request encoder emits `thinking.type=disabled` and omits
`reasoning_effort`. Driver validation requires the disabled mode, omitted
reasoning policy, and plan without `ReasoningSelection` to agree before access
leases or endpoint work. Sessions and all continuation/replay encoders remain
enabled-only. No shared capability, contract, facade, model, currentness, or
provider-acceptance claim changed.
