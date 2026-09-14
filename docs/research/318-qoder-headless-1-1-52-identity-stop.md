# 318 Qoder Headless 1.1.52 Identity Stop

Status: promoted; identity evidence only. This record lands a typed stop and
does not change the production claim.

Owner: Tom
Date: 2026-09-14
Card: g05.070 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 151, 200, 256, 308, and 317; the Qoder
prepared-integration guide; and the official npm `@qoder-ai/qodercli` registry.

## Answer

Official npm stable `latest` is `1.1.52`, published
`2026-09-14T12:07:54.560Z`; `1.1.35-beta.2` stays on the separate beta channel.
The exact `qoder.package` point stays at `1.1.25`. The selected headless route
cannot move to `1.1.52` as a compatible extension: the route's own argv
`--max-turns 8` stops being historical inert history at **`1.1.30`** and becomes
the AgentLoop turn ceiling for `qodercli --print`. That is a selected
run-lifecycle and bounded-limit failure change, and it needs an operator ruling
on the route's turn-binding policy before any claim edit.

## Method and channel boundary

The official dist-tags, full version list, and publication times were rechecked
at the start of the run and again immediately before the identity commit. The
exact `1.1.25` baseline and all **27** published stable successors
`1.1.26..=1.1.52` were retrieved from the official registry into `/tmp`; npm
SHA-1 (`shasum`) and SHA-512 (`integrity`) values were read from the packument,
tarball SHA-256 values were computed and recorded, and every package was
extracted and inspected without executing a downloaded artifact. `1.1.53` was
queried as the first later stable and is not published (`404`).

No prompt, login, credential, provider session, installation, host update, or
provider credit was used. `qodercli` remains absent from this host, so there is
no installed `--version` observation. Host Node `22.23.2` satisfies the
package's declared `>=20.0.0` engine at every compared point. The six
`@qoder-ai/qodercli-ripgrep-*` optional platform packages were not downloaded,
extracted, or executed.

The compared stable points are `1.1.25`, then `1.1.26` through `1.1.52`
inclusive. Alpha, beta, and rc spellings are prerelease channels and are not
stable authority.

The reproducible identity and inventory records are frozen in
[`qoder-headless-1.1.52`](../../crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.52/):
`identity.json` holds the complete publication and digest record,
`dist-inventory.json` holds the extracted tree and consecutive-hop deltas, and
`protocol.json` holds the selected-surface presence map and the per-hop
`--max-turns` authority classification. The mutation-sensitive assertions live
in
[`qoder_1_1_52_delta_ledger.rs`](../../crates/swallowtail-adapter-qoder/tests/qoder_1_1_52_delta_ledger.rs).

## Complete shipped-tree result

The package holds 33 files at every point `1.1.25..=1.1.46`, 35 at `1.1.47`,
29 at `1.1.48`, and 28 at `1.1.49..=1.1.52`; unpacked size grows from
`62 086 670` to `67 904 596` bytes. There are 27 consecutive deltas with no
path added or removed except:

- added at `1.1.46..1.1.47`:
  `package/bundle/vendor/qoder-security/bin/qodersec-update.cmd` and
  `package/bundle/vendor/qoder-security/bin/qodersec-update.sh`
- removed at `1.1.47..1.1.48`: the six
  `package/bundle/sandbox-macos-*.sb` seatbelt profiles
- removed at `1.1.48..1.1.49`: `package/bundle/policies/sandbox-default.toml`

`package/bundle/qodercli.js` — the shipped CLI bundle that implements the
selected print route — is non-byte-identical at **every** hop, so the selected
literals were checked at all 28 points rather than inferred from semver
continuity. `package/bundle/qoder-npm-dispatcher.cjs` is byte-identical across
`1.1.25..=1.1.28` and byte-identical again, with a new revision, across
`1.1.29..=1.1.52`. `package/postinstall.cjs` is byte-identical across the whole
chain. `package/package.json` is version-stamped at every hop, so its hash is a
version fingerprint and not a behaviour signal.

Package `bin`, `type`, `files`, `engines` (`>=20.0.0`), `os`, `scripts`,
`gitHead` (absent), and `repository` (absent) are stable at every point. The
only dependency-metadata change is `@silvia-odwyer/photon-node@0.3.4` added to
`dependencies` at `1.1.38`; it is install-time only and does not alter the
shipped selected entrypoint.

## Selected-surface classification

The selected invocation is unchanged at every point:

```text
qodercli --print --output-format stream-json --permission-mode dont_ask
--max-turns 8 --no-session-persistence --cwd <cwd> <prompt>
```

All selected literals are present at all 28 points: `--print`,
`--output-format`, `stream-json`, `--permission-mode`, `dont_ask`,
`--max-turns`, `--no-session-persistence`, `--cwd`, `--acp`,
`error_max_turns`, `error_during_execution`, `Operation aborted`,
`aborted_streaming`, `protocol_version`, `stream_event`, and
`--include-partial-messages`.

Mode dispatch is structurally unchanged. `--remote-control` still selects the
remote worker, `--acp` still wins over `--print`, SDK stdio still requires an
SDK entrypoint plus bidirectional `--input-format stream-json` /
`--output-format stream-json`, and `--print`/`--prompt` still select the
headless path. `1.1.52` adds a hidden `--sdk` flag and a `daemon` kind that is
only reachable through that flag; the route never passes it. The tagged
`assistant`, `stream_event`, and `result` envelopes are structurally identical
at `1.1.25` and `1.1.52`, and the adapter still ignores `system` and
`stream_event` while decoding `assistant` text and the terminal `result`
`subtype`/`is_error` pair.

Permission mode is unchanged: the accepted mode set is still
`default`, `accept_edits`, `bypass_permissions`, `dont_ask`, `auto`, and
`(x.yolo || x.dangerouslySkipPermissions) → bypass_permissions` is the same
expression at both ends of the window. Authentication (`QODER_PERSONAL_ACCESS_TOKEN`
or persisted `qoder login` state), `--no-session-persistence` print-only
retention, `-w`/`--cwd` working resource, one owned stdio child, join-or-kill
cleanup, and the required host process deadline are all unchanged.

Two bounded deltas are recorded rather than promoted:

- The `system`/`init` `protocol_version` string moves `1.2.0 → 1.3.0` at
  `1.1.29` and `1.3.0 → 1.4.0` at `1.1.43`. The adapter treats `system`
  envelopes as ignored and does not decode this field, so the change is inert
  for the selected decoder.
- `1.1.52` additionally recognises abort terminal reasons `aborted_tools` and
  `abort` and rewrites abort-shaped errors to `Operation aborted`. The
  adapter's selected signals — `terminal_reason == "aborted_streaming"` and an
  `errors` entry equal to `Operation aborted` — are still emitted, so the
  selected cancellation classification is preserved and only upstream
  recognition widened.

The removed sandbox assets are a bounded unselected delta. The seatbelt
profiles and `policies/sandbox-default.toml` feed the CLI's own sandbox feature,
which is only enabled by argv `-s`/`--sandbox` or by the host-local
`tools.sandbox` setting. The route argv never selects a sandbox profile, and the
prepared integration already states that ambient-host isolation is not
filesystem or descendant-process containment.

## The stop boundary: `--max-turns` stops being inert

Research 200 proved that at exact `1.1.25` the route argv `--max-turns 8` was
historical inert history: the CLI value was copied only to Config
`maxSessionTurns` for the `error_max_turns` text formatter, while the selected
CLI headless session bound the package constant `1000`, and `driveQuery` passed
`maxTurns: this.config.maxTurns ?? 1000` into the AgentLoop.

The all-hop ledger shows that property ending at exactly one hop:

| Point | `driveQuery` bound | Headless session config bound | CLI runner takes argv `maxTurns` | `--max-turns` argParser | CLI value → `Config.maxSessionTurns` |
| --- | --- | --- | --- | --- | --- |
| `1.1.25..=1.1.29` | `this.config.maxTurns ?? <const 1000>` | package constant `1000` | no | no | yes |
| `1.1.30..=1.1.52` | `this.config.maxTurns` | value forwarded from argv | yes | yes | no |

The smallest exact counterexample is therefore `1.1.29 → 1.1.30`. At `1.1.30`
the CLI `--max-turns` option gains a numeric `argParser`, both headless mode
entry points begin forwarding `argv.maxTurns` into the headless session, that
session begins binding the forwarded value (normalising `0` to `undefined`),
and `driveQuery` loses its fixed `1000` fallback. The AgentLoop loop-top guard
`null != d && DA.turnCount >= d` is unchanged, so from `1.1.30` onward the
selected route's real ceiling is the argv value `8`.

Effect on the selected route: a print run that could previously continue for up
to 1000 turns now terminates with the `error_max_turns` terminal result at 8
turns. That changes the selected run's bounded-limit lifecycle and the failure
the adapter projects through `swallowtail.qoder.headless.max_turns`, and it
turns a previously corpus-only decoder path into reachable route behaviour.

## Decision and disposition

The identity-first decision is `stop`. No claim edit lands:

- `QODER_PACKAGE_VERSION` stays exact `1.1.25`
- claim `qoder.headless.package-window-1` stays one exact `QualifiedOnly`
  maintained point
- behavior revision `qoder.headless.stdio-stream-json-v1` is unchanged
- no range, second point, exclusion, or unverified-newer posture is added
- the Qoder prepared guide, route matrix, activity matrix, feature matrix, and
  architecture ceiling stay as they are, because the qualified point did not move

Research 151's `1.1.25` decoder specimens and Research 256's empty deliver-now
skill-visibility disposition are untouched. Package `bundle/builtin/*/SKILL.md`
files, init `skills`/`plugins` fields, and `qoder skills list` are not selected
run visibility, and `g05.039`/`g05.040` stay independently gated.

The route now returns to the operator via Chatterbox for a planning ruling: on a
`1.1.30`-or-later point, does the route keep `--max-turns 8` as a real ceiling on
a new behavior revision, choose a different bound, or stay at `1.1.25`? That is a
new public lifecycle decision under Contract 029 rather than something this
lane may settle by inference.

## Validation

Provider-free only. The identity corpus and its mutation-sensitive Rust ledger
test compile and pass; no downloaded artifact was executed; no provider
operation, prompt, login, credential use, installation, or host update occurred.
