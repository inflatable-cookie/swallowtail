# 211 v0.3.2 Local Source Candidate

Status: completed
Owner: Tom
Created: 2026-08-11
Milestone: `../067-v0-3-2-source-patch-release.md`
Depends on: card 210

## Goal

Prepare one complete local `v0.3.2` source candidate and pass every configured
credential-free release gate.

## Acceptance

- [x] version, manifests, internal requirements, lock, changelog, release
      notes, package/route/API baselines, and examples agree
- [x] all release gates pass against the final prepared tree
- [x] no commit, push, workflow, tag, registry, consumer, or provider mutation
      runs

## Evidence

- `effigy release prepare --yes --check-gates --version 0.3.2` prepared the
  local candidate and passed all 11 configured gates
- 1,625 workspace tests passed; 17 were skipped
- the isolated source consumer passed; its temporary validation commit is not
  a candidate source identity
- `.release-prepared.json` records base HEAD `d9b7bab93671f3fbd2b3166ed3d3de8a9f8a462d`;
  no candidate commit or tag exists yet

## Auto-Continuation

No. Return the complete local candidate for operator review.
