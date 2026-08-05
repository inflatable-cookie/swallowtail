# 036 Reconciliation Then Attachment Composition

Status: planned
Owner: Tom
Created: 2026-08-05
Depends on: g03.035
Vision tags: restart continuity, exact observation, session attachment
Contract refs: 017, 037, 046, 048, 050
Planning state: cards 093-096 planned

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

- [ ] define portable observation-plus-attachment outcomes without flattening
      either phase
- [ ] keep active, waiting, unknown, failed, and stale evidence fail-closed
- [ ] compose Codex and OpenCode reconciliation with exact load/replay
- [ ] compose Kimi local exact-turn reconciliation with replay-free resume

## Execution Plan

- [ ] card 093: promote the settled-session attachment rules into Contract 050
- [ ] card 094: realize the consuming runtime sequence and provider-free
      conformance
- [ ] card 095: map Codex app-server and OpenCode HTTP
- [ ] card 096: map Kimi local server and close public/package acceptance

## Boundaries

- no attachment after reconciliation failure, active or waiting evidence,
  unknown ownership, stale binding, or terminal ambiguity
- no attachment presented as read-only reconciliation
- no dynamic provider, route, model, credential, or session selection
- no callback answer, interruption, retry, prompt replay, import, management,
  or cleanup authority
- no reuse of a reconciliation checkpoint as an attachment binding

## Acceptance Criteria

- [ ] preparation binds both operations before provider work
- [ ] reconciliation always precedes attachment
- [ ] settled attachment preserves the complete reconciliation outcome beside
      the distinct live-session result
- [ ] Codex and OpenCode retain bounded ordered replay
- [ ] Kimi local reports replay-free resume honestly
- [ ] any first-phase failure or ineligible state issues no attachment request
- [ ] focused and affected-package validation pass

## Lane Runway

Planned after g03.035. Card 093 is the contract gate. Implementation cannot
begin while Contract 050 still permits exactly one restoration method.
