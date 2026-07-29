# Gemini ACP Bounded Write Contract Fit And Corpus

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

What exact contract and offline corpus are required for one Gemini ACP
bounded workspace text-write profile?

## Contract Fit

No shared type or lifecycle expansion is required.

- Contract 010 owns `WorkingResourceIo` and host-scoped resource leases.
- Contract 013 binds `ResourceAccess::ReadWrite`, exact lease agreement, and
  visible `AmbientHost` posture.
- Contract 015 owns independent ACP filesystem capability negotiation and
  reverse requests.
- Contract 017 defines bounded UTF-8 replacement or creation and keeps it
  independent from provider approval and process containment.
- Contract 023 keeps provider modes and optional sandboxing separate.
- Contract 029 limits the guarantee to exact qualified interface segments.
- Contract 033 keeps Gemini's ambient configuration agreement explicit.
- Contract 041 keeps provider permission requests independent from filesystem
  callback authority.

The selected profile remains ambient. A successful callback proves only that
the host completed one bounded write under its lease. It does not prove that
the Gemini process or descendants lacked other ambient authority.

## Exact Route

| Dimension | Selected value |
| --- | --- |
| integration | Gemini CLI |
| driver | `swallowtail.gemini.acp` |
| transport | ACP v1 over stdio |
| interface version | exact `0.51.0` |
| operation | interactive session |
| working resource | one required filesystem lease |
| access | `ResourceAccess::ReadWrite` |
| callback | whole UTF-8 create or replacement |
| provider mode | `auto_edit`; ACP mode id `autoEdit` |
| isolation | `AmbientHost` |
| configuration | explicit `Ambient` agreement |
| endpoint | Gemini Developer API |
| credential | caller-selected API-key reference |
| support authority | unchanged provider-supported route |

The existing read-only profile remains unchanged and continues to advertise
no write callback.

## Frozen Corpus

Deterministic fixtures must prove:

- the read profile advertises `writeTextFile: false`, requests a read lease,
  launches Plan Mode, and rejects any write request
- the write profile advertises `writeTextFile: true`, requests a `ReadWrite`
  lease, launches `auto_edit`, and accepts only returned mode `autoEdit`
- one in-root create and one replacement call the host service with bounded
  UTF-8 content
- read-only access, absolute-root mismatch, traversal, symlink escape,
  oversized content, wrong resource, wrong scope, wrong host, wrong session,
  malformed params, and unsupported callback reject before mutation
- provider permission callbacks remain rejected and cancel the active turn;
  write authority does not answer or approve them
- cancellation, disconnect, provider failure, and callback failure join the
  process and task before releasing the resource lease
- diagnostics expose no path, content, prompt, raw ACP payload, credential, or
  provider-private error

The existing host-service conformance owns canonical path and symlink
enforcement. Gemini fixtures prove the adapter's exact access, correlation,
dispatch, and cleanup mapping without reimplementing host path policy.

## Implementation Boundary

Card 109 may:

1. add an explicit read-only versus bounded-write Gemini ACP session profile
2. make capability, access policy, lease, invocation, negotiation, and
   callback dispatch depend on that profile
3. add exact `0.51.0` fixtures and focused conformance

It may not add a sandbox, containment record, generic filesystem API, shell
authority, permission approval, ambient-policy mutation, or support for an
unqualified newer ACP version.

## Sources

- [Gemini CLI `0.51.0` ACP filesystem service](https://github.com/google-gemini/gemini-cli/blob/8d951de3855750d5f8219d65ae22524b606133b6/packages/cli/src/acp/acpFileSystemService.ts)
- [Gemini CLI `0.51.0` ACP session manager](https://github.com/google-gemini/gemini-cli/blob/8d951de3855750d5f8219d65ae22524b606133b6/packages/cli/src/acp/acpSessionManager.ts)
- [Gemini CLI `0.51.0` write-file tool](https://github.com/google-gemini/gemini-cli/blob/8d951de3855750d5f8219d65ae22524b606133b6/packages/core/src/tools/write-file.ts)
- [ACP filesystem schema](https://github.com/agentclientprotocol/agent-client-protocol/blob/main/schema/schema.json)
