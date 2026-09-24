# 2026-09-24 Grok Build ACP 1.0.41 Identity

## Result

Official Grok Build npm `latest` and `alpha` moved to `1.0.41` (published
2026-09-22T16:39:47.725Z; packument modified 2026-09-22T21:33:00.806Z;
`gitHead` `4220f3b224a672ff2641e35ba78ef6b0c6fd7069`) after the `1.0.40`
study. Host `grok` is missing; that is not a gap and was not installed.

The `1.0.41` launcher and linux-x64 platform tarballs were verified
against their published `sha1`/`sha512` integrity, decompressed, hashed,
and probed; darwin-arm64 was cross-checked. Downloaded artifacts were
never executed. `xai-org/grok-build` releases and tags are empty, so the
published artifacts are the support authority.

The `1.0.41` selected-literal presence map is byte-identical to `1.0.40`
on both linux-x64 (`4cceb3e6fc78…`) and darwin-arm64 (`4a548e4dc768…`).
The embedded default-model document is unchanged (`9d6924ec760a…`);
default `grok-4.6`, ids `grok-4.6`/`grok-4.5`, efforts
`xhigh`/`high`/`medium`/`low`. All 62 mapped-core ACP modules persist; the
only module delta is one added unmapped `subagent_handoff` module. Shipped
trees stay wrapper 5 / platform 4; only `package.json` and `bin/grok.br`
change. Classification is a compatible extension at the current official
stable. The exact `1.0.30` catalogue claim and the `1.0.4`/`1.0.5`
registered-tool courier stay independently bounded and do not move.

Evidence is frozen in
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.41/` and
enforced by the `grok_1_0_41_identity` suite.

## Next

Apply the compatible-extension decision on the Grok Build ACP claim
surface only, raising the maintained window to `1.0.41`. Do not reopen the
catalogue exact pin.
