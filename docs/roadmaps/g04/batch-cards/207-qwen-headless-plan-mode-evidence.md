# 207 Qwen Headless Plan-Mode Evidence

Status: ready
Owner: Tom
Created: 2026-08-26
Milestone: [g04.075 Qwen Headless Plan Mode](../075-qwen-headless-plan-mode.md)
Depends on: Research 173, 189, 198, and 216; Contracts 012, 023, 029, 033, 034

## Goal

Determine whether exact maintained Qwen Code points apply
`--approval-mode plan` to every selected headless child with behavior
equivalent to portable `HarnessMode::Plan`. Promote an honest empty set if the
fixed argument cannot be tied to complete immutable Plan behavior without
provider work.

## Work

1. Reuse and verify exact package/source identities for `0.21.15`, `0.22.0`,
   and `0.22.1`. Digest every decisive parser, config, approval, tool, stream,
   session, and child-launch source. Current docs/main may corroborate only.
2. Freeze `--approval-mode` parsing: canonical value spelling, shared
   `APPROVAL_MODES`, missing/empty/invalid/repeated values, aliases,
   precedence, option placement, omission, and local parse failures.
3. Trace `plan` from argv through settings/config construction, safe-mode
   application, approval policy, prompt/system behavior, tool registry and
   filtering, stream-json initialization, user-message dispatch, and terminal
   result.
4. Prove exact Plan semantics against Contract 034. Inventory every
   write/process/network/shell/tool seam, mode-change path, slash command,
   workflow/subagent/team path, hook/extension/MCP/config source, and any way a
   child could widen from Plan without a new consumer operation.
5. Keep existing `--safe-mode`, `--core-tools`, `--exclude-tools`, read-only
   working resource, and `AmbientHost` separate from Plan behavior. Determine
   whether Plan conflicts with, shadows, or changes any selected safeguard.
6. Trace structured runs, reasoning-control runs, first interactive turns,
   continued turns with private provider session id, explicit resume, and
   fresh context-losing replacement. Prove each new child receives one fixed
   selected mode for its full lifetime.
7. Freeze output and observation truth. Record whether any stream event
   confirms requested or effective approval mode. Do not infer observation
   from argv, parser acceptance, prompt text, or tool absence.
8. Freeze ambient config precedence, mutation, persistence, transcripts,
   session state, cancellation, deadline, failure, terminal, and joined cleanup.
9. Audit prepared input, capability profile, plan/evidence, request, driver,
   command builder, fixtures, guide, feature matrix, examples, and API baseline.
   Name the smallest exact portable binding.
10. Prove omission retains exact current `--approval-mode default` bytes and
    behavior across every child shape.
11. Classify each exact route/version/value row as deliver now,
    evidence-gated, intentionally withheld, or not applicable. Keep deprecated,
    unpublished, and `UnverifiedNewer` points separate.
12. Promote Research 222 with an exact deliver-now table or explicit empty set.
    Update the milestone/card state and closeout honestly.

## Acceptance Criteria

- [ ] exact artifact/source identities and decisive digests are frozen
- [ ] parser, precedence, omission, invalid/repeated, and failure truth is
      settled
- [ ] Plan semantics, safe-mode/tool composition, mode-change seams, output,
      persistence, and every child lifecycle have exact dispositions
- [ ] behavioral Plan equivalence is separated from permission, resource,
      isolation, sandbox, shell/process/network, and account authority
- [ ] requested, planned, dispatched, accepted, applied/effective, and observed
      states are not conflated
- [ ] production preparation, plan/evidence, driver, argv, fixtures, docs, and
      API seams are audited
- [ ] Research 222 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-qwen
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
git diff --check
```

Auto-continue to card 208 only when Research 222 admits a non-empty exact
`qwen.headless` `HarnessMode::Plan` row with complete behavior proved without a
provider prompt.

## Stop Conditions

- exact source, applicability, behavioral equivalence, or complete mode/tool
  coverage remains ambiguous
- Plan is parser-only, prompt-only, ambiently overrideable, or can widen during
  one selected child
- any child shape fails to reapply the immutable selected mode
- deterministic proof needs login, account inspection, provider prompting,
  tool execution, paid work, config mutation, or a contract change

## Out Of Scope

- production binding, other approval modes, writable profiles, tool-policy
  changes, live provider work, currentness, release, merge, rollover, or g04
  closure

