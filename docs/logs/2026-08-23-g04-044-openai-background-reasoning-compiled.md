# 2026-08-23 g04.044 OpenAI Background Reasoning Correction Compiled

## Change

- promoted named follow-up `g04.043-R1` into g04.044
- compiled cards 122-123 as one serial correction and acceptance lane
- reused promoted Research 191 instead of repeating the exact-model evidence
- reserved the route-local closeout before dispatch

## Decision

Research 191 proves exact GPT-5.6 reasoning
`none|low|medium|high|xhigh|max`. The current OpenAI background guide and
validator additionally admit `minimal`. Contract 040 forbids nearest-value,
default, or fallback substitution, so the correction removes only that
route-local admission and rejects it before effects.

The mapping change receives a new exact opaque facade point and private
behavior revision under Contract 029. Removing previously guaranteed behavior
is breaking under Contract 036, so the eventual shared closeout must record a
next-minor disposition. This lane does not select or publish a release.

## Next

Execute g04.044 cards 122-123 in one isolated worker worktree and open one PR.
