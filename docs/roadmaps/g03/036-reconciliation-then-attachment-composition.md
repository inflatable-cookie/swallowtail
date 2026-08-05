# 036 Reconciliation Then Attachment Composition

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.035
Vision tags: restart continuity, exact observation, session attachment
Contract refs: 017, 037, 046, 048, 050
Planning state: completed; g03.037 card 097 ready

## Problem

Codex app-server and OpenCode HTTP support exact load/replay as well as stronger
read-only reconciliation. Kimi local server supports exact-turn reconciliation
plus replay-free resume. The current facade correctly selects reconciliation,
but a consumer seeking a usable settled session must still orchestrate a
second route-specific attachment operation.

## Generation Runway Goal

Add one explicit prepared sequence that observes first and attaches only when
the exact reconciled state permits it.

## Goals

- [x] define portable observation-plus-attachment outcomes without flattening
      either phase
- [x] keep active, waiting, unknown, stale, and reconciliation-failure evidence
      fail-closed
- [x] compose Codex and OpenCode reconciliation with exact load/replay
- [x] compose Kimi local exact-turn reconciliation with replay-free resume

## Execution Plan

- [x] card 093: promote the settled-session attachment rules into Contract 050
- [x] card 094: realize the consuming runtime sequence and provider-free
      conformance
- [x] card 095: map Codex app-server and OpenCode HTTP
- [x] card 096: map Kimi local server and close public/package acceptance

## Boundaries

- no attachment after reconciliation failure, active or waiting evidence,
  unknown ownership, stale binding, or terminal ambiguity
- no attachment presented as read-only reconciliation
- no dynamic provider, route, model, credential, or session selection
- no callback answer, interruption, retry, prompt replay, import, management,
  or cleanup authority
- no reuse of a reconciliation checkpoint as an attachment binding

## Acceptance Criteria

- [x] preparation binds both operations before provider work
- [x] reconciliation always precedes attachment
- [x] settled attachment preserves the complete reconciliation outcome beside
      the distinct live-session result
- [x] Codex and OpenCode retain bounded ordered replay
- [x] Kimi local reports replay-free resume honestly
- [x] any first-phase failure or ineligible state issues no attachment request
- [x] focused and affected-package validation pass

## Lane Runway

Milestone complete. Continue to g03.037 card 097. Its independent evidence
gates do not inherit attachment authority from this milestone.
