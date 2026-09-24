# Research 339: Command Code 1.65.0 Identity

Status: promoted; identity evidence. The claim rebind lands in g06.021 with
this record.

Owner: Tom
Date: 2026-09-24
Card: g06.021
Authority: Contract 029; Research 317, 330; the Command Code
prepared-integration guide; and the official npm `command-code` registry.

## Answer

Official npm stable `latest` is `1.65.0`, published
`2026-09-23T23:26:57.823Z`. The exact `command-code.npm` point can move from
`1.54.0` to `1.65.0` as a compatible extension of the selected headless
surface. This is one exact `QualifiedOnly` point, not a range. Research 330
paid-model live acceptance remains bound to `1.54.0`. Research 116/118 remain
bound to `1.15.1`. None of that live evidence transfers to `1.65.0`.

## Method and channel boundary

The official `dist-tags`, full version list, and publication times were
rechecked. The previous ceiling and all 19 published stable successors were
retrieved from the official registry into `/tmp`; npm SHA-1 and SHA-512
integrity values were checked, and tarball SHA-256 values were recorded. Every
package was extracted and inspected without executing a downloaded artifact.
No prompt, login, credential, provider session, installation, host update, or
provider credit was used. The installed package is still `1.54.0`; that was
observed from `package.json` and the launcher digest, not by running the
binary. The host was not changed.

The compared stable points are `1.54.0`, followed by `1.54.1`, `1.54.2`,
`1.55.0`, `1.55.1`, `1.56.0`, `1.56.1`, `1.56.2`, `1.57.0`, `1.58.0`,
`1.58.1`, `1.59.0`, `1.60.0`, `1.61.0`, `1.62.0`, `1.62.1`, `1.63.0`,
`1.64.0`, `1.64.1`, and `1.65.0`. `1.66.0` was queried as the first later
stable and is not published. Alpha, beta, and rc tags are prerelease channels
and are not stable authority. The `1.54.0` tarball SHA-256
`ede20384f9f32279e224c699ef5b9d6966da182acf90ca8ad943524b1ad151e4` reproduces
Research 317.

The latest tarball SHA-256 is
`eba58e569b23e03cd6721d9ee8c0ba0ea5ae3cc879e669c39c49b5da63bfc204`; its
published SHA-1 is `dfe718958eedc4c008493ad585ed28ff7b3e58a6`, integrity is
`sha512-BThYhIklhCYKF1/n8ElO+zqwlOC8vymRU40h0xi76to+jNP0asHRy5fdKHlwl2c1uztZLsfBLHGcurbYFuiCvQ==`,
and it contains 72 files. `dist/index.mjs` is byte-identical across the
complete chain at
`157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb`.

The reproducible identity and inventory records are frozen in
[`command-code-1.65.0`](../../crates/swallowtail-adapter-command-code/tests/fixtures/command-code-1.65.0/):
`identity.json` contains the complete npm publication record,
`dist-inventory.json` contains every extracted path and consecutive-hop
delta, and `protocol.json` contains the selected-surface presence map.

## Complete shipped-tree result

The package contains 71 files at `1.54.0` and 72 at `1.65.0`. There are 19
consecutive deltas, no removed paths, and one additive bundled skill path:
`dist/bundled/loop/SKILL.md` at `1.54.2..1.55.0`. Other changed paths are
`dist/cli.mjs`, `package.json`, `CHANGELOG.md`, the VSIX, and bundled
knowledge/docs. `dist/index.mjs` is byte-identical. `dist/cli.mjs` changes at
every hop, so the selected literals were checked at every point. The package
`bin`, main/type/files, engine (`>=22`), and license remain stable. The only
package.json field hop besides version is a `1.54.0..1.54.1` vitest
devDependency pin change.

## Selected-surface classification

All 20 points contain the selected invocation literals: `-p`/`--print`, JSON
output, plan permission mode, onboarding/update/trust/skills/session flags,
explicit model, and max-turns. The selected structured invocation remains:

```text
-p --output-format json --permission-mode plan --skip-onboarding
--no-session --no-auto-update --trust --no-skills --max-turns 8 -m <model>
```

`plan` remains a canonical `--permission-mode` choice. `--accept-edits`
(1.59.0) aliases `auto-accept` and is not emitted. Experimental sandbox
`--headless`/`--watch` (1.64.1) are a separate subcommand, not print mode.

The selected AgentEvent literals remain present at every point. Result and
usage keys are unchanged. `compaction_outcome` (1.59.0) is an unmapped
compaction event. `/loop` internals `delay_seconds`, `loop_owned`, and
`schedule_wakeup` (1.55.0) are unmapped skill surfaces; the route already
passes `--no-skills`. Exit-code 10 credit classification, stdin delivery,
redaction, cancellation, process ownership, deadlines, and joined cleanup
remain the existing mapped implementation boundaries; downloaded artifacts
were not live-executed.

No new driver, public lifecycle, authority, authentication, permission,
model, retention, failure, or continuation boundary is admitted.

## Decision and live-evidence boundary

The identity-first decision is `compatible-extension`: rebind the exact
`command-code.npm` claim point to `1.65.0`, retain claim
`command-code.headless-window-1`, behavior revision
`command-code.agent-event-ndjson-v1`, and `QualifiedOnly`, and reject the
unpublished `1.66.0`. Do not infer a range and do not retain `1.54.0` as a
second accepted point.

The provider-free all-hop evidence supports the selected decoder, invocation,
and local process/lifecycle shape. It does not migrate live acceptance:
Research 330 stays the exact-`1.54.0` paid-model record, and Research 116/118
stay exact-`1.15.1`. Live-derived cells stay gated at `1.65.0`.
