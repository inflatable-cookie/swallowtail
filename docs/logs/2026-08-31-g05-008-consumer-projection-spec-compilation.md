# 2026-08-31 g05.008 Consumer Projection Spec Compilation

Status: complete; card 021 ready
Owner: Tom
Generation: g05
Milestone: g05.008
Spec: 012
Planning base: `a12dc5f695dc8ad68e6ec92e89df907b6786c253`

## Operator Decisions

The operator affirmed the post-card-020 recommendation:

1. promote the reviewed consumer route-feature/control census as g05's next
   provisional spec and contract-discussion lane
2. use one dedicated composing contract rather than amendments to Contracts
   037, 047, or 057
3. defer a closed availability-reason taxonomy and preserve existing source
   dimensions plus bounded safe reasons

## Planning Result

Spec 012 promotes the census synthesis without converting it into runtime
authority. It defines three lifecycle-separated views, exact source and
snapshot binding, immutable replacement, descriptive-only authority, and a
review oracle against stale, cross-model, and observation-as-mutation assembly.

g05.008 and card 021 reserve Contract 061. Card 021 is one docs-only promotion
batch: write the dedicated contract, update its indexes, archive Spec 012, and
return to planning. It cannot edit code, amend existing contracts, claim an
exhaustive reason taxonomy, contact a provider, or compile implementation.

## Readiness

Card 021 has bounded scope, current governing refs, acceptance criteria,
documentation validation, explicit stop conditions, a public-boundary review
oracle, and a no-auto-continuation close. One serial worker handoff is prepared.

No watcher, skill-visibility, currentness, PR 127, Darwin, provider, or Bedrock
work is authorized.

## Next

Publish this planning batch and its single worker handoff on `main`. Then run
card 021 through the manual worker/PR loop. Do not start the worker from an
unpublished handoff.
