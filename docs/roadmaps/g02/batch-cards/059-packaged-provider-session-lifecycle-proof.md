# 059 Packaged Provider Session Lifecycle Proof

Status: superseded
Owner: Tom
Created: 2026-07-26
Updated: 2026-07-30
Milestone: `../019-provider-session-lifecycle-acceptance-and-handoff.md`

## Objective

Prove supported and unsupported provider-session lifecycle behavior from
extracted package artifacts, then refresh the held local soak baseline.

## Governing Refs

- Contracts 011, 029, 036-038
- card 058
- package-family and provider-facade validation tasks

## Scope

1. Build a clean local 23-package candidate without publication.
2. Execute Codex archive/restore/delete, Claude close/delete, and OpenCode
   delete through extracted prepared and low-level APIs.
3. Prove explicit unsupported behavior for Kimi and Gemini ACP.
4. Prove not-applicable routes expose no fabricated management operation.
5. Run existing provider-wide, Nucleus, Soundcheck, and packaged Codex
   deterministic checks.
6. Record additive public API and guaranteed-behavior classification.
7. Supersede the prior local soak candidate only after all gates pass.

## Acceptance Criteria

- [x] extracted artifacts pass every supported lifecycle fixture
- [x] unsupported actions stop before provider effects
- [x] all 22 route postures match card 058
- [x] existing prepared route and consumer proofs remain green
- [ ] source, parent, archives, checksums, and evidence reproduce
- [x] no credential, provider call, push, tag, registry, workflow, owner, or
      release mutation occurs

## Validation

- package prepare and verify selectors
- packaged provider-wide facade and management suites
- packaged Nucleus, Soundcheck, and Codex deterministic proofs
- public API, MSRV, docs, content, route, and repository QA

## Stop Conditions

- a package omits required management APIs
- an unsupported route dispatches
- package proof needs live authentication or provider mutation
- source provenance is dirty or non-reproducible

## Auto-Continuation

No. The later provider-wide package proof superseded retained-candidate
replacement.

## Checkpoint

A clean transient snapshot passed the complete candidate path. The package
assembler first exposed one real topology defect: Claude Agent's remote ACP
lifecycle dev dependency was absent from the local registry patch set. The
patch set is now centralized and includes all seven internal packages used
while assembling or testing extracted artifacts.

The new package lifecycle gate:

- checks the source-bundled 22-route matrix
- proves exactly Codex, Claude Agent, and OpenCode extracted adapters declare
  `ProviderSessionManagement`
- proves Kimi, Gemini, and all not-applicable adapters expose no fabricated
  management role
- executes nine focused extracted-artifact suites covering provider-neutral
  conformance, exact ranges, production prepared mappings, remote ACP, and
  failure-before-effect behavior
- emits checksummed lifecycle evidence for every route posture

Transient evidence passed:

- 23 package archives assembled and reproduced byte-for-byte
- source bundle, parent, package lists, and checksums verified
- 20 provider-facade suites passed across all 22 routes
- 33 focused provider-lifecycle tests passed
- Nucleus: 15 deterministic tests passed; two live tests ignored
- Soundcheck: six deterministic tests passed; one live test ignored
- packaged Codex: 105 tests passed
- no credential or provider call occurred

## Disposition

Card 136 later assembled all 23 package archives, compiled the extracted
workspace, and executed 14 packaged lifecycle suites across all five
management adapters. That broader proof preserves the behavior evidence this
card sought.

The only unclosed criterion here was refreshing the retained `0.1.0`
publication candidate from newer canonical history. Registry publication is
no longer part of the active Swallowtail runway: the operator requires months
of working-application evidence before reopening it. Repeating the large
candidate gate after every additive change would not improve that evidence.

The transient checkpoint remains valid historical evidence. Card 059 is
superseded by card 136 and no candidate replacement is queued.
