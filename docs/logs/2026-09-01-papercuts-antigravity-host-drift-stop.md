# Papercuts Antigravity host `agy` auto-update ownership stop

Date: 2026-09-01
Branch: `worker/papercuts-antigravity-host-drift`
Base: `4c11544add62b07e35af1d0033ea57492ecfc56e`
Papercut: Host `agy` auto-updated from 1.1.9 to 1.1.19 mid-research
  (2026-08-24)

## Outcome

Evidence-backed stop. No Swallowtail-owned helper, live probe, card
mechanism, or production classify/spawn path can honestly enforce an
immediate version observation and qualified-range refusal before agent-direct
host `agy` work. Entry stays open. Research 205 stays historical evidence;
this lane did not edit it.

## What drifted

Card 161 authorized promptless help/listing. Research 205 records that PATH
`agy` first reported `1.1.9` with stdout help byte-identical to frozen
`antigravity-cli-1.1.9/help.txt` (SHA-256
`c64e4bf74262cebba7d161d29e1632682f64f844c6ca1a718c77a1fa4e8f8343`), then later
in the same session reported `1.1.19` with help on stderr. `1.1.19` is live
`UnverifiedNewer` observation only.

This lane did not run host `agy --print`, `--version`, or help; did not
contact a provider; did not install or update the host. `command -v agy`
resolved to `/Users/tom/.local/bin/agy` and was not invoked.

## Inventory: surfaces that can run host `agy`

| Surface | Runs host `agy`? | Version observe then qualified-range refuse? |
| --- | --- | --- |
| `scripts/` | no `agy` token | n/a |
| `effigy.toml` | no Antigravity live-probe selector | n/a |
| `swallowtail-adapter-antigravity` features | no `live-probes` | n/a |
| `scripts/run-with-isolated-home.sh` | execs `"$@"` after HOME isolation | no argv or version gate |
| Production discovery | host-approved target + `--version` at prepare | classifies; `AllowUnverified` **permits** `1.1.19` as `UnverifiedNewer` |
| Production catalogue | `models` after prepare | no re-probe; spawn is provider work |
| Production headless / continuation | `--print` … after prepare | no re-probe; spawn is provider work |
| Examples | call `prepare_antigravity` only | compile-shaped; no probe helper |
| Card 161 / Research 205 method | agent-direct PATH help / `agy agents` | authorized listing; no refuse |
| Version-currentness skill / checkpoint | record PATH `--version` when present | observation only; checkpoint does not refuse or pin |
| Frozen `antigravity-cli-1.1.9` help fixture | extracted-artifact specimen | preferred version-scoped help already exists; unused unless the agent chooses it |

`ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME` is `"agy"`. Discovery still requires an
explicit host-approved target; it does not intercept later PATH invocations.

Production `is_permitted` is true for both `Qualified` and `UnverifiedNewer`.
Foundation already probes `1.1.18` as discovered `UnverifiedNewer`. Changing
`AllowUnverified` to refuse later stables is a Contract 029 claim change, not
this papercut. Adding a spawn-time re-probe would be unplanned runtime
behavior.

A dedicated version-gate wrapper would be fail-open against agent-direct PATH
`agy`, the method that actually drifted. Widening `run-with-isolated-home.sh`
would also wrap unrelated isolated binaries.

Official extracted artifacts are already the identity method (Research 177)
and the frozen help specimen. Nothing in the repo forces a research session to
use them instead of PATH `agy`.

## Proof on this worktree

Throwaway fake `agy` first on `PATH` (deleted after). No host binary, no
provider contact:

1. Isolated-home `--version` → stdout `1.1.9`.
2. Isolated-home `--help` after that fake drifted → exit `0`; help on stderr
   (`drifted 1.1.19`); wrapper did not refuse.
3. Isolated-home `--version` → stdout `1.1.19`.
4. Isolated-home `agents` after drift → exit `0`.
5. Agent-direct PATH `--help` with no prior version observation → exit `0`.
6. Frozen fixture digest still
   `c64e4bf74262cebba7d161d29e1632682f64f844c6ca1a718c77a1fa4e8f8343`.

Falsification: Swallowtail isolation wrapping does not record-and-refuse
qualified-range drift. Direct host invocation has no Swallowtail intercept.
Production classification would still permit `1.1.19`. Checkbox stays open.

No adapter, script, Research 205, card, matrix, or currentness edit.

## Residuals

- Papercut remains open until a host/harness policy, extracted-artifact
  pin, or agent-unskippable intercept can refuse PATH `agy` once it leaves the
  named qualified range (or research cards are required to use frozen official
  artifacts for version-scoped help and cannot skip that). Swallowtail cannot
  own that intercept today.
- Research 205's `1.1.9` → `1.1.19` notes stay historical; they are not newly
  qualified evidence.
- Next open Swallowtail papercut after this one:
  llama.cpp context-size proofs widen the god-file warning baseline
  (2026-08-24). Invalid-`--agent` no-prompt, launcher worktree-cleanup, and
  Effigy graph-explore timeout remain open external handoffs.

## Validation

- Docs-only stop record after the log and PAPERCUTS serial edits:
  `effigy qa:docs:index:logs`; `effigy qa:docs:links` (15 front-door + 1127
  research/log files); `effigy qa:northstar`.
- `effigy --json scan god-files` → 383 (7 critical / 42 high / 334 warning);
  inherited, no Rust change.
- `git diff --check`
