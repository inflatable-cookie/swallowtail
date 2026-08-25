# 2026-08-25 g04.062 Anthropic Adaptive Thinking Compiled

Status: compiled
Owner: Tom
Milestone: g04.062
Cards: 173-175

## What Changed

- reassessed the remaining per-route feature inventory after g04.061
- corrected the stale Anthropic effort inventory row already delivered by
  g04.037
- selected adapter-local adaptive thinking on `anthropic.messages`
- compiled g04.062 and cards 173-175 as one serial evidence-first worker lane
- reserved Research 209 and the route-local closeout before dispatch
- kept g04 open and Contract 029 currentness standing

## Selection Basis

Current official Anthropic documentation names adaptive thinking as a separate
Messages request control and requires complete unmodified signed thinking
blocks to accompany tool-result continuation. Contracts 030 and 044 already
cover bounded provider-private continuation and prohibit hidden-reasoning
disclosure.

The production gap is exact and local. The current Anthropic stream grammar
rejects thinking blocks, while private session history reconstructs only the
assistant tool-use block. g04.062 therefore selects omitted-display adaptive
thinking, not readable summaries: enable the provider behavior, retain only
the signed private continuation needed for the tool loop, and expose no thought
content.

Research 209 must still freeze the exact model/profile/display row and every
possible private block shape. Cards 174-175 continue only for a non-empty exact
set and no shared-contract requirement.

## Reassessment Notes

Codex verbosity still lacks exact selected-model support evidence. llama.cpp
reasoning still lacks exact model/template semantics. Kiro profiles, OpenCode
permissions, sandbox, skills/memory, and multi-agent controls remain authority
or containment gates. Ollama `think=max` remains a `high` alias on the relevant
exact family. None is stronger than the Anthropic route-local protocol gap.

## Validation Plan

Card 173 uses current official public documentation and local source/fixtures
only. If it promotes a non-empty set, cards 174-175 run focused Anthropic
formatting, validation, affected-package verification, examples, route/docs/API
and index checks, doctor, and diff checks. Default QA makes no credentialed or
paid provider request.

## Next Move

Execute g04.062 cards 173-175 serially in one isolated worker worktree and open
one PR. Stop honestly after card 173 if Research 209 is empty or the exact
private-block gate fails. Do not merge or close g04.
