# 110 Copilot CLI ACP Effort Evidence

Status: complete
Owner: Tom
Created: 2026-08-22
Milestone: [g04.040 Copilot CLI ACP Session Effort](../040-copilot-cli-acp-session-effort.md)
Depends on: Research 049, 149, and 159

## Goal

Freeze exact current official and package evidence for Copilot CLI ACP
server-start effort, then define the smallest value and session-lifetime subset
that can map to portable reasoning selection on exact package `1.0.80`.

## Method

1. Freeze the official ACP-server reference for `--effort`,
   `--reasoning-effort`, allowed values, startup scope, inheritance, defaults,
   and inability to change the setting through `session/new`.
2. Inspect exact official `@github/copilot` `1.0.80` package/help/source
   artifacts. Record source identity, stable secret-free specimens, SHA-256
   digests, and the existing `copilot-cli.acp.stdio-v1` behavior boundary.
3. Determine the canonical argv spelling and syntax Swallowtail would emit.
   Treat the second flag name as an upstream alias, not a second public option.
4. Classify `low`, `medium`, `high`, `xhigh`, and `max` separately. Do not infer
   an exact value from defaults, response text, model behavior, or aliases.
5. Prove the adapter's one-child/one-session topology aligns startup lifetime
   with prepared-session lifetime, including first prompt, later prompts, and
   fresh context-losing replacement.
6. Decide whether a server-level initial effort with no selected model can map
   to `ReasoningSelection` under Contract 040. Separate dispatched, accepted,
   and effective states. No clamp or nearest-value mapping is permitted.
7. Classify tool-filter flags and dangerous permission flags as separate and
   out of scope; their proximity in the upstream table grants no capability.
8. Replace the pre-indexed Research 188 reservation with deliver-now,
   evidence-gated, withheld, not-applicable, and obsolete dispositions. Do not
   edit the shared research index.

Do not install or authenticate Copilot CLI, inspect account state, send a
prompt, or infer effective effort from provider output.

## Acceptance Criteria

- [x] exact current official and exact `1.0.80` evidence is frozen without
      secrets
- [x] canonical syntax, alias posture, values, default, and lifetime are explicit
- [x] the no-model-route Contract 040 disposition is explicit
- [x] every candidate value and session stage has a disposition
- [x] Research 188 is promoted and the existing route behavior boundary is
      either retained with evidence or named as a stop
- [x] production code, claims, matrices, architecture, and changelog are unchanged
- [x] `effigy validate:focused swallowtail-adapter-copilot-cli` passes
- [x] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [x] `git diff --check` passes

Auto-continue to card 111 only when at least one exact value maps to portable
reasoning on package `1.0.80` without a contract, facade, or version-segment gap.

## Stop Conditions

- exact `1.0.80` lacks or contradicts the current documented flags
- the interface clamps, substitutes, or silently ignores a candidate value
- exact support requires unknown selected-model capability
- startup scope cannot be made preparation-fixed across fresh replacement
- mapping needs a generic argv/configuration surface

## Out Of Scope

- production binding or dispatch
- tool filters, permissions, TCP, login, currentness, or another Copilot route
- live provider/account work
- shared research/log/roadmap indexes and shared closeout surfaces

## Closeout

Research 188 freezes official ACP-server and exact `1.0.80` `app.js` evidence
for `--effort` / `--reasoning-effort`. The five documented values parse as one
commander option and are stored as ACP session-initial effort, but entitled
application is model-gated and can substitute the model default. `copilot-cli.acp`
has no selected model. No portable `ReasoningSelection` row survives. Card 111
is not executed. The current `copilot --acp --stdio` boundary is retained.
