# 032 Retained Operation Reconciliation Candidate Gate

Status: active
Owner: Tom
Created: 2026-08-04
Depends on: g03.031
Vision tags: provider continuity, exact recovery, retained operation
Contract refs: 021, 022, 042, 048
Planning state: card 080 ready; cards 081-082 planned

## Problem

ACP replay cannot satisfy the read-only reconciliation contract. Gemini CLI
headless retains local transcripts, while Anthropic Managed Agents retain
provider sessions and authoritative events. Neither candidate yet has a
durable exact operation binding accepted for restart observation.

## Goals

- [ ] qualify Gemini transcript identity, terminal evidence, and read path
- [ ] qualify Anthropic managed-operation identity, retention, and event lookup
- [ ] select one exact implementation mapping or close both candidates
- [ ] preserve route-specific retention, deletion, and control boundaries

## Execution Plan

- [ ] card 080: Gemini durable-transcript reconciliation qualification
- [ ] card 081: Anthropic managed-operation recovery qualification
- [ ] card 082: compare exact evidence, select a mapping, and compile only the
  implementation runway which passes Contract 048

## Boundaries

- no prompt replay, resume, retry, callback answer, interrupt, or deletion
- no terminal inference from transcript presence, session idle, or process exit
- no raw path, provider id, or consumer-manufactured binding as authority
- no capability inheritance across another route in the same provider family
- no authenticated provider work unless an accepting card explicitly requires it

## Acceptance Criteria

- [ ] each route has exact version, identity, retention, and observation evidence
- [ ] read-only observation and ordinary continuation remain separate
- [ ] exact terminal claims require exact provider operation or turn attribution
- [ ] failures and negative qualifications remain visible without fake support
- [ ] any implementation cards bind one exact qualified route

## Next Planning Checkpoint

After card 082, execute the selected mapping or return the sole Next Task to
the g03 evidence gate when neither candidate passes.
