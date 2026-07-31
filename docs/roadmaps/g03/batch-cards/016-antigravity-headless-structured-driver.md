# 016 Antigravity Headless Structured Driver

Status: completed
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

- [x] one run has one exact terminal result
- [x] invalid model selection remains a typed visible failure
- [x] schema-constrained output and stream activity remain distinct evidence
- [x] tool and subagent payloads are bounded and safely mapped
- [x] usage fields retain exact provider semantics
- [x] permission-required tools are not auto-approved
- [x] sandboxing remains explicit and optional
- [x] focused structured and activity conformance passes

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

Completed. Continue to card 017.

## Result

Research 079 qualifies exact `agy` `1.1.9` headless stream JSON from official
documentation, the installed help surface, and deterministic synthetic
fixtures. The separate `swallowtail.antigravity.headless` driver binds one
explicit model and resource, supports optional low/medium/high effort and
bounded inline JSON Schema, and preserves ambient versus provider-enforced
isolation without selecting dangerous permission bypass.

The bounded event pump validates exact init identity and request-review mode,
projects assistant, tool, completion-only subagent, usage, and terminal
evidence, and rejects missing or duplicate terminal results. Tool arguments,
tool output, provider stderr, workspace paths, prompt text, and subagent URIs
remain outside stable events and diagnostics. Cancellation, deadlines, process
exit, event delivery, and joined cleanup remain distinct outcomes.

Focused validation passed 22 tests across three binaries plus warnings-denied
checking in one second. No live prompt, login mutation, credential read,
consumer edit, broad workspace suite, or publication ran.
