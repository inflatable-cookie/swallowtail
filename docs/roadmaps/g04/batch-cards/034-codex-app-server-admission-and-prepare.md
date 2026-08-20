# 034 Codex App-Server Admission And Prepare

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../012-installed-codex-app-server.md`
Depends on: card 033

## Goal

Admit a Codex app-server instance through Contract 057 and reuse
`prepare_codex(AppServer)`.

## Scope

1. Consumer-assembled catalog plus `admit_instance`.
2. ChatGPT subscription uses `codex_chatgpt_subscription_access_profile`:
   no `CredentialRef`, no API-key collection.
3. No URL-open, loopback, or device-code ports. Do not extract ChatGPT
   tokens.
4. Existing `prepare_codex(AppServer)` still prepares after admission.

## Out Of Scope

- refresh, 029/032 update, overlay (card 035)
- live login or install
- hosted OAuth
- reading login files into portable records

## Acceptance Criteria

- [ ] admission writes `AdmittedInstanceRecord` through the store
- [ ] stored records carry no secret bytes
- [ ] missing browser ports do not fail this ChatGPT path
- [ ] `prepare_codex(AppServer)` still accepts the admitted identity and
      access profile

## Validation

- `effigy validate:focused swallowtail-adapter-codex swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 035.

## Stop Conditions

- Stop if ChatGPT tokens enter the store or diagnostics.
- Stop if this route opens a browser.
