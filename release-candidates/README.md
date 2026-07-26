# Release Candidates

This directory retains textual, reviewable candidate evidence. Binary source
bundles and `.crate` archives stay under `.effigy/release-candidates/`.

Candidate evidence grants no publication, registry, tag, push, release,
workflow, or consumer authority.

The active candidate is the post-hardening local soak baseline. All package,
route, isolated-consumer, and accepted application evidence passes. It is not
a publication authorization.

## Candidates

- `0.1.0/` — active non-published post-hardening 23-package candidate
- `.effigy/release-candidates/superseded/0.1.0-5326e6f4b24d/` — superseded
  technically passing rebuild with stale packaged currentness
- `.effigy/release-candidates/superseded/0.1.0-f142d927767f/` — superseded
  pre-hardening canonical-history candidate
- `.effigy/release-candidates/superseded/0.1.0-6c0e8d9b5b05/` — superseded
  compile-only candidate
- `.effigy/release-candidates/superseded/0.1.0-e68ab15b279d/` — superseded
  provisional candidate
- `.effigy/release-candidates/superseded/0.1.0-73c7f5b5b561/` — superseded
  parentless provider-wide candidate
- `.effigy/release-candidates/superseded/0.1.0-e9ead4d35fb7/` — superseded
  canonical-history candidate with stale packaged release wording

Only the exact `0.1.0/` path is active. Superseded directories are immutable
historical evidence.
