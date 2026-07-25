# 145 Claude Agent ACP Range Conformance And Closeout

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../048-post-grok-hold-provider-coverage-continuation.md`

## Objective

Prove the Claude Agent ACP range through unchanged public profiles and close
roadmap 048 without disturbing the held Grok lane.

## Governing Refs

- Research 032
- cards 142-144
- Claude Agent ACP range contracts and records
- Contract 011
- roadmap g01.048

## Scope

1. Apply the exact profile selected by card 143 without importing Claude
   identity into shared conformance.
2. Prove qualified baseline, intermediate milestone segments,
   latest-qualified, permitted unverified-newer, and rejected versions.
3. Prove local and remote-authoritative topology.
4. Prove exact model binding, public-API access, ambient configuration,
   ambient process isolation, read-tool policy, permission rejection, failure,
   cancellation, deadline, disconnect, cleanup, and redaction.
5. Prove terminal auth, subscription login, provider switching, writes, shell,
   sandbox, and implicit fallback stay unavailable.
6. Run full repository QA.
7. Return to a deliberate generation checkpoint.

## Acceptance Criteria

- [x] unchanged profiles remain provider-neutral
- [x] version, topology, access, and lifecycle evidence match the frozen route
- [x] unverified-newer execution remains visible and never becomes qualified
- [x] subscription auth and sandbox exclusions remain explicit
- [x] live probes remain separately gated
- [x] full QA passes or failures are recorded honestly
- [x] roadmap and front-door currentness close coherently
- [x] one deliberate generation checkpoint remains next

## Validation

- focused conformance tests
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- shared profile changes hide provider-specific behavior
- either topology needs uncontracted authority
- a newer-version route silently changes access, model, configuration, or
  lifecycle behavior
- full QA exposes a contract-level defect

## Auto-Continuation

No. Return to a deliberate generation checkpoint.

## Outcome

The unchanged `LongLivedAcpHarness` profile covers the portable lifecycle,
process, callback, topology, redaction, and no-fallback boundaries. No
provider name or Claude-specific behavior entered `swallowtail-testkit`.

Adapter-local conformance proves:

- qualified baseline, intermediate revisions, latest-qualified, excluded,
  incompatible, prerelease, and visible unverified-newer points
- every private behavior milestone plus stable newer execution under local and
  remote-authoritative host identities
- exact public-API-key audience, model, ambient configuration, ambient-host
  isolation, read-only resource authority, and provider-native tool selection
- no write, load, resume, consumer tool-call, provider-network, external-search,
  filesystem-boundary, or sandbox capability
- terminal-auth advertisement rejection with joined resource and credential
  cleanup
- permission rejection, cancellation, deadline, disconnect, model drift,
  access mismatch, output redaction, and credential-last cleanup

Roadmap 048 is complete. Held Grok cards 138-141 remain intact and do not
become Claude or generation-closeout prerequisites.

## Evidence

- `cargo test -p swallowtail-adapter-claude-agent` — 14 passed
- `cargo clippy -p swallowtail-adapter-claude-agent --all-targets -- -D warnings`
  — passed
- `effigy qa` — passed
- full inventory: 658 tests, 654 passed and four gated probes ignored
- `effigy doctor` — unchanged inherited 19 findings: 12 warnings, 7 errors
- no live account, provider request, package installation, or container used
- card 146 is ready as the generation-disposition checkpoint
