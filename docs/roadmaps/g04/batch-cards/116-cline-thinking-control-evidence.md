# 116 Cline Thinking Control Evidence

Status: ready
Owner: Tom
Created: 2026-08-22
Milestone: [g04.042 Cline Thinking Controls](../042-cline-thinking-controls.md)
Depends on: Research 143, 146, 147, and 159

## Goal

Freeze exact current official and package `3.0.55` evidence for Cline thinking
controls, then define the smallest route/value subset that can map exactly to
portable reasoning selection on `cline.acp` and `cline.headless`.

## Method

1. Freeze current official CLI, ACP, and headless references for
   `--thinking`, syntax, values, omission, defaults, provider/model support,
   precedence, and lifetime. Record contradictions rather than choosing the
   friendliest description.
2. Inspect exact official `cline@3.0.55` package/help/source in a disposable
   directory. Record package/source identity, stable secret-free specimens,
   SHA-256 digests, and the existing ACP/headless behavior boundaries. Do not
   install it onto the host.
3. Trace parsing and application separately for `cline --acp` and
   `cline --json`. Prove whether each uses the same parsed option, changes it,
   persists it, or resolves it later against provider/model state.
4. Classify `none`, `low`, `medium`, `high`, and `xhigh` separately for each
   route. Prove aliases, optional-argument behavior, normalization, clamp,
   fallback, ignored values, default substitution, and exact rejection.
5. Decide whether upstream `none` has the same exact semantics as portable
   `ReasoningMode::Off`. Do not admit it from spelling or intent alone.
   `minimal` and `max` are outside the documented candidate set.
6. Prove provider/model qualification. If either route selects no model, name
   whether any value is invariant across every exact supported provider/model
   path. Do not infer support from model catalogues or emitted reasoning.
7. Prove lifetime and precedence independently: ACP server start, first and
   later prompt in one child, and fresh context-losing replacement; headless
   one-run child and absence. Identify ambient or persisted-setting leakage.
8. Decide whether either mapping needs a private behavior revision at exact
   `3.0.55`. Do not change the compatibility ceiling or qualify a newer point;
   Contract 029 currentness stays in its standing lane.
9. Replace the pre-indexed Research 190 reservation with deliver-now,
   evidence-gated, withheld, not-applicable, and obsolete dispositions for
   every `(route, value)` row. Do not edit the shared research index.

No provider prompt, login, credential/account inspection, user/project config
mutation, package installation, live catalogue, or live process session is
authorized.

## Acceptance Criteria

- [ ] current official and exact `3.0.55` evidence is frozen without secrets
- [ ] official omission/default contradictions are resolved by exact package
      evidence or left explicitly unresolved
- [ ] ACP and headless parse, application, precedence, and lifetime are
      classified independently
- [ ] every candidate value has exact normalization, model, clamp, default,
      and support disposition
- [ ] `none` versus portable `off` has an evidence-backed disposition
- [ ] compatibility behavior-revision truth is explicit and the claim ceiling
      is unchanged
- [ ] Research 190 is promoted
- [ ] `effigy validate:focused swallowtail-adapter-cline` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

Auto-continue to card 117 only when Research 190 contains at least one exact
route/value deliver-now row with no model-inference, normalization, default,
configuration, or lifetime gap.

## Stop Conditions

- exact `3.0.55` lacks or contradicts the documented surface
- all values are provider/model-dependent without an exact selected model
- the package aliases, clamps, substitutes, persists, or ignores every
  candidate
- upstream `none` is not exact portable `off`
- ACP or headless cannot keep the selected value inside its named lifetime
- mapping needs user-config mutation, a synthetic config root, generic argv,
  a contract change, or a compatibility-currentness change

## Out Of Scope

- production binding or dispatch
- model, plan, permission, tool, search, timeout, retry, or sibling-route work
- live provider/account work, installation, or host configuration
- shared research/log/roadmap indexes and shared closeout surfaces
