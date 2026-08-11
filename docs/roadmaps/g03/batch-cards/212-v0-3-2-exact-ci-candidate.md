# 212 v0.3.2 Exact CI Candidate

Status: planned
Owner: Tom
Created: 2026-08-11
Milestone: `../067-v0-3-2-source-patch-release.md`
Depends on: card 211; explicit operator authorization

## Goal

Commit and push the accepted candidate, then require canonical CI at its exact
SHA without creating a tag.

## Acceptance

- [ ] local and remote canonical branch resolve to the accepted commit
- [ ] every canonical CI job passes against that SHA
- [ ] no tag, GitHub Release, registry, consumer, or provider mutation runs

## Auto-Continuation

No. Await separate exact tag authorization.
