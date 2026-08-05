# 104 Interactive Harness Replacement Recovery

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../038-provider-wide-interactive-crash-recovery.md`
Depends on: card 103

## Goal

Expose a fresh-session replacement action for interactive harness routes which
cannot restore provider context.

## Scope

1. Wrap already prepared ordinary session creation.
2. Map Antigravity continuation, Gemini ACP, Pi RPC, and Qwen continuation.
3. Preserve the interrupted consumer turn as unresolved.
4. Report provider-context loss in the outcome variant.
5. Prove no prompt, retry, provider lookup, or old-session mutation occurs.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-antigravity swallowtail-adapter-gemini swallowtail-adapter-pi`
- `effigy validate:focused swallowtail-adapter-qwen`

## Stop Conditions

- stop if replacement can be mistaken for attachment or continuation
- stop if a route requires prompt replay to become usable

## Auto-Continuation

Continue to card 105 when all four prepared mappings pass.

## Outcome

- Antigravity continuation, Gemini ACP, Pi RPC, and Qwen continuation expose
  `FreshSessionReplacement`
- replacement opens only the already prepared session and retains the lost
  turn as unresolved
- no prompt, transcript, provider-session lookup, or old-session mutation is
  part of replacement
- focused four-adapter validation passed: 160 tests
