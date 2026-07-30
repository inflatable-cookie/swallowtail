# Grok Authentication Gate And Discovery Start

Date: 2026-07-30
Status: completed

## Changed

- revalidated current xAI CLI, ACP, authentication, npm, and registry evidence
- matched installed signed Grok Build `0.2.114` to the public darwin-arm64
  artifact
- proved existing `cached_token` activation and empty session allocation
  without login, API key, prompt, model request, or tool request
- promoted the narrow activation rule into Contract 015
- archived Spec 003
- promoted the held Grok backlog into roadmap g02.043 and cards 142-145
- added the `swallowtail-adapter-grok` discovery crate
- added exact stable-channel version parsing and source-revision enforcement
- added one exact qualified point plus explicit unverified-newer stable posture

## Live Evidence

Exact `0.2.114` initialization advertised `cached_token` and `grok.com`, with
`cached_token` as default. One headless activation succeeded against the
existing subscription login. The authentication file hash did not change.

`session/new` created one normal empty durable local session plus Grok-owned
bookkeeping and bundled guide cache. Swallowtail did not delete ambient state
by path convention.

The authentication response carried provider-private account metadata. It was
discarded. No raw account, entitlement, credential, host path, or session id
entered repository state.

## Version Boundary

- guaranteed: exact `0.2.114`
- historical incompatible: inspected `0.2.0` and `0.2.111`
- uninspected older: `0.2.112` and `0.2.113`
- permitted only as unverified newer: later stable observations
- incompatible: alpha, prerelease, malformed, or wrong exact source revision

The npm launcher `latest`, platform `latest`, alpha tag, ACP registry version,
installed channel, wire version, and Swallowtail behavior range remain
separate.

## Validation

- 10 focused authenticated Grok protocol tests pass
- 16 combined Grok discovery and protocol tests pass
- focused warnings-denied Grok clippy passes
- Effigy docs QA passes
- Cargo metadata reports 24 unique workspace packages including Grok
- `git diff --check` passes
- no broad workspace or package suite ran

Effigy has no `qa:metadata` selector; the bounded `cargo metadata --no-deps`
check supplied package-graph evidence instead.

## Closeout

Cards 143-145 and roadmap g02.043 are complete. Discovery-owned identity now
reaches exact prepared interactive and structured operations with cached-token
activation, durable local retention, activity, cancellation, deadlines, and
joined cleanup.

No live provider prompt is authorized by this batch.

## Next

See the Grok structured conformance closeout. Hold at the g02 stabilization
checkpoint.
