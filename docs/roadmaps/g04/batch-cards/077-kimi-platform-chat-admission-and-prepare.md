# 077 Kimi Platform Chat Admission And Prepare

Status: ready
Owner: Tom
Created: 2026-08-21
Milestone: `../024-hosted-api-key-kimi-platform-chat.md`
Depends on: card 076

## Goal

Admit a `kimi-platform.chat` instance, collect its API key as an opaque
`CredentialRef`, and reuse `prepare_kimi_platform_direct` after admission.

## Scope

1. Admit one consumer-selected descriptor through Contract 057 with an opaque
   endpoint config ref.
2. Run API-key collection without URL-open, loopback, device-code, or login
   helper ports.
3. Store only the submitted `CredentialRef`, never secret bytes.
4. Add the route-local handoff from the admitted endpoint and credential refs
   into `KimiPlatformPreparationInput`.
5. Keep the exact `api.moonshot.ai` audience, pay-as-you-go metering,
   provider-supported authority, and `kimi-platform-chat-2026-07-21` facade.
6. Fail closed on route, endpoint-ref, credential-ref, audience, or host drift.

## Out Of Scope

- refresh, subject, catalogue, inference, overlay, or 047 assembly (card 078)
- resolving endpoint or credential values into portable records
- provider prompts, live catalogue, allowance spend, account, or billing work
- retries, tools, reusable sessions, provider state, or fallback

## Acceptance Criteria

- admission writes an `AdmittedInstanceRecord` for the exact route
- API-key completion writes one redacted `CredentialRef`
- missing browser-oriented ports do not fail API-key collection
- preparation retypes admitted refs without copying their values
- `prepare_kimi_platform_direct` accepts the admitted identity and access
  profile only after admission

## Validation

- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-runtime swallowtail-host-local`
- `git diff --check`
- `effigy package:api` if public API changes

## Auto-Continuation

Yes, into card 078.

## Stop Conditions

- Stop if secret bytes or the endpoint URL enter portable state or diagnostics.
- Stop if preparation moves before admission.
- Stop if another Kimi audience or route can substitute.
