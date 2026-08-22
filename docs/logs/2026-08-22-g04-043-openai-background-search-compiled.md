# 2026-08-22 g04.043 OpenAI Background Search Compiled

## Change

- reassessed the remaining promoted per-route feature inventory after the Cline
  evidence stop
- selected provider-owned web search on the existing `openai.background`
  Responses route
- compiled g04.043 and cards 119-121 as one serial evidence-first worker lane
- reserved Research 191 and the route-local closeout record before dispatch

## Decision

Current official OpenAI surfaces name `web_search` as the selected Responses
tool for new integrations, use it with `gpt-5.6`, expose a positive maximum
total built-in-tool-call field, and can return complete source evidence. The
existing route already owns exact `gpt-5.6`, public API-key billing, background
execution, streaming, one reattachment, retrieve, cancellation, deletion,
detachment, and reconciliation. Contract 041 and the portable external-search
policy already exist.

That combination makes this a stronger next evidence gate than route families
that still require model mutation, permission widening, experimental process
topology, or new portable vocabulary. It does not prove that search composes
with the route's exact background request or lifecycle.

Card 119 must freeze the exact current request, model, source, activity,
failure, lifecycle, and opaque-facade truth. Cards 120-121 may continue only
for Research 191 deliver-now rows. Search stays provider-owned, bounded,
optional, and independent from reasoning and structured output. No OpenAI
implementation or capability claim was made during compilation.

## Next

Execute g04.043 cards 119-121 in one isolated worker worktree and open one PR.
