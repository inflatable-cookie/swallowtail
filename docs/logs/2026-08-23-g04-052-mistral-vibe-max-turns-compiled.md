# 2026-08-23 g04.052 Mistral Vibe Maximum Turns Compiled

## Change

- reassessed the remaining promoted per-route inventory after g04.051
- selected exact `mistral-vibe.headless` `2.24.2` caller-decreasing maximum
  turns as the next coherent route-local family
- compiled g04.052 and cards 145-147 as one serial evidence-first worker lane
- reserved Research 199 and the route-local closeout before dispatch
- kept g04 active; no generation closure or rollover was authorized

## Decision

The route already emits fixed `--max-turns 8`. Current official documentation
and exact `2.24.2` source expose native pre-turn enforcement, so a smaller
typed caller-selected positive envelope is plausible. Upstream parser breadth
does not settle the public API: zero stops before an assistant turn, negative
values appear accepted, and flag omission is unbounded.

Card 145 must close exact counting, off-by-one behavior, process/stderr/stream/
terminal truth, partial events, cleanup, and the useful public domain. Cards
146-147 continue only for a non-empty Research 199 deliver-now set. Caller
omission stays `--max-turns 8`; output mode, plan agent, trust, workdir,
access, host deadline, cancellation, failure, and cleanup do not change. No
production claim or implementation was introduced during compilation.

## Next

Execute g04.052 cards 145-147 serially in one isolated worker worktree and
open one PR. Stop honestly after card 145 if the exact deliver-now set is empty
or requires shared contract/currentness work. After merge, reconcile this
route-local milestone and reassess the remaining inventory. Keep g04 open
until the operator directs otherwise.
