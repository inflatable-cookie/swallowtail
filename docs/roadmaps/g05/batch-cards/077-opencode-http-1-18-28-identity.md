# 077 OpenCode HTTP 1.18.28 Identity

Status: complete; compatible extension admitted through `1.18.28`; production claims unchanged; card 078 remains serial
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../028-opencode-http-1-18-28-useful-newer.md`
Depends on: Contract 029; Research 284; frozen OpenCode `1.18.20` corpora; official npm/GitHub `1.18.28`

## Goal

Freeze exact official identity and mapped artifact deltas for every published
OpenCode stable `1.18.21..=1.18.28`, then name one Contract 029 segment outcome
without changing production claims or executing provider work.

## Scope

1. Re-probe npm `opencode-ai` latest and GitHub `anomalyco/opencode` stable tag
   consensus. Record publication times, package digests, tag commits, and the
   installed host `opencode --version` observation without updating it.
2. Extract every published hop after `1.18.20` into `/tmp`. Build exact file
   inventories and SHA-256 ledgers. Do not infer compatibility from changelog.
3. Compare every changed file feeding the selected OpenAPI, HTTP/SSE events,
   model/session catalogue, import/history/reconciliation/delete, structured
   run, interactive session, callback, usage, and detachment behavior.
4. Bound added or changed files that cannot affect mapped behavior with exact
   reasons. Preserve historical gaps and fixtures.
5. Freeze Research 285 and a secret-free `opencode-1.18.28` identity corpus
   with mutation-sensitive delta-ledger tests.
6. Recheck official latest after repairs and immediately before the identity
   commit. Add pre-commit stables as further hops under Contract 029; a stable
   published after the identity commit remains `UnverifiedNewer`.
7. Record one outcome: compatible extension, private milestone, new facade, or
   stop. Name card 078 only for an admitted segment.

## Out Of Scope

Production claim edits; `selection.rs`; live server, prompt, login, install, or
host update; new route operations; OpenCode web-search reopening; Contract 061
Candidate L; other currentness families; release work.

## Acceptance Criteria

- official npm/GitHub identity and every intermediate hop are reproducible
- mapped file changes are classified from artifacts, not release prose
- fixture assertions fail when exact changed/unchanged sets drift
- current production claims are byte-identical in the identity commit
- one honest segment outcome is recorded

## Validation

- `cargo fmt -p swallowtail-adapter-opencode -- --check`
- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:roadmaps:numbers`
- `git diff --check`

## Review Oracle

Invariant: identity and mapped deltas are proved before claims move.

Smallest counterexample: one published hop omitted, a changed mapped file left
unclassified, a compatibility conclusion drawn only from changelog, or a
production claim edited in this commit.

## Auto-Continuation

Yes, to card 078 only after an admitted segment is recorded.

## Stop Conditions

Channel disagreement; major-line reset; identity disagreement; mapped
surface/capability/authority change without deterministic mapping; new public
operation or facade required; provider contact needed.

## Result

Compatible extension admitted. Research 285 and the secret-free
`opencode-1.18.28` corpus freeze official npm/GitHub identity for every
published hop `1.18.21..=1.18.28`, exact npm and implementation-source tree
deltas, two bounded unselected OpenAPI changes, and mapped run-internal changes.
Selected route declarations and handlers remain byte-identical. The finite
300-second upstream response/chunk timeout default at `1.18.27` closes through
existing mapped failure and detachment handling and requires no new adapter
operation or authority. Production claims and historical fixtures are
unchanged. Serial card 078 may apply the proved `surface-19` extension.
