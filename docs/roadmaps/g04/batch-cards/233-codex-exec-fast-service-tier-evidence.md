# 233 Codex Exec Fast Service-Tier Evidence

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.083 Parallel Per-Route Feature Qualification II](../083-second-parallel-per-route-feature-qualification.md)
Depends on: g04.054; g04.066; g04.082 closeout
Research: [234 Codex Exec Fast Service-Tier Evidence](../../../research/234-codex-exec-fast-service-tier-evidence.md)

## Goal

Freeze exact Codex exec Fast/service-tier configuration, feature gate, model
membership, access, dispatch, returned-state, billing, and lifecycle truth,
then promote Research 234 with a closed deliver-now table or an honest empty set.

## Work

1. [x] Keep route `codex.exec`, maintained qualified segments
       `0.80.0..=0.81.0`, `0.84.0..=0.107.0`, and `0.110.0..=0.149.1`,
       structured JSONL, read-only sandbox, approval `never`, selected model,
       and existing verbosity behavior unchanged.
2. [x] Freeze tagged source, schemas, model metadata, and official docs for
       `service_tier`, `features.fast_mode`, `/fast`, precedence, defaults,
       unsupported behavior, account/billing rules, and request construction.
3. [x] Locate the exact published introduction point, then determine the
       minimal relationship between the feature gate and
       selected service tier. Do not treat `/fast`, `fast_mode`, and
       `service_tier = "fast"` as synonyms without source proof.
4. [x] Build a closed version/model/access/value/profile table. Separate
       ChatGPT-credit and API-key billing profiles; do not infer one from the other.
5. [x] Trace the exec `--config` seam, child isolation, model selection, request
       bytes, JSONL events, terminal state, usage, and cleanup. Prove unsupported
       rows reject before process, credential, resource, or provider work.
6. [x] Separate requested, config-encoded, dispatched, accepted, effective,
       returned, billed, and latency-observed truth. Catalogue advertisement is
       membership evidence only when tied to the selected exact model.
7. [x] Prove omission retains current exec argv/config and behavior, including
       the delivered model-verbosity surface.
8. [x] Audit prepared inputs/evidence, plan/request agreement, command builder,
       decoder, fixtures, guide, matrices, and API baseline without production edits.
9. [x] Promote Research 234 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact version/model/access/config table or honest empty set is recorded
- [x] feature-gate and service-tier composition is exact, not inferred
- [x] unsupported rows reject before effects and omission stays unchanged
- [x] requested, returned, billing, and observed latency truth stay separate
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-codex
effigy qa:northstar
git diff --check
```

## Stop Conditions

- model membership depends on a live account catalogue or mutable entitlement
- `fast_mode`/`service_tier` precedence, request dispatch, or returned state is
  ambiguous or silently substitutes
- exact access and billing profile cannot be closed before process work
- proof needs login, credential, provider request, paid work, install/update,
  ambient configuration mutation, or a shared-contract change

## Out Of Scope

Codex app-server, generic Fast vocabulary, search, personality, Plan effort,
multi-agent, production binding, live provider work, currentness, release,
merge, shared closeout, rollover, or g04 closure.

## Closeout

Research 234 promotes an honest empty deliver-now set. Gate, config,
catalog membership, billing split, and omission research are frozen:
`features.fast_mode` gates tier dispatch; `/fast` is TUI-only; legacy config
`fast` normalizes to wire `priority`; five bundled-catalog slugs advertise tier
id `priority` at `0.149.1`. Live-catalog substitution, silent unsupported-tier
downgrade inside Codex, and absent pre-prompt `service_tier` observation block
caller-selected Fast binding without unconfirmed substitution. Frozen evidence:
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/exec-fast-service-tier-range.json`.
Production binding not authorized.
