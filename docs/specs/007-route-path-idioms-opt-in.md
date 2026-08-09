# 007 Route-Path Idioms Opt-In

Status: draft
Owner: Tom
Updated: 2026-08-09

## Purpose

Decide how the idioms mechanism (Contract 055) becomes an opt-in route
feature: one-time host registration, a portable session option bound into
the prepared plan, and a fixed runtime fold rule into the existing
developer-instructions channel — so consumers stop re-wiring selection and
delivery per session.

## Scope

In:

- optional `IdiomSource` and `IdiomRecorder` ports on the execution-host
  service set (Contract 010 amendment)
- optional `SessionOptions` idioms field (Contract 012 amendment) binding
  source identity and maximum
- immutable prepared-plan binding of the opt-in (Contract 037 amendment)
- one fixed bounded fold rule rendering selected idiom constraints into the
  developer-instructions field
- a route capability so non-advertising routes reject the opt-in at
  preflight
- first proof on Codex app-server (Nucleus's primary interactive route)
- bounded Nucleus adoption delta as the testbed handoff

Out:

- prompt authorship: the runtime renders only opted-in source content under
  the fixed rule; hosts keep composing their own instructions
- learned backends and the correction-loop proxy (later evidence-gated lane)
- signal semantics: accept/reject/edit remain consumer-owned product
  decisions fed to the registered recorder
- any non-Codex route proof beyond the capability gate
- provider payload changes

## Decisions Needed

Settled 2026-08-09:

1. host port shape — mirror `DiagnosticObserver` on the execution-host
   service set (Contract 056)
2. fold rule — consumer instructions first; labeled idioms block appends
   after, bounded with truncation marker (Contract 056)
3. capability name — `idioms_session_option` (Contract 056)
4. first route — Codex app-server proof (Contract 056)
5. Nucleus adoption scope — interactive session path only; signal wiring
   stays consumer-owned and out of the delta (Contract 056)
6. runtime dependency floor — runtime gains `swallowtail-idioms` (Contract
   056)

## Acceptance Criteria

- [x] Research 119 records evidence and recommendation
- [x] Contract 056 and amendments to 010/012/037/055 govern the surface
- [x] spec records the host-gated fold exception to the 055 boundary
- [ ] roadmap and ready cards sequence runtime, Codex proof, and Nucleus
      adoption
- [ ] conformance covers fold determinism, redaction and bounds, fail-closed
      preflight, and missing-source rejection

## Promotion Targets

- `docs/contracts/056-route-path-idioms-opt-in.md`
- amendments to contracts 010, 012, 037, and 055
- `docs/architecture/system-architecture.md`
- a roadmap and ready cards under `docs/roadmaps/g03/`
- Research 119 promotion into logs on close
