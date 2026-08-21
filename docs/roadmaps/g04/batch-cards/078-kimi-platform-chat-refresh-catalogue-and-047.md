# 078 Kimi Platform Chat Refresh, Catalogue, And 047

Status: completed
Owner: Tom
Created: 2026-08-21
Milestone: `../024-hosted-api-key-kimi-platform-chat.md`
Depends on: card 077

## Goal

Complete the deterministic Contract 057 lifecycle proof for the admitted Kimi
Platform instance without widening its existing operation or selection rules.

## Scope

1. Refresh host-supplied access status without changing enablement.
2. Record authenticated subject as `Absent`; do not probe the provider account.
3. Reuse the existing prepared catalogue and exact `moonshot` / `kimi-k3`
   model identity.
4. Reuse one explicit K3 inference attempt with explicit reasoning and output
   bound; do not add retry authority.
5. Assemble the 047 snapshot from exact prepared evidence and prove
   `Ready` / `NotReady` is unchanged.
6. Keep overlay identity exact and provider defaults separate from consumer
   defaults.
7. Update the Kimi prepared guide and connection-lifecycle guide for the
   realized addable path; keep the production route inventory unchanged.

## Out Of Scope

- live provider, credential, account, billing, or catalogue probes
- hosted OAuth, Kimi Membership, Kimi Code, or Kimi local server
- tools, reusable sessions, provider-state management, retry, or fallback
- changes to Contracts 047 or 057
- OpenHands production wiring

## Acceptance Criteria

- a disabled instance may refresh to ready access dimensions
- subject fields are `Absent` and remain outside 047
- catalogue and one explicit K3 attempt prepare only after admission
- 047 retains exact instance, route, provider, model, facade, and access truth
- overlay cannot change `Ready` / `NotReady`
- deterministic guides and examples name the realized lifecycle
- `release-baselines/public-api-0.3.3` remains unchanged

## Validation

- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-kimi-platform swallowtail-runtime swallowtail-host-local swallowtail-testkit`
- `effigy qa:routes`
- `effigy qa:guides`
- `effigy qa:northstar`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

No. Record merge reality before selecting another descriptor tranche.

## Stop Conditions

- Stop if selection readiness, provider/model identity, or the exact facade
  changes.
- Stop if the implementation adds provider work, retries, sessions, tools, or
  another Kimi audience.
- Stop if hosted OAuth or OpenHands enters scope.
