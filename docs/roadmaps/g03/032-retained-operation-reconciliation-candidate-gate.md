# 032 Retained Operation Reconciliation Candidate Gate

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.031
Vision tags: provider continuity, exact recovery, retained operation
Contract refs: 021, 022, 042, 048
Planning state: cards 080-082 completed

## Problem

ACP replay cannot satisfy the read-only reconciliation contract. Gemini CLI
headless retains local transcripts, while Anthropic Managed Agents retain
provider sessions and authoritative events. Neither candidate yet has a
durable exact operation binding accepted for restart observation.

## Goals

- [x] qualify Gemini transcript identity, terminal evidence, and read path
- [x] qualify Anthropic managed-operation identity, retention, and event lookup
- [x] select one exact implementation mapping or close both candidates
- [x] preserve route-specific retention, deletion, and control boundaries

## Execution Plan

- [x] card 080: Gemini durable-transcript reconciliation qualification
- [x] card 081: Anthropic managed-operation recovery qualification
- [x] card 082: compare exact evidence, select a mapping, and compile only the
  implementation runway which passes Contract 048

## Boundaries

- no prompt replay, resume, retry, callback answer, interrupt, or deletion
- no terminal inference from transcript presence, session idle, or process exit
- no raw path, provider id, or consumer-manufactured binding as authority
- no capability inheritance across another route in the same provider family
- no authenticated provider work unless an accepting card explicitly requires it

## Acceptance Criteria

- [x] each route has exact version, identity, retention, and observation evidence
- [x] read-only observation and ordinary continuation remain separate
- [x] exact terminal claims require exact provider operation or turn attribution
- [x] failures and negative qualifications remain visible without fake support
- [x] any implementation cards bind one exact qualified route

## Next Planning Checkpoint

Complete. Gemini fails because listing may mutate retained state and lacks
terminal evidence. Anthropic passes the exact read-only observation gate.
Continue with g03.033 card 083 to repair Gemini management truth before the
selected Anthropic implementation.
