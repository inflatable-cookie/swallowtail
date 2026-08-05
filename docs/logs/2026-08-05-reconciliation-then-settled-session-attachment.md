# 2026-08-05 Reconciliation Then Settled Session Attachment

Roadmap: `../roadmaps/g03/036-reconciliation-then-attachment-composition.md`
Cards: 093-096

## Changed

- extended Contract 050 with a separate consuming observe-then-attach sequence
- added `PreparedSettledSessionRestoration` and phase-aware outcomes and
  failures
- required immutable route binding across independently prepared operations
- mapped Codex app-server and OpenCode HTTP to their managed bounded load paths
- mapped Kimi local server to managed replay-free resume
- documented both the common observation-only facade and the stronger optional
  settled-session path

## Current State

Reconciliation always runs first. Completed, failed, cancelled, or
inactive-unresolved evidence admits the fixed attachment. Active, waiting, or
unknown evidence returns observation only. A reconciliation failure starts no
attachment; an attachment failure retains the complete reconciliation.

Codex and OpenCode return `SettledSessionAttachment::Loaded` with bounded
ordered replay. Kimi local server returns `SettledSessionAttachment::Resumed`
and no replay. The existing provider-wide `PreparedWorkingStateRestoration`
facade remains unchanged.

No authenticated provider work, provider prompt, callback answer,
interruption, live provider session mutation, or external network probe ran.

## Validation

- focused runtime, Codex, OpenCode, and Kimi validation: 511 passed
- affected-package proof passed for Codex, OpenCode, and Kimi
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

## Next Move

Execute card 097. Revalidate Pi RPC and Alibaba Conversations independently;
do not infer retained-session authority from this milestone.
