# 185 Command Code Interactive Continuity Corpus

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../060-command-code-interactive-continuity.md`
Depends on: Research 118

## Goal

Freeze retained-session and exact-resume NDJSON fixtures and failure shapes
before interactive Rust behavior lands.

## Scope

1. Add sanitized first-turn retained and second-turn `--resume` success streams.
2. Freeze bad-id, cross-cwd (as preparation rejection notes), `--continue`, and
   `--fork-session` rejection fixtures or assertion records.
3. Keep `--no-session` structured-run fixtures distinct.
4. Record that durable transcripts are project-scoped under
   `~/.commandcode/projects/` without copying private JSONL into the repo.

## Acceptance

- [x] fixtures omit credentials, account ids, private home paths, prompts,
      thinking, and tool bodies
- [x] session ids in fixtures are stable sanitized UUIDs
- [x] success streams still terminate with one `result` line
- [x] forbidden selectors have explicit fail-closed expectations

## Evidence

- `tests/fixtures/command-code-1.15.1/interactive-first-turn.jsonl` and
  `interactive-resume-turn.jsonl` plus `negative-cases.json` / command-vector
  assertions cover first turn, exact resume, and forbidden selectors.
- Corpus tests in `tests/corpus/interactive.rs` and `src/command.rs` unit
  tests keep structured `--no-session` distinct from interactive retention.

## Validation

- focused fixture/parser tests introduced by this card
- covered by later card 186/187 focused package validation

## Stop Conditions

- stop if exact `--resume` cannot be distinguished from ambient `--continue`
- stop if sanitized fixtures cannot keep turn correlation

## Auto-Continuation

Continue to card 186 once the corpus is ready for the interactive driver.
