# g04.023 047 Presentation Metadata

Date: 2026-08-21
Roadmap: `../roadmaps/g04/023-047-presentation-metadata.md`
Cards: `../roadmaps/g04/batch-cards/065-047-presentation-field-inventory.md`,
`../roadmaps/g04/batch-cards/066-047-presentation-contract-amendment.md`,
`../roadmaps/g04/batch-cards/067-047-presentation-fields.md`
Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-1e989752`
Worker branch: `t3code/presentation-metadata-handoff`
Reviewable PR: https://github.com/inflatable-cookie/swallowtail/pull/23

## Result

The field inventory selected one optional 047 presentation field:
`instance_label`, copied from the exact host-owned label stored on the
Contract 057 admitted-instance record. No model-level presentation fields
entered 047. Overlay hide, ordinal, consumer-default, and favourite remain
overlay markers. Accent color and other chrome remain consumer-owned.

Contracts 047 and 057 and the system architecture now name that field and its
boundary. The 047 `Ready` / `NotReady` formula is unchanged. Authenticated-
subject values, emails, tokens, targets, provider defaults, and other
consumer-policy fields remain outside the snapshot.

The runtime realization adds `ConfiguredProviderInstanceAdmission::with_label`
and `ConfiguredProviderInstanceRecord::label`. Focused tests prove the label
projects onto both `Ready` and `NotReady` records without affecting readiness.
The additive API is recorded only in `public-api-unreleased`; the immutable
`public-api-0.3.3` baseline is unchanged.

## Validation

- `effigy qa:docs:index:logs` — passed for cards 065 and 066
- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-testkit` — 396 tests passed
- `git diff --check` — passed
- `effigy package:api` — passed
- `effigy tasks` — passed
- `effigy test --plan` — passed; workspace `cargo nextest run`
- `effigy doctor` — baseline failure: `scan.god-files` reports 343 findings (40 errors); one `scan.generated-in-src` warning

No live provider, install, login, billing, OAuth, or g04.024 work was done.

## Next

Review and merge the worker PR. After merge, compile g04.024 before starting
its named Kimi Platform implementation cards.
