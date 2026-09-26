# 353 Antigravity Headless Adaptation To Current Official

Status: promoted. The catalogue claim advances to official `1.2.11`. The
`antigravity.headless` claim does not move; this record raises the exact
Contract 023 options to the operator.

Owner: Tom
Date: 2026-09-26
Card: swallowtail#065 (`antigravity.headless` adaptation, plan item
`version-currentness`)
Authority: Contracts 017, 023, and 029; Research 177, 283, 323, and 346; the
Antigravity prepared guide; and the official GitHub channel. Tom's 2026-09-15
ruling still holds: official release notes are the behavioural authority.

## Outcome

`antigravity.headless` cannot reach the current official stable `1.2.11`
without a Contract 023 acceptance. No later release publishes a finite
model-request retry bound or a disable control, so qualifying headless is a
provider-managed-retry exception and belongs to the operator, exactly as the
Research 346 decision predicted.

The same probe advances the catalogue claim from `1.2.7` to `1.2.11`: no
published hop names a selected `agy models` change. Headless stays at
maintained `1.1.9..=1.1.17`, its unqualified gap becomes `1.1.18..=1.2.11`,
and the Contract 023 options below are the ruling request.

## Remaining AllowUnverified rank

Named family only.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity catalogue and headless | not installed | catalogue `1.1.9..=1.2.11`; headless `1.1.9..=1.1.17` | operator-dispatched adaptation for the named headless stop; official GitHub stable is `1.2.11` |

Gemini is out of scope. `antigravity-acp` is another family. `1.1.8` stays
independently incompatible.

## Method

Re-probed official GitHub releases, tags, and assets on 2026-09-26. Official
latest is `1.2.11`, published `2026-09-25T04:22:17Z`, tag commit
`6dadd6227a49905f475d22b7f0afe59493229595`. The lane was promoted against
`1.2.7`, so the hop set extended to every stable published in between:
`1.2.8`, `1.2.9`, `1.2.10`, and `1.2.11`. The identity commit had not landed,
so Contract 029's In-Run Latest Movement rule covers the extension.

Retrieved the official linux-x64 and mac-arm64 tarballs for all four points
into `/tmp`. Every tarball SHA-256 matches GitHub's declared asset digest,
every archive holds only the single `antigravity` binary, and neither a
`1.2.12` release nor tag exists. Re-downloading `1.2.7` reproduced the
Research 346 digests, so the previous ceiling stands. Public compares
`1.2.7...1.2.8` through `1.2.10...1.2.11` are each one commit and change only
`CHANGELOG.md`, so the published release notes carry the behavioural record.

No provider operation, prompt, login, credential, installation, host update,
or downloaded-binary execution occurred. Host `agy` is not on `PATH`.

Exhaustive binary forensics stayed stopped under Tom's 2026-09-15 ruling. One
bounded targeted check asked whether the published `1.2.11` artifact exposes a
model-retry control name. It does (`AGY_CLI_MODEL_API_MAX_RETRIES`), but the
string is unpublished, its semantics are unverified, and it is recorded only
as an unresolved lead under the second ruling option below. It is not
behavioural authority.

The official docs site still publishes `--print-timeout` default `5m`
(https://antigravity.google/docs/cli/headless) while the `1.2.6` release note
publishes unlimited. Artifact identity agrees across channels and the notes
remain the behavioural authority, so this is a documentation lag, not a
channel disagreement; the docs page publishes no retry control either.

## Identity

| Version | Published | Tag commit | linux-x64 tarball SHA-256 / size | linux-x64 binary SHA-256 / size / Build ID | mac-arm64 tarball SHA-256 |
| --- | --- | --- | --- | --- | --- |
| `1.2.8` | 2026-09-22T04:12:17Z | `ad7d70342a108687a1b36db7573050de3e9c2c3e` | `244752206d1f65c01aff489628f1df51f1a3fddacaa8ed74984661ebb6d09136` / 59525457 | `c20434f0b9278196498069dac5a0a2e72bc0b5f8aebdf17c5d535b5369b76f67` / 216797392 / `65997a9e3a37b8a6666307959500a78e` | `f77d57d99ca83e50a3a767d70621c99962ce4242e5470220f3adbf1c50844538` |
| `1.2.9` | 2026-09-23T04:42:15Z | `818089f390e240921bb597b7a22ce9c96cdf7fe6` | `d9850373f3df866011024a961fa9740cc4adaac060eebe9c70fbf263ac6b2624` / 59668209 | `1dbb10f8295cc1ad2e558bd006c7808fe53b6c7f678a887eb557b576bb591711` / 217424080 / `268d50b56fcfeb754baa4ec32408b0b4` | `2b2671c846f62cb1159817517e4a9fff3e9a2ffe01f9dd0b3e227298d88b46f6` |
| `1.2.10` | 2026-09-24T06:26:54Z | `59a894a009e1ec73129112c8d744ff6e9eb5a81d` | `77cb69251292aa35b0b662f91f704f06dd787b72f7902a62db8c6d692989203e` / 60286675 | `aea7ed8df1e79b716c0ccd14c7d8086db75b14da535ffb7febdec9a488deff67` / 218566864 / `5d98c91992eba8921435a28d79f35d87` | `95d5d8ab8870b849a157f647bb4d9953184f97855cbfbedebdedcc421ca5b435` |
| `1.2.11` | 2026-09-25T04:22:17Z | `6dadd6227a49905f475d22b7f0afe59493229595` | `c91c62c5e6fa954f5a7e1d7b9ad417d749db4aa60a4ba0b3d604dec1b645d190` / 60414410 | `ec7cf797ecb0e1d91ddf3b6d9d6c1d616bb89f78a5b0e43536b72a7fce695f56` / 219545808 / `257bedb917787ceb25bbe0d36687c663` | `437a813cd7c606ccbb3180886887fc69361c28fe8e880327b3b82201afa900cc` |

Every mac-arm64 extracted digest and size is in the frozen corpus. The
Research 346 `1.2.7` linux-x64 tarball
`e410dd56d8c213ef12643d3ff5eaaab57a17e05bbf72e9415322f23879fc4a18` and
mac-arm64 tarball
`ce9fe3f4d6f44a2b1c83b334fc5c8f2975079959e24dd805e10eb49ab8c76a7e`
reproduce.

The live semver appears as a standalone string in the `1.2.10` and `1.2.11`
binaries. In `1.2.8` and `1.2.9` the bounded check finds it only fused to an
adjacent literal, so this record claims no clean per-release version literal
for those two points.

## Published selected-path classification per claim

The selected flag set is unchanged in the notes: `--print`,
`--output-format`, `--model`, `--mode`, `--sandbox`, `--effort`,
`--json-schema`, exact `--conversation`, and `models`.

| Hop | Catalogue | Headless |
| --- | --- | --- |
| `1.2.7→1.2.8` | unchanged | shutdown cancels open streaming connections immediately, so quitting no longer waits about 5 s |
| `1.2.8→1.2.9` | unchanged | headless daemon background processes now terminate when the run ends; headless runs wait for background tasks until the `--print-timeout` deadline, up to a 30-minute cap |
| `1.2.9→1.2.10` | unchanged | headless runs that streamed part of a response and then ended on a model or agent error now exit `3` with the `AGY_ERROR` line and the partial response in JSON error output |
| `1.2.10→1.2.11` | unchanged | reasoning-effort levels improved for models with different support, still selected with `--effort`; project custom agents now resolve in headless runs |

No hop names a selected `agy models` change, so the catalogue claim is a
compatible extension of
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` through `1.2.11`. Every
headless change lands on the model-request loop, the `-p`/`--prompt` path, or
the selected `--effort` mapping that the `1.1.22` stop already blocks. The
`1.2.11` `--effort` note is a selected mapping change of its own and would
need an adapter-private milestone whenever headless reopens.

## Why headless still stops

The Research 283 `1.1.22` stop stands and every later note widens or reshapes
the same behaviour without bounding it:

- `1.1.28` retries transient model errors "for much longer" with exponential
  backoff and changes `--print-timeout` expiry to partial output plus a
  successful exit.
- `1.2.1` adds automatic in-process retry for `502`, `503`, `504`,
  per-minute `429`, and mid-stream interruptions.
- `1.2.6` changes the headless default `--print-timeout` from 5 minutes to
  unlimited and adds the typed `AGY_ERROR` exit-3 failure.
- `1.2.7` caps only the per-attempt backoff at 30 seconds.
- `1.2.9` and `1.2.10` bound the background-task wait and the terminal error
  shape, not the retry attempt count.
- `1.2.11` moves the selected `--effort` mapping and publishes no retry
  control.

No release publishes a finite attempt bound, a disable control, or a
deterministic provider-neutral mapping. Contract 023 keeps the host deadline
distinct from provider-native retry, so the Swallowtail deadline and
`--print-timeout` cannot substitute. No separate acceptance exists for this
lane. Under Tom's ruling, the absence of a published bound is recorded
directly as the stop without deriving a hidden bound.

## Ruling request

Every option needs an operator ruling. The route moves no claim until one
lands.

1. **Accept the published provider-managed retry (Contract 023 exception).**
   Qualify headless `1.1.18..=1.2.11` on a new adapter-private milestone
   behaviour revision that accepts provider-native model-request retry as a
   provider-native budget. This is consumer-visible: a turn may outlive any
   local attempt count, the only terminating bound remains the Swallowtail
   host deadline, and the published per-attempt backoff cap is 30 seconds
   with no attempt bound. It also needs the `1.2.11` `--effort` mapping
   recorded in that milestone.
2. **Require a finite or disabling retry pin (Contract 029 pinned settings).**
   Qualify only if the approved route environment pins a model-retry control
   to a finite or disabling value. The `1.2.11` artifact carries the string
   `AGY_CLI_MODEL_API_MAX_RETRIES`, but it is absent from the official release
   notes and docs and its semantics are unverified, so this option needs an
   operator ruling and its own evidence before any claim edit. It also
   changes the route's required approved environment.
3. **Keep the ceiling.** Keep headless at `1.1.17` and open a fresh adaptation
   task. Contract 029's No Terminal Stop makes this a terminal stop unless it
   names that successor, so it is listed only for completeness.

## Decision

- **Catalogue: compatible-extension.** Advance to maintained
  `1.1.9..=1.2.11` on
  `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`. Qualify `1.2.8`
  through `1.2.10`. Synthetic `1.2.12` stays `UnverifiedNewer`.
- **Headless: stop; ruling requested.** Keep `1.1.9..=1.1.17`. The gap
  becomes `1.1.18..=1.2.11`. Do not raise
  `ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION`, add a milestone, or change
  the behaviour revision.
- No new public operation, no flatten onto Gemini CLI or `antigravity-acp`,
  no provider call, and no host change. Decoder specimens stay on
  `antigravity-cli-1.1.9`.

## Validation

Provider-free. `effigy validate:focused swallowtail-adapter-antigravity`,
`effigy package:verify-affected swallowtail-adapter-antigravity`,
`effigy qa:docs`, and `effigy qa:routes` pass. No downloaded artifact was
executed; no provider operation, prompt, login, credential use, installation,
or host update occurred.

## Sources

- [GitHub `1.2.11`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.2.11)
  and the release notes for `1.2.8` through `1.2.11`
- [Changelog at `1.2.11`](https://github.com/google-antigravity/antigravity-cli/blob/1.2.11/CHANGELOG.md)
- official `agy_cli_linux_x64.tar.gz` and `agy_cli_mac_arm64.tar.gz` for
  `1.2.8`..=`1.2.11`
- official `agy_cli_linux_x64.tar.gz` for `1.2.7` (Research 346 boundary
  reproduction)
- [Antigravity CLI headless documentation](https://antigravity.google/docs/cli/headless)
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.11/`
- [Research 283](./283-antigravity-1-1-26-identity.md),
  [Research 323](./323-antigravity-1-2-2-identity.md), and
  [Research 346](./346-antigravity-1-2-7-identity.md)
- [Contract 023](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md)
  and [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
