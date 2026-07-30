# 137 Observable Activity Consumer Handoff

Status: ready
Owner: Tom
Created: 2026-07-29
Milestone: `../040-provider-wide-activity-acceptance-and-consumer-handoff.md`
Depends on: card 136

## Goal

Produce bounded Nucleus and Soundcheck adoption handoffs while keeping
consumer persistence and presentation downstream.

## Scope

1. Define the Nucleus projection boundary:
   - consume one prepared route activity profile
   - persist assistant messages and work activity separately
   - correlate by runtime turn and activity id
   - group and collapse consecutive work in the chat view
   - label reasoning as summaries
   - retain exact unavailable and completion-only states
2. Define Soundcheck's bounded structured-run projection.
3. Document migration away from provider-native event parsing and broad
   generic progress.
4. Preserve consumer-owned thread lifecycle, authorization, retention,
   review, analytics, and UI.
5. Record exact continuation cards and one next task.
6. Close or re-scope the observable-activity programme.

## Out Of Scope

- editing Nucleus or Soundcheck
- prescribing a consumer database or UI framework
- automatic consumer migration
- release publication

## Acceptance Criteria

- [ ] the handoff uses only public prepared and runtime APIs
- [ ] Nucleus owns durable messages, activities, grouping, and collapse
- [ ] Soundcheck can ignore rich activity without losing final output
- [ ] no consumer must switch on provider-native event names
- [ ] reasoning disclosure language is exact
- [ ] provider-specific gaps remain visible
- [ ] one explicit next task remains

## Validation

- compile the public handoff examples
- `effigy qa:docs`
- `effigy qa:routes`
- `effigy package:api`

## Stop Conditions

- Ask the operator before any consumer repository edit.
- Do not close with a consumer requirement that Contract 044 cannot express.

## Auto-Continuation

No. This card closes the programme and returns the Nucleus adoption decision
to the operator.
