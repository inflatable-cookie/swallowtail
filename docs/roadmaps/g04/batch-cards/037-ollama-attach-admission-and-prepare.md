# 037 Ollama Attach Admission And Prepare

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../013-local-ollama-attach.md`
Depends on: card 036

## Goal

Admit an Ollama attach instance through Contract 057 and reuse
`prepare_ollama_attached`.

## Scope

1. Consumer-assembled catalog plus `admit_instance`.
2. Local-unauthenticated access: no `CredentialRef`, no sign-in loop.
3. Opaque endpoint config ref. Model tag and digest stay prepare-time
   identities, not 057 admission fields.
4. Existing `prepare_ollama_attached` still prepares after admission.

## Out Of Scope

- refresh, 029 update, overlay (card 038)
- live runtime start, pull, or install
- hosted OAuth
- reading endpoint URLs into portable records

## Acceptance Criteria

- [ ] admission writes `AdmittedInstanceRecord` through the store
- [ ] stored records carry no secret bytes and no credential refs
- [ ] `prepare_ollama_attached` still accepts the admitted identity and
      local-unauthenticated access profile

## Validation

- `effigy validate:focused swallowtail-adapter-ollama swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 038.

## Stop Conditions

- Stop if endpoint URLs enter the store or diagnostics.
- Stop if Swallowtail starts the Ollama process.
