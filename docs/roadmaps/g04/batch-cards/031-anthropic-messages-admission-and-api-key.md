# 031 Anthropic Messages Admission And API-Key Collection

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../011-hosted-api-key-anthropic-messages.md`
Depends on: card 030

## Goal

Admit an Anthropic Messages instance through Contract 057 and collect the
API key as `CredentialRef`.

## Scope

1. Consumer-assembled catalog plus `admit_instance`.
2. API-key collection through the 057 loop. Host supplies `CredentialRef`.
3. No URL-open, loopback, or device-code ports.
4. Existing `prepare_anthropic_direct` still prepares after admission.

## Out Of Scope

- refresh, subject, overlay (card 032)
- live Messages calls
- reading `ANTHROPIC_API_KEY` into portable records
- changing the prepared facade contract

## Acceptance Criteria

- [ ] admission writes `AdmittedInstanceRecord` through the store
- [ ] complete writes `CredentialRef`, never secret bytes
- [ ] missing browser ports do not fail API-key collection
- [ ] `prepare_anthropic_direct` still accepts the admitted identity and
      access profile

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 032.

## Stop Conditions

- Stop if secret bytes enter the store or diagnostics.
- Stop if this route opens a browser.
