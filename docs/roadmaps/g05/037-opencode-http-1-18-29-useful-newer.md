# g05.037 OpenCode HTTP 1.18.29 Useful Newer

Status: complete; card 136 qualified compatible `surface-19` through `1.18.29`
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Depends on: Contract 029; Research 285; qualified OpenCode HTTP `1.18.28`; g05.028
Vision tags: route currentness, OpenCode HTTP, compatibility

## Purpose

Qualify current official npm/GitHub `opencode-ai` `1.18.29` for the exact
`opencode.http` / `opencode.server` family. Preserve identity-before-claim,
the `1.14.48` baseline, every historical segment and gap, the existing HTTP/SSE
facade, and `AllowUnverified`.

After g05.028, official `1.18.29` remained `UnverifiedNewer`. This family
covers that single published hop. Official latest was re-probed at identity
and remained `1.18.29`.

## Runway

1. Card 135 freezes official npm/GitHub `1.18.29` against the `1.18.28`
   ceiling, compares shipped files feeding mapped HTTP/SSE behavior, and writes
   Research 292 plus a secret-free `1.18.29` fixture corpus. No claim changes.
2. Apply Contract 029's In-Run Latest Movement rule before the identity commit:
   add and recompute any newly published stable hop; stop only for a mapped
   surface/capability/authority change, major-line reset, or channel conflict.
3. Continue to card 136 only for an admitted compatible segment or private
   milestone named by card 135.
4. Card 136 changes only the admitted claim segment and matching exact
   downstream truth, then stops for exact-head review.

## Boundary

One family only. No OpenCode provider contact, prompt, login, live server,
install, host update, new operation, web-search reopening, Contract 061
Candidate L projection, Gemini deferral lift, or release work.

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

| Field | Card 135 |
| --- | --- |
| Readiness | complete |
| Prerequisites | Research 285; frozen `opencode-1.18.28` and historical compatibility corpora; current `main`; npm/GitHub consensus on official `1.18.29` |
| Completion conditions | Research 292 with official identity for hop `1.18.28` → `1.18.29`, host observation, deterministic mapped artifact-tree ledger, one segment outcome, and zero claim edits; card result filled; named validation green |
| Owned mutable paths | `docs/research/292-*.md`; `docs/research/README.md` one index line; `crates/swallowtail-adapter-opencode/tests/**` identity fixtures and delta-ledger tests only; card 135 result/status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/generation-index.md`, `docs/roadmaps/standing-lanes.md`, `docs/logs/README.md`; coordinator edits these at closeout |
| Forbidden paths | `crates/swallowtail-adapter-opencode/src/**`; existing historical fixture contents except load-bearing historical-key assertions; every other crate; route/feature matrices; guides; `CHANGELOG.md`; contracts; architecture |
| Approved concurrent siblings | g05.035 card 114 and disjoint g05.029/g05.034 owners |
| Serial edges | card 136 follows only after card 135 records an admitted segment |
| Worker capability class | evidence-first identity worker; npm/GitHub artifact download and deterministic tree comparison; Rust fixture authoring; no provider credentials |
| Acceptance evidence | official npm metadata/tarballs and GitHub tags; exact per-hop file inventory/digests; mapped OpenAPI and implementation-source deltas; frozen host observation; mutation-sensitive tests |
| Review oracle | identity evidence and zero claim edits in one commit; smallest counterexample is an uninspected changed mapped file, a changelog-only compatibility claim, or a moved selection constant |
| Stop conditions | channel disagreement; major-line reset; mapped surface/capability/authority change without deterministic mapping; identity disagreement; live evidence becomes necessary |
| Escalation owner | operator via Chatterbox for policy/authority; coordinator for mechanical blockers |

### Card 136 Manifest

Promoted planning commit: the `main` commit that introduces this section.
Card 135 recorded the admitted compatible `surface-19` segment through
Research 292, so card 136 is the serial follow-on.

| Field | Card 136 |
| --- | --- |
| Readiness | complete |
| Prerequisites | card 135 identity evidence with the admitted `surface-19` extension; Research 292 and the `opencode-1.18.29` corpus |
| Completion conditions | `OPENCODE_LATEST_QUALIFIED_VERSION` raised only to the admitted ceiling; published hop `1.18.29` plus the first later unverified point tested; claim fixtures, route and feature matrices, prepared guide, architecture ceilings, `CHANGELOG.md` `[Unreleased]`, standing-lane claim text, and one claim log updated; identity and claim as two commits in one PR; pre-push official-latest recheck applied under Contract 029's In-Run Latest Movement rule without reopening the frozen identity segment |
| Owned mutable paths | `crates/swallowtail-adapter-opencode/src/selection.rs` and its selection tests; `crates/swallowtail-adapter-opencode/tests/**` claim fixtures; `docs/guides/opencode-http-prepared-integration.md`; `docs/guides/provider-route-matrix.md` and `provider-solution-feature-matrix.csv` OpenCode version cell only; architecture ceiling lines naming this bound; `CHANGELOG.md` `[Unreleased]`; the standing-lane OpenCode claim paragraph; one new `docs/logs/` claim entry plus its index line; card 136 result and status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/generation-index.md`, the rest of `docs/roadmaps/standing-lanes.md` |
| Forbidden paths | OpenCode Contract 061 projection surfaces and Candidate L rows; web-search work; every other crate; contracts; historical fixture contents except restored historical-key assertions; any public API change |
| Approved concurrent siblings | g05.035 card 114 and disjoint g05.029/g05.034 owners |
| Serial edges | none after card 135 |
| Worker capability class | Rust claim worker with fixture discipline; no provider credentials |
| Acceptance evidence | selection tests over the hop and the synthetic later point; matrix, guide, changelog, standing-lane, and log agreement with `selection.rs`; focused, package-affected, route, docs, and Northstar gates green |
| Review oracle | only the admitted segment changes; the smallest counterexample is a widened or narrowed range, a lost gap, or a guide or matrix cell that disagrees with the selection constant |
| Stop conditions | evidence requires a private milestone or new revision the identity card did not admit; official latest moves after the identity commit (record `UnverifiedNewer`, do not reopen); any public API change becomes necessary |
| Escalation owner | operator via Chatterbox for claim policy; coordinator for mechanical blockers |

## Acceptance

- [x] official identity is reproducible for the published hop after `1.18.28`
- [x] every changed shipped file feeding mapped behavior is classified
- [x] identity evidence lands before any claim edit
- [x] historical segments, gaps, claim IDs, and unrelated OpenCode work survive
- [x] only an admitted segment reaches card 136
