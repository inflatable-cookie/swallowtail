# 054 llama.cpp Attached Admission And Prepare

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../019-local-llama-cpp-attached.md`
Depends on: card 053

## Goal

Admit a llama.cpp attached instance through Contract 057 and reuse
`prepare_llama_cpp_attached`.

## Scope

1. Consumer-assembled catalog plus `admit_instance`.
2. Reuse `llama_cpp_attached_access_profile`: `LocalUnauthenticated`,
   `LocalCompute`, audience `llama.cpp.attached`,
   `IntegrationMaintainerSupported`, no `CredentialRef`. Do not use
   `llama_cpp_owned_access_profile`.
3. Opaque endpoint config ref. Exact opaque b9910/f5525f7e7 binding stays
   prepare-time. No unverified-newer.
4. Existing `prepare_llama_cpp_attached` still prepares after admission
   with a host `InstanceTargetRef`.
5. No URL-open, loopback, or device-code ports. Swallowtail does not start
   or stop the server.

## Out Of Scope

- refresh, 029 update, overlay (card 055)
- live `/health`, install, or process start
- hosted OAuth
- reading endpoint URLs into portable records
- owned-serving prepare

## Acceptance Criteria

- [x] admission writes `AdmittedInstanceRecord` through the store
- [x] stored records carry no secret bytes and no credential refs
- [x] missing browser ports do not fail this path
- [x] `prepare_llama_cpp_attached` still accepts the admitted identity and
      attached access profile

## Validation

- `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

Yes, into card 055.

## Stop Conditions

- Stop if endpoint URLs enter the store or diagnostics.
- Stop if Swallowtail starts or stops the attached server.
- Stop if the owned access profile is used on this row.
