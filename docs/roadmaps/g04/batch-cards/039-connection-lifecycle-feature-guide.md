# 039 Connection Lifecycle Feature Guide

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../014-connection-lifecycle-consumer-path.md`
Depends on: completed g04.013

## Goal

Publish a Contract 052 feature guide for the realized Contract 057
connection lifecycle.

## Scope

1. Write `docs/guides/connection-lifecycle.md`. Cover portable records and
   entry points: `AddableRouteDescriptor`, consumer-assembled
   `AddableRouteCatalog`, `admit_instance`, config and credential field
   descriptors, library-owned sign-in loop through host ports, API-key
   collection as `CredentialRef`, `refresh_readiness`,
   `observe_authenticated_subject`, `observe_instance_update`,
   `apply_stored_model_presentation_overlay`, the store port, and the
   optional host-local adapters.
2. Name route applicability exactly: only `anthropic.messages`,
   `codex.app-server`, and `ollama.attached` currently export addable
   descriptors. Remaining production routes stay on the prepared-facade
   path. Hosted interactive OAuth is not a realized consumer path.
3. Name ordering: catalog → admit → credential or skip → refresh and
   optional subject → 047 snapshot plus overlay → existing prepared
   facade. Preparation stays after admission. Enablement is not
   readiness. Overlay cannot change `Ready` / `NotReady`.
4. Name consumer responsibilities and forbidden inferences: consumers
   assemble catalogs and own persistence, UI, and selection policy;
   Swallowtail is not a server, router, secret store, or login client.
   Subject is never an instance id or routing key. Catalogue rows
   without `provider_id` stay unmarked.
5. Index the guide from `docs/guides/README.md`. Add short Key Concepts
   terms that point at it.
6. Do not add the integration-guide-map feature family yet.

## Out Of Scope

- route-guide amendments (card 040)
- compiling examples and guide-map family row (card 041)
- hosted OAuth
- new adapter descriptors
- Contract or checker edits

## Acceptance Criteria

- [ ] the feature guide names the portable 057 records, roles, and
      prepared-facade handoff
- [ ] only the three first-proof routes are listed as addable
- [ ] Key Concepts and the guides index link the new file
- [ ] `qa:guides` still passes because the family row is not added yet
- [ ] the guide does not invent a router, credential workflow, or
      persistence model

## Validation

- `effigy qa:docs`
- `effigy qa:guides`
- `git diff --check`

## Auto-Continuation

Yes, into card 040.

## Stop Conditions

- Stop if the guide claims addable descriptors for routes that do not
      export them.
- Stop if a family row or portable-feature token is added before
      examples exist.
- Stop if hosted OAuth is documented as realized.
