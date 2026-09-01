# Papercuts Antigravity invalid-`--agent` no-prompt ownership stop

Date: 2026-09-01
Branch: `worker/papercuts-antigravity-agent-probes`
Base: `fffc64ecaeeab0efe2df3e84608d06127cca7cb4`
Papercut: Antigravity invalid-`--agent` probes crossed card 161's no-prompt
  boundary (2026-08-24)

## Outcome

Evidence-backed stop. No Swallowtail-owned guard, procedure, or test can
honestly prevent the same agent-direct host `agy --print` method. Entry stays
open. Unauthorized runs stay authority-boundary / `UnverifiedNewer` incidents
only.

## What crossed the boundary

Card 161 authorized promptless help/listing and forbade provider prompts.
Research 205 records two unauthorized `--print` / `--output-format json`
probes from that session:

| Incident | Argv shape (relative to `agy`) | Observed |
| --- | --- | --- |
| 1 | `--print noop --model gemini-3.6-flash-high --agent swallowtail-nonexistent-agent-zzzz --output-format json --print-timeout 5s` | exit `0`; JSON `status: SUCCESS`; response present; nonzero usage; stderr empty; stdout SHA-256 `51da1803f9b6c8f6ee9271c413cde72cfc103b0e0c9b755fa3b060287956fafb` |
| 2 | same shape with whitespace-only `--agent` | exit `0`; JSON `status: SUCCESS`; response present; second provider turn |

Host PATH drifted `1.1.9` → `1.1.19` in the same session, so the executing
binary is not frozen as qualified `1.1.9..=1.1.17`. This lane did not rerun
`agy --print`, contact a provider, or treat those outcomes as fail-open proof.

## Ownership

| Claimed fix piece | Owner | Swallowtail lever |
| --- | --- | --- |
| Refuse provider `--print` unless a card authorizes it | evidence card / agent / host policy | none; card 161 already forbade it |
| Stop after one boundary breach | same | none |
| Intercept host PATH `agy` argv | host `agy` / harness command policy | none |
| Production `--agent` omission | `swallowtail-adapter-antigravity` `headless_command.rs` | already omits `--agent`; does not wrap research probes |

Swallowtail surfaces checked and empty for this intercept:

- `scripts/`: no `agy` wrapper, no `--print-timeout` probe helper
- `scripts/run-with-isolated-home.sh`: isolates `HOME` / named provider-home
  vars, then execs `"$@"` with no argv filter
- adapter source: `--print` is the production headless wire; `--agent` is
  absent from `headless_command.rs`
- non-fixture adapter tests: no `--agent` token
- frozen `antigravity-cli-1.1.9/help.txt` SHA-256
  `c64e4bf74262cebba7d161d29e1632682f64f844c6ca1a718c77a1fa4e8f8343`
  still matches Research 205; help advertises `--print` / `-p` / `--prompt`
  as the prompt path

A dedicated promptless-probe wrapper would be fail-open against agent-direct
`agy`. Widening `run-with-isolated-home.sh` to refuse `--print` would also
block legitimate isolated headless wires. Production argv tests cannot see
host PATH research probes.

Current authority already records the incidents correctly (Research 205
correction; g04.058 closeout). Recording is not recurrence prevention.

## Proof on this worktree

Throwaway fake `agy` on `PATH` (printf argv, exit 0; deleted after). No host
binary, no provider contact:

1. Isolated-home forward of incident 1 argv:
   `FAKE_AGY_ARGV:--print noop --model gemini-3.6-flash-high --agent swallowtail-nonexistent-agent-zzzz --output-format json --print-timeout 5s`
2. Isolated-home forward of whitespace-only `--agent` argv: same flags, child
   received.
3. Agent-direct `PATH` invoke of incident 1 with no Swallowtail wrapper: same
   forwarded argv.
4. Source check: `headless_command.rs` contains `"--print"` and does not
   contain `--agent`.

Falsification: Swallowtail isolation wrapping does not refuse `--print` or
`--agent`. Direct host invocation has no Swallowtail intercept. Checkbox stays
open.

No adapter, script, Research 205, card, matrix, or currentness edit.

## Residuals

- Papercut remains open until a host/harness policy can refuse unauthorized
  `agy --print` (or an equivalent intercept that agents cannot skip). Swallowtail
  cannot own that intercept.
- Unauthorized 2026-08-24 runs stay incidents; they do not qualify
  `1.1.9..=1.1.17` fail-open `--agent` behavior.
- Next open Swallowtail papercut after this one:
  Host `agy` auto-updated from 1.1.9 to 1.1.19 mid-research (2026-08-24).
  Launcher worktree-cleanup and Effigy graph-explore timeout remain open
  external handoffs.

## Validation

- Docs-only stop record after the log and PAPERCUTS serial edits:
  `effigy qa:docs:index:logs`; `effigy qa:docs:links` (15 front-door + 1121
  research/log files); `effigy qa:northstar`.
- `effigy --json scan god-files` → 384 (7 critical / 42 high / 335 warning);
  inherited, no Rust change.
- `git diff --check`
