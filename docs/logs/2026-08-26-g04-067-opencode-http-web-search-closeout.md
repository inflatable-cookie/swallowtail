# 2026-08-26 g04.067 OpenCode HTTP Web Search Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.067
Cards: 187 complete; 188-189 blocked
Branch: `t3code/review-opencode-web-search-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-cfab66d3`
Base: `9f003d9aa17b09ad188080fa2b37203aee00dda5` (`origin/main` at dispatch)
Planning base ancestor: `6409af0c472595a2dcd02a25fff2ddb933da562c`
PR: pending

## Result

Card 187 completed an exact `v1.18.20` source and production-seam audit.
Research 214 admits no deliver-now web-search row. Cards 188 and 189 are
blocked and were not executed. The OpenCode HTTP adapter, prepared facade,
session-create JSON, guide, matrices, and API baseline are unchanged. No
install, server mutation, credential capture, account/backend inspection,
provider prompt, hosted search, or paid operation was used.

## Evidence Stop

Exact tagged source registers native tool `websearch` and asks permission
`websearch` against the query. Last-match session rules can allow, ask, or
deny, and a `*` deny hides the tool. That permission syntax is not a
host-bindable availability row.

`webSearchEnabled` is true only for provider `opencode` or `opencode-go`, or
when attached-server Exa/Parallel runtime flags are set. Execute always POSTs
to Exa or Parallel MCP. Without env override, backend choice is a checksum of
the session id assigned at create time. Existing prepared evidence does not
observe those env/key/reachability facts. Tagged tools docs claim unauthenticated
Exa MCP; they omit Parallel, which source still selects on half of unflagged
sessions.

The compatible shared pair remains `HostApproved` plus `Enabled`, but this
lane cannot bind it without ambient inference. Current structured runs stay
Denied+Disabled. Interactive sessions stay ambient network with Disabled
search. Deny-first session JSON is unchanged.

## Changed Surfaces

- `docs/research/214-opencode-http-web-search-evidence.md`: promoted exact
  tag, permission, visibility, backend, production audit, claim strength, and
  empty deliver-now table
- cards 187-189, g04.067, programme, triage, indexes, this closeout

No production code, public API, shared contract/runtime, guide capability,
matrix, or changelog edit.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-opencode` — 110 tests passed
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code changed. Doctor was not re-run; the inherited 378 god-file
baseline is unchanged by docs-only edits.

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing. Do not compile the next family from this closeout.
