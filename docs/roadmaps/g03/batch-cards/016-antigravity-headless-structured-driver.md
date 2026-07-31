# 016 Antigravity Headless Structured Driver

Status: planned
Owner: Tom
Created: 2026-07-31
Milestone: `../006-antigravity-personal-harness-foundation.md`
Depends on: card 015

## Goal

Implement Antigravity's headless JSON and stream-JSON surface as a bounded
structured-run driver with exact model, schema, permission, and activity truth.

## Scope

1. Freeze and parse init, step-update, tool, subagent, usage, error, and result
   events from the exact artifact.
2. Implement explicit model and effort selection plus JSON-schema output.
3. Map bounded activity, usage, cancellation, deadlines, and joined cleanup.
4. Preserve ambient permissions, soft denial, and optional sandbox profiles.
5. Reject dangerous permission bypass and implicit model fallback.

## Acceptance Criteria

- [ ] one run has one exact terminal result
- [ ] invalid model selection remains a typed visible failure
- [ ] schema-constrained output and stream activity remain distinct evidence
- [ ] tool and subagent payloads are bounded and safely mapped
- [ ] usage fields retain exact provider semantics
- [ ] permission-required tools are not auto-approved
- [ ] sandboxing remains explicit and optional
- [ ] focused structured and activity conformance passes

## Validation

- `effigy validate:focused swallowtail-adapter-antigravity`
- focused structured, schema, activity, usage, permission, cancellation,
  deadline, and cleanup tests
- no broad workspace suite or live prompt

## Stop Conditions

- Stop if provider output cannot be sanitized without losing correlation.
- Stop if a bounded run requires dangerous permission bypass.
- Do not infer interactive callback support from TUI behavior.

## Auto-Continuation

Yes. Continue to card 017 after focused structured validation passes.

