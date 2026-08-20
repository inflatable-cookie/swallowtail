# g04.009 Model Presentation Overlay

Date: 2026-08-20
Roadmap: `../roadmaps/g04/009-model-presentation-overlay.md`
Cards: `../roadmaps/g04/batch-cards/025-apply-model-presentation-overlay.md`,
`../roadmaps/g04/batch-cards/026-overlay-refusals.md`

## Result

Consumers can project hide, ordinal, consumer-default, and favourite onto
one bound 047 catalogue result.

`apply_model_presentation_overlay` keys markers to exact configured-instance,
provider, and model ids. Provider catalogue default stays on the overlay
row and is not rewritten as consumer-default. The 047 snapshot is not
mutated. `Ready` / `NotReady` is copied unchanged.

Unknown model ids fail closed. Markers whose instance id does not match the
catalogue instance fail closed. Overlay cannot change selection readiness to
`Ready`. Mixed gateway rows remain consumer assembly of several catalogues.

Additive API is in `public-api-unreleased` for runtime. `public-api-0.3.3`
is unchanged. No production adapter crate changed.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-d4a5fc8c`
Worker branch: `t3code/read-model-presentation-overlay-handoff`

PR: https://github.com/inflatable-cookie/swallowtail/pull/8

## Next

Awaiting review. Merge is operator-authorised. First-proof stays planned.
