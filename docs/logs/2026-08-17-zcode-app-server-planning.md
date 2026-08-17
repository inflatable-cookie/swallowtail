# ZCode App-Server Planning

Date: 2026-08-17
Roadmap: g03.071

## Outcome

Promoted ZCode app-server into Swallowtail planning without collapsing it
onto OpenCode or hosted GLM HTTP:

- Research 126
- Spec 010
- Milestone g03.071 with completed cards 226-228 and ready card 229

First subset: exact runtime `0.16.3`, owned-process line-delimited JSON
stdio, structured run, content-free reasoning progress, harness-owned tool
activity, usage, process-kill cancel. `--print`, ACP, history, native
stop, and Z.AI official stay outside.

Live proof may use a host-local OpenAI-compatible endpoint through the
custom-provider path; that does not qualify Z.AI official.

This host has no `ZCode.app`. Probe evidence used isolated npm
`zcode-app-cli@3.7.7-13` vendoring `zcode.cjs` from desktop `3.7.7`.
Launcher and desktop About numbers are not the compatibility axis.

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar:docs-front-door`

## Next

Superseded by cards 227-228. Continue from
[2026-08-17 ZCode App-Server Driver And Prepared Facade](./2026-08-17-zcode-app-server-driver-and-facade.md).

## Card 226 closeout

Card 226 froze exact runtime `0.16.3` plus `zcode.cjs` digest
`3e3433d90fa502e5d02498dfde6c2090df898331359bcfe5f3dbc9a1d00b685f` on axis
`zcode.runtime`. Launcher `3.7.7-13` and desktop About `3.7.7` are recorded
and are not the axis.

Added redacted line-delimited JSON fixtures for handshake, text success,
tool success, tool error, missing credential, namespaced unknown events,
and create-without-preferences rejection. Framing has no `jsonrpc` field.
Create blocks on `session/requestRuntimePreferences`. The create result is
a snapshot, not the event stream.

Validation: `python3 scripts/check-zcode-app-server-corpus.py -v` passed 13
tests. Cards 227-228 later implemented the driver and prepared facade.
