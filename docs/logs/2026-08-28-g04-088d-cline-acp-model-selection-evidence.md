# 2026-08-28 g04.088d Cline ACP Model-Selection Evidence

Status: complete
Card: 251
Research: 248

## Boundary

Evidence only. This lane owns card 251, Research 248, this log, and optional
new Cline-local frozen evidence. Shared planning and production stay unchanged.

## Target

Close exact provider/model membership, route agreement, pre-prompt selection,
confirmation, persistence/restoration, lifecycle, failure, and omission truth.

## Finding

Honest empty deliver-now set.

Exact `cline.acp` `3.0.55` advertises provider/model on `session/new` and
accepts `session/set_config_option` with returned `configOptions` confirmation.
That is the Plan protocol pattern, not closed model membership.

Stops that closed the set:

1. Provider defaults stay ambient (`CLINE_PROVIDER` / auth result / `cline`).
2. Model options come from `Llms.getModelsForProvider`; `cline` builds an
   OpenRouter-generated catalogue plus aliases. Contract 020 forbids promoting
   that as a `ModelRoute`.
3. `set_config_option` `model` assigns any string; unlike `mode`, foreign ids
   are not rejected before provider effects.
4. Observing live membership needs auth/catalogue work this lane forbids.
5. Echo `currentValue` without membership rejection is not closed confirmation.

Not imported from Research 221: headless `saveProviderSettings`. ACP
early-return skips that path; model/provider set-config stay session-private.
Fresh `session/load` still rebuilds ambient defaults.

## Evidence

- Research 248 promoted empty set
- `crates/swallowtail-adapter-cline/tests/fixtures/cline-acp-3.0.55/model-selection-evidence.json`
- tagged sources at `cli-v3.0.55` /
  `ad442cbb6a81d21773ceabc1398ea5eb58170718`

## Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy qa:northstar
git diff --check
```

## Unresolved / Later

Reopen only if an exact package point fixes provider identity from route facts,
closes model membership with pre-effect rejection, and keeps confirmation
without live catalogue or unauthorized durable mutation. No production binding
from this lane.
