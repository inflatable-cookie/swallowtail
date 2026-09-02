# 048 Codex 0.152.1 Identity

Status: completed
Owner: Tom
Created: 2026-09-02
Updated: 2026-09-02
Milestone: `../020-codex-0-152-1-useful-newer.md`
Depends on: Contract 029; Research 274; official stable `0.152.1`

## Goal

Freeze exact official Codex `0.152.1` identity and classify its selected
exec, app-server, lifecycle, and catalogue surfaces without changing a claim
or executing a downloaded binary.

## Scope

1. Recheck npm, GitHub tag/commit, wrapper and platform packages, extracted
   binary digests, sizes, and version-literal bytes.
2. Keep host `0.150.1` observation-only. Do not install or update it.
3. Compare the complete `0.152.0` → `0.152.1` shipped trees and the complete
   GitHub source delta between the two release tag commits.
4. Compare selected exec and app-server surfaces with the frozen `0.152.0`
   corpus through byte-identical generating sources and upstream-published
   schema digests; keep additive unmapped surfaces unmapped.
5. Add Research 275 and one secret-free `0.152.1` identity/protocol corpus
   with a delta-ledger test.
6. Commit identity evidence before any selection, matrix, guide, changelog,
   or standing-lane claim edit.
7. Record compatible extension, private milestone, new revision, or stop.

## Out Of Scope

Production claim edits, feature-specific widening, another family, provider
contact, login, install, host update, live probe, projection, watcher, skill,
papercut, release, execution of downloaded binaries, or broad workspace work.

## Acceptance Criteria

- official identity is corroborated through independent official channels
- mapped and material unmapped additions are explicit
- current production claims are byte-for-byte unchanged in this commit
- fixture provenance, digests, and negative boundaries are load-bearing
- card 049 continues only for an admitted segment

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, to card 049 only after an admitted segment is recorded.

## Result

Official stable remained exact `0.152.1`. Host `0.150.1` keeps its recorded
identity. Research 275 and the frozen corpus landed in the identity-only
commit `97babfe0`; production claims remained unchanged. Downloaded binaries
were hashed and never executed; the shipped-tree and source-tree deltas are
Guardian, test, and version-bump bounded, so the selected protocol
classifies as a compatible extension and card 049 continued.
