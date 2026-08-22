# 113 Qwen Headless Reasoning Effort Evidence

Status: ready
Owner: Tom
Created: 2026-08-22
Milestone: [g04.041 Qwen Headless Reasoning Effort](../041-qwen-headless-reasoning-effort.md)
Depends on: Research 017, 081, 082, 159, and 173

## Goal

Freeze exact current official and package `0.21.15` evidence for Qwen reasoning
effort, then define the smallest model/value subset with an operation-private
headless transport that can map to portable reasoning selection.

## Method

1. Freeze current official headless, settings, model-provider, and command
   references for `model.reasoningEffort`, `/effort`, named values, precedence,
   defaults, model/provider qualification, and lifetime.
2. Inspect exact official `@qwen-code/qwen-code@0.21.15` package/help/source in
   a disposable directory. Record package/source identity, stable secret-free
   specimens, SHA-256 digests, and the existing behavior boundary. Do not
   install it onto the host.
3. Find the exact configuration read path. Distinguish command argv, one-child
   environment, stdin/request data, explicit config-file selection, ambient
   user/project settings, and interactive `/effort` state. Do not treat these
   as interchangeable.
4. Prove precedence against ambient settings and whether one planned value can
   be supplied without editing user/project config or creating a synthetic
   config root. Classify the transport against Contracts 033 and 040.
5. Classify `low`, `medium`, `high`, `xhigh`, and `max` separately for each
   exact model candidate. Prove parsing/admission, supported values, aliases,
   normalization, clamping, fallback, and default substitution from source or
   deterministic package behavior. Do not infer support from emitted text.
6. Prove lifetime on the exact selected transport: one structured-run child,
   first interactive-turn child, later `--resume` children, and fresh
   context-losing replacement. Identify any process, session, turn, or
   persisted-setting leakage.
7. Decide whether the new mapping needs a private behavior revision and
   whether it is qualified only at exact `0.21.15`. Do not retroactively widen
   the mapping across `0.21.0..=0.21.14` without a frozen corpus.
8. Replace the pre-indexed Research 189 reservation with deliver-now,
   evidence-gated, withheld, not-applicable, and obsolete dispositions. Do not
   edit the shared research index.

No provider prompt, login, credential read, account inspection, user-config
mutation, package installation, or live catalogue is authorized.

## Acceptance Criteria

- [ ] exact current official and exact `0.21.15` evidence is frozen without
      secrets
- [ ] `/effort` and headless process transport are separated explicitly
- [ ] transport, precedence, configuration posture, and mutation truth are
      explicit
- [ ] every model/value candidate has clamp, default, and support disposition
- [ ] run, first-turn, resumed-turn, and fresh-replacement lifetime is explicit
- [ ] version segment and private behavior-revision disposition is explicit
- [ ] Research 189 is promoted and the production claim is unchanged
- [ ] `effigy validate:focused swallowtail-adapter-qwen` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

Auto-continue to card 114 only when at least one exact model/value row has a
process-private Contract 040 transport with no Contract 033 lease gap.

## Stop Conditions

- exact `0.21.15` lacks or contradicts the documented surface
- headless selection requires ambient/global config mutation or `/effort`
- selection needs a synthetic file tree or isolated config root
- the interface clamps, substitutes, or silently ignores every candidate
- exact model support cannot be frozen
- lifetime cannot remain exact across resumed children and fresh replacement
- mapping needs a generic argv/configuration surface or contract change

## Out Of Scope

- production binding or dispatch
- model catalogue changes, tool/search/permission controls, or sibling routes
- live provider/account work, installation, or host configuration
- shared research/log/roadmap indexes and shared closeout surfaces
