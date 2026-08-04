# 031 ACP Retained History Reconciliation Qualification

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.030
Vision tags: provider continuity, retained history, exact recovery
Contract refs: 017, 042, 048
Planning state: card 079 completed

## Problem

Claude Agent ACP and Kimi ACP can reload retained provider history after their
child process is gone. That may recover useful transcript truth, but it does
not prove that the original live turn survived or expose exact terminal state.

## Goals

- [x] qualify exact retained-history surfaces for both ACP routes
- [x] separate history recovery from live-turn, session-resume, and callback authority
- [x] select the strongest honest first mapping or close the candidate with evidence
- [x] keep provider and transport inheritance forbidden

## Execution Plan

- [x] card 079: compare qualified Claude Agent and Kimi ACP load/history
  evidence, classify the portable fit, and compile implementation cards only
  when one route passes

## Boundaries

- no authenticated provider work unless a later accepting card explicitly requires it
- no prompt replay, callback answer, provider request, resume, or subagent control
- no terminal inference from history presence or process exit
- no generic ACP capability inheritance

## Acceptance Criteria

- [x] both routes have exact version, binding, and history-source evidence
- [x] the result names attribution strength and failure-closed rules
- [x] no implementation runway was compiled for a route which failed the gate

## Next Planning Checkpoint

Complete. Both ACP routes fail the read-only gate because history is exposed
only through stateful `session/load`. Continue with g03.032 card 080 to qualify
Gemini CLI's durable transcript without prompt replay.
