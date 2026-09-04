# g05.028 OpenCode HTTP 1.18.28 Useful Newer

Status: complete; card 078 qualified compatible `surface-19` through `1.18.28`
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Depends on: Contract 029; Research 284; qualified OpenCode HTTP `1.18.20`; tagged `v0.4.0`
Vision tags: route currentness, OpenCode HTTP, compatibility

## Purpose

Qualify current official npm/GitHub `opencode-ai` `1.18.28` for the exact
`opencode.http` / `opencode.server` family. Preserve identity-before-claim,
the `1.14.48` baseline, every historical segment and gap, the existing HTTP/SSE
facade, and `AllowUnverified`.

Research 284 ranked this family first: official npm and GitHub agree on
`1.18.28`; the installed host is `1.18.18`, already inside the qualified
`1.14.48..=1.18.20` range; and eight published patch hops remain above the
ceiling without an active stop or deferral. The official latest was re-probed
at promotion and remained `1.18.28`.

## Runway

1. Card 077 freezes every published hop `1.18.21..=1.18.28` from official npm
   and GitHub artifacts, compares all shipped files feeding mapped HTTP/SSE,
   session-management, callback, usage, and lifecycle behavior, and writes
   Research 285 plus a secret-free `1.18.28` fixture corpus. No claim changes.
2. Apply Contract 029's In-Run Latest Movement rule before the identity commit:
   add and recompute any newly published stable hop; stop only for a mapped
   surface/capability/authority change, major-line reset, or channel conflict.
3. Continue to card 078 only for an admitted compatible segment or private
   milestone named by card 077.
4. Card 078 changes only the admitted claim segment and matching exact
   downstream truth, then stops for exact-head review.

## Boundary

One family only. No OpenCode provider contact, prompt, login, live server,
install, host update, new operation, web-search reopening, Contract 061
Candidate L projection, Gemini deferral lift, or release work.

## Batch Cards

- [077 OpenCode HTTP 1.18.28 Identity](batch-cards/077-opencode-http-1-18-28-identity.md) — complete; compatible `surface-19` admitted
- [078 OpenCode HTTP 1.18.28 Claim](batch-cards/078-opencode-http-1-18-28-claim.md) — complete; qualified through `1.18.28`

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

| Field | Card 077 |
| --- | --- |
| Readiness | ready |
| Prerequisites | Research 284; frozen `opencode-1.18.20` and historical compatibility corpora; current `main`; npm/GitHub consensus on official `1.18.28` |
| Completion conditions | Research 285 with official identity for every hop `1.18.21..=1.18.28`, host observation, deterministic mapped artifact-tree ledger, one segment outcome, and zero claim edits; card result filled; named validation green |
| Owned mutable paths | `docs/research/285-*.md`; `docs/research/README.md` one index line; `crates/swallowtail-adapter-opencode/tests/**` identity fixtures and delta-ledger tests only; card 077 result/status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/roadmaps/standing-lanes.md`, `docs/logs/README.md`; coordinator edits these at closeout |
| Forbidden paths | `crates/swallowtail-adapter-opencode/src/**`; existing historical fixture contents; every other crate; route/feature matrices; guides; `CHANGELOG.md`; contracts; architecture; Candidate L projection surfaces |
| Approved concurrent siblings | g05.009 cards 074, 075, and 076 |
| Serial edges | card 078 follows only after card 077 records an admitted segment |
| Worker capability class | evidence-first identity worker; npm/GitHub artifact download and deterministic tree comparison; Rust fixture authoring; no provider credentials |
| Acceptance evidence | official npm metadata/tarballs and GitHub tags; exact per-hop file inventory/digests; mapped OpenAPI and implementation-source deltas; frozen host observation; mutation-sensitive tests |
| Review oracle | identity evidence and zero claim edits in one commit; smallest counterexample is an uninspected changed mapped file, a changelog-only compatibility claim, or a moved selection constant |
| Stop conditions | channel disagreement; major-line reset; mapped surface/capability/authority change without deterministic mapping; identity disagreement; live evidence becomes necessary |
| Escalation owner | operator via Chatterbox for policy/authority; coordinator for mechanical blockers |

### Card 078 Manifest

Promoted planning commit: the `main` commit that introduces this section.
Card 077 recorded the admitted compatible `surface-19` segment through
Research 285, so card 078 is the serial follow-on.

| Field | Card 078 |
| --- | --- |
| Readiness | ready |
| Prerequisites | card 077 merged at `99e91aa8` with the admitted `surface-19` extension; Research 285 and the `opencode-1.18.28` corpus on `main` |
| Completion conditions | `OPENCODE_LATEST_QUALIFIED_VERSION` raised only to the admitted ceiling; every published hop `1.18.21..=1.18.28` plus the first later unverified point tested; claim fixtures, route and feature matrices, prepared guide, architecture ceilings, `CHANGELOG.md` `[Unreleased]`, standing-lane claim text, and one claim log updated; identity and claim as two commits in one PR; pre-push official-latest recheck applied under Contract 029's In-Run Latest Movement rule without reopening the frozen identity segment |
| Owned mutable paths | `crates/swallowtail-adapter-opencode/src/selection.rs` and its selection tests; `crates/swallowtail-adapter-opencode/tests/**` claim fixtures; `docs/guides/opencode-http-prepared-integration.md`; `docs/guides/provider-route-matrix.md` and `provider-solution-feature-matrix.csv` OpenCode cells only; architecture ceiling lines naming this bound; `CHANGELOG.md` `[Unreleased]`; the standing-lane OpenCode claim paragraph; one new `docs/logs/` claim entry plus its index line; card 078 result and status; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, the rest of `docs/roadmaps/standing-lanes.md`; coordinator edits these at closeout |
| Forbidden paths | OpenCode Contract 061 projection surfaces and Candidate L rows; web-search work; every other crate; contracts; historical fixture contents; any public API change |
| Approved concurrent siblings | g05.009 cards 074, 075, and 076 |
| Serial edges | none after card 077 |
| Worker capability class | Rust claim worker with fixture discipline; no provider credentials |
| Acceptance evidence | selection tests over every hop and the synthetic later point; matrix, guide, changelog, standing-lane, and log agreement with `selection.rs`; focused, package-affected, route, docs, and Northstar gates green |
| Review oracle | only the admitted segment changes; the smallest counterexample is a widened or narrowed range, a lost gap, or a guide or matrix cell that disagrees with the selection constant |
| Stop conditions | evidence requires a private milestone or new revision the identity card did not admit; official latest moves after the identity commit (record `UnverifiedNewer`, do not reopen); any public API change becomes necessary |
| Escalation owner | operator via Chatterbox for claim policy; coordinator for mechanical blockers |

## Acceptance

- [ ] official identity is reproducible for every published hop after `1.18.20`
- [ ] every changed shipped file feeding mapped behavior is classified
- [ ] identity evidence lands before any claim edit
- [ ] historical segments, gaps, claim IDs, and unrelated OpenCode work survive
- [ ] only an admitted segment reaches card 078
