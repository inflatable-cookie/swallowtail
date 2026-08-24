# 165 Deep Agents ACP Model Selection Binding

Status: ready
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.059 Deep Agents ACP Model Selection](../059-deepagents-acp-model-selection.md)
Depends on: card 164; non-empty Research 206 deliver-now table

## Goal

Bind only Research 206's admitted Deep Agents provider/model rows through typed
prepared inputs, immutable plan/request agreement, exact child argv, and
provider-key access agreement.

## Work

1. Add the smallest adapter-local typed model-selection surface admitted by
   Research 206. Do not expose arbitrary argv or a generic settings map.
2. Add optional selection to session preparation. Caller omission must retain
   existing constructors, capability truth, and empty child argv.
3. Carry the exact provider/model value through prepared evidence, plan
   constraints, request agreement, session state, and fresh restoration where
   admitted.
4. Require the selected provider to agree with one explicit host-owned access
   profile before process spawn. Never inspect or materialize key bytes.
5. Emit exactly one `--model <provider:model>` pair on explicit selection and
   no model tokens on omission.
6. Validate any Research 206 exact ACP confirmation field before accepting
   output. If the admitted claim is dispatch-only, keep effective/observed
   truth explicitly unavailable.
7. Map invalid value, plan/request mismatch, access mismatch, missing/wrong
   credential, provider rejection, and any confirmation drift to stable safe
   failures without fallback.
8. Preserve working resource, `AmbientHost`, host callback rejection,
   permission behavior, deadline, cancellation, terminal, and joined cleanup.
9. Add focused tests and guidance only for delivered rows.

## Acceptance Criteria

- [ ] only Research 206 deliver-now rows prepare
- [ ] selection appears in capability/constraint, prepared evidence, immutable
      plan, request, access agreement, and command truth
- [ ] omission keeps existing empty argv and behavior
- [ ] explicit selection emits exactly one `--model <provider:model>` pair
- [ ] invalid, unsupported, or access-mismatched values fail before spawn
- [ ] missing/wrong key or provider rejection never triggers fallback
- [ ] effective/observed truth is validated or explicitly withheld as Research
      206 requires
- [ ] one selection remains immutable for the child/session and any admitted
      fresh replacement
- [ ] access, resource, isolation, permission, callback, deadline,
      cancellation, failure, and cleanup behavior stay intact
- [ ] public API baseline and example reflect only the admitted typed surface
- [ ] `cargo fmt -p swallowtail-adapter-deepagents` passes
- [ ] `effigy validate:focused swallowtail-adapter-deepagents` passes
- [ ] `effigy package:verify-affected swallowtail-adapter-deepagents` passes
- [ ] `git diff --check` passes

## Stop Conditions

- Research 206 is empty or ambiguous
- binding requires shared contract/runtime changes, credential materialization,
  generic configuration, or a breaking public API
- provider/access agreement cannot fail before spawn
- implementation would claim effective model without admitted confirmation

## Out Of Scope

- other Deep Agents flags/routes, live auth/prompt work, currentness, release,
  merge, generation rollover, or g04 closure
