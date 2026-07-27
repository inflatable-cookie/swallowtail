# OpenCode Deletion Range Corpus

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/055-opencode-deletion-range-corpus.md`

## Change

Research 039 audits `session.delete` at all 45 exact OpenCode tag commits in
the unchanged `1.14.48..=1.18.4` qualified range.

The extraction verified every existing full OpenAPI SHA-256, selected the
delete operation, recursively followed local JSON references, sorted object
keys, and hashed the closed operation-plus-reference object. The resulting
corpus records:

- two delete-schema revisions, split at `1.15.6`
- eight exact published segments
- every tag commit, publication date, full OpenAPI digest, existing execution
  surface, delete digest, component count, and runtime evidence revision
- every unpublished, outer, and prerelease exclusion
- `1.18.5` as unverified-newer, not guaranteed

The original six-route corpus, 18 execution surfaces, 20 execution segments,
and production behavior ids are unchanged.

## Decision

Every supported route returns `true` after provider-declared data deletion,
rejects a missing target with 404, and recursively deletes provider children.
The qualified result is `ProviderDataDeleted` with
`ProviderDefinedDescendants`, never hard erasure.

The route has no busy guard. `1.14.51` adds background-job cancellation, but
the full range still requires an inactive Swallowtail management target.

Optional server Basic authentication applies outside the legacy generated
OpenAPI. A 5xx or transport loss after dispatch leaves provider truth
unconfirmed. Raw server bodies are not stable diagnostics. There is no retry,
archive, restore, active-handle management, server ownership, or local-file
deletion authority.

## Validation

- exact tagged OpenAPI extraction: 45 full digests matched; two delete
  closures
- OpenCode adapter: 44 tests pass; one installed probe skipped
- `effigy check:rust`: pass
- `effigy format:check`: pass
- docs, Northstar, and diff checks: pass
- `effigy doctor`: unchanged baseline of 25 findings
  (17 warnings, 8 errors)

## Next

Card 056 is ready. Add one bound inactive-session delete operation through the
existing attached HTTP driver and prepared facade. Keep endpoint, access,
version, provider-data, descendant, uncertainty, and external-server truth
exact.
