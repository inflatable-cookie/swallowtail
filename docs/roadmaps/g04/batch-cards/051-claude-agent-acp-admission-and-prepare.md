# 051 Claude Agent ACP Admission And Prepare

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../018-installed-claude-agent-acp.md`
Depends on: card 050

## Goal

Admit a Claude Agent ACP instance through Contract 057 and reuse
`prepare_claude_agent`.

## Scope

1. Consumer-assembled catalog plus `admit_instance`.
2. Local subscription uses `LocalUnauthenticated`,
   `SubscriptionAllowance`, audience `api.anthropic.com`,
   `IntegrationMaintainerSupported`, no `CredentialRef`.
3. No URL-open, loopback, or device-code ports. Do not extract keychain
   bytes.
4. Existing `prepare_claude_agent` still prepares after admission.
5. API-key pay-as-you-go stays a separate explicit profile, not this
   addable row.

## Out Of Scope

- refresh, 029/032 update, overlay (card 052)
- live login or install
- hosted OAuth
- reading login files into portable records

## Acceptance Criteria

- [ ] admission writes `AdmittedInstanceRecord` through the store
- [ ] stored records carry no secret bytes and no credential refs
- [ ] missing browser ports do not fail this subscription path
- [ ] `prepare_claude_agent` still accepts the admitted identity and
      subscription access profile

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 052.

## Stop Conditions

- Stop if keychain bytes or API keys enter the store or diagnostics.
- Stop if this route opens a browser.
