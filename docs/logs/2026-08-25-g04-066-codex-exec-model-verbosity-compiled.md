# 2026-08-25 g04.066 Codex Exec Model Verbosity Compiled

Status: compiled
Owner: Tom
Milestone: g04.066
Cards: 184-186

## What Changed

- reassessed the remaining promoted per-route feature inventory after the
  g04.065 Ultracode evidence stop
- kept Claude headless Fast, teams, autocompact, maximum-turn, and spend-cap
  controls outside this lane because exact help, topology, access, or billing
  boundaries remain weaker
- selected Codex Exec `model_verbosity` as the next route-local family
- compiled g04.066 and cards 184-186 as one serial evidence-first worker lane
- reserved Research 213 and the route-local closeout before dispatch
- kept g04 open and Contract 029 currentness standing

## Selection Basis

The maintained `codex.exec` route already owns one suppressed-config ephemeral
child, selects one exact model, and binds typed config overrides for reasoning
and search. Current official Codex schema exposes closed
`low|medium|high` verbosity, while official model metadata separately exposes
`support_verbosity` and `default_verbosity`.

That split provides an exact fail-closed evidence path: card 184 must bind the
release-tag CLI, provider, selected model, supported values, default, parser,
precedence, and request behavior. The config key alone does not qualify a row,
and no provider-acceptance or effective-output claim is preauthorized.

## Validation Plan

Card 184 uses current official public documentation, exact `rust-v0.149.1`
source, extracted local parser/config cases, and repository source/fixtures
only. If it promotes a non-empty set, cards 185-186 run focused Codex adapter
formatting, validation, affected-package verification, examples, route/docs/API
and index checks, doctor, and diff checks. Default QA makes no credentialed
prompt or paid provider request.

## Next Move

Execute g04.066 cards 184-186 serially in one isolated worker worktree and open
one PR. Stop honestly after card 184 if Research 213 is empty or the exact
model/provider/precedence gate fails. Do not merge or close g04.
