# 2026-08-19 Qoder Headless Prepared Facade

## Result

Card 280 added `prepare_qoder_headless` and one bounded stream-json print
operation on `swallowtail-adapter-qoder`. Preflight names
`swallowtail.qoder.headless` and exact `qoder.package` `1.1.25`. Access
stays host-owned `LocalUnauthenticated` with entitlement `Unknown`.
Swallowtail does not bind a credential lease, select a model route, pass
`--yolo` / `bypass_permissions` / `accept_edits`, or flatten ACP or SDK
stdio. `--permission-mode dont_ask`, `--max-turns 8`, and
`--no-session-persistence` stay in driver argv. Missing working-resource
authority, `qoder.acp` axis, and unqualified packages fail before stream
work. Current source stays 37 packages and 44 production routes.

`effigy validate:focused swallowtail-adapter-qoder` (28 tests) and
`effigy package:verify-affected swallowtail-adapter-qoder` passed. No
live install, login, or `--print` prompt.

## Next

Implement the Qoder headless package and route acceptance (card 281).
