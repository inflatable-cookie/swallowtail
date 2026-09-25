# g06.038 Claude Agent ACP 0.81.2 Useful Newer

Owner: Tom
Created: 2026-09-25
Depends on: Contract 029 (Upgrade Workflow, In-Run Latest Movement); Research 335; completed g06.013
Vision tags: route currentness, Claude Agent ACP, useful-newer

## Outcome

Qualify the `claude-agent.acp` family from the qualified ceiling `0.79.0`
through official npm/GitHub/ACP-registry stable `0.81.2` as a compatible
extension of `claude-agent.acp.initialize-meta-extensions-v7`.

## Why It Matters

g06.013 left the ceiling at `0.79.0`. Official stable moved through
`0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2`. Contract 029 currentness does
not keep g06 open; this milestone records the standing-lane qualification
the way g06.013 recorded the previous ceiling.

## Ready-State Rubric

- [x] Official npm, GitHub, and ACP-registry `claude-acp` agree on `0.81.2`.
- [x] Every published hop `0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2` was
      retrieved and inventoried. The previous `0.79.0` tarball reproduces
      Research 335.
- [x] Research 352 names a compatible extension of
      `initialize-meta-extensions-v7` through `0.81.2`.
- [x] The claim, route rows, feature-matrix cell, prepared-guide ceiling,
      and the Contracts 015 and 038 range sentences name `0.81.2`.

## Decisions

- One family: `claude-agent.acp` only. Claude Code headless and
  response-only stay at their current ceilings. The Claude Agent SDK
  sidecar stays a separate family.
- Compatible extension: keep the baseline `0.53.0`, claim id
  `claude-agent.acp.window-2`, behavior revision, `AllowUnverified`, and
  exclusion `0.58.0`. Extend the maintained segment to `0.66.0..=0.81.2`.
- The ACP SDK pin move `1.4.0` → `1.5.0` is not a wire reset: published
  `dist/acp.js` is byte-identical and protocol version stays `1`.
- Do not map `notice`, `compaction_update`, managed policy, usage-model
  `_meta`, or the Agent SDK pin. Do not add exclusions for preview-only
  `0.79.1` or `0.80.1`.
- Synthetic later `UnverifiedNewer` is `0.81.3`.
- Host `claude-agent-acp` is absent from `PATH`. Missing host is not a gap.
  Do not install, and do not execute downloaded tarballs. No provider
  prompt or live session.
- Contract 015 and Contract 038 change only the qualified-range numbers.
- Do not edit the Next Task pointer, generation status, handoffs, lifecycle
  projection, or `.northstar/`.

## Dispatch manifest

- **State:** extracurricular standing-lane qualification; one family.
- **Owned mutable paths:** Claude Agent ACP selection and its ACP identity,
  delta-ledger, and activity-corpus tests and fixtures under
  `crates/swallowtail-adapter-claude-agent/`; the Claude Agent prepared
  guide; the route and feature matrix rows naming this family; Contracts
  015 and 038 only as the qualified-range record; architecture sentences
  that name this family's ceiling; `CHANGELOG.md` `[Unreleased]`; Research
  352 and its index line; the family identity and claim logs and their
  index lines; this milestone and its `## Tasks` link.
- **Excluded:** other families; Claude Code; the Claude Agent SDK sidecar;
  Gemini; Antigravity headless; Kimi; the Next Task pointer;
  `docs/roadmaps/README.md` Next Task wording; generation disposition;
  lifecycle projection; handoffs; `.northstar/`; release, tag, publication.
- **Escalation:** stop and ask if identity needs a major-line reset, a new
  public operation, a family flatten, or a live session.

## Work

1. Freeze identity for `0.81.2` with the complete dist inventory.
2. Recheck official stable before the claim and again before the final push.
3. Apply the compatible-extension claim. Record the raised range where the
   durable surfaces already name this family's ceiling.
4. Run the named focused, package, route, and docs gates.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The window extends through `0.81.2` | A range stops at `0.79.0` or runs past official stable | selection, matrices, guide, and contract range sentences name `0.81.2` |
| Segment shape is a compatible extension | A new behavior id or a flattened Claude Code claim | claim id and behavior revision unchanged; Claude Code ceilings unchanged |
| Unpublished `0.58.0` stays out | The standing hole is qualified | `0.58.0` remains excluded |
| Later stable is unverified | `0.81.2` is left `UnverifiedNewer` with no incompatible reason | `0.81.3` is the synthetic later point |
| No provider work | A tarball is executed or a live initialize is sent | fixture records both as false |

## Stop conditions

Stop and leave the claim unchanged if channels disagree, if a hop resets
the mapped protocol, if a new public operation appears on the selected
route, or if qualifying the hop would flatten this family onto Claude Code
or the Claude Agent SDK.

## Evidence

Research 352 and the frozen corpus
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.81.2/`.
Focused and affected-package validation for
`swallowtail-adapter-claude-agent`, plus the named route and docs gates.
