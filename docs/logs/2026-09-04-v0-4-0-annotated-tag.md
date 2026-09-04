# v0.4.0 Annotated Tag And Post-Release Runway

Date: 2026-09-04
Roadmap: `../roadmaps/g05/021-v0-4-0-release-readiness.md`
Card: `../roadmaps/g05/batch-cards/052-v0-4-0-consumer-proof-and-operator-tag-gate.md`

## Result

Annotated source tag `v0.4.0` exists on the canonical repository. The operator
authorized both local tag creation and tag push on 2026-09-04 with the exact
message frozen in the Card 052 closeout. Local and remote peels resolve to
`56f3913ac99af44b6ff45384cfc53a0adea587ba`. The annotated tag object is
`6f398b9f0fedae4215ea7f58fdf04f888871e540`. Immutable `v0.3.3` remains at
`51d186208e75dca4c04f077dd7179ec3c2fafae9`.

The tagged tree is the coordinated 40-package, 49-route pre-1.0 minor. It
includes `pi.sdk-sidecar` and `claude-agent.sdk`. No GitHub Release,
crates.io publication, binary, sidecar, installer, or model artifact exists.

This closeout commit on `main` is not the tag identity. The tag was not
moved.

## Tag CI

Tag-triggered run
https://github.com/inflatable-cookie/swallowtail/actions/runs/33870017023
passed all six jobs at head SHA `56f3913ac99af44b6ff45384cfc53a0adea587ba`
on the first attempt. Pre-tag evidence at the same SHA: workflow-dispatch
run 33853812785, the external source consumer, and one authenticated
Nucleus Agent Chat smoke.

## Reconciled Surfaces

Release note, releases index, root README, Contract 036 tagged identity,
roadmap g05.021, card 052, the g05 README, generation index, standing lanes,
and the roadmaps front door now describe the tagged state. The
feature/currentness freeze is lifted. The resolved Claude SDK shared-lifecycle
triage note was removed; cards 057-061 and the accepted root-only degraded
cleanup decision closed it.

## Post-Release Direction

Operator-confirmed on 2026-09-04: Research 276's first candidate is compiled
as g05.026 Kimi Code local server `0.40.1` useful newer. Card 062 identity is
ready and card 063 claim is gated behind an admitted segment. The `0.40.0`
Bash tool `cwd` restriction removal is an authority question first, so an
honest stop is an acceptable outcome. Antigravity `1.1.24` (parked PR 182)
queues behind it because currentness is serial. The g05.009 Contract 061
provider-operation observation decision remains an operator conversation, not
a worker lane.

## Next

Implement g05.026 card 062 through the coordinator. The dispatch manifest is
in the g05.026 roadmap.
