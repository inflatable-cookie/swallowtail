# 065 Kimi Code Successor Currentness And Fixture Delta

Status: complete
Owner: Tom
Updated: 2026-07-21
Milestone: `../018-kimi-code-acp-portability-proof.md`

## Objective

Repair the Kimi `0.28.1` source identity, prove it is the maintained Kimi Code
successor, and freeze its exact executable and upgrade boundary before adapting
provider behavior.

## Governing References

- `../../../contracts/015-acp-v1-negotiation-and-client-callbacks.md`
- `../../../contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `../../../research/006-kimi-code-acp-currentness-and-persistent-session-evidence.md`
- `../../../research/009-native-macos-kimi-containment-and-successor-delta.md`
- `../../../research/010-kimi-code-successor-and-artifact-currentness-repair.md`
- `056-kimi-acp-protocol-fixtures.md`

## Scope

- pin one exact maintained Kimi Code release, artifact, source revision, ACP
  schema, SDK, wire, and access route
- compare successor new, load, resume, replay, prompt, permission, write,
  cancellation, disconnect, and close behavior with the legacy corpus
- freeze the documented stdin/stdout/stderr split and initialization
  capability matrix
- distinguish ACP `0.23`, SDK package, schema release, wire version, and Kimi
  artifact version from current tagged evidence
- replace or retain fixtures assertion by assertion from current tagged source
  and deterministic transcripts
- freeze isolated `KIMI_CODE_HOME`, delegated login, built-in tool, subagent,
  background-work, MCP, plugin, model-selection, and sign-in exclusions
- record the exact executable packaging and signing inputs required by the
  selected macOS helper proof
- define an explicit upgrade gate; disable provider self-upgrade and reject
  unapproved executable versions before session work

## Out Of Scope

- process containment implementation
- production Kimi driver mapping
- live account, OAuth mutation, inference, plugin, MCP, or shell execution
- consumer repository changes

## Acceptance Criteria

- [x] no legacy version, source hash, schema, or session behavior is silently
      attributed to the successor
- [x] shared ACP framing changes only when current wire evidence requires it
- [x] load, resume, replay, write, and cancellation differences stay explicit
- [x] the successor claims no separate resume unless current tagged evidence
      advertises it
- [x] local shell execution is not mistaken for ACP terminal callback
      mediation
- [x] image, embedded-context, MCP, session-list, unstable model-selection,
      plugin, background, and subagent capabilities are excluded or separately
      proven
- [x] membership OAuth and API-key access remain separate configured instances
- [x] the pinned executable and isolated state-root requirements are usable by
      card 057 without exposing credentials
- [x] default fixtures require no Kimi binary, account, or network request
- [x] no floating `latest`, ambient `PATH`, or `kimi upgrade` behavior can
      change the supported executable silently

## Evidence

- official releases identify `0.28.1` as the latest maintained TypeScript Kimi
  Code successor; the docs changelog's `0.26.0` entry is older
- annotated tag object `0032545b...` peels to source commit `efacf045...`; the
  previous fixture mislabeled the tag object as a commit
- current tagged source retains separate load-with-replay and resume-without-
  replay, write callbacks, native turn cancellation, and connection-owned close
- ACP `schema-v1.19.1` remains current; its tag object and source commit are now
  distinct fixture identities
- the exact official arm64 archive and extracted Mach-O digests are pinned
- the upstream Developer ID signature is valid but lacks App Sandbox and
  inheritance entitlements; card 057 must re-sign and record the deployment
  helper digest without weakening runtime or containment checks
- `KIMI_CODE_HOME` and `KIMI_CODE_NO_AUTO_UPDATE=1` are frozen; ambient home,
  ambient `PATH`, floating release, self-update, and upgrade-command paths fail
  closed

## Validation Result

- all 22 `swallowtail-protocol-acp` tests pass, including two new successor,
  artifact, exclusion, and upgrade-gate assertions
- focused warnings-denied Rust lint and all-target compile pass
- full repository QA passes with 259 tests; two installed/live probes remain
  gated and ignored
- formatting, docs, Northstar, and diff checks pass
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors

## Validation

- deterministic successor protocol fixtures
- focused ACP and fixture tests
- focused clippy and all-target compile
- `cargo fmt --all -- --check`
- `git diff --check`

## Stop Conditions

- current tagged source and official schema disagree materially
- exact successor ACP behavior requires a live account to freeze
- packaging or access authority would require credential extraction

## Auto-Continuation

No. Card 065 is complete. Research 012 and card 066 later clear card 058 for
explicit ambient execution.
