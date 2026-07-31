# 037 Cursor Exact-Build Claims And Conformance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../014-cursor-agent-2026-07-23-range-maintenance.md`
Depends on: card 036

## Goal

Add the second exact milestone without weakening either opaque build gate or
inventing a calendar range.

## Acceptance Criteria

- [x] catalogue, ACP, and headless claims use two exact milestones
- [x] both qualified dates reject unknown same-day build revisions
- [x] intermediate dates remain unsupported
- [x] later dates remain permitted as unverified newer
- [x] focused Cursor validation passes

## Evidence

All three claim ids advance to `release-window-2`. Thirty-four focused tests
pass across six binaries, including the new secret-free compatibility corpus.
