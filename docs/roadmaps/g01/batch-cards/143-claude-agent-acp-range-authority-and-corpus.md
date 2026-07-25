# 143 Claude Agent ACP Range Authority And Corpus

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../048-post-grok-hold-provider-coverage-continuation.md`

## Objective

Qualify or split the Claude Agent ACP `0.52.0..=0.61.0` candidate range and
freeze the first deterministic harness corpus before production implementation.

## Governing Refs

- Research 032
- Contracts 005-012, 014, 015, 017, 023, 029, 032, and 033
- card 142
- roadmap g01.048

## Scope

1. Snapshot every published semantic version in the candidate range.
2. Inspect exact package, dependency, tagged-source, changelog, and safe
   version-observation evidence at `0.52.0`, `0.53.0`, `0.54.0`, `0.60.0`,
   and `0.61.0`.
3. Keep wrapper, ACP SDK, Agent SDK, nested native binary, ACP wire, provider
   API, and model versions separate.
4. Qualify or split exact behavior segments for:
   - installed adapter discovery
   - ACP v1 initialization
   - new session and one active prompt
   - exact model binding
   - text, reasoning, read-tool, permission, plan, usage, and terminal events
     supported by exact evidence
   - cancellation, deadline, disconnect, failure, and process close
5. Freeze the first access profile:
   - Anthropic public API
   - one host-approved API-key lease
   - no terminal auth, Claude subscription, login, logout, or credential
     mutation
6. Freeze explicit `Ambient` configuration and `AmbientHost` isolation.
7. Build independent raw ACP fixtures from exact tagged source and maintained
   mock-backed tests.
8. Add only the shared records required by the frozen subset.
9. Rebaseline card 144 to exact qualified segments and fixtures.

## Acceptance Criteria

- [x] every candidate release and known publication gap is explicit
- [x] baseline, milestones, latest-qualified point, exclusions, and
      unverified-newer posture are evidence-backed
- [x] all nested version axes remain distinct
- [x] public-API key access does not become terminal or subscription auth
- [x] configuration, process isolation, tool policy, and sandboxing remain
      independent
- [x] deterministic fixtures cover every supported lifecycle edge and drift
- [x] no live account is required by default tests
- [x] any missing shared contract is promoted before dependent fixtures
- [x] card 144 names exact qualified segments before becoming ready

## Validation

- focused record, protocol, and fixture tests
- warnings-denied focused clippy
- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- exact source or artifacts do not support safe installed observation
- nested native-binary replacement cannot be excluded or represented honestly
- session creation requires terminal auth or credential mutation
- exact model binding would require implicit provider or model fallback
- required composite-version, access, configuration, or lifecycle authority
  lacks a durable contract
- deterministic evidence requires live provider access

## Auto-Continuation

Yes, only when exact qualified segments, contracts, records, and deterministic
fixtures are complete and card 144 is rebaselined to ready.

## Outcome

The candidate range split:

- `0.52.0` is incompatible because its `--cli --version` path reports the
  wrapper version and it predates correlated tool-call/permission ordering
- qualified baseline is `0.53.0`
- qualified latest is `0.61.0`
- unpublished `0.58.0` is excluded
- stable versions above `0.61.0` remain executable only as visible
  unverified-newer points

Four private revisions preserve additive differences:

1. `0.53.0` — baseline v1
2. `0.54.0..=0.59.0`, excluding `0.58.0` — session-config v2
3. `0.60.0` — provider-capability v3
4. `0.61.0` — steering-metadata v4

All 11 published candidate points are frozen with tag, ACP SDK, Agent SDK, and
native Claude Code evidence. Network-denied, empty-home probes confirm exact
native versions `2.1.191..=2.1.217` with no state creation. The production
range begins at nested native `2.1.195`.

Independent fixtures cover initialization milestones, exact model
confirmation, read-tool selection, reasoning, usage, permission rejection,
cancellation, access failure, model drift, disconnect, and redaction. No live
account is required.

No new shared contract is needed. The existing ACP, access, ambient harness,
version, installed-observation, and configuration contracts cover the subset.

## Evidence

- all 52 `swallowtail-protocol-acp` tests pass, including eight Claude Agent
  ACP corpus tests
- focused warnings-denied Clippy, formatting, docs QA, Northstar QA, and diff
  checks pass
- Doctor remains at the inherited 19 findings with no new oversized-file debt
- exact source and native artifact evidence is recorded in
  `release-corpus.json`
- card 144 is rebaselined and ready
