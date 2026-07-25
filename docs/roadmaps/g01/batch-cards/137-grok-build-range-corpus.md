# 137 Grok Build Range Corpus

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../047-grok-build-maintained-acp-range.md`

## Objective

Determine the exact Grok Build releases and behavior milestones Swallowtail can
qualify before any production driver work.

## Governing Refs

- Research 030
- Contracts 005, 006, 012, 013, 015, 017, 023, 029, 032, 033, and 034
- roadmap g01.047

## Scope

1. Snapshot the official docs, ACP registry `1.0.0` Grok entry, npm release
   list, launcher packages, platform packages, integrity values, and exact
   published gaps.
2. Inspect exact artifacts without installing, updating, authenticating, or
   making provider requests.
3. Identify release milestones for:
   - `grok version`
   - `grok --no-auto-update agent stdio`
   - ACP initialization, wire/schema/SDK evidence, and selected lifecycle
   - exact model configuration
   - pre-existing delegated OAuth
   - restrictive read-only tools and permission mode
   - ambient configuration sources
   - local session retention
4. Freeze bounded raw ACP transcripts for the baseline, latest candidate, both
   sides of each milestone, every exclusion, and deliberate drift cases.
5. Publish one compatibility claim only for exact releases proven by the
   corpus. Permit later stable exact releases as unverified newer only when
   runtime drift still fails safely.
6. Keep package, executable, ACP wire, schema, SDK, registry, model, and
   instance versions independent.

## Boundaries

- no production driver
- no package installation or package-manager execution
- no provider login, account, credential, model request, or paid inference
- no inferred continuous interval from semver or release cadence
- no ACP v2 support claim
- no API-key route
- no sandbox or containment claim
- no Nucleus or Soundcheck edit

## Acceptance Criteria

- [x] every qualified release has exact artifact and behavior evidence; no
      release is qualified
- [x] unpublished and incompatible points remain explicit
- [x] observed milestones remain unmapped until exact transitions are known
- [x] no release is mislabeled as unverified newer without a qualified baseline
- [x] permissions do not imply isolation
- [x] registry presence does not imply installation, access, or support
- [x] normalized corpus and diagnostics contain no credential or private state
- [x] card 138 remains blocked because authentication needs new evidence

## Validation

- `cargo test -p swallowtail-protocol-acp` — 44 passed
- `cargo clippy -p swallowtail-protocol-acp --all-targets -- -D warnings` —
  passed
- `cargo fmt --all -- --check` — passed after formatting the new test
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings:
  12 warnings and 7 errors
- `git diff --check` — passed

## Evidence Required

- source URLs, access date, exact versions, integrity hashes, and gaps
- behavior milestone table
- frozen manifest and ACP corpus
- qualified, unverified-newer, and incompatible boundary tests
- explicit remaining auth, retention, protocol, and topology risks

## Stop Conditions

- exact artifacts cannot be inspected safely without installation, auth, or
  provider network access
- no release exposes the bounded read-only ACP subset
- a material lifecycle or authority difference lacks a shared contract
- source terms or source contradictions prevent an honest support claim

## Auto-Continuation

No. Exact delegated-auth evidence requires operator authorization or matching
maintained documentation.

## Outcome

Research 031 and seven offline protocol tests freeze the 111 published
`0.2.x` points as two exact runs around missing `0.2.48`. They inspect exact
`0.2.0` and `0.2.111` launcher and darwin-arm64 artifacts without installation,
credentials, authentication, or provider requests.

The two points differ in version-probe state mutation, bundled ACP SDK, model,
and reasoning options. Their transition releases are unknown, so no continuous
segment is inferred. Direct `0.2.111 --no-auto-update --version` is the only
inspected Contract 032-safe discovery candidate.

Both exact agents negotiate ACP v1 with read-only client callbacks but require
authentication before session use. Current public documentation expects
`cached_token` or `xai.api_key`; the exact unauthenticated artifacts advertise
only `grok.com`. The bundled current docs also disprove the planned bounded
read-only claim: plan mode permits shell and subagent bypasses, the `dontAsk`
CLI value is not enforcing, ambient project configuration is loaded, and
hooks may fail open.

No release is qualified. Spec 003 captures the missing activation-only
delegated-auth decision. Card 138 is blocked on explicit authorization for a
no-prompt existing-credential probe or maintained documentation matching the
artifact.

## Evidence

- Research 031 and Spec 003
- exact release, artifact, signature, state, protocol, model, and auth fixtures
- seven passing Grok protocol corpus tests
- 44 passing affected protocol-crate tests
- warnings-denied focused clippy
