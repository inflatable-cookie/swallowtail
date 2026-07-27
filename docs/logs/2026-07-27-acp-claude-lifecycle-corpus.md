# ACP And Claude Lifecycle Corpus

Date: 2026-07-27

## Change

Card 052 completes the protocol-currentness batch for roadmap g02.017.

Research 038 pins ACP stable schema `v1.20.0`, source commit
`5e89c71497fe07dd4ae633c181a17224f4a8956d`, and stable schema SHA-256
`92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796`.
The selected lifecycle shape is unchanged across stable schema `1.18.0`
through `1.20.0`. Historical Gemini and Kimi pins remain untouched.

The protocol corpus adds independent close-only, delete-only, omitted, null,
success, and error evidence. Stdio and explicit remote transports use the same
bounded `swallowtail_protocol_acp::Message` records. Generic ACP delete remains
`HistoryRemoved`.

The Claude corpus freezes handlers, tests, ACP SDKs, and Agent SDKs at every
qualified behavior milestone plus tag-only `0.58.0`.

## Decision

Claude native close:

- requires an active session
- interrupts and releases in-memory resources
- preserves persistent history
- rejects missing and repeated close

Claude delete:

- tears down an active target before deletion
- deletes inactive targets directly
- removes the primary local transcript and sibling session directory
- qualifies `ProviderDataDeleted` with `ProviderDefinedDescendants`
- rejects missing and repeated delete

The claim covers Claude Agent harness state only. It excludes secure erasure,
Anthropic API service data, account analytics, logs, and backups.

Published Claude Agent `0.62.0` remains unverified-newer. Its current source
does not extend the guaranteed `0.53.0..=0.61.0` window.

## Validation

- ACP protocol, remote transport, and Claude Agent suites: 87 tests pass
- `effigy check:rust`: pass
- `effigy format:check`: pass
- docs, Northstar, and diff checks: pass
- `effigy doctor`: unchanged baseline of 25 findings
  (17 warnings, 8 errors)

## Next

Card 053 is ready. Map qualified native close into Claude handle cleanup and
add one prepared inactive-session delete operation. Keep exact capability,
version, access, effect, and cleanup truth.
