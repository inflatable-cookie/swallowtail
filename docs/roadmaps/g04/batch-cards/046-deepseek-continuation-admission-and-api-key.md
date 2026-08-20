# 046 DeepSeek Continuation Admission And API-Key

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../016-hosted-api-key-deepseek-continuation.md`
Depends on: card 045

## Goal

Admit a DeepSeek continuation instance through Contract 057 and collect
the API key as `CredentialRef`.

## Scope

1. Consumer-assembled catalog plus `admit_instance`.
2. API-key collection through the 057 loop. Host supplies `CredentialRef`.
3. No URL-open, loopback, or device-code ports.
4. Existing `prepare_deepseek_direct` still prepares after admission.

## Out Of Scope

- refresh, subject, overlay (card 047)
- live Open Platform calls
- reading environment values into portable records
- changing the prepared facade contract

## Acceptance Criteria

- [ ] admission writes `AdmittedInstanceRecord` through the store
- [ ] complete writes `CredentialRef`, never secret bytes
- [ ] missing browser ports do not fail API-key collection
- [ ] `prepare_deepseek_direct` still accepts the admitted identity and
      access profile

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 047.

## Stop Conditions

- Stop if secret bytes enter the store or diagnostics.
- Stop if this route opens a browser.
