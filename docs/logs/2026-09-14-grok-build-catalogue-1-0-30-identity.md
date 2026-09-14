# 2026-09-14 Grok Build Catalogue 1.0.30 Identity

## Result

The separately prepared `grok-build.catalogue` route was requalified from its
exact `1.0.25` point toward official and installed stable `1.0.30`. npm
`@xai-official/grok` reports `latest` `1.0.30` and `alpha` `1.0.31`; `1.0.31`
is a channel candidate, not the stable target, and the first unpublished
stable after `latest` is `1.0.32`. The installed host `grok 1.0.30
(04b7ffed98c6) [stable]` is byte-identical to the brotli-decompressed official
`darwin-arm64` payload (`d53b6e543e48…`) and was observed, never changed.

The `darwin-arm64` platform tarball for the previous exact point and every
later stable (`1.0.25..=1.0.30`) was verified against its published digest,
decompressed, hashed, and probed; downloaded artifacts were never executed.
All six tarball, brotli, and executable digests reproduce the Research 314 ACP
corpus, and the `1.0.25` executable reproduces Research 305 and the previous
installed host.

Every catalogue-specific input is stable across the whole hop set: the exact
root flag `--no-auto-update`, the `models` subcommand option set with no
`PROMPT` argument, the authentication preamble literals, the `*`/`-` bullet
format literals with byte-identical occurrence counts, the
`xai-grok-pager/src/models.rs` module path, and the embedded
`default_models.json` document (`9d6924ec760a…`, 2323 bytes, two copies,
default `grok-4.6`, ids `grok-4.6`/`grok-4.5`). The combined surface digest is
`aa3ad436d5a6c8eb893d1091c02f0cadbdabe9338bce38a4205d9a04768d3e02`. The
classification is a compatible extension of the exact `QualifiedOnly`
catalogue point with the behavior revision unchanged.

Production claims stayed at exact `1.0.25` in this record. Five
mutation-sensitive identity tests enforce the hop ledger, argv grammar, and
literal counts from
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.30-catalogue/`.

## Next

Apply the exact-point advance on the Grok Build catalogue claim surface
through g05.067, then run the provider-free gate and the installed exact
`1.0.30` prompt-free authenticated catalogue observation.
