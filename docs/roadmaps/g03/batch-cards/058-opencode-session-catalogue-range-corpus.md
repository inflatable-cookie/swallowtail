# 058 OpenCode Session Catalogue Range Corpus

Status: planned
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

- [ ] every guaranteed milestone proves the full import chain
- [ ] partial milestones remain visibly unavailable
- [ ] directory and endpoint binding differences are explicit
- [ ] message ordering and size bounds have deterministic evidence
- [ ] malformed and incomplete responses remain fail closed
- [ ] focused fixture tests pass
- [ ] card 059 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-opencode`
- `git diff --check`
- no attached live server or broad suite

## Auto-Continuation

Yes. Continue to card 059 after corpus acceptance.
