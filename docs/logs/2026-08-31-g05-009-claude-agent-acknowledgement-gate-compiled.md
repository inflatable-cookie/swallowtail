# 2026-08-31 g05.009 Claude Agent Acknowledgement Gate Compiled

Status: complete; card 031 ready
Owner: Tom
Date: 2026-08-31
Contracts: 037, 047, 057, 061

## Result

The operator approved Card 030's two-point `claude-agent.acp` gate:
adapter-local retention of exact provider-effective and rejected reasoning,
and one additive adapter-owned open-with-projection outcome/failure preserving
the existing `ClaudeAgentPreparedSession::open_session`.

The gate fixes the exact adapter-owned API family, source split, state
transitions, malformed/unknown failure boundary, shared-open lifecycle, and
provider-free proof. It changes no runtime/core public decision and grants no
authority to Kimi or Cline.

Candidate D now passes the Batch 9.4 readiness rubric and is promoted as ready
card 031. The implementation tranche owns the Claude Agent adapter package's
complete 53-row remainder: 30 `claude-agent.acp`, 12
`claude-code.headless`, and 11 `claude-code.response-only`. It combines the
public-baseline repair with the package proof rather than creating a
public-API-only micro-card.

## Current State

- g05.009 returns to `strict-ready`
- card 031 is the sole ready implementation card and sole Next Task
- 148 census rows remain proved; candidate D's 53 rows are not counted until
  implementation review and merge
- candidates B, C, E-G, and I-L remain unpromoted
- `kimi-code.acp`, `cline.acp`, `EffectiveReasoningSetup`, negotiated
  model-option observation, and provider-session catalogue observation remain
  later route-local gate work
- no Rust, manifest, release baseline, contract, architecture, census, or
  provider claim changed in this planning batch
- no provider was contacted and no live probe ran

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Move

Implement card 031 as one Claude Agent adapter PR. Stop after its exact 53-row
proof for orchestrator review before reassessing another Batch 9.4 candidate.

## Authority

- [card 031](../roadmaps/g05/batch-cards/031-contract-061-claude-agent-package-and-acknowledgement.md)
- [public-baseline gate](../triage/2026-08-31-contract-061-claude-agent-acknowledgement-public-baseline-gate.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
