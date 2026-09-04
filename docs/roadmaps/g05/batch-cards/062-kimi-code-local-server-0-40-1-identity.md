# 062 Kimi Code Local Server 0.41.0 Identity (retargeted from 0.40.1)

Status: complete; stop. Official `0.41.0` identity corroborated; Bash `cwd` uncontained; ceiling stays `0.38.0`; card 063 not admitted
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../026-kimi-code-local-server-0-40-1-useful-newer.md`
Depends on: Contract 029; Contracts 017 and 023; Research 270 and 276; tagged `v0.4.0`; official stable `0.41.0`

## Goal

Freeze exact official `@moonshot-ai/kimi-code` `0.41.0` identity for the
`kimi-code.local-server` family, classify its selected REST/WebSocket v2
surfaces against the frozen `0.38.0` corpus, and answer the process-authority
question raised by the `0.40.0` Bash `cwd` change, without changing a claim or
executing a downloaded binary.

## Scope

1. Recheck npm, GitHub tag/commit, tarball integrity, platform archives and
   sidecars, extracted artifacts, and selected git blobs for `0.41.0`. Do not
   infer identity from registry `latest` alone.
2. Record publication adjacency through `0.39.0`, `0.39.1`, `0.40.0`, and
   `0.40.1`, and the first unpublished later stable. The `0.40.1` identity
   already collected in this lane is retained as adjacency evidence.
3. Keep host `0.34.0` observation-only. Do not install, update, replace, or
   run it.
4. Recompute rather than trust the frozen `0.38.0` local-server corpus
   digests.
5. Compare the selected `kimi web` and `kap-server` surfaces that feed the
   route: auth middleware, REST model catalog, ws-control, session and turn
   events, approval and question exchange, archive and restore, heartbeat
   ping, and remote-terminal removal. Derive mapped and unmapped ledgers from
   production authority, not changelog prose alone.
6. Trace the `0.40.0` Bash tool `cwd` restriction removal as an authority
   question before a revision-label question. Name, from code and Contracts
   017 and 023, whether any Swallowtail control, server profile, or provider
   boundary contains it for a local-server client, or whether nothing does.
   Compare the answer with the A2 ACP reasoning and say whether the same
   conclusion applies to this transport.
7. Keep `kimi-code.acp` and `kimi-code.headless` claims, corpora, and
   conclusions untouched; record any installed-harness delta as an
   observation only.
8. Add Research 282 and one secret-free `0.41.0` local-server identity corpus
   with a delta-ledger test.
9. Commit identity evidence before any selection, matrix, guide, changelog,
   or standing-lane claim edit.
10. Record exactly one outcome: compatible extension, private milestone, new
    revision, or stop, with the authority trace as its first justification.

## Out Of Scope

Production claim edits, `local_server/selection.rs`, ACP or headless changes,
Kimi Platform Chat, another family, Gemini, provider contact, model request,
authentication, catalogue or session work, install, host update, live
server or probe, projection, skill, papercut fixes, g05.009 card 034, release,
or execution of downloaded official binaries.

## Acceptance Criteria

- official identity is corroborated through independent official channels
- the `0.39.0` web/server deltas and the `0.40.0` `cwd` change each have an
  explicit contained/uncontained answer with the tracing evidence
- mapped and material unmapped additions are explicit
- current production claims are byte-for-byte unchanged in this commit
- fixture provenance, digests, and negative boundaries are load-bearing
- card 063 continues only for an admitted segment; a stop names the ceiling
  that stays and the reopen trigger, if any

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: this commit changes evidence only; every production claim byte is
unchanged and the authority conclusion names a traced control or its absence.

Smallest counterexample: a moved selection constant, a widened or narrowed
range, a fixture whose digest was copied rather than recomputed, or a
"compatible extension" verdict on the `cwd` change without a named containing
boundary.

## Auto-Continuation

Yes, to card 063 only after an admitted segment is recorded. A stop ends the
milestone at this card and returns to the coordinator for closeout.

## Retarget Record

The first run froze `0.40.1` identity cleanly, then found official latest
had moved to `0.41.0` (published 2026-09-04T11:01:04.740Z) and stopped on
the named condition without committing. On 2026-09-04 the operator chose to
retarget this card to `0.41.0` rather than qualify a point behind official
latest or stop at the `0.38.0` ceiling. If latest moves again before push,
stop and ask; a second move raises the freeze-at-dispatch policy question to
the operator instead of another retarget.

## Stop Conditions

Official latest moves again during the run; npm and GitHub identity disagree; the
`cwd` change widens local process authority with no contained boundary; a
selected surface changed without a deterministic provider-neutral mapping; a
new driver/facade revision is required.

## Result

Stop. Official `@moonshot-ai/kimi-code` `0.41.0` identity is corroborated from
npm and GitHub. Frozen `0.38.0` local-server corpus digests recompute.
Selected REST/WebSocket v2 protocol files are comment-only at `0.39.0` and
byte-identical through `0.41.0`. The `0.40.0` Bash `cwd` restriction removal
is uncontained for this transport: `RuntimeWorkspaceView.resolve` no longer
asserts workspace membership, Swallowtail launches `kimi web` under
`AmbientHost` with no process cwd, and Contracts 017 and 023 name no
containing control. Same conclusion as ACP A2, different mechanism.
Production claims are unchanged. Ceiling stays `0.38.0`. Reopen when a named
Swallowtail or provider containing boundary exists for Bash `cwd` on the
local-server client. Card 063 is not admitted. Evidence:
[Research 282](../../../research/282-kimi-code-local-server-0-41-0-identity.md)
and `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.41.0/`.
