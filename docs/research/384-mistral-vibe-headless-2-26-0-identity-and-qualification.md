# Research 384: Mistral Vibe Headless 2.26.0 Identity and Qualification

Observed 2026-10-08. Scope is `mistral-vibe.headless` on
`mistral-vibe.release` only. The official PyPI stable and GitHub latest stable
release both identify `2.26.0`. The GitHub release is
[`v2.26.0`](https://github.com/mistralai/mistral-vibe/releases/tag/v2.26.0),
published on October 6, and points to commit
`376f6a33413a3eec9b3795b0c0e004066c47b5c`. PyPI's `2.26.0` sdist SHA-256 is
`86ee13da13f9ca6b2f023cc5ca254a81bd99eeb5d2caa7b9e791421531bb2f5c`; the five
platform wheel digests and full file inventories are retained in the
[`2.26.0` fixture](../../crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.26.0/).
PyPI provenance names commit `7cb91894c40bb25173abcfa36e5ea2b4b81eb28c`.
That commit differs from the GitHub tag commit, but the complete `vibe/` source
tree matches the tag byte-for-byte; the shipped wheels and sdist are the
runtime authority. The `2.26.0` sdist matches the tag's `vibe/` tree
byte-for-byte. The retained `2.25.4` and `2.25.5` sdists differ only in the
build-injected `vibe/observability/sentry.py` file.

The previous qualified ceiling is `2.25.4`. PyPI published stable hops after
that ceiling are `2.25.5`, `2.25.7`, `2.25.8`, and `2.26.0`. The stable points
`2.25.6` and `2.25.9` are absent from both PyPI and GitHub refs. The full
wheel inventories include all seventeen wheels across the retained point and
each published hop, including the native Unified Harness and Rust TUI binaries
introduced in `2.25.7`. No package or host runtime was installed or executed;
the host had no `vibe` executable.

| Published hop | Audited input files changed | Finding |
| --- | ---: | --- |
| `2.25.4` → `2.25.5` | 54 | The local-home bootstrap, config/session/telemetry changes and selected tools are classified. Skill invocation is narrowed to model-invocable skills; web-fetch approval binds to a normalized origin with bounded redirects; Todo action validation is stricter. Plan's workspace write tools remain `never`, and headless approval requests are denied. |
| `2.25.5` → `2.25.7` | 20 | The programmatic stream changes only in an unselected teleport condition. The Rust TUI gains explicit `VIBE_CLI` selection; `--legacy-harness` still pins the legacy harness. The route requires `VIBE_CLI` unset or `python` so an explicit `rust` setting cannot replace the headless CLI. |
| `2.25.7` → `2.25.8` | 17 | Agent metadata and permission matcher updates do not change the selected builtin Plan profile or its write/edit `never` overrides. |
| `2.25.8` → `2.26.0` | 33 | Unified Harness becomes the default and Rust TUI gains automatic rollout, but `--legacy-harness` blocks both automatic selections. Explicit `VIBE_CLI=rust` remains outside the route. Tool-call ID repair, title metadata, and context notice wording do not change the stream envelope or decoder contract. |

`surface-ledger.json` records hashes and classifications for 78 selected-surface
inputs and guard paths at each published hop. `dist-inventory.json` records every regular file in
each PyPI wheel, exact archive digests, and deterministic tree-manifest
digests. All PyPI wheels contain a build-injected
`vibe/observability/sentry.py` with the release DSN; telemetry stays gated by
the provider configuration. Platform wheel Python sources match after
normalizing Windows CRLF line endings, and each normalized tree matches the
corresponding GitHub tag tree with the Sentry injection accounted for. Native
module and `vibe-rs` binaries retain separate exact platform digests and remain
unmapped.

The fixed command remains `vibe --prompt --output streaming --max-turns N
--trust --agent plan --workdir DIR --legacy-harness`, with caller-decreasing
`N` in `1..=8` and omission fixed at `8`. The selected `vibe/cli/programmatic.py`
serializer is byte-identical from `2.25.4` through `2.25.5`, changes at
`2.25.7` only in the teleport branch (not selected), and then remains
byte-identical through `2.26.0`. The fixed Plan profile keeps `write_file`
and `edit` at `never`; `--auto-approve`, `--yolo`, smart approval,
teleport, ACP, and the Unified Harness stay unmapped. New app-server fields are
additive. The `2.26.0` context-budget notice changes wording while retaining
the notice frame already accepted by the decoder.

The source review includes the changed selected tool/configuration paths. At
`2.25.5`, the `skill` tool filters to model-invocable skills, the Todo tool
validates its `read`/`write` action and duplicate IDs, and `web_fetch` binds
approval to a normalized URL origin and caps redirects at 20. These retain
their existing provider-internal tool shapes; Swallowtail still supplies no
selected skill bundle, registered tools, or consumer MCP servers. Shell and
permission matcher updates leave the fixed Plan `write_file` and `edit`
permissions at `never`; the headless programmatic harness denies callbacks.
Connector and MCP error formatting, config merging, session lease metadata,
and telemetry fields are classified in the per-hop ledger; they add no
Swallowtail operation, credential lease, or usage evidence.

The `2.26.0` `session_logging.generate_titles` default is enabled for
interactive CLI/Desktop clients only. The selected programmatic client uses
`entrypoint="programmatic"`, so it does not start the new title completion or
add a secondary provider request. `VIBE_CLI=rust` has explicitly selected the
Rust TUI since `2.25.7`; on `2.26.0`, automatic rollout is blocked by
`--legacy-harness`, while explicit Rust selection still takes precedence. The
approved execution environment must leave `VIBE_CLI` unset or set it to
`python`; Swallowtail does not alter that environment.

This is a compatible extension of the existing behavior, not a new driver or
public lifecycle. The maintained `2.25.4..=2.26.0` segment retains claim
`mistral-vibe.headless.release-window-1` and behavior
`mistral-vibe.headless.stdio-streaming-v1`, excluding unpublished `2.25.6` and
`2.25.9`. Later official stable points remain `UnverifiedNewer` under
`AllowUnverified`. The independent `mistral-vibe.acp` route, credential or
provider-session work, and provider prompts remain outside this qualification.
