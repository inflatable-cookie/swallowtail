# 2026-08-21 Pi SDK Sidecar Route Planning

## Change

- qualified Pi's official `0.84.2` SDK as the fuller route surface
- promoted foreign-language SDK sidecar rules into Contract 019
- added a separate qualified-only Pi SDK sidecar posture to Contract 029
- compiled g04.033 and cards 089-092
- promoted the Pi continuity workaround note

## Decision

The official Pi SDK is TypeScript, so the Swallowtail boundary is a
source-tagged Node sidecar with a strict private wire. It is not SDK-native.
The application provisions the exact Node runtime, sidecar entry point, and
SDK dependency; Swallowtail does not install or discover them.

The SDK exposes the missing Contract 017 seam through
`switchSession(sessionPath, { cwdOverride })`, effective `runtime.cwd`, and
typed session messages. The worker must prove the complete boundary before the
route becomes production.

`pi.rpc` remains production during that proof. The acceptance card records
coexistence or explicit deprecation from realized evidence. Architecture stays
unchanged until the route exists in code.

## Validation

- `git diff --check`
- `effigy qa:docs`
- `effigy qa:northstar`
