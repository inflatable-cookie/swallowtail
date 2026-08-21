# g04.022 Further Addable Inventory

Date: 2026-08-21
Roadmap: `../roadmaps/g04/022-further-addable-inventory.md`
Handoff: `../handoffs/20260821-084456-g04-022-further-addable-inventory.md`
Research: `../research/171-further-addable-route-inventory.md`
Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-df15a6ad`
Worker branch: `t3code/further-addable-inventory`

## Result

Cards 062-064 are complete. Research 171 inventories all 47 production
routes:

- six current addable rows reuse their descriptors and prepared facades
- 26 remaining routes are later adapter-local descriptor work on proved
  hosted API-key or installed shapes
- 15 remaining routes stay gated behind separate transport, ownership,
  provider-state, cloud-identity, or sibling-route boundaries

The hosted OAuth gate stays parked. `claude-code.headless`,
`claude-code.response-only`, and `llama-cpp.owned` stay off sibling addable
rows. OpenHands still has no production route. No adapter crate or runtime
file changed.

## Tranche

g04.024 is named for `kimi-platform.chat` after g04.023. It reuses the proved
hosted direct API-key shape and existing `prepare_kimi_platform_direct`, with
an opaque endpoint ref and `CredentialRef`-only key collection. Its
implementation cards are not compiled or ready.

g04.023 cards 065-067 are the next ready pointer. This worker did not start
g04.023 or g04.024.

## Validation

- `effigy qa:docs:index:research` — passed
- `effigy qa:docs:index:roadmaps:g04` — passed
- `git diff --check` — passed
- `effigy tasks` — passed
- `effigy test --plan` — passed; workspace `cargo nextest run`
- `effigy doctor` — failed on the pre-existing `scan.god-files` error (343
  findings, 40 errors); one generated-in-src warning also remains

## PR

Reviewable PR: https://github.com/inflatable-cookie/swallowtail/pull/20

Review is pending against the current pushed `main` tip. Merge is not
authorised by this handoff.

## Next

Push this worker branch and open the reviewable PR. The operator/orchestrator
reviews and separately authorises merge; then g04.023 may start.
