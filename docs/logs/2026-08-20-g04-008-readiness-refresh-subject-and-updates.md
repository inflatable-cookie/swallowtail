# g04.008 Readiness Refresh, Subject, And Updates

Date: 2026-08-20
Roadmap: `../roadmaps/g04/008-readiness-refresh-subject-and-updates.md`
Cards: `../roadmaps/g04/batch-cards/022-readiness-refresh.md`,
`../roadmaps/g04/batch-cards/023-authenticated-subject-observation.md`,
`../roadmaps/g04/batch-cards/024-instance-update-observation.md`

## Result

Consumers can refresh access dimensions, observe a provider-disclosed
subject, and project an update affordance from existing 029/032 evidence.

`refresh_readiness` writes `AccessStatus` onto one admitted instance. A
disabled instance can refresh to ready dimensions; an enabled instance can
refresh to not-ready dimensions. Enablement is unchanged. 047 stays an
immutable snapshot the consumer replaces.

`observe_authenticated_subject` is redacted by default. Adapters can report
a field as `SubjectDisclosure::Absent`. Revealed email stays out of `Debug`.
Subject is not stored on `AdmittedInstanceRecord`, not a 047 field, and not
a default diagnostic.

`observe_instance_update` reuses a Contract 029 claim and optional Contract
032 installed-executable observation. It does not install, authenticate, or
admit an instance. There is no second currentness system.

Additive API is in `public-api-unreleased` for core and runtime.
`public-api-0.3.3` is unchanged. No production adapter crate changed.
Host-local JSON and memory stores persist refreshed access status.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-7881116e`
Worker branch: `t3code/readiness-refresh-subject-updates`

PR: https://github.com/inflatable-cookie/swallowtail/pull/7

## Next

g04.008 PR awaiting review. Merge is operator-authorised. g04.009 overlay
stays planned.
