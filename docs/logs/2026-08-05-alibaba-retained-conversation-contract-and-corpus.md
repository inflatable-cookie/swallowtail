# 2026-08-05 Alibaba Retained Conversation Contract And Corpus

Roadmap: `../roadmaps/g03/037-retained-session-recovery-promotion.md`
Card: 098

## Changed

- extended ordinary resume bindings and load requests with exact resource-free
  posture and a separate persistence fingerprint
- kept existing resource-bound load, replay-free resume, and provider-operation
  checkpoints strict
- separated Alibaba retained preservation from operation-owned delete-on-close
- froze exact conversation retrieval and ascending item pagination
- added bounded completed-message replay projection: 100 items and 512 KiB per
  page; 10 pages, 1,000 items, and 4 MiB overall
- froze non-ready dispositions for foreign, missing, deleted, malformed,
  oversized, stale, and uncertain conversations
- kept destructive cleanup behind a separate management binding; the persisted
  resume record grants no deletion authority

## Current State

The common runtime can persist and restore an exact resource-free provider
session binding and build a matching load request. Existing resource-bound
routes reject absent resources explicitly.

Alibaba has deterministic retrieval, pagination, replay, and failure corpus.
Its production prepared retained route is not implemented yet. The existing
prepared conversation remains delete-on-close. Cleanup-only restart from a
resume record remains unsupported pending separate management-binding
persistence.

No authenticated provider work, external request, conversation mutation,
prompt, or paid inference ran.

## Validation

- required focused core, runtime, testkit, and Alibaba validation: 309 passed
- compatibility validation for Claude Agent, Codex, Kimi, and OpenCode: 441
  passed
- affected Alibaba package proof passed
- structural doctor remains red on the known oversized-file baseline; the new
  replay parser and retained corpus test stay below warning thresholds
- `git diff --check`

## Next Move

Execute card 100. Implement only the separately prepared retained Alibaba
profile: exact open/load binding, retrieve-then-list replay, preservation on
close, live continuation, and separately authorized cleanup.
