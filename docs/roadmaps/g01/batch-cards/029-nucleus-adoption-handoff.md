# Nucleus Adoption Handoff

Status: completed
Owner: Tom
Roadmap: 008 Nucleus Interactive-Session Readiness
Updated: 2026-07-19

## Goal

Record a bounded downstream replacement for Nucleus's live Codex runtime behind
its existing session facade, with explicit rollback and validation.

## Scope

- mapping from `AgentSessionRuntime` operations to Swallowtail roles and handles
- host-service adapter boundary for Nucleus resource and process authority
- tool declaration/callback bridge with Nucleus-owned execution and receipts
- provider reference, model metadata, event, and terminal-outcome mapping
- dependency pin, feature gate, rollback, and downstream acceptance checklist

## Out Of Scope

- edits in the Nucleus repository
- wholesale replacement of `nucleus-agent-protocol` or `codex_supervision`
- task/goal/memory/persistence migration
- runtime stability or release publication

## Acceptance Criteria

- the first consumer change can be isolated behind the existing live registry
- Nucleus retains all product authority and durable state
- provider refs and normalized events map without raw payload persistence
- rollback restores the prior facade implementation without data migration
- roadmap 008 can close without claiming Nucleus adoption is complete

## Validation

- public-API mapping review
- consumer-owned test and manual acceptance checklist
- documentation and dependency audit
- `effigy qa`

## Stop Condition

Stop if replacement requires moving Nucleus control-plane records into
Swallowtail or changing durable product schemas.

## Closeout

- current Nucleus sources were inspected read-only; no consumer file changed
- the handoff targets one Nucleus-owned implementation behind the existing
  `AgentSessionRuntime` and `AgentAdapterRegistry` seam
- model catalog, session open, turn, event, callback, outcome, provider-ref,
  and cleanup mappings are explicit while Nucleus keeps product authority
- the initial slice is embedded-host only and rejects remote-authoritative
  resources before provider work until Nucleus owns a real remote invocation
  route
- tool-enabled stored sessions open fresh with transcript context because the
  current provider resume schema cannot safely redeclare Nucleus tools
- dependency pinning, temporary feature selection, downstream sequencing,
  acceptance, rollback, and legacy removal are recorded in
  `../nucleus-adoption-handoff.md`
- roadmap 008 closes without claiming that downstream adoption has happened
