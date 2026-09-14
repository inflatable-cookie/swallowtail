# Grok Build catalogue 1.0.30 identity

Frozen against the official npm `@xai-official/grok` stable channel on
2026-09-14 for the separately prepared `grok-build.catalogue` route. The
previous exact catalogue point was `1.0.25`; official `latest` and the
installed host are both `1.0.30`, and `alpha` `1.0.31` is a channel candidate,
not the stable target.

`identity.json` freezes:

- the npm `latest`/`alpha` pointers, the published stables compared
  (`1.0.25..=1.0.30`), and the first unpublished stable after `latest`
  (`1.0.32`);
- host identity: `grok 1.0.30 (04b7ffed98c6) [stable]`, executable SHA-256
  `d53b6e543e48…`, 141869568 bytes, equal to the brotli-decompressed
  `bin/grok.br` from the official `@xai-official/grok-darwin-arm64@1.0.30`
  package;
- every hop's official platform tarball, brotli payload, and decompressed
  executable digest and size, reproduced from the Research 314 ACP identity
  corpus in this run;
- the catalogue-specific surface: exact argv `["--no-auto-update", "models"]`,
  the root-flag placement grammar, the subcommand option set, the shipped
  authentication preamble and `*`/`-` bullet format literals with their exact
  counts, the `xai-grok-pager/src/models.rs` module path, and the embedded
  default-model document digest, size, and copy count.

The catalogue literal counts and the embedded default-model document are
byte-identical across every compared hop. The live listing owns membership,
order, and default; the embedded document only supplements matching exact ids,
unknown valid ids pass through with empty metadata, and absent source fields
stay absent. The `1.0.25` fixture under
`../grok-1.0.25-model-catalogue/` and Research 305/306 stay as history: they
are not a second accepted catalogue point.

`models.txt` and `unknown-ids.txt` are the fake-process stdout specimens for
the accepted document and the unknown-id pass-through. They are byte-identical
to the historical `1.0.25` specimens: Research 316 proves the shipped
authentication preamble and `*`/`-` bullet grammar do not change from `1.0.25`
through `1.0.30`, so the same document bytes are valid at the new exact point.
The accepted `1.0.30` live capsule is `live-capsule.json`.

No downloaded artifact was executed. No prompt, inference, model session,
tool, install, or host update occurred. The only provider contact was the
final prompt-free authenticated `--no-auto-update models` observation recorded
in `live-capsule.json`; the `--help`/`--version` argv-grammar probes are
provider-free exits.
