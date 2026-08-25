# 184 Codex Exec Model Verbosity Evidence

Status: complete
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.066 Codex Exec Model Verbosity](../066-codex-exec-model-verbosity.md)
Depends on: Research 201; Contracts 020, 029, 033, 040

## Goal

Determine the exact Codex CLI versions, providers, selected models, values, and
Exec profiles on which `model_verbosity` can be dispatched through a closed
adapter-local selection without ambient-config authority, silent unsupported-
model behavior, or a live-provider inference. Promote an honest empty set if
any required fact remains unproved.

## Work

1. Retrieve and digest current official Codex configuration documentation plus
   exact `rust-v0.149.1` source for `config.schema.json`, `models.json`, model
   info, config loading/overrides, request construction, Exec command handling,
   and release identity. Record dates, revisions, and decisive hashes in
   Research 213.
2. Freeze the exact `model_verbosity` syntax and closed value domain. Determine
   whether the CLI parser rejects unknown values locally and whether `--config`
   is parsed before authentication or provider work.
3. Freeze the exact release-tag model rows that declare
   `support_verbosity=true`, their `default_verbosity`, provider transport, API
   support, availability, aliases, and any model-source or account-dependent
   overrides. Do not infer support from model-name prefixes.
4. Trace exact request construction. Prove which providers use the Responses
   API verbosity field, when configured verbosity is omitted, ignored, warned,
   defaulted, or serialized, and whether selected-model metadata gates it.
5. Freeze CLI precedence between explicit `--config`, ignored user/project
   config, environment, model presets, catalogue metadata, and defaults for the
   maintained `0.122.0..=0.149.1` suppressed-config behavior. Classify older
   retained/ambient segments separately and withhold them by default.
6. Audit the production prepared Exec input, model selection, plan/evidence,
   behavior classification, argument construction, validation, events,
   activity, usage, cancellation, deadline, cleanup, fixtures, guide, and API
   baseline. Name the smallest safe adapter-local delta.
7. Classify each selected model/value/profile row as deliver now,
   evidence-gated, intentionally withheld, or not applicable. Separate
   selected, planned, dispatched, provider-accepted, effective, and observed
   truth.
8. Prove omission retains current argv. Determine whether explicit low,
   medium, and high can compose with every admitted reasoning/search/schema/
   image profile without translation, fallback, or a second provider setting.
9. Select any required adapter-private behavior, claim, or model-route revision.
   Do not change the Contract 029 version ceiling in this feature lane.
10. Promote Research 213 with one exact deliver-now table or explicit empty
    set. Update the milestone/card state and reserved closeout honestly.

## Acceptance Criteria

- [x] exact official sources, release tag, dates, revisions, and hashes are
      recorded
- [x] CLI/version, provider, selected-model, value, default, and profile rows
      are explicit
- [x] parser, precedence, omission, unknown, unsupported, ignore, fallback,
      and warning truth is settled
- [x] selected/planned/dispatched/accepted/effective/observed claims are split
- [x] existing Exec input, plan/evidence, command, composition, and lifecycle
      seams are audited
- [x] Research 213 contains a non-empty exact table or honest empty set
- [x] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes
- [x] `effigy validate:focused swallowtail-adapter-codex`, `effigy
      qa:northstar`, relevant indexes, and `git diff --check` pass

## Stop Conditions

- exact tagged source, selected-model support, provider mapping, config
  precedence, or unsupported behavior remains ambiguous
- delivery needs a live prompt, login/account inspection, ambient config,
  model-family inference, generic settings, sibling-route promotion, shared
  authority, or a public lifecycle change

## Out Of Scope

- production binding, app-server, Fast/service tier, personality, multi-agent,
  live access, currentness, release, merge, rollover, or g04 closure
