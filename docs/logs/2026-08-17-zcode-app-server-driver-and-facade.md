# ZCode App-Server Driver And Prepared Facade

Date: 2026-08-17
Roadmap: g03.071 cards 227-228

## Outcome

Added `swallowtail-adapter-zcode` as an exact installed app-server route:

- family `zcode`, driver `swallowtail.zcode.app-server`, axis `zcode.runtime`
  `0.16.3`
- payload digest admission on `zcode.cjs`; launcher `3.7.7-13` and desktop
  About `3.7.7` are provenance, not the pin
- spawn argv is `app-server --settings <host-config>`; never TUI, `--print`,
  or `yolo`
- create answers `session/requestRuntimePreferences` before the create
  result; send `{accepted:true}` is enqueue only
- Swallowtail folds idle from `turn.completed` / `turn.failed` and
  force-stops the owned process; no `session/stop`
- kill-after-complete does not rewrite Completed/ProviderFailed into
  process failure
- `MISSING_CREDENTIAL` maps to `swallowtail.zcode.app_server.credential_missing`
- unknown events require a `zcode/` namespace
- prepared facade is `prepare_zcode_app_server` → `prepare_run` →
  `start_run` with host-approved Node, payload, config, cwd, mode
  (`plan`|`build`), provider, and model

OpenCode, hosted GLM HTTP, ACP, `--print`, native stop, and Z.AI official
stay outside. Architecture, Contract 036, guides, matrices, and live
selectors remain card 229.

## Validation

- `effigy validate:focused swallowtail-adapter-zcode` — 19 tests, Clippy
  warnings denied

## Next

Implement g03.071 card 229. Keep current-source vs immutable `v0.3.2`
package counts distinct.
