# 321 Mistral Vibe 2.25.4 Identity

Status: promoted; identity evidence. The claim rebind lands in g05.073 with
this record.

Owner: Tom
Date: 2026-09-14
Card: g05.073 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 150, 199, 252, 308, and 320; the Mistral
Vibe prepared guide; and the official GitHub and PyPI channels.

## Answer

Official GitHub/PyPI latest is `2.25.4`. The exact `mistral-vibe.release`
point can move from `2.24.2` to `2.25.4` as a compatible extension of the
selected headless surface, with one mechanical adaptation: the selected argv
adds adapter-private `--legacy-harness` because `2.25.1` introduces
ambient-config-driven harness selection that could otherwise swap the
session backend without any Swallowtail-visible argv change. The
`mistral-vibe.headless.stdio-streaming-v1` behavior revision is unchanged.
This is one exact `QualifiedOnly` point, not a range. GitHub-only `2.24.4`
stays a named PyPI packaging gap.

## Method and channel boundary

All nine GitHub tags (`v2.24.2`, `v2.24.3`, `v2.24.4`, `v2.24.5`, `v2.25.0`,
`v2.25.1`, `v2.25.2`, `v2.25.3`, `v2.25.4`) were retrieved as exact source
tarballs into `/tmp` and the eight published PyPI points as sdist+wheel
pairs whose SHA-256 digests match the registry metadata exactly; the
`2.24.2` pair reproduces Research 150 byte-for-byte. Both channels were
rechecked at run start and again immediately before the identity commit:
GitHub latest stays `v2.25.4` (lightweight tag commit
`19b5b74faa78d0816b8d4d4c7d7543fc3520678c`, release published
`2026-09-12T18:39:34Z`, not prerelease) and PyPI latest stays matching
non-yanked `2.25.4`. PyPI carries seven of the eight successors; `2.24.4`
is absent there. Every wheel's `vibe` tree is byte-identical to its sdist's;
each sdist differs from its tag only by build-time injected Sentry DSN
constants in `vibe/observability/sentry.py`, a packaging property already
present at the `2.24.2` baseline and gated off the selected wire by
`enable_telemetry`. Host `vibe` remains absent. No artifact was executed;
no extraction of a platform archive beyond listing zip contents occurred;
no prompt, login, credential, installation, or host mutation was used.

## Selected-surface result

The selected wire is byte-stable across all nine points:

- `vibe/cli/programmatic.py` (`36a50089…`): the whole streaming print wire —
  completed-only public-history NDJSON, `entry.id` dedup,
  `model_dump(mode="json", by_alias=True)` lines, `CallbackRequested` →
  `deny_callback`, the LIMIT check on the last turn's `stop_reason`, and
  `session.close()` in `finally`.
- `vibe/core/middleware.py` (`ede809d7…`): `TurnLimitMiddleware` with the
  `steps - 1 >= max_turns` pre-LLM check and the `Turn limit of {N} reached`
  stop reason.
- `tests/cli/test_programmatic.py` (`42cb58a1…`): upstream's own
  programmatic contract tests are byte-identical at every tag.
- `SessionOptions = AgentConfig` field shape (`agent`, `auto_approve`,
  `disabled_tools`, `max_turns`, `headless`, `trust_workspace`, `cwd`,
  `workspace_roots`) is unchanged; the `--max-turns` parser domain
  (`type=int`, no default) is unchanged; the TurnLimitMiddleware install
  condition is unchanged.
- The public-history discriminated union
  (`message|reasoning|effect|callback|checkpoint|notice`),
  `PublicEntryGenerationStatus` (`in_progress|completed`), the
  `PublicTurnStopReason` (`LIMIT`-only), and the `ContentBlock` union
  (`text|image|resource`) are unchanged. The Rust decoder reads named
  fields from generic JSON values, so additive optional entry fields are
  decoder-invisible, and no unknown discriminator can appear.
- The builtin `plan` profile definition is byte-identical from baseline
  through `2.25.4`; `--agent plan` application, headless callback denial,
  `--workdir` expanduser/resolve/chdir authority, and the trust-for-session
  block are unchanged.
- Missing API key fails with identical stderr text and exit 1 at baseline
  and `2.25.4`; `2.25.0` only factors that existing behavior into
  `require_api_key_or_onboard` and adds a redundant in-loop check.
- Console script `vibe = vibe.cli.entrypoint:main` is unchanged at all
  nine points.

## Per-hop ledger (changed selected inputs, all classified)

- `2.24.2..2.24.3`: guarded cosmetic `_set_process_title`; unsupported-image
  error text uses model display name; model display names in schema. Wire
  unchanged.
- `2.24.3..2.24.4` (**GitHub-only; PyPI gap**): background session-title
  machinery behind `auto_title_enabled=False`; profile `instructions` field;
  agent registry rediscover for directory moves (worktree surface);
  additive plugin/MCP/connector catalogue client methods;
  `PublicRetryState` turn-state model; MCP/connector auth-required events;
  snapshot history-page merge — all decoder-invisible or unselected.
- `2.24.4..2.24.5`: config-load error handling; `_agent_selection` reroutes
  only when `--auto-approve` is passed without `--agent` (never our argv);
  GrowthBook extra-models coercion. Wire unchanged.
- `2.24.5..2.25.0`: hidden `--internal-posix-pty-helper` interception and
  `update` alias (never on selected argv); `require_api_key_or_onboard`
  factoring; turn-queue API and `TurnQueueUpdated` events (headless
  enqueues exactly one turn via `act()`); Session content-block union for
  the app-server client protocol (public history union unchanged);
  config-gated registry-skill sync. Wire unchanged.
- `2.25.0..2.25.1`: **material** — harness selection. See below. Also
  `--smart-approve` flag and `smart-approve` builtin profile
  (Unified-Harness-only, ships dark behind rollout flags),
  `AgentSafety.SMART`, `resolve_default_agent` smart-approve branch
  (unreachable because `--agent plan` is always passed), approval `reason`
  field, `TurnErrorCode.INVALID_API_KEY` (consumed by unified adapter, TUI,
  and ACP, not the legacy headless path), teleport link rename, closing
  guard hardening the dropped-connection-during-close error path.
- `2.25.1..2.25.2`: worktree-holder `pending_hold` (worktree surface);
  config-write robustness. Wire unchanged.
- `2.25.2..2.25.3`: comment-only parser change; autocomplete watcher
  default flip. Wire unchanged.
- `2.25.3..2.25.4`: additive shell-command permission-analysis modules and
  presentation/UI changes; Plan's write/edit `never` overrides and headless
  denial unchanged. Selected entrypoint unchanged.

## Harness selection: the material finding

`2.25.1` rewires session startup to `resolve_harness_selection`
(`vibe/_experimental_harness.py`): precedence is `--legacy-harness`, then
`--experimental-harness`, then the ambient GrowthBook rollout eval cache
(`~/.vibe/experiment_eval_cache.json`, read without the API key, 7-day TTL),
then default legacy — and the rollout branch requires the internal
`mistralai_vibe_local_harness` module to be importable.

Channel evidence: that module is absent from every official PyPI wheel and
sdist, so the rollout can never fire on the pip channel. The GitHub
`vibe-darwin-aarch64` release zip starts bundling
`_internal/mistralai_vibe_local_harness/_native.abi3.so` at `2.25.1`
(zips listed, never extracted or executed): `2.24.2`..`2.25.0` do not
bundle it, `2.25.1`..`2.25.4` do. On a zip install of `2.25.1` or later
with an ambient rollout cache resolving `unified`, the selected argv would
execute the native Unified Harness backend instead of the legacy Python
harness. That native backend's tool-authority behavior is not statically
classifiable, and execution to classify it is out of bounds.

Resolution: the selected argv adds `--legacy-harness`. The flag exists
unconditionally at `2.25.1`..`2.25.4`, has first precedence, and pins the
legacy Python harness deterministically on both channels — exactly the
backend the frozen corpus covers. This is adapter-private policy like
`--trust` and `--agent plan`, mechanically adaptable without a new public
lifecycle, authority, or operation. The Unified Harness, `--smart-approve`,
and the `smart-approve` profile stay unmapped.

## Effect on Research 150, 199, and 252

Research 150's wire identity, Research 199's caller-decreasing
`--max-turns` `1..=8` binding with omission argv `8`, and Research 252's
Plan-only profile boundary carry over unchanged: every source fact they
freeze is byte-stable through `2.25.4`, and the typed maximum-turns
selection needs no change. Research 252's empty beyond-Plan deliver-now set
gains a new closed row: `smart-approve` is Unified-Harness-only and ships
dark, so it cannot be a non-widening primary on this route.

## Decision and disposition

Identity-first decision: a compatible exact-point rebind. The
`mistral-vibe.headless.release-window-1` claim moves to one maintained
`2.25.4` point with the unchanged `mistral-vibe.headless.stdio-streaming-v1`
behavior revision and `QualifiedOnly` posture, and the selected argv gains
adapter-private `--legacy-harness`. No range, second exact point, exclusion
entry, or unverified-newer posture is added. The frozen
`mistral-vibe-headless-2.24.2` decoder corpus still covers the selected
wire because every selected-surface input is unchanged. The new evidence
corpus is
[`mistral-vibe-headless-2.25.4`](../../crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.25.4/)
with the mutation-sensitive assertions in
[`mistral_vibe_2_25_4_delta_ledger.rs`](../../crates/swallowtail-adapter-mistral-vibe/tests/mistral_vibe_2_25_4_delta_ledger.rs).

## Non-goals

- mapping the Unified Harness, `--smart-approve`, or any beyond-Plan profile
- live `--prompt`, login, `--setup`, or provider credit
- installing Vibe, updating the host, or executing a downloaded artifact
- qualifying GitHub-only `2.24.4` or inferring a range across successors
- `vibe-acp`, TUI, continue/resume, teleport, `--output json`/`text`
