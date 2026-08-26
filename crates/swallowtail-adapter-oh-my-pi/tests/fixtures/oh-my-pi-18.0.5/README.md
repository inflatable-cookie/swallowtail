# Oh My Pi 18.0.5 currentness corpus

Secret-free identity freeze for npm `@oh-my-pi/pi-coding-agent@18.0.5`
against qualified `oh-my-pi.package` through `17.4.0`.

This is a 17→18 major-line reset, not a default UnverifiedNewer bump.
Assigned official was `18.0.5`. Observed npm `latest` during the run is
`18.0.6` (published 2026-08-26T08:28:41.651Z). Skill stop: official
latest moved, so `18.0.5` stays unqualified and this run opens no claim.

This freeze does not settle the future 18.x segment. Exact-current
`18.0.6` needs a later identity investigation and an operator segment
decision. Contract 029: no silent inheritance of the prior major window
in this run.

Official artifacts stayed in `/tmp`. Extracted `--version` was not run:
the unpacked tarball needs `@oh-my-pi/pi-natives`. Host `omp` was not
on `PATH`. No provider prompt.

Selected `--mode rpc` flags and RPC v2 commands remain. Framing,
`rpc-client.ts`, `rpc-messages.ts`, and `message-framing.ts` are
byte-identical to `v17.4.0`. `rpc-types.ts` and `rpc-mode.ts` change at
`18.0.0` (optional select `optionDetails`; more accurate builtin
`agentInvoked`; unused `runCommandInBackground`). Adapter mapping does
not read those extras. Decoder specimen stays `oh-my-pi-rpc-17.2.9`.

Do not flatten onto `pi.package`. Keep unpublished `18.0.2`. Same-line
`17.4.1` / `17.4.2` are a later 17.x useful-newer card, not this one.
