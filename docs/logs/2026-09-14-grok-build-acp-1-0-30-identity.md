# 2026-09-14 Grok Build ACP 1.0.30 Identity

## Result

Official Grok Build npm stable `1.0.30` was frozen against the `1.0.5` ACP
ceiling. `@xai-official/grok` reports `latest` `1.0.30` (published
2026-09-11T23:11:16.162Z) and `alpha` `1.0.31` (published
2026-09-13T15:01:07.004Z); the alpha candidate is not the stable target.
Under the same channel rule Research 294 used, every published
non-prerelease version after the ceiling is a stable, so all 25 points
`1.0.6..=1.0.30` are hops. `1.0.24`, recorded as alpha at Research 294, now
sits below `latest` and counts as a published stable; the registry does not
record dist-tag history. The first unpublished stable after `latest` is
`1.0.32`.

The installed host `grok 1.0.30 (04b7ffed98c6) [stable]` is byte-identical to
the brotli-decompressed official `darwin-arm64` payload
(`d53b6e543e48…`); the host was observed and never changed. Every hop's
launcher and platform tarball was verified against its published
`sha1`/`sha512` integrity, decompressed, hashed, and probed; downloaded
artifacts were never executed. The `1.0.5` platform digest and executable
reproduce the frozen corpus, `1.0.25` reproduces Research 305, and the
`1.0.13` revision reproduces Research 294. The public `xai-org/grok-build`
mirror does not carry the npm `gitHead` commits, so published artifacts are
the support authority.

Every mapped ACP method, callback, key, permission id, auth literal, model
and effort literal, and vendor channel is present in all 26 compared
executables with a byte-identical presence map. The embedded default-model
document changes once, at `1.0.11`, only by dropping the unread
`show_model_fingerprint` key; the default stays `grok-4.6`, the ids stay
`grok-4.6`/`grok-4.5`, and the efforts stay `xhigh`/`high`/`medium`/`low`.
The embedded ACP module inventory keeps all 62 mapped-core modules at every
hop and only adds feature modules or performs internal renames and crate
splits. The shipped file inventory is stable except the native payload and
its version-bearing `package.json`, plus the `1.0.14` launcher bootstrap
split, which is outside the ACP wire. The classification is a compatible
extension; the exact `1.0.25` catalogue claim and the `1.0.4`/`1.0.5`
registered-tool courier stay independently bounded and do not move.

Production claims stayed at `1.0.5` in this record; thirteen mutation-
sensitive identity tests enforce that boundary across the frozen identity,
protocol, and dist inventories.

## Next

Apply the compatible-extension decision on the Grok Build ACP claim surface
through g05.064.
