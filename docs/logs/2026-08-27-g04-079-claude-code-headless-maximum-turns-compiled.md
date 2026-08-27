# 2026-08-27 g04.079 Claude Code Headless Maximum Turns Compiled

Status: complete
Owner: Tom

## Changed

- reassessed the remaining per-route feature inventory after g04.078
- selected Claude Code headless maximum agentic turns as the next evidence
  candidate
- compiled g04.079 and serial cards 219-221
- reserved Research 226
- kept delivery conditional on exact qualified-artifact support, native loop
  enforcement, and explicit-argv precedence over the ambient env equivalent

## Why This Lane

`claude-code.headless` already owns one read-only Plan-mode child, one bounded
structured-run lifecycle, fixed tools, exact selected model/reasoning, stream
decoding, deadline, terminal mapping, and joined cleanup. Current official
documentation names `--max-turns` as a positive print-mode maximum over
agentic tool-use turns and names `CLAUDE_CODE_MAX_TURNS` as its lower-precedence
environment equivalent.

The exact qualified `2.1.220..=2.1.241` artifacts do not inherit those mutable
claims. The frozen `2.1.241` help specimen does not advertise the flag, while
current docs say help is incomplete. Research 226 must settle exact support,
numeric parsing, counted-turn meaning, loop enforcement, terminal subtype, and
environment precedence before a typed binding exists.

Autocompact was not selected because ambient environment can override its CLI
and the route cannot inspect that approved environment. Fast and spend controls
also carry account/billing authority. Agent/team controls change topology.
Maximum turns has a narrower explicit-argv seam and a direct native terminal
shape, but remains evidence-gated.

## Deliberate Exclusions

Portable budgets, output-token/tool/cost/wall-time controls, autocompact, Fast,
Ultracode, schema, advisor, agents/teams, fallback, permission changes,
response-only/ACP work, live provider work, currentness, release, merge,
generation rollover, and g04 closure remain outside the lane.

## Execution Shape

One serial worker lane:

1. card 219 freezes exact evidence and promotes Research 226;
2. card 220 runs only for a non-empty exact deliver-now set;
3. card 221 proves route-local acceptance and closes the lane.

The evidence card may stop the milestone honestly. Parser acceptance without
exact enforcement and terminal truth produces an empty deliver-now set.

## Next

Execute g04.079 cards 219-221 serially. Stop after card 219 if Research 226 is
empty or any decision gate fires. Keep g04 open. Contract 029 currentness
remains standing.
