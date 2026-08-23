# 2026-08-23 g04.049 OpenAI Background Service Tier Compiled

## Change

- reassessed the remaining promoted per-route feature inventory after g04.048
- rejected Bedrock Runtime as the immediate lane because its exact-pin SDK
  identity mismatch requires separate operator-authorised currentness work
- selected `openai.background` Responses `service_tier`
- compiled g04.049 and cards 136-138 as one serial evidence-first worker lane
- reserved Research 196 and the route-local closeout before dispatch

## Decision

Current official OpenAI Responses create and retrieve references place
`service_tier` on the same request and returned response object used by the
existing background route. They name omission/project-default behavior,
standard, Flex, Fast/Priority, and access-controlled Ultrafast processing, and
warn that the returned tier can differ from the requested tier.

Card 136 must freeze the complete current enum, exact `gpt-5.6` applicability,
access gates, ordinary/detached/reconciled profile truth, and the observation
boundary. Cards 137-138 may continue only for a non-empty Research 196
deliver-now set. No portable capability, shared checkpoint, service-tier
claim, or implementation was introduced during compilation.

## Next

Execute g04.049 cards 136-138 serially in one isolated worker worktree and open
one PR.
