# g04.020 Config-Ref Prepare Merged

Date: 2026-08-21
Roadmap: `../roadmaps/g04/020-config-ref-prepare-handoff.md`
PR: https://github.com/inflatable-cookie/swallowtail/pull/17

## Result

PR 17 fast-forwarded onto `main` at
`3d7616555a94233b8d03a5f3f20382b6a62a084c`. Review comment
https://github.com/inflatable-cookie/swallowtail/pull/17#issuecomment-5362952936
is the canonical verdict. `v0.3.3` still peels to `51d18620`.

Admitted config refs feed the six addable `prepare_*` entries through
route-local `from_admitted`. The host still resolves values. Contract 037
still binds the exact target. 047 stays free of targets.

## Next

Dispatch g04.021 unmarked overlay rows. Do not invent a catalogue
`provider_id`. Do not start 022 or 023.
