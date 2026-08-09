# 186 Command Code Interactive Session Driver

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../060-command-code-interactive-continuity.md`
Depends on: card 185

## Goal

Implement Contract 043 interactive continuity on the existing Command Code
package beside the structured-run path.

## Scope

1. Advertise `InteractiveSession` on the headless driver descriptor.
2. First-turn command omits `--no-session` and omits resume selectors.
3. Later turns append exact private `--resume <sessionId>` from the prior clean
   turn observation.
4. Prepared session facade binds durable-allowed retention for interactive only;
   structured runs keep prohibited retention and `--no-session`.
5. Reject `--continue`, `--fork-session`, and consumer-supplied arbitrary resume
   ids outside the private continuity handle.

## Acceptance

- [x] structured-run path behavior remains unchanged
- [x] interactive first/later turn argument vectors match Research 118
- [x] private continuity does not mint public `SessionResumeBinding`
- [x] focused package validation passes without credentials

## Evidence

- `CommandCodeHeadlessDriver` advertises interactive session beside structured
  run; `prepare_session` / `open_session` and private resume live in
  `prepared/session.rs` and `session/`.
- Public load/resume stay unsupported; `resume_binding()` remains `None`.

## Validation

- `effigy validate:focused swallowtail-adapter-command-code` — 29 passed

## Stop Conditions

- stop if interactive work would require scanning `~/.commandcode/projects`
- stop if retention policy cannot stay split by operation shape

## Auto-Continuation

Continue to card 187 once the driver and prepared session facade are green.
