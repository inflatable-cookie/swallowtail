# Research 317: Command Code 1.54.0 Identity

Status: promoted; identity evidence only. Production claim changes land in
g05.068 after this record.

Owner: Tom
Date: 2026-09-14
Card: g05.068 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 116, 118, 308, 316; the Command Code
prepared-integration guide; and the official npm `command-code` registry.

## Answer

Official npm stable `latest` is `1.54.0`, published
`2026-09-14T01:59:24.573Z`. The exact `command-code.npm` point can move from
`1.15.1` to `1.54.0` as a compatible extension of the selected headless
surface. This is one exact `QualifiedOnly` point, not a range. Research 116's
authenticated completion, tool, usage, and credit-failure observations and
Research 118's two-turn private-id continuation remain bound to `1.15.1` and
do not transfer.

## Method and channel boundary

The official `dist-tags`, full version list, and publication times were
rechecked. The baseline and all 67 published stable successors were retrieved
from the official registry into `/tmp`; npm SHA-1 and SHA-512 integrity values
were checked, and tarball SHA-256 values were recorded. Every package was
extracted and inspected without executing a downloaded artifact. No prompt,
login, credential, provider session, installation, host update, or provider
credit was used. The installed `command-code --version` observation remains
exact `1.15.1`; it was not changed.

The compared stable points are `1.15.1`, followed by
`1.16.0`, `1.17.0`, `1.18.0`, `1.18.1`, `1.19.0`, `1.19.1`, `1.20.0`,
`1.21.0`, `1.22.0`, `1.23.0`, `1.23.1`, `1.23.2`, `1.24.0`, `1.25.0`,
`1.26.0`, `1.27.0`, `1.27.1`, `1.27.2`, `1.28.0`, `1.28.1`, `1.28.2`,
`1.28.3`, `1.28.4`, `1.29.0`, `1.30.0`, `1.30.1`, `1.31.0`, `1.32.0`,
`1.32.1`, `1.32.2`, `1.33.0`, `1.33.1`, `1.34.0`, `1.35.0`, `1.35.1`,
`1.36.0`, `1.37.0`, `1.38.0`, `1.38.1`, `1.38.2`, `1.39.0`, `1.39.1`,
`1.39.2`, `1.39.3`, `1.40.0`, `1.40.1`, `1.41.0`, `1.42.0`, `1.43.0`,
`1.44.0`, `1.45.0`, `1.46.0`, `1.47.0`, `1.47.1`, `1.48.0`, `1.49.0`,
`1.49.1`, `1.50.0`, `1.50.1`, `1.51.0`, `1.51.1`, `1.51.2`, `1.51.3`,
`1.52.0`, `1.53.0`, `1.53.1`, and `1.54.0`. `1.54.1` was queried as the
first later stable and is not published. Alpha, beta, and rc tags are
prerelease channels and are not stable authority.

The installed host's Node `22.23.2` satisfies the official package's declared
`>=22` engine. The host launcher `dist/index.mjs` is byte-identical to the
official entrypoint at every compared point. The latest tarball SHA-256 is
`ede20384f9f32279e224c699ef5b9d6966da182acf90ca8ad943524b1ad151e4`; its
published SHA-1 is `aeab4ff76224d486c71bbfe24cf16aae9b75e24e`, integrity is
`sha512-AM19Tk6dSORLSGpw1YcCHdrIDY9GvRfx8hyY0evCnCX8qGRh0gLncSrIJkcK1CulY9hoD3+zHGR1DKMxa4K2lg==`, and it contains 71 files.

The reproducible identity and inventory records are frozen in
[`command-code-1.54.0`](../../crates/swallowtail-adapter-command-code/tests/fixtures/command-code-1.54.0/):
`identity.json` contains the complete npm publication record,
`dist-inventory.json` contains every extracted path and consecutive-hop
delta, and `protocol.json` contains the selected-surface presence map.

## Complete shipped-tree result

The package contains 63 files at the baseline and 71 at `1.54.0`. There are
67 consecutive deltas, no removed paths, and eight additive bundled reference
paths. The full per-hop changed and identical sets, hashes for the key files,
and package metadata are in `dist-inventory.json`. The only additions are:

- `dist/bundled/config/references/permissions.md`
- `dist/bundled/config/references/settings.md`
- `dist/bundled/config/SKILL.md`
- `dist/bundled/command-code-knowledge/reference/plans.md`
- `dist/bundled/command-code-knowledge/reference/tools.md`
- `dist/bundled/design/references/accessibility.md`
- `dist/bundled/design/references/severity.md`
- `dist/bundled/command-code-knowledge/reference/byok.md`

`dist/index.mjs` is byte-identical across the complete chain. `dist/cli.mjs`
changes at every hop, so the selected literals were checked at every point,
not inferred from semver or a changelog. The package `bin`, main/type/files,
engine, license, and README boundaries remain stable. Dependency metadata
changes at `1.23.2` and `1.27.2`; those install-time changes do not alter the
selected shipped entrypoint or route.

## Selected-surface classification

All 68 points contain the selected invocation literals: `-p`, JSON output,
plan permission mode, onboarding/update/trust/skills/session flags, explicit
model, and max-turns. The selected structured invocation remains:

```text
-p --output-format json --permission-mode plan --skip-onboarding
--no-session --no-auto-update --trust --no-skills --max-turns 8 -m <model>
```

The selected AgentEvent literals remain present at every point: run/turn and
message boundaries, model request/trace, thinking, text, message update,
tool queued/running/completed, turn/run end, and run error. The selected result
and usage keys remain `type`, `subtype`, `sessionId`, `stopReason`, `usage`,
`durationMs`, `finalText`, `inputTokens`, `outputTokens`, `cacheReadTokens`,
and `cacheWriteTokens`. `run_end.result.nextState` remains outside the
adapter's authority. Exit-code 10 credit classification, stdin delivery,
redaction, cancellation, process ownership, deadlines, and joined cleanup
remain the existing mapped implementation boundaries; downloaded artifacts
were not live-executed.

The complete inventory also bounds unmapped additions. `--tools-all` and
`--tools-enable` first appear at `1.19.1`; `--local-only` first appears at
`1.54.0`; `--effort` is present but unselected. `interaction_requested` and
`interaction_resolved` appear at `1.54.0` and are unknown namespaced activity,
never terminal authority. TUI, Provider API, mods/taste, session catalogue or
export, aliases, and other options remain unselected. No new driver, public
lifecycle, authority, authentication, permission, model, retention, failure,
or continuation boundary is admitted.

## Decision and live-evidence boundary

The identity-first decision is `compatible-extension`: rebind the exact
`command-code.npm` claim point to `1.54.0`, retain claim
`command-code.headless-window-1`, behavior revision
`command-code.agent-event-ndjson-v1`, and `QualifiedOnly`, and reject the
unpublished `1.54.1`. Do not infer a range and do not retain `1.15.1` as a
second accepted point.

The provider-free all-hop evidence supports the selected decoder, invocation,
and local process/lifecycle shape. It does not migrate live acceptance:
Research 116 stays the `1.15.1` record for authenticated completion, tool
lifecycle, usage, and credit failure, and Research 118 stays the `1.15.1`
record for private exact-id interactive continuation. The corresponding
feature cells are gated in the claim batch with a named follow-up for exact
`1.54.0` live requalification. No old live result is presented as a
`1.54.0` observation.
