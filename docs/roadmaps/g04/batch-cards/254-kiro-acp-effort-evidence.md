# 254 Kiro ACP Effort Evidence

Status: complete
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.089 Sixth Parallel Per-Route Feature Qualification](../089-sixth-parallel-per-route-feature-qualification.md)
Depends on: g03.093-g03.096; g04.088 closeout
Research: [251 Kiro ACP Effort Evidence](../../../research/251-kiro-acp-effort-evidence.md)

## Goal

Freeze exact Kiro ACP effort version, model, value, spawn/session application,
confirmation, lifecycle, and omission truth. Promote Research 251 with a
closed deliver-now table or an honest empty set.

## Work

1. [x] Keep route `kiro.acp`, exact `2.18.1`, host-approved executable and
       environment, provider-supported host account state, current ACP
       lifecycle, and current permission posture unchanged.
2. [x] Freeze official `--effort low|medium|high|xhigh|max` documentation,
       precedence, model dependence, ACP spawn docs, session configuration,
       request-use leads, and failure/omission disposition from public pages.
       [ ] Exact `2.18.1` package/source parser: **stopped** — CDN ranged GET
       returned HTTP 403; binary parser bytes unrecovered (Research 251).
3. [x] Determine whether effort belongs before ACP startup, at session open,
       or to a model-selection surface. Do not infer support from an
       unsupported `session/set_model` method.
4. [x] Build a closed version/model/value/lifecycle table. Separate requested,
       parsed, dispatched, accepted, effective, returned, and observed truth.
5. [x] Withhold every candidate: no ACP reject-before-provider seam closed
       without package/live proof; omission retains exact `kiro-cli acp`
       argv and behavior.
6. [x] Audit prepared sessions, spawn plan/evidence, ACP fixtures, guide,
       matrices, and API baseline without production changes.
7. [x] Promote Research 251 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact version/model/value/lifecycle table or honest empty set exists
- [x] no non-empty row admitted; empty set names the closed gates
- [x] headless or interactive CLI evidence is not silently promoted onto ACP
- [x] unsupported candidates withheld; reject-before-provider seam unclosed
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Result

Honest empty set. Official ACP docs omit `--effort`; chat `--effort` and
model-dependent `/effort` stay off this route; `session/set_model` and
`_kiro.dev/*` stay unmapped; exact `2.18.1` package parser unrecovered
(CDN 403). Omission retains `["acp"]`. See Research 251 and
`crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.18.1-effort-evidence/`.

## Validation

```sh
effigy validate:focused swallowtail-adapter-kiro
effigy qa:northstar
git diff --check
```

## Stop Conditions

- effort membership depends on live model/account state
- ACP startup or session bytes do not carry and confirm the selection
- proof needs login, credentials, provider prompts, paid work, install/update,
  host mutation, or shared-contract change

## Out Of Scope

Kiro headless, agent profile, cloud sessions, trust-all tools, model routing,
production binding, live provider work, currentness, release, shared closeout,
rollover, or g04 closure.
