# 221 Claude Code Headless Maximum Turns Acceptance

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.079 Claude Code Headless Maximum Turns](../079-claude-code-headless-maximum-turns.md)
Depends on: card 220

## Goal

Prove exact maximum-turn argv, preserved omission, fail-closed value/version
gating, native limit-reached terminal truth, and unchanged headless lifecycle,
then produce one review-ready route-local closeout.

## Scope

1. Add deterministic preparation, command, decoder, fixture, facade, and
   scenario coverage for every Research 226 deliver-now row and rejection
   boundary.
2. Assert exact version/value membership, immutable prepared/driver agreement,
   canonical one-time argv, and explicit-argv precedence over the ambient env
   equivalent without reading or mutating host environment state.
3. Prove omission emits no `--max-turns` and retains the complete existing
   command and approved-environment handoff.
4. Prove zero, overflow, unqualified versions, stale evidence, raw values,
   duplicate intent, and mismatched low-level use reject before process work.
5. Prove the exact native limit-reached stream/result subtype, `num_turns`,
   usage, stop-reason, text, exit, failure mapping, activity, and cleanup that
   Research 226 admits. Do not fabricate provider success for a native bound.
6. Keep requested, prepared, dispatched, parser-accepted, enforced, reached,
   and observed truth separate in tests and docs.
7. Assert model/reasoning selection, Plan/read-only authority, fixed tools,
   strict MCP, provider configuration, deadline, cancellation, retention,
   process ownership, and joined cleanup remain unchanged.
8. Update the Claude guide, relevant route/feature matrices, changelog,
   Research 226, roadmap/card state, programme, triage, logs, indexes, and sole
   Next Task.
9. Regenerate and review the API baseline only when the public surface changes.
10. Run the complete named validation once for the batch. Record inherited
    doctor findings and exact drift.

## Acceptance Criteria

- [x] every admitted exact row dispatches one canonical `--max-turns N`
- [x] omission and every existing model/reasoning row retain exact prior
      behavior
- [x] unsupported version/value and prepared/driver mismatches fail before
      process work
- [x] native limit reached maps exactly and still joins all owned work before
      resource release
- [x] no output-token, tool, cost, wall-time, provider-state, configuration,
      retention, isolation, or lifecycle claim widens
- [x] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy check:examples
effigy package:api
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy doctor
git diff --check
```

## Stop Conditions

- any selected row cannot prove exact immutable dispatch and native enforcement
- omission, stream decoding, activity, terminal mapping, or lifecycle regresses
- acceptance requires live provider work, environment mutation, authority
  widening, or unrelated repair

## Out Of Scope

- another feature/route, portable budget work, currentness, publication, merge,
  generation rollover, or g04 closure
