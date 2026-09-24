# g06.018 Claude Code Current-Release Qualification

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029 (No Terminal Stop); Research 331, 338; completed g06.017
Vision tags: route currentness, Claude Code, safe mode, adaptation

## Outcome

Qualify both Claude Code stream-JSON axes through the official stable current
at run time (`2.1.281` at planning), adapting the adapter to the `2.1.280`
`--safe-mode` change instead of holding the ceiling at `2.1.278`.

## Why It Matters

g06.017 recorded a typed stop: from `2.1.280`, `--safe-mode` no longer skips
all plugin hook registration and keeps `@builtin` plugin hooks. The
response-only route passes `--safe-mode` as part of its isolation. Tom ruled on
2026-09-24 that stopping at an older version is not acceptable, and Contract
029 now makes every stop a transient work item. This task owns that work item.

## Ready-State Rubric

- [x] The selected-surface change is named exactly from frozen artifacts
      (Research 338).
- [x] The operator direction is recorded (Contract 029, No Terminal Stop).
- [x] Official stable moved past the stop (`2.1.281` at planning), so the
      in-run latest rule applies from the start.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Target the official stable current when the run starts, re-probed before
  push per Contract 029 In-Run Latest Movement. Every published hop after
  `2.1.278` gets identity; unpublished neighbours stay incompatible gaps.
- Headless does not pass `--safe-mode`; classify its hops on their own
  evidence. Do not hold headless back because of a response-only change.
- Response-only is resolved by the first branch that the evidence supports:
  1. **Inert.** Every `@builtin` plugin hook that safe mode keeps is inert for
     the response-only invocation — no context injection, no prompt or output
     rewrite, no command, tool, network or file effect under `--tools ""`,
     empty strict MCP config and disabled slash commands. Extend the segment
     and record why the kept hooks are provider-owned surface, like built-in
     tools.
  2. **Switchable.** A hook can affect the turn, and a documented flag or
     setting on the selected surface disables it (for example a `--settings`
     object or a plugin-disable switch). Add an adapter-private milestone from
     `2.1.280` that pins that switch, and qualify.
  3. **Neither.** A hook can affect the turn and nothing disables it. Prepare
     the narrowed response-only isolation claim and its evidence, then
     escalate: narrowing a consumer-visible guarantee is an operator ruling.
     This is the only branch that stops before the claim edit.
- Keep watcher exact `2.1.251`, every feature-specific exact set, and the
  separate Claude Agent ACP and SDK families untouched. No flattening.

## Dispatch manifest

- **State:** ready; one family adaptation lane; highest priority in g06.
- **Completion:** both axes qualified through the run's official stable, or
  branch 3 escalated with the narrowed claim prepared; independent exact-head
  review accepts the head.
- **Owned mutable paths:** Claude Code family code under
  `crates/swallowtail-adapter-claude-agent/src/claude_code_*`; its fixtures and
  identity tests under `crates/swallowtail-adapter-claude-agent/tests/`; the
  Claude prepared guide and the route and feature matrix rows naming this
  family; `CHANGELOG.md` `[Unreleased]`; one new research record and its index
  line; the family's identity and claim logs and their index lines;
  `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; watcher code and exact `2.1.251`; Claude
  Agent ACP and SDK surfaces; other families; contracts; Next Task and the
  generation runway; release, tag, publication.
- **Worker evidence:** the authenticated run result and the new research
  record.
- **Escalation:** operator via Chatterbox for branch 3; Queue coordinator for
  mechanical blockers.

## Work

1. Re-probe official npm and GitHub; freeze identity for every published hop
   after `2.1.278` through current latest on `darwin-arm64` and `linux-x64`,
   reproducing Research 338 for `2.1.280`.
2. Inventory every `@builtin` plugin at each hop: name, registered hook
   events, and what each hook does, from embedded source.
3. Search the selected surface for a supported switch that disables those
   hooks, and record where it is defined.
4. Classify each hop per axis, pick the response-only branch, and implement
   it: segment extension, milestone with pinned switch, or prepared narrowing.
5. Raise both `latest_qualified` points and update the guide and matrices.
6. Run the named validation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| The ceiling moves | The run ends with both axes still at `2.1.278` without a branch-3 escalation | claims name the run's official stable |
| Branch choice rests on evidence | "Inert" is asserted from a changelog or help text | each kept hook is cited from embedded source with its events and effect |
| A pinned switch is real | The milestone passes a flag the selected binary does not define | the switch is found in the frozen artifact's option or settings parser |
| Older hops keep their arguments | The new switch leaks into `2.1.220..=2.1.278` | argument tests per segment |
| Families stay separate | ACP, SDK or watcher pins move | no such path in the diff |
| Nothing ran live | A downloaded binary was executed | method records hash-and-read only |

## Stop conditions

Branch 3 above. Also stop if identity disagrees across official channels, if a
hop needs a public lifecycle change, or if latest makes a major-line reset.

## Evidence

Research 331 and 338; the new research record and fixtures. Focused and
affected-package validation for `swallowtail-adapter-claude-agent`, route and
docs QA.

## Next Task

Chatterbox reconciles the claim after closeout. If a pinned switch was added,
Chatterbox decides whether a live response-only gate is worth requesting.
