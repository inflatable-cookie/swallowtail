# 2026-08-19 Pi ACP Identity Negative

## Result

Card 282 closed `pi.acp` as negative evidence. Official
`@earendil-works/pi-coding-agent@0.84.2` still has no native ACP mode
(`--mode` is text/json/rpc). Registry `pi-acp@0.0.33` is community
`svkozak/pi-acp`; it speaks ACP by spawning `pi --mode rpc --no-themes`.
That collapses onto already-qualified `pi.rpc`. Swallowtail does not wrap
the community adapter and does not add a package.

Host `pi` `0.83.0` was inspected for help/version only. No install, login,
`initialize`, or prompt. `pi.rpc` claims are unchanged. Cards 283-285 are
superseded. Current source stays 37 packages and 45 routes.

## Validation

- `effigy qa:northstar`
- `effigy validate:focused swallowtail-adapter-pi` (44 tests) for the identity fixture test
- docs index checks for logs, research, roadmaps, g03, and batch cards
- `effigy qa:docs:next-action:roadmaps`

## Next

Implement the secondary-wave source and disposition gate (card 286).
