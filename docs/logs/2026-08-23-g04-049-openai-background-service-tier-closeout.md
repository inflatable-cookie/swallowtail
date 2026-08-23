# 2026-08-23 g04.049 OpenAI Background Service Tier Closeout

Status: stopped after evidence; PR pending
Owner: Tom
Milestone: g04.049
Cards: 136 complete; 137-138 blocked
Branch: `t3code/read-background-service-tier-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-12466bfd`
Base: `8d49f7049e4372fc304580a5f75ce7d77983ca45` (`origin/main` at dispatch)
PR: pending
Review: awaiting worker evidence and PR
Merge: none; worker must not merge

The launcher-provided worktree and branch differ from the handoff
placeholders. They were used as supplied.

## Outcome

Card 136 is complete as an evidence stop. Research 196 freezes current
official OpenAI Responses create, retrieve, streaming-events schema, Fast
mode, Flex, background, and exact `gpt-5.6-sol` surfaces with secret-free
specimens and digests. The complete current enum is
`auto|default|flex|scale|priority|fast|ultrafast`. Omission behaves as
project-configured `auto`. `fast` is a GPT-5.6 request alias of returned
`priority`. `ultrafast` is access-controlled. `scale` is schema-only. The
returned tier may differ from the request, including Fast ramp-rate
downgrade to `default`.

The route's public API-key payg profile does not prove project settings or
tier enrollment. Current `ResponseSnapshot` and portable
`ProviderObservation` cannot expose returned-tier observation without a new
adapter-local or shared API. `ProviderRunCheckpoint` cannot retain selected
or returned tier, so detachment and reconciliation profiles are withheld.
No value/profile is dispatch-only-safe under those gaps. Deliver-now rows:
none. Cards 137 and 138 were not executed.

## Route-Local Surfaces

Changed:

- `docs/research/196-openai-background-service-tier-evidence.md`: promoted
  evidence stop with official source digests, secret-free specimens, exact
  enum/access/observation/lifecycle dispositions, and empty deliver-now set
- `docs/roadmaps/g04/049-openai-background-service-tier.md`: stopped after
  card 136; cards 137-138 blocked; current facade and omitted-create claim
  retained
- `docs/roadmaps/g04/batch-cards/136-openai-background-service-tier-evidence.md`:
  complete with evidence-stop closeout
- `docs/roadmaps/g04/batch-cards/137-openai-background-service-tier-binding.md`:
  blocked; not executed
- `docs/roadmaps/g04/batch-cards/138-openai-background-service-tier-acceptance.md`:
  blocked; not executed
- this closeout log

Unchanged: `crates/swallowtail-adapter-openai/**`, OpenAI route fixtures,
`docs/guides/openai-background-prepared-integration.md`, the unreleased
package API baseline, and all shared surfaces.

## Shared-Surface Closeout

Recorded here for orchestrator merge closeout; not applied on this branch:

- architecture and route/feature matrices remain unchanged because no
  service-tier behavior shipped
- `CHANGELOG.md` remains unchanged because the lane changed no production
  behavior
- programme/front doors and the sole Next Task stay on the dispatch text
  until orchestrator merge closeout
- batch-card, research, and log indexes still describe card 136 as ready /
  Research 196 as reserved until orchestrator updates them
- Contract 029 currentness remains in its standing lane
- matrix assertions and release baselines remain unchanged

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-openai`
- `effigy validate:focused swallowtail-adapter-openai` — 54 tests passed
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

`effigy doctor` reproduces the inherited baseline: 374 god-file findings
(329 warnings, 45 errors) and one generated-in-src warning. This lane added
no code finding and no `PAPERCUTS.md` entry. Card 137/138 binding-only
checks have no subject because no binding was admitted.

## Unresolved

A later lane may reopen the candidate only with exact official or provider
evidence for this caller's access/enrollment, a route-local observation path
that keeps requested and returned tier distinct without a shared portable
API, and durable selected/returned truth across ordinary, detached, and
reconciled profiles, or an explicit ordinary-run-only subset that does not
need those surfaces. No live provider proof was attempted. Contract 029
currentness was not changed.
