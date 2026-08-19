# 2026-08-19 Kiro ACP Package And Route Acceptance

## Result

Card 294 accepted `kiro.acp` as an unreleased additive production route.

Current source is 39 packages and 46 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Package `swallowtail-adapter-kiro` is separately
selectable. Exact claim remains installer-manifest `2.18.1` /
`kiro-cli.release`, spawn `kiro-cli acp`, field `prompt`, qualified-only.
Swallowtail does not pass `--cloud`, `--agent`, or `--trust-all-tools`, does
not log in, does not bind `KIRO_API_KEY` as a credential lease, and does not
flatten onto `kiro-cli chat --no-interactive`.

Live install, login, and prompt were not justified: this host has no
`kiro-cli`, and the card forbids unbounded live qualification. Deterministic
acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-kiro swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-kiro`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the Deep Agents ACP identity corpus (card 299).
