# Research 458: Mistral Vibe Headless 3.0.2 Identity Stop

Status: promoted. Identity stop. The claim does not move.

Question: `mistral-vibe.headless` stays behind official stable. The next
adaptation qualifies the then-current official stable by replacing
`--legacy-harness` and `--agent plan` and rebinding the turn-limit failure.
That is an adapter-private milestone, or a new driver or facade revision if
the public lifecycle changes. It is not a silent extension of
`mistral-vibe.headless.stdio-streaming-v1`.

| Family | Baseline | Latest qualified | Posture | Official at observation | Outcome |
| --- | --- | --- | --- | --- | --- |
| `mistral-vibe.headless` | `2.25.4` | `2.26.0` | `AllowUnverified` | `3.0.2` | identity stop |

Unpublished `2.25.6` and `2.25.9` stay excluded. `2.26.1` through `3.0.2`
stay `UnverifiedNewer`. `AllowUnverified` stays while the adaptation is open.

## Method

Observed 2026-10-26. Scope is `mistral-vibe.headless` on
`mistral-vibe.release` only. Official channels are the PyPI project
`mistral-vibe` and GitHub releases of `mistralai/mistral-vibe`. Preview,
nightly, alpha, and rc releases were ignored.

The [identity fixture](../../crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-3.0.2/identity.json)
freezes the ceiling sdist and every published stable hop after it: `2.26.1`,
`3.0.0`, `3.0.1`, `3.0.2`. Each PyPI sdist and the Linux x86_64 wheel were
downloaded and hashed. The other platform wheel digests are registry rows,
marked not locally verified. Selected sdist files were compared byte-for-byte
with the GitHub tag tarball. Linux-wheel Python copies of those files match
the sdist. The [complete hop inventory](../../crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-3.0.2/dist-inventory.json)
records every regular file in those sdists and linux wheels and the
added/removed/changed path sets at each hop. The [surface ledger](../../crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-3.0.2/surface-ledger.json)
classifies each hop.

PyPI provenance names publisher GitHub `mistralai/mistral-vibe` workflow
`release.yml`. The publish statement predicate is null and carries no source
commit. The byte match to the tag tarball is the correlation.

The host is Linux x86_64. `vibe` was not on `PATH`. No artifact was
installed or executed. No provider prompt, credential, or live session
was used. GitHub release assets for `v3.0.0` and later are `vibe-acp-*`
only. Runtime authority for this route is the PyPI wheel. Its console
script is `vibe = vibe.cli.launcher:main`. The wheel ships
`vibe/_bin/vibe-rs` and excludes `vibe/cli-rust/**`; the sdist contains
that Rust source.

## Official channel

PyPI `info.version` is `3.0.2`, not yanked. GitHub latest stable is
[`v3.0.2`](https://github.com/mistralai/mistral-vibe/releases/tag/v3.0.2),
not a prerelease or draft, published `2026-10-22T15:34:08Z`, tag commit
`36f7b3a4e10440e90f98080492530b8f4d46960d`. The channels agree. No other
stable sits between `2.26.0` and `3.0.2`.

| Version | GitHub tag commit | Sdist SHA-256 | Linux x86_64 wheel SHA-256 |
| --- | --- | --- | --- |
| `2.26.0` | `376f6a33413a3eec9b3795b0c0e004066c47b5c1` | `86ee13da13f9ca6b2f023cc5ca254a81bd99eeb5d2caa7b9e791421531bb2f5c` | `d8230b8abd7c39a8e90d60884b1f5ee8c95ffb459c62c21f3ea0e3041117a114` |
| `2.26.1` | `4ae5c5955da8e761318916bdb1a7f46f0b50c7d0` | `e92402b2a1705c66463cad9d1f33d549ea2d8fc1bb608ec228c7acaf49ef4332` | `1ceb43e38a2e82cb8c54e1032b011874f8422e998cf87b4a3fdf58704133de55` |
| `3.0.0` | `a0c78d0a3f3f822c275f1dec7054e40d1050f966` | `49d90b1bce61ae2a2835e19bb24237958fa1323f9a2f12ce1968b9333bb3fee7` | `b45de79af6f496257583c8817a785bbc8578955331b69c4c36d4b80990ca8a2b` |
| `3.0.1` | `dd2c04ce717c417731ab89c333cd0a72541e33ba` | `b4d1ab55d6ccfc65c658fb19b09b46734e3bf14fd70ca90a5b227a5c640c5882` | `3016ea464719ac99a62f6c5ff4ffa013f3ac0c5ddf03c3def486133ebdc7094f` |
| `3.0.2` | `36f7b3a4e10440e90f98080492530b8f4d46960d` | `cb62be5743194f84a63d3e955afeeedd3e3741442fd19b118ec43ec532ddacd3` | `9dae259db0651a3978653bed1a87005ded40532942f3c143e2d8228fce0f7563` |

`requires_python` is `>=3.12` at `2.26.0`, `2.26.1`, `3.0.1`, and `3.0.2`.
It is `>=3.14` at `3.0.0` only. That packaging fact is not the stop.

## Selected protocol at the ceiling

Research 384 qualified `2.25.4..=2.26.0` as claim
`mistral-vibe.headless.release-window-1` and behavior
`mistral-vibe.headless.stdio-streaming-v1`. The fixed argv is
`vibe --prompt --output streaming --max-turns N --trust --agent plan --workdir DIR --legacy-harness`.
The decoder classifies `swallowtail.mistral-vibe.headless.max_turns` only
when stderr contains `The configured conversation limit was reached`. A
non-zero exit without that phrase is a generic provider failure. At
`2.26.0`, `vibe/cli/programmatic.py` raises `ProgrammaticLimitError` with
that phrase, and `vibe/cli/cli.py` prints it and exits 1. `--legacy-harness`
pins the Python CLI. `--agent plan` keeps workspace writes at `never`.
`vibe/utils/vibe_home.py` defaults to `~/.vibe` and honors `VIBE_HOME`.

## Identity by hop

`2.26.0` → `2.26.1` is the first selected break. The launcher is
byte-identical (`db6a00e0…`) and can still reach the Python CLI.
`--legacy-harness` remains in the same eight Python files, and the builtin
`plan` member remains. `programmatic.py` is still present, but the limit
phrase is absent from every Python file in the Linux wheel.
`TurnStop.LIMIT` maps to `RunOutcome.TURN_LIMIT`, whose exit code is 3.
The headless path no longer prints the mapped stderr phrase. A successful
stream alone does not keep this hop compatible. Config home is unchanged.

`2.26.1` → `3.0.0` is a major-line reset of the selected argv and process
authority. `vibe/cli/launcher.py` (`8903c0fc…`, identical through `3.0.2`)
always execs the bundled Rust CLI and no longer reads `VIBE_CLI`.
`programmatic.py`, `cli.py`, and `entrypoint.py` are gone.
`--legacy-harness` is absent from every Python file and from
`vibe/cli-rust/src/cli.rs`. The shipped `vibe-rs` phrase count for that
flag is 0 at every hop, including `2.26.x`, because the flag lived in
Python. `VIBE_CLI` occurs twice in the `2.26.x` binary and zero times at
`3.0.x`. The limit phrase occurs once in the `2.26.x` binary and zero
times at `3.0.x`.

The Rust CLI still has `--prompt`, `--output` `streaming`, `--max-turns`,
`--trust`, `--agent`, and `--workdir`. Clap rejects the unknown
`--legacy-harness` pin. `3.0.0` adds unmapped `--time-limit`,
`--prompt-file`, `--output-dir`, and `--agent-socket`. `3.0.1` adds
unmapped `--max-runs` and `--runs-per-app-server`. The `3.0.1` to `3.0.2`
`cli.rs` delta adds `Cli::prompt_given`, which is true when `-p TEXT` or a
non-empty `--prompt-file` already supplies the prompt. `main.rs` then skips
the stdin read in that case so a caller can hold stdin open without EOF.
That is unmapped stdin-skip logic. It does not restore `--legacy-harness`,
the builtin `plan` agent, or the `2.26.0` turn-limit stderr. TUI
mouse-capture and file-index timing also move; those stay outside the
selected headless path.

The builtin `plan` agent is removed. `REMOVED_BUILTIN_AGENTS` maps `plan`
to `ask`. `_replace_removed_default_agent` rewrites the name only when it
equals `config.default_agent` and is not a discovered profile. An explicit
`--agent plan` that is not discovered raises `ValueError`. Remapping a
config default to `ask` does not preserve the plan never-overrides
contract. `PLANS_DIR` leaves `vibe/core/paths/_vibe_home.py`.

Message and reasoning entries gain `outcome`. The decoder ignores unknown
JSON fields, so `outcome` alone is not a decoder break. The stop is the
limit failure, then the removed flag, the removed plan builtin, and the
always-Rust launcher. Streaming stays one JSON object per completed entry.

`3.0.0` → `3.0.1` and `3.0.1` → `3.0.2` keep that reset.
`vibe/core/agents/models.py` is byte-identical from `3.0.0` through
`3.0.2` (`3e240ce9…`). `headless_output.rs` and `app_server/models.py`
change at `3.0.1` and stay byte-identical at `3.0.2`. `requires_python`
returns to `>=3.12`. Neither hop restores the `2.26.0` argv or the
turn-limit stderr.

`vibe/utils/vibe_home.py` is byte-identical across all five versions
(`df7191d3…`). The default remains `Path.home() / ".vibe"`, and `VIBE_HOME`
is still honored. Config home did not move. Additive cache files and the
removed `PLANS_DIR` are unmapped.

## Decision

- Keep claim `mistral-vibe.headless.release-window-1` and behavior
  `mistral-vibe.headless.stdio-streaming-v1`.
- Keep the maintained window `2.25.4..=2.26.0`, `AllowUnverified`, and the
  unpublished gaps `2.25.6` and `2.25.9`.
- Leave `2.26.1`, `3.0.0`, `3.0.1`, and `3.0.2` as `UnverifiedNewer`.
  Assess stays permitted. Classify stays empty.
- Do not extend the ceiling through `2.26.1`. The turn-limit failure is
  already a selected-surface change.
- The next adaptation replaces `--legacy-harness` and `--agent plan` and
  rebinds turn-limit failure from the stderr phrase plus exit 1 to the
  `TURN_LIMIT` exit 3 path, then qualifies the then-current official
  stable. Claim id and behavior revision stay until that task changes them
  on purpose. `UnverifiedNewer` stays while the task is open.

The independent `mistral-vibe.acp` route stays outside this stop.

## Sources

- [PyPI project `mistral-vibe`](https://pypi.org/pypi/mistral-vibe/json)
- [GitHub `v3.0.2`](https://github.com/mistralai/mistral-vibe/releases/tag/v3.0.2),
  plus tags `v2.26.0`, `v2.26.1`, `v3.0.0`, and `v3.0.1`
- Downloaded PyPI sdists and Linux x86_64 wheels named in `identity.json`
- Complete sdist and linux-wheel trees in `dist-inventory.json`
- Research 384 for the qualified `2.26.0` ceiling
- Decoder `LIMIT_STDERR` in `src/driver/events.rs` and `LEGACY_HARNESS_PIN`
  in `src/command.rs`
