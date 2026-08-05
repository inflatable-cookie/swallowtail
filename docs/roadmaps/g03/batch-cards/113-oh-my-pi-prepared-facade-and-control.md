# 113 Oh My Pi Prepared Facade And Control

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../040-oh-my-pi-rpc-foundation.md`
Depends on: card 112

## Goal

Expose local-auth preparation, exact model and reasoning control, bounded input,
typed questions, activity, usage, cancellation, and fresh replacement.

## Acceptance

- [x] no credential reference or credential host service is required
- [x] model and reasoning are set and confirmed exactly
- [x] prepared run, session, and catalogue paths remain distinct
- [x] declined options fail before provider work

## Completion

`prepare_oh_my_pi_rpc` binds local OMP auth, exact provider/model, optional
reasoning, ambient read authority, typed questions, one bounded PNG, activity,
usage, cancellation, and fresh context-losing replacement. Unsupported options
fail during preparation or preflight.
