---
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
---

# g05.009 Card 079 Contract 061 Compound Acknowledgement Runtime Baseline

## Objective

Realize Card 076's operator-accepted compound acknowledgement design in the
runtime and testkit packages, then regenerate their public API baselines.

## Scope

Own only `crates/swallowtail-runtime/**`, `crates/swallowtail-testkit/**`, the
two named additive public API baselines, `CHANGELOG.md` `[Unreleased]`, Card
079's `## Result`, and append-only `PAPERCUTS.md`. Adapters, core, contracts,
Card 034, and reserved closeout surfaces are forbidden.

## Execution

Fetch origin before preflight. Implement the exact names, constructors,
diagnostic, admission rules, six testkit assertions, and terminally
undispatched Plan truth from Card 076's accepted result. Use Effigy package
selectors and provider-free validation. Regenerate baselines with the
repository script and confirm additive-only changes.

## Validation and review

Run the manifest-named formatting, focused/package, API, docs, Northstar,
god-file, and diff gates. Push one reviewable PR and request an independent
cross-model exact-head review in this same worker workspace. Do not merge.

## Handoff

Report the exact PR head, owned-path diff, baseline additivity, all validation
results, and any deviation from Card 076 to Chatterbox. Coordinator owns the
merge gate, reserved closeout, and frontier recomputation.
