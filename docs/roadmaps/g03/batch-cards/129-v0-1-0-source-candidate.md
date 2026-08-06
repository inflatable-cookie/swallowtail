# 129 v0.1.0 Source Candidate

Status: active
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

- [ ] one clean non-root commit contains the complete source-tag candidate
- [ ] full release simulation passes from that exact commit
- [ ] the external consumer resolves every selected package from exact
      canonical `HEAD`, not a synthetic snapshot
- [ ] package set, semantic API, internal topology, dependency lock, commit,
      and parent digests are recorded
- [ ] no prepare, tag, push, registry, GitHub Release, authenticated provider,
      or consumer mutation runs

## Auto-Continuation

No. Card 130 requires explicit operator review of the exact candidate.
