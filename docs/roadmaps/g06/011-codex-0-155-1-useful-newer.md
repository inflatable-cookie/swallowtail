# g06.011 Codex 0.155.1 Useful Newer

Owner: Tom
Created: 2026-09-21
Depends on: Contract 029; Research 311, 331/333; completed g05.061
Vision tags: route currentness, Codex, useful-newer

## Outcome

Qualify the Codex family (exec and app-server) from the qualified ceiling
`0.154.0` through official npm/GitHub stable `0.155.1`. Identity names the
segment shape first; both axes rise together.

## Why It Matters

g05.061 left the Codex ceiling at `0.154.0`. Official stable moved to
`0.155.1`. AllowUnverified already admits later points; it is not permission to
leave the current official stable unverified.

## Ready-State Rubric

- [x] Official npm `@openai/codex@latest` and GitHub release `rust-v0.155.1`
      agree.
- [x] Both published hops `0.155.0` and `0.155.1` were retrieved and
      inventoried.
- [x] The identity record names a compatible extension.
- [x] The claim raises both axes together.

## Decisions

- One family: `codex.exec` and `codex.app-server` rise together.
- Compatible extension: keep baselines, claim ids, behavior revisions,
  `AllowUnverified`, and the historical unpublished gaps.
- `0.154.1` was never published and joins the interior gap set alongside
  `0.149.2`, `0.150.2`, `0.151.1`, and `0.152.2`.
- Later `UnverifiedNewer` is the next unpublished stable. The `alpha` dist-tag
  is not stable authority.
- Host `codex` is absent from `PATH`. Missing host is not a gap. Do not
  install, and do not execute downloaded binaries. No provider prompt or live
  session.
- Do not edit the Next Task pointer, generation status, handoffs, or northstar
  loop state.

## Dispatch manifest

- **State:** ready; one family useful-newer lane entering at review.
- **Owned mutable paths on revision:** this PR's existing Codex selection
  source and lifecycle selection, Codex identity and compatibility-corpus tests
  and fixtures under `crates/swallowtail-adapter-codex/`, the Codex prepared
  guide, the affected provider and activity matrix rows, the CHANGELOG
  `[Unreleased]` entry, the lane's research record and its indexes/logs.
- **Excluded:** other families, SDK or ACP native pins, workflow, dependency,
  release, provider mutation, the Next Task pointer, `docs/roadmaps/README.md`,
  generation runway and lifecycle projection.
- **Escalation:** operator via Chatterbox for scope, number, or contract
  questions; Queue coordinator for mechanical blockers.

## Work

1. Freeze identity for `0.155.1` with the per-hop artifact and inventory
   evidence. No production claim.
2. Recheck official stable. If it moved before the claim landed, extend hops
   under Contract 029 In-Run Latest Movement.
3. Apply the compatible-extension claim on both axes.
4. Run focused adapter validation and the named docs gates.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Both axes qualify through `0.155.1` | One axis is left behind | selection claims name both raised ceilings |
| The gap set is truthful | A newly published hop is treated as a gap, or a never-published hop is treated as compatible | gap set includes `0.154.1` with publication evidence |
| Selected surfaces are unchanged | A selected exec flag or app-server method moved | frozen per-hop inventories and compatibility corpus |
| No provider or host mutation | The CLI is installed or an artifact executed | absence from `PATH` recorded; artifacts hashed only |

## Stop conditions

Stop and ask if a hop changes selected argv, app-server methods, authority, or
the published schema; if the npm and GitHub channels disagree; or if one axis
must stay behind.

## Evidence

The lane's research record (renumbered from the colliding `331` on admission).
Fixtures under `crates/swallowtail-adapter-codex/tests/codex_0_155_1_identity/`.
Probe artifacts live outside the repository.

## External PR Review Entry Authority

Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the
remaining externally authored currentness PRs through review entry, accepted the
pilot-first sequence, attested Grok 4.6 as the author model for the cloud
sessions, directed that the Claude Agent ACP PR be held until last with its
contract change flagged, and directed that the failing Qwen lane be admitted
and repaired through the ordinary revision path.

The external work and its evidence are submitted claims, not independent
acceptance. Tom declared Grok 4.6 as the external author model; canonical
declaration: `xai/grok-4.6`, GitHub author `betterthanclay`, authoring system
Cursor cloud driven by Grok Bot.

Queue starts an independent reviewer, not a worker. Changes requested authorize
one ordinary revision loop in the same workspace and branch, scoped to this
Codex qualification and its existing changed surfaces. Queue owns merge and
lifecycle closeout. Intake preparation may publish this planning and merge it
into the existing branch; the exclusions above constrain revision workers, not
this operator-approved planning preparation or hook-owned closeout.

Canonical handoff: `docs/handoffs/20260921-g06-011-external-pr-review-entry.md`.

Preserve the external branch and workspace after completion. Other external PRs
are not dispatched by this approval. Research numbers `331` and `332` are
retained by earlier lanes; this lane takes the next free number.
