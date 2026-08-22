# 2026-08-22 g04.040 Copilot CLI ACP Effort Compiled

## Change

- reassessed the remaining promoted per-route feature inventory after the
  initial five-family sequence
- selected Copilot CLI ACP session effort as the next bounded route/control
  family
- compiled g04.040 and cards 110-112 as one serial worker lane
- reserved Research 188 and the route-local closeout record before dispatch

## Decision

Copilot CLI ACP is the cleanest next exact-transport candidate. Official
current documentation names server-start effort values and says every session
on that server inherits the setting. Swallowtail already owns one child for one
bounded prepared session, so process and session lifetime can align without
ambient configuration mutation or per-turn override.

That current documentation does not itself prove exact package `1.0.80`, and
the route does not select a model. Card 110 therefore owns package, syntax,
value, lifetime, and Contract 040 qualification. Cards 111-112 may continue
only for Research 188 deliver-now rows. A clamp, model-capability gap, facade
change, or empty useful subset returns to the orchestrator.

Qwen effort remains promoted for a later evidence lane because its current
surface is a global setting with provider/model clamp behavior. Cline thinking
is a boolean spawn control, not yet an exact portable effort ladder. Parked
route families were not selected.

## Next

Execute g04.040 cards 110-112 in one isolated worker worktree and open one PR.
