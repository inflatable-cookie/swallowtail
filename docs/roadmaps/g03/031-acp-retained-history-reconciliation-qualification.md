# 031 ACP Retained History Reconciliation Qualification

Status: active
Owner: Tom
Created: 2026-08-04
Depends on: g03.030
Vision tags: provider continuity, retained history, exact recovery
Contract refs: 017, 042, 048
Planning state: card 079 ready

## Problem

Claude Agent ACP and Kimi ACP can reload retained provider history after their
child process is gone. That may recover useful transcript truth, but it does
not prove that the original live turn survived or expose exact terminal state.

## Goals

- [ ] qualify exact retained-history surfaces for both ACP routes
- [ ] separate history recovery from live-turn, session-resume, and callback authority
- [ ] select the strongest honest first mapping or close the candidate with evidence
- [ ] keep provider and transport inheritance forbidden

## Execution Plan

- [ ] card 079: compare qualified Claude Agent and Kimi ACP load/history
  evidence, classify the portable fit, and compile implementation cards only
  when one route passes

## Boundaries

- no authenticated provider work unless a later accepting card explicitly requires it
- no prompt replay, callback answer, provider request, resume, or subagent control
- no terminal inference from history presence or process exit
- no generic ACP capability inheritance

## Acceptance Criteria

- [ ] both routes have exact version, binding, and history-source evidence
- [ ] the result names attribution strength and failure-closed rules
- [ ] any implementation runway cites the exact qualified route

## Next Planning Checkpoint

After card 079, either compile the selected ACP mapping or record the blocking
evidence and return to the next retained-operation candidate.
