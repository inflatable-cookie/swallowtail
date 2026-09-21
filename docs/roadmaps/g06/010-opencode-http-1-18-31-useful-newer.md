# g06.010 OpenCode HTTP 1.18.31 Useful Newer

Owner: Tom
Created: 2026-09-21
Depends on: Contract 029; Research 292, 304, 331/332; completed g05.051
Vision tags: route currentness, OpenCode HTTP, useful-newer

## Outcome

Qualify the `opencode.server` / `opencode.http` family from the qualified
ceiling `1.18.30` through official npm/GitHub stable `1.18.31`. Identity names
the segment shape first; the claim raises the published window only.

## Why It Matters

g05.051 left `1.18.31` as visible `UnverifiedNewer`. AllowUnverified already
admits later points; it is not permission to leave the current official stable
unverified.

## Ready-State Rubric

- [x] Official npm `opencode-ai@latest` and the GitHub latest release agree on
      `1.18.31`.
- [x] Both hops `1.18.30` and `1.18.31` were retrieved and deterministically
      inventoried.
- [x] The identity record names a compatible extension rather than a new
      facade.
- [x] The claim raises the published segment only.

## Decisions

- One family: `opencode.server` `surface-19` only. OpenCode ACP and web search
  are separate surfaces and stay closed.
- Compatible extension: keep the baseline `1.14.48`, claim id, behavior
  revision, historical gaps and `AllowUnverified`.
- Later `UnverifiedNewer` is the next unpublished stable.
- Host `opencode` is absent from `PATH` and must not be installed. Do not
  execute downloaded artifacts. No provider prompt or live session.
- Do not edit the Next Task pointer, generation status, handoffs, or northstar
  loop state.

## Dispatch manifest

- **State:** ready; one family useful-newer lane entering at review.
- **Owned mutable paths on revision:** this PR's existing OpenCode selection
  source, OpenCode identity and delta-ledger tests and fixtures under
  `crates/swallowtail-adapter-opencode/`, the OpenCode attached prepared guide,
  the affected provider matrix rows, the CHANGELOG `[Unreleased]` entry, the
  lane's research record and its indexes/logs.
- **Excluded:** other families, the Next Task pointer, `docs/roadmaps/README.md`,
  generation runway, lifecycle projection, other PRs' scope.
- **Escalation:** operator via Chatterbox for scope, number, or contract
  questions; Queue coordinator for mechanical blockers.

## Work

1. Freeze identity for `1.18.31` with the deterministic per-hop inventory. No
   production claim.
2. Recheck official stable. If it moved before the claim landed, extend hops
   under Contract 029 In-Run Latest Movement.
3. Apply the compatible-extension claim to the published window.
4. Run focused adapter validation and the named docs gates.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The window extends to `1.18.31` only | A range is inferred past the current stable | selection claims and matrix rows name `1.14.48..=1.18.31` |
| Selected HTTP/SSE sources are unchanged | A route file or the OpenAPI document changed | frozen per-hop inventories and delta ledger |
| Baselines and gaps survive | A historical gap or the baseline changes silently | claim id, baseline and gap set unchanged |
| No provider or host mutation | The CLI is installed or an artifact executed | absence from `PATH` recorded; artifacts hashed only |

## Stop conditions

Stop and ask if a hop changes selected argv, HTTP/SSE shape, authority, or the
OpenAPI document; if the npm and GitHub channels disagree; or if the change is
a new facade rather than a compatible extension.

## Evidence

The lane's research record (renumbered from the colliding `331` on admission).
Fixtures under
`crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.31/`.
Probe artifacts live outside the repository.

## External PR Review Entry Authority

Tom authorized the Queue Oracle on 2026-09-21 to prepare and take over the
remaining externally authored currentness PRs through review entry, and on
2026-09-21 accepted the pilot-first sequence, attested Grok 4.6 as the author
model for the cloud sessions, directed that the Claude Agent ACP PR be held
until last with its contract change flagged, and directed that the failing Qwen
lane be admitted and repaired through the ordinary revision path.

The external work and its evidence are submitted claims, not independent
acceptance. Tom declared Grok 4.6 as the external author model; canonical
declaration: `xai/grok-4.6`, GitHub author `betterthanclay`, authoring system
Cursor cloud driven by Grok Bot.

Queue starts an independent reviewer, not a worker. Changes requested authorize
one ordinary revision loop in the same workspace and branch, scoped to this
OpenCode qualification and its existing changed surfaces. Queue owns merge and
lifecycle closeout. Intake preparation may publish this planning and merge it
into the existing branch; the exclusions above constrain revision workers, not
this operator-approved planning preparation or hook-owned closeout.

Canonical handoff: `docs/handoffs/20260921-g06-010-external-pr-review-entry.md`.

Preserve the external branch and workspace after completion. Other external PRs
are not dispatched by this approval. Research number `331` is retained by
g06.009; this lane takes the next free number, and later admissions reconcile
their own collisions before intake.
