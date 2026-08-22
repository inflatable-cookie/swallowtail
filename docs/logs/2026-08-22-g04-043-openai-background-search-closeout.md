# 2026-08-22 g04.043 OpenAI Background Search Closeout

Status: stopped after evidence; worker closeout ready for review
Owner: Tom
Milestone: g04.043
Cards: 119 complete; 120-121 blocked
Branch: `t3code/openai-background-search`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-e3fd4917`
Base: `4179bff8a618007214f09bd99c765e65474eba03` (`origin/main` at dispatch)
PR: pending
Review: pending
Merge: none; worker must not merge

## Outcome

Card 119 is complete as an evidence stop. Research 191 freezes current
official OpenAI web-search, Responses, background, model, and response-event
surfaces with secret-free specimens and digests. The docs establish the
provider-owned `web_search` tool, exact `gpt-5.6` feature support, explicit
provider access posture, a positive `max_tool_calls: 1` candidate,
`tool_choice: "auto"`, source inclusion, and the individual background
lifecycle controls.

They do not prove the exact composition required by this route: search events
through background streaming and one reattachment, source/output terminal
truth, search failure/rejection shape, account/project policy, or portable
activity and facade mapping. No deliver-now row exists. Cards 120 and 121
were not executed.

## Route-Local Surfaces

Changed:

- `docs/research/191-openai-background-web-search-evidence.md`: promoted
  evidence stop with official source digests, secret-free specimens, exact
  semantic findings, compatibility classification, and empty deliver-now set
- `docs/roadmaps/g04/043-openai-background-hosted-search.md`: stopped after
  card 119; cards 120-121 blocked; current facade and tool-free claim retained
- `docs/roadmaps/g04/batch-cards/119-openai-background-search-evidence.md`:
  complete with evidence-stop closeout
- `docs/roadmaps/g04/batch-cards/120-openai-background-search-binding.md`:
  blocked; not executed
- `docs/roadmaps/g04/batch-cards/121-openai-background-search-acceptance.md`:
  blocked; not executed
- this closeout log

Unchanged: `crates/swallowtail-adapter-openai/**`, OpenAI route fixtures,
`docs/guides/openai-background-prepared-integration.md`, the unreleased
package API baseline, and all shared surfaces.

## Shared-Surface Closeout Delta

No shared-surface delta is requested from this worker. After review, the
orchestrator must decide whether to record the evidence stop in architecture,
route and feature matrices, programme/front doors, indexes, matrix
assertions, changelog, and the sole Next Task. Those surfaces remain
orchestrator-owned and were not edited here.

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-openai`
- `effigy validate:focused swallowtail-adapter-openai` — 49 tests passed
- `effigy package:verify-affected swallowtail-adapter-openai`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy package:api` — 40 package APIs passed
- `git diff --check`

`effigy doctor` reproduces the inherited baseline: 371 god-file findings
(326 warnings, 45 errors) and one generated-in-src warning. This lane added
no code finding and no `PAPERCUTS.md` entry. Card 120/121 binding-only checks
have no subject because no binding was admitted.

## Unresolved

A later lane may reopen the candidate only with exact official or provider
evidence for the full `gpt-5.6` background composition, search-event and
source continuity across stream/reattach/retrieve, account/project access,
failure and usage truth, and the private facade/activity mapping. No live
provider proof was attempted. Contract 029 currentness remains in its
standing lane and was not changed.
