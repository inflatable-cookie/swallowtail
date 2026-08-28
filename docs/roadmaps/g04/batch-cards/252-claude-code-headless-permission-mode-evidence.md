# 252 Claude Code Headless Permission-Mode Evidence

Status: complete
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.089 Sixth Parallel Per-Route Feature Qualification](../089-sixth-parallel-per-route-feature-qualification.md)
Depends on: g04.079; g04.083-g04.088 closeout
Research: [249 Claude Code Headless Permission-Mode Evidence](../../../research/249-claude-code-headless-permission-mode-evidence.md)

## Goal

Freeze exact Claude Code headless permission-mode version, value, access,
resource, tool, application, terminal, lifecycle, and omission truth. Promote
Research 249 with a closed deliver-now table or an honest empty set.

## Work

1. [x] Keep route `claude-code.headless`, every published qualified
       `2.1.220..=2.1.241` point, local-subscription access, current read tools,
       no session persistence, and current lifecycle unchanged.
2. [x] Freeze official permission-mode documentation plus exact package
       declarations, parser membership, aliases, precedence, application,
       prompt/tool behavior, terminal shape, and failures.
3. [x] Classify `default|acceptEdits|auto|dontAsk` against current Plan.
       Keep `bypassPermissions` excluded. Do not treat a provider label as a
       portable resource or containment guarantee.
4. [x] Build a closed version/mode/resource/tool/lifecycle table. Separate
       requested mode, configured tools, permission callbacks, effective
       authority, provider behavior, and host isolation.
5. [x] Prove unsupported and incompatible rows reject before prompt effects;
       prove omission retains exact current Plan argv and behavior.
6. [x] Audit prepared input/evidence, command builder, decoder, fixtures,
       guide, matrices, and API baseline without production changes.
7. [x] Promote Research 249 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact version/mode/resource/tool/lifecycle table or honest empty set exists
- [x] a non-empty row closes authority, application, terminal, cleanup, and omission — N/A; empty set
- [x] no row silently widens write, tool, approval, or isolation authority
- [x] `bypassPermissions` is not admitted or made a default
- [x] unsupported rows reject before prompt effects
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:northstar
git diff --check
```

## Stop Conditions

- mode truth depends on live prompts, credentials, account state, paid work, or
  ambient configuration mutation
- provider permission behavior cannot be separated from host isolation
- a candidate widens authority without a consumer-mediated contract
- proof needs install/update or shared-contract change

## Out Of Scope

Claude response-only or ACP, advisor, Fast, compaction, spend cap, maximum
turns, Agent teams, production binding, live provider work, currentness,
release, shared closeout, rollover, or g04 closure.

## Result

Honest empty set. Every published `2.1.220..=2.1.241` point advertises and
parses `--permission-mode` with help choices
`acceptEdits|auto|bypassPermissions|manual|dontAsk|plan`; `default` is
accepted as Manual and `manual` aliases to it. `acceptEdits` and `auto` widen
authority relative to Plan. `default`/`dontAsk` do not close Plan-equivalent
application or operation-private effective authority under selected
setting-sources. `bypassPermissions` stays excluded. Omission retains
`--permission-mode plan`. Fixture:
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-permission-mode.json`.
