# 219 Claude Code Headless Maximum Turns Evidence

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.079 Claude Code Headless Maximum Turns](../079-claude-code-headless-maximum-turns.md)
Depends on: g04.055; g04.065 closeout; g04.078 closeout

## Goal

Freeze exact qualified Claude Code parser, precedence, counted-turn,
enforcement, terminal, and observation truth for `--max-turns`, then promote
Research 226 with a non-empty exact deliver-now table or an honest empty set.

## Work

1. Reuse and verify the exact route, driver, compatibility axis, qualified
   window, behavior revision, package identities, artifact digests, and
   headless command from Research 202. Current documentation is a lead only.
2. Freeze the flag's exact support boundary across qualified
   `2.1.220..=2.1.241`. Use official packages/native artifacts and decisive
   implementation evidence; do not infer range membership from endpoint help.
3. Freeze parsing for positive integers plus zero, negatives, signs, whitespace,
   padding, fractions, exponent form, empty, missing, repeated, overflow, and
   trailing junk. Record aliases, separators, diagnostics, stderr, exit status,
   and repeated-value precedence.
4. Freeze `CLAUDE_CODE_MAX_TURNS`: accepted domain, invalid behavior, default
   when absent, CLI precedence when both are present, and whether any settings
   key competes. Do not inspect or mutate the host's approved environment.
5. Trace the selected value from argv through stored options and the agent loop.
   Define exactly what increments the counter: tool-use round trips, assistant
   messages, provider requests, retries, or another unit.
6. Trace limit reached through result subtype, `num_turns`, stop reason, usage,
   assistant/result ordering, text availability, stderr, process exit, and the
   current decoder. Distinguish native provider failure from host deadline and
   cancellation.
7. Determine whether a positive selected value can be ignored, clamped,
   replaced, reset, or bypassed by model behavior, tools, hooks, settings,
   environment, resume state, or provider configuration on this exact
   no-session-persistence route.
8. Audit prepared input/result, immutable plan/evidence, driver command,
   version assessment, stream decoder, activity, fixtures, guide, matrices,
   examples, changelog, and API baseline. Name the smallest closed binding or
   the missing exact fact.
9. Prove omission emits no `--max-turns`, retains exact current argv and
   approved environment, and makes no new unlimited-execution claim.
10. Classify every candidate exact version/value row as deliver now,
    evidence-gated, intentionally withheld, or not applicable. Do not create a
    range from mutable docs or parser acceptance alone.
11. Keep turns distinct from output tokens, tool-call budgets, wall time, cost,
    context size, retries, and portable generation controls. Do not modify
    `claude-code.response-only` or ACP.
12. Promote Research 226 with exact sources, digests, matrices, and a non-empty
    deliver-now table or explicit empty set. Update milestone/card state and
    close out honestly.

## Acceptance Criteria

- [x] exact identity, source/artifact digests, support boundary, parser,
      precedence, and defaults are frozen
- [x] counted-turn, enforcement, limit-reached, stream, exit, and observation
      truth have exact dispositions
- [x] requested, prepared, dispatched, parser-accepted, enforced, reached, and
      observed state remain distinct
- [x] production preparation, driver, decoder, fixtures, docs, and API seams
      are audited
- [x] Research 226 contains a non-empty exact table or honest empty set
- [x] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 220 only when Research 226 admits a non-empty exact row
with deterministic native enforcement and explicit-argv precedence over the
ambient environment.

## Stop Conditions

- exact support, parser, counted-turn, precedence, enforcement, terminal, or
  exit truth remains ambiguous
- a selected value can be ignored, clamped, replaced, or shadowed
- deterministic proof needs provider prompting, account work, paid work,
  environment mutation, installation/update, or a shared contract change

## Out Of Scope

- production binding, portable budget APIs, sibling routes, other Claude Code
  controls, live provider work, currentness, release, merge, rollover, or g04
  closure
