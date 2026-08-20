# 040 First-Proof Route Guide Amendments

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../014-connection-lifecycle-consumer-path.md`
Depends on: card 039

## Goal

Amend the three first-proof route guides so a consumer can list, admit,
and refresh those routes through Contract 057 before the existing
prepared facade.

## Scope

1. Amend `docs/guides/anthropic-direct-prepared-integration.md` for
   `anthropic.messages`: hosted addable descriptor, secret API-key field
   with environment name `ANTHROPIC_API_KEY`, opaque endpoint config,
   Credential host service, `CredentialRef` collection, no URL-open
   ports, subject Absent, overlay keys `anthropic` catalogue rows,
   then `prepare_anthropic_direct`.
2. Amend `docs/guides/codex-prepared-integration.md` for
   `codex.app-server` only: installed addable descriptor, no credential
   field, ChatGPT cached local login, Process host service, binary path
   and environment config, 029/032 update observation, unmarked
   catalogue rows. Do not claim `codex.exec` has an addable descriptor.
3. Amend `docs/guides/ollama-attached-prepared-integration.md` for
   `ollama.attached`: local-runtime addable descriptor, no credential,
   Network host service, opaque endpoint config, no install/start/pull,
   model tag and digest stay prepare-time, 029 update with 032
   unobserved unless an executable is supplied, unmarked catalogue rows,
   then `prepare_ollama_attached`.
4. Keep each guide's existing prepared-facade example as the canonical
   route-map example. Link the feature guide. Point at the forthcoming
   057 examples without claiming they already compile.
5. Do not amend other route guides. Do not change route-map coverage
   states.

## Out Of Scope

- compiling examples, checker token, and guide-map family (card 041)
- hosted OAuth
- new adapter descriptors
- replacing `prepared_direct.rs`, `prepared_discovery.rs`, or
      `prepared_attached.rs` as the route-map examples

## Acceptance Criteria

- [ ] each first-proof route guide names addable id, topology, host
      service, credential or its absence, config fields, refresh,
      subject, update, overlay, and the prepare handoff
- [ ] `codex.exec` is not documented as addable
- [ ] remaining production route guides are unchanged
- [ ] no secret bytes, endpoint URLs, or ChatGPT tokens appear as
      portable records
- [ ] `qa:guides` still passes

## Validation

- `effigy qa:docs`
- `effigy qa:guides`
- `git diff --check`

## Auto-Continuation

Yes, into card 041.

## Stop Conditions

- Stop if a non-first-proof route is documented as addable.
- Stop if overlay invents a Codex or Ollama `provider_id`.
- Stop if 047 `Ready` / `NotReady` is described as overlay-mutable.
