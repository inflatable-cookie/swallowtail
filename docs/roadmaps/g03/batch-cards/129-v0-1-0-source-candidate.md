# 129 v0.1.0 Source Candidate

Status: complete
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 128

## Goal

Produce complete deterministic evidence for one exact clean source commit.

## Scope

1. Run the full source-tag gate from clean canonical history.
2. Compile an isolated consumer against the exact candidate revision.
3. Reconfirm the recorded working-application smoke remains applicable.
4. Record commit, parent, package set, API baseline, dependency graph, and
   validation digests.
5. Make no tag or remote mutation.

## Validation

- complete source-tag candidate selector
- external exact-revision consumer smoke
- clean worktree and remote ancestry check

## Acceptance

- [x] one clean non-root commit contains the complete source-tag candidate
- [x] full release simulation passes from that exact commit
- [x] the external consumer resolves every selected package from exact
      canonical `HEAD`, not a synthetic snapshot
- [x] package set, semantic API, internal topology, dependency lock, commit,
      and parent digests are recorded
- [x] no prepare, tag, push, registry, GitHub Release, authenticated provider,
      or consumer mutation runs

## Evidence

- candidate: the clean completion commit containing this card; its immutable
  SHA is the card 130 handoff input because a commit cannot contain its own
  hash
- parent: `13bcec5124c7db5c9704dadf2df9956cbfe64430`
- implementation base parent: `8bd29856cbb449e1268747f6105b3bbbc3e8cca5`
- branch and remote: `main`, `git@github.com:inflatable-cookie/swallowtail.git`
- package set: 27 entries; digest
  `5716070028ada9e88b1ab233df477a778686fca697f29de5418eeec92229faba`
- semantic API inventory aggregate:
  `96740685be17f721cd241490008f90ba50166128cf168dcf9b8d4f4202eac254`
- internal dependency topology:
  `2fc3ad4e6e61b9519c4923ad35c7891f7f29816ce9ae50eadd73f12d0030e5e0`
- dependency lock:
  `55b097bb2a10056018ac064c83d6075701e94b67346d4e69dfca82041e042d06`
- `effigy release simulate` and `effigy release status --check-gates` pass all
  11 gates; 1,463 tests pass and 11 are skipped
- the isolated source consumer resolves `swallowtail-core`,
  `swallowtail-runtime`, `swallowtail-host-local`, and
  `swallowtail-adapter-codex` from the exact clean candidate revision
- `effigy release prepare --plan` reports only the intended changelog
  promotion; no release state exists and `v0.1.0` remains absent
- no authenticated provider work ran

## Auto-Continuation

No. Card 130 requires explicit operator review of the exact candidate.
