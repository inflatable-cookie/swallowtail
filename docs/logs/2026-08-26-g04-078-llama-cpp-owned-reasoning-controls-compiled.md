# 2026-08-26 g04.078 llama.cpp Owned Reasoning Controls Compiled

Status: complete
Owner: Tom

## Changed

- reassessed the remaining per-route feature inventory after g04.077
- selected exact llama.cpp owned-runtime reasoning selection and budget as the
  next evidence candidate
- compiled g04.078 and serial cards 216-218
- reserved Research 225
- kept delivery conditional on exact model/template applicability and
  preflight-bindable behavior

## Why This Lane

Exact llama.cpp `b10069` exposes `--reasoning on|off|auto` and
`--reasoning-budget -1|0|N`. The owned route already controls that exact server
child, immutable launch plan, operator-supplied model path, context size,
readiness, cancellation, terminal state, and joined cleanup.

This is a credible serving-owned seam, but not yet a model reasoning claim.
The effective behavior depends on GGUF/chat-template capabilities and reasoning
tags. Research 225 must prove that the route can bind or reject those facts
before process work. Parser acceptance alone authorizes no implementation.

Codex Plan-mode effort was not selected. The app-server route already carries
the active model and reasoning effort inside the collaboration-mode preset, so
a second route-local setting would duplicate existing selection rather than
close a clear feature gap.

## Deliberate Exclusions

Portable reasoning APIs, raw values, model selection/download, chat-template
changes, reasoning formats, attached-route inference, live model work,
currentness, release, merge, generation rollover, and g04 closure remain
outside the lane.

## Execution Shape

One serial worker lane:

1. card 216 freezes exact evidence and promotes Research 225;
2. card 217 runs only for a non-empty exact deliver-now set;
3. card 218 proves route-local acceptance and closes the lane.

The evidence card may stop the milestone honestly. A model/template condition
that cannot be bound before process work produces an empty deliver-now set.

## Next

Execute g04.078 cards 216-218 serially. Stop after card 216 if Research 225 is
empty or any decision gate fires. Keep g04 open. Contract 029 currentness
remains standing.
