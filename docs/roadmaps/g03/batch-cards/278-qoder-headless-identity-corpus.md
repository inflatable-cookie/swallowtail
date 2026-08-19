# 278 Qoder Headless Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../091-qoder-headless-route.md`
Depends on: Card 261; Research 143

## Goal

Freeze official identity and the first deterministic corpus for Qoder Headless. Candidate routes are `qoder.headless`. Do not edit production claims.

## Scope

Record official release/package identity, executable or protocol entrypoint, selected help/schema/event samples, authentication boundary, working-resource and process topology, and the smallest drift-rejecting fixture set. Decide whether each candidate route remains admitted, splits, or closes as negative evidence. Planned package boundary is `swallowtail-adapter-qoder`; Pi extends the existing package.

## Out Of Scope

driver implementation, prepared API, production matrix changes, live provider work, installation, login, and version-range claims

## Acceptance Criteria

- [x] Exact source identity and route disposition are recorded.
- [x] Protocol/event evidence is saved in an adapter-local corpus plan.
- [x] Authentication, authority, isolation, cancellation, and cleanup limits are explicit.
- [x] No claim changes before the next card.

## Validation

`effigy qa:northstar`; source and fixture review only.

## Stop Conditions

Stop if the route is undocumented, prompt-only, UI-only, or requires hidden credential state to establish its wire shape.

## Auto-Continuation

Continue to card 279 after the route and corpus shape are named.

## Evidence

Research 151; `docs/logs/2026-08-19-qoder-headless-1-1-25-identity.md`;
fixtures under `crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.25/`.
`effigy qa:northstar` passed. No production claim.
