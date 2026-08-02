# 058 OpenCode Session Catalogue Range Corpus

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../022-opencode-external-session-discovery-and-import.md`
Depends on: card 057

## Goal

Freeze the exact OpenCode list-to-continuation behavior across the maintained
attached-server range before changing the production driver.

## Scope

1. Capture exact list, lookup, status, message, load, and continuation API
   closures at every supported milestone.
2. Record directory scoping, pagination, child-session, status, and message
   differences.
3. Add deterministic success, malformed, stale, and unsupported fixtures.
4. Preserve exact endpoint, access, server-revision, and resource identity.
5. Select the guaranteed import segments without widening incomplete points.

## Out Of Scope

- production driver changes
- child-session import unless complete evidence supports it
- server management, provider prompt, or live network selector

## Acceptance Criteria

- [x] every guaranteed milestone proves the full import chain
- [x] partial milestones remain visibly unavailable
- [x] directory and endpoint binding differences are explicit
- [x] message ordering and size bounds have deterministic evidence
- [x] malformed and incomplete responses remain fail closed
- [x] focused fixture tests pass
- [x] card 059 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-opencode`
- `git diff --check`
- no attached live server or broad suite

## Auto-Continuation

Yes. Continue to card 059 after corpus acceptance.

## Evidence

- Research 095 recursively closes list, status, lookup, messages, and
  continuation against all 51 exact qualified OpenAPI documents
- seven exact import surfaces map onto 12 published-version segments; every
  current qualified release maps once, while semantic gaps and `1.18.11+`
  remain unavailable
- deterministic fixtures bind endpoint, Basic-auth lease, directory,
  health/session version, offset pagination, root/child/status projection,
  ordered history, and no-binding stale outcomes
- child import, cross-directory import, and project/account-wide scans remain
  excluded
- 86 focused OpenCode tests passed; no attached server or live provider ran
