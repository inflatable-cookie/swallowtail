# 212 v0.3.2 Exact CI Candidate

Status: completed
Owner: Tom
Created: 2026-08-11
Milestone: `../067-v0-3-2-source-patch-release.md`
Depends on: card 211; explicit operator authorization

## Goal

Commit and push the accepted candidate, then require canonical CI at its exact
SHA without creating a tag.

## Acceptance

- [x] local and remote canonical branch resolve to the accepted commit
- [x] every canonical CI job passes against that SHA
- [x] no tag, GitHub Release, registry, consumer, or provider mutation runs

## Completion Evidence

- the accepted candidate was committed and pushed to canonical `main`
- the unchanged `CI` workflow ran by explicit dispatch because branch pushes
  do not trigger it; all five jobs passed at the exact candidate SHA
- local `HEAD`, remote `main`, and the workflow head SHA matched
- no local or remote `v0.3.2` tag exists

## Auto-Continuation

No. Await separate exact tag authorization.
