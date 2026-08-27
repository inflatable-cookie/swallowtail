# 2026-08-27 g04.081 Pi SDK Sidecar Reasoning Selection Compiled

Status: complete
Owner: Tom

## Changed

- reassessed the current production feature matrix after g04.080
- selected reasoning selection on `pi.sdk-sidecar` as the next route-local
  evidence candidate
- compiled g04.081 and serial cards 225-227
- reserved Research 228
- kept delivery conditional on exact static model/value membership,
  pre-effect rejection, and effective-state confirmation

## Why This Lane

The historical feature inventory predates much of the delivered programme.
The current matrix still reports `pi.sdk-sidecar` reasoning selection as `No`,
while the source-tagged sidecar already carries an intentionally unused seam:
optional bootstrap `thinkingLevel`, SDK construction forwarding, and
`session.thinkingLevel` in bootstrap/state snapshots.

That seam is stronger than another speculative flag. It is also unsafe to bind
directly: exact Pi 0.84.2 source clamps requested levels to model capability.
Research 228 must therefore freeze a closed provider/model/value table and
prove that every claimed lifecycle can compare the reported effective value
before readiness. A reasoning boolean, emitted thought, or accepted string is
not enough.

## Deliberate Exclusions

Dynamic level changes, cycling, model switching, raw settings, a generic
options map, `pi.rpc`, newer SDK currentness, live provider work, release,
merge, generation rollover, and g04 closure remain outside the lane.

## Execution Shape

One serial worker lane:

1. card 225 freezes exact evidence and promotes Research 228;
2. card 226 runs only for a non-empty exact deliver-now set;
3. card 227 proves route-local acceptance and closes the lane.

The evidence card may stop the milestone honestly. Omission retains existing
Pi default/stored behavior and no portable selection claim.

## Next

Execute g04.081 cards 225-227 serially. Stop after card 225 if Research 228 is
empty or any decision gate fires. Keep g04 open. Contract 029 currentness
remains standing.
