# 052 Claude Agent ACP Refresh, Update, And Subject

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../018-installed-claude-agent-acp.md`
Depends on: card 051

## Goal

Refresh access status, project 029/032 update observation, and keep
authenticated subject Absent for an admitted Claude Agent ACP instance.

## Scope

1. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement is
   unchanged.
2. `observe_instance_update` reuses `claude_agent_acp_claim` and optional
   032 installed-executable evidence.
3. Subject is `Absent`. Do not scrape Claude account email.
4. Catalogue or session-negotiated model rows without `provider_id` stay
   unmarked. Do not invent a provider id so overlay can key.
5. Deterministic harnesses only.

## Out Of Scope

- live login, install, or version probes
- hosted OAuth
- llama.cpp descriptors
- Contract 052 consumer-path publication

## Acceptance Criteria

- [ ] a disabled instance can refresh to ready access dimensions
- [ ] subject fields are Absent, not 047 fields
- [ ] update observation reuses 029 classification and optional 032
      evidence
- [ ] overlay does not invent a Claude Agent catalogue provider id
- [ ] `public-api-0.3.3` stays immutable

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime swallowtail-testkit`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

No. Compile llama.cpp attached only after this installed proof. Hosted
OAuth stays gated.

## Stop Conditions

- Stop if overlay invents a provider id or changes `Ready` / `NotReady`.
- Stop if subject becomes an instance id or routing key.
- Stop if a live login probe starts.
