# 2026-09-12 g05.052 Grok 1.0.25 Model Catalogue Ruling

## Result

Typed non-admission: no admissible non-prompt catalogue seam exists on exact
installed Grok `1.0.25`, so no `ModelCatalog` driver, descriptor, claim,
prepared operation, projection row, matrix cell, baseline, or API surface
lands. The feature matrix `pre_session_model_catalogue` cell for Grok stays
unavailable under its existing provider-limitation evidence.
`grok_build_acp_claim`, `grok_build_model_for_version`, and released `v0.5.0`
are unchanged. The worker's admitted implementation was fully reverted; the
branch carries only Research 305, this ruling, and the task record.

Independent review of the admitted head (PR 315, reviewCommentId 5648500001)
proved the admission defective on two static counts, both confirmed without
executing the catalogue command:

1. The admitted argv `["models"]` disabled no update action, while the
   adapter's own probe and ACP process pass `--no-auto-update` and the root
   CLI documents it. `grok --no-auto-update models --help` exits 0 (root
   flag accepted in root position); `grok models --no-auto-update --help`
   fails, so only `["--no-auto-update", "models"]` would have qualified.
2. The `models` path admits optional online model-catalog fetches
   (`[features] remote_fetch`, default true, fleet-pinnable with managed
   precedence; `remote_config/fetch.rs`, fetch/succeeded/timed-out/fallback
   literals). No CLI flag or `GROK_*` environment suppression exists, and
   host/fleet configuration is out of bounds for preparation. Contract 020
   admits a dedicated harness catalogue only with provider invocation
   disabled; `--no-auto-update` covers update checks, not the catalog fetch.
   A narrowed endpoint/scope statement is unprovable across host config,
   fleet policy, cache, and auth state.

Typed ruling code: `swallowtail.grok.catalogue_not_admitted`,
`provider_invocation_not_suppressible` (plus contributory
`update_action_unsuppressed`), scoped to exact `1.0.25` with no version range
and no `UnverifiedNewer` path. Re-admission needs static proof of a
provider-suppressed listing in a future release, frozen first. Research 305.
g05.052.

No catalogue command was executed, no provider was contacted, no credential
was read, no host was changed, and no Desktop, release, tag, or publication
follows.

## Validation

`cargo fmt -p swallowtail-adapter-grok -- --check`, `effigy validate:focused
swallowtail-adapter-grok`, `effigy package:verify-affected
swallowtail-adapter-grok`, `effigy check:examples`, `effigy package:api`,
`effigy qa:routes`, `effigy qa:consumer-docs`, `effigy qa:northstar`, the
research, logs, roadmaps, g05, roadmap-number, status and next-action checks,
and `git diff --check` — all against the reverted tree, proving claims
unchanged and every inventory at its frozen count (49 routes, 87 activity
operations, 36 harness routes).

## Merge And Closeout

Worker PR 315 targets `main` from the queue-owned branch; merge, review, and
closeout SHAs are recorded here by the queue at closeout. Desktop Chatterbox
receives this ruling in place of a catalogue capsule: there is no source SHA
to pin and no later-release adoption boundary to watch beyond the stated
re-admission gate.
