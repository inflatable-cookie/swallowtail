# Research 443: Grok Build Catalogue Default-Header Mapping

Status: complete for the selected channel and the catalogue route only. The
catalogue claim gains one private behaviour milestone and a second segment; the
exact `1.0.30` point and its original mapping stay. The ACP claim is untouched.

Observed 2026-10-26 on `grok-build.executable`, `grok-build.catalogue` only.
No vendor process, credential, prompt, session, inference, install, host
mutation, consumer change, upstream post, or tag occurred. Downloaded
artifacts were brotli-decompressed and statically inspected, never executed.

## Refreshed channel

Official npm `@xai-official/grok` now reports `latest` `1.0.70` and `alpha`
`1.0.70`; the alpha tag no longer sits ahead of the selected stable. Published
stable `1.0.31` through `1.0.70` are forty contiguous selected-channel hops
after the preserved exact `1.0.30` point. Research 405's `1.0.46` was the
selected stable on 2026-10-08; it is now a historical point inside the window,
not the ceiling. `1.0.47` through `1.0.49`, which Research 405 recorded as
untagged publishes, are ordinary published points in this window.

The ACP route keeps its own, independently qualified `1.0.46` ceiling. Official
`1.0.47..=1.0.70` are permitted there only as `UnverifiedNewer` under that
claim's allow-unverified posture, and this record does not qualify ACP behaviour
on any of them.

## Artifact corpus

The [1.0.70 selection corpus](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.70-catalogue/README.md)
freezes every compared point `1.0.30` through `1.0.70`:

- registry `dist-tags`, published times, source commit identifiers where the
  vendor publishes them, npm SRI and SHA-1 values, locally computed tarball
  SHA-256 values, and the wrapper's optional platform pin per hop;
- complete shipped-file inventories for the wrapper (5 files) and the Darwin
  ARM64 package (4 files) with per-file size and SHA-256, and the added,
  removed, changed and identical path sets for every hop;
- each decompressed executable's size and SHA-256, its single embedded
  `<version> (<revision>)` self-reported build id, the whole-binary catalogue
  literal counts, both embedded `default_models.json` documents, and the exact
  catalogue formatter class with its format pieces and argument placement.

Every downloaded wrapper and platform tarball reproduced its published
integrity and SHA-1. The overlapping `1.0.30..=1.0.46` values reproduce the
Research 405 corpus exactly — runtime digests and sizes, tarball digests,
marker counts, catalogue literal counts, embedded documents and per-file
inventories — and the new test asserts that reproduction instead of rewriting
the frozen file. Across all forty-one points only the two manifests and
`bin/grok.br` change on the platform side; the wrapper README also moves once,
at `1.0.56`. No path is added or removed.

## Shipped formatter and control flow

The public official source for the original formatter is `xai-org/grok-build`
(Rust), at `SOURCE_REV` `559751fdcec02d413e4c57c8832ab275e4f44980`
(`crates/codegen/xai-grok-pager/src/models.rs`). It prints the auth-status
banner, a blank line, then `Default model: {}` for
`state.current_model_id`, a blank line, `Available models:`, and one row per
entry: `  * {} (default)` when the row id equals `current_model_id`, otherwise
`  - {}`. `crates/codegen/xai-grok-shell/src/cli_models.rs` shows that state
arriving as an ACP `SessionModelState` from an `x.ai/models/list` request after
one initialize, so the "default" is the provider-reported current model and may
name an id that is not among the listed rows.

The public source ends at the `1.0.45`-era sync, so the two later formatter
classes are established from the shipped Darwin ARM64 executables by static
format-piece and argument decode (no execution):

- **`1.0.46..=1.0.61`, note as a piece.** In exact `1.0.46` (runtime SHA-256
  `e8daa302…`) the static piece array at file offset `0x8ab4650`
  (`__DATA_CONST,__const` VMA `0x108ab4650`) holds
  `("Default model: ", 15)` then `(" (not in the list below)\n", 25)`, and the
  code at `0x103646504..0x103646550` builds
  `Arguments { pieces: (that array, 2), args: (…, 1) }`. The single argument is
  the same current-model value the row loop compares with `bcmp` against each
  listed id, and its formatter at `0x103b73d7c` reads a compact-string value
  with data at `+0x10`. The emitted header is therefore
  `Default model: <default id> (not in the list below)`: the marker follows an
  interpolated id and never replaces it. The branch is taken when the row loop
  finds no match, and the zero-row path (`cbz` at `0x1036464a4`) joins the same
  marker branch. The alternative array at `0x108ab4670`
  (`("Default model: ", 15)`, `("\n", 1)`) keeps the plain header for a listed
  default. Row arrays (`0x108ab4690`, `"Available models:\n"`, `0x108ab46a0`
  `"  * "` + `0x108ab46c0` `" (default)\n"`, `0x108ab46d0` `"  - "`) are the
  same ones frozen for `1.0.30`.
- **`1.0.62..=1.0.70`, note as an argument.** From `1.0.62` (runtime SHA-256
  `ec434e83…`) the same output is built from four pieces — `"Default model: "`,
  `""`, `"\n\nAvailable models:"`, `"\n"` — and three arguments at
  `0x100141f50..0x100141f88` (`__DATA_CONST` piece array VMA `0x109caad48`);
  `1.0.70` (runtime SHA-256 `5c8a6882…`) is identical at
  `0x10016a3c8..0x10016a3f8` with piece array VMA `0x109fcb9d8`. The first
  argument is again the current-model value, the second is the marker literal
  as a `&str` whose length the code selects as `24` when no listed row matches
  (`0x10016a1e0` → `0x10016a33c`, and `0x10016a244..0x10016a254` for the
  no-match path) and `0` when one does, and the third is an empty string. The
  emitted text is unchanged from the previous class; only literal pooling and
  argument placement moved. Rows are `"\n  * "` + id + `" (default)"` or
  `"\n  - "` + id, and the `*` marker is still chosen by exact id equality
  (`0x10016b790..0x10016b888`).
- **`1.0.30..=1.0.45`, no marker.** The piece array at `1.0.31`
  (file offset `0x8352310`) holds `("Default model: ", 15)` then `("\n", 1)`,
  and the whole-binary marker count is zero at every hop in this range.

All three classes print the auth-status preamble, the two blank lines, the
`Default model: ` prefix, `Available models:`, and `  * <id> (default)` /
`  - <id>` rows. Research 405 read the pooled bytes as an `(not in the list
below)` literal with an empty id; the argument decode above refines that: the
two-space run is string pooling, with piece 0 ending in a space and the marker
piece beginning with one. A header can still render with an empty id — the
formatter interpolates `current_model_id` unconditionally — but then no default
id was reported at all, not an id elided by the marker.

| Hops | Header pieces | Arguments | Marker count | Class |
| --- | --- | --- | --- | --- |
| `1.0.30..=1.0.45` | `Default model: `, `\n` | 1 (id) | 0 | listed default only |
| `1.0.46..=1.0.61` | `Default model: `, ` (not in the list below)\n` | 1 (id) | 1 | note as piece |
| `1.0.62..=1.0.70` | `Default model: `, ``, `\n\nAvailable models:`, `\n` | 3 (id, note-or-empty, empty) | 1 | note as argument |

## Mapping decision

The existing `ModelCatalog` vocabulary can represent every emitted state
without a public change: entries are the listed rows in provider order, and
`is_default` is `true` only for the single row whose id equals a listed default.
For the note form no entry is the default, which is the honest observation for
a default the source reports outside the list; Contract 020 already says
catalogue presence does not prove a default model, and nothing here invents a
listed row, promotes the embedded `default_models.json` default, or claims an
unlisted id as invokable. The frozen supplemental document still aligns by
exact id equality and still contributes nothing for an unlisted id.

Decode is gated by the qualified behaviour revision:

- `grok-build.catalogue.models-text-v1` (unchanged) accepts only the listed
  default: a valid header id and exactly one matching `* (default)` row. It
  still fails closed on the marker form, so the frozen `1.0.30` corpus and its
  regression keep their original meaning.
- `grok-build.catalogue.models-text-v2` accepts three consistent states:
  listed (valid id plus its single marked row), unlisted (valid id with the
  exact marker, no marked row, and no row equal to that id), and absent (an
  elided id with or without the marker, no marked row). Every other
  combination — a marked row without a matching listed id, two marked rows, a
  marker that names a listed row, a non-exact marker, two headers, or an
  invalid id before the marker — fails closed with the existing
  `catalogue_default_ambiguous` or `catalogue_invalid` diagnostic.

## Claim and prepared path

`grok-build.catalogue.executable-window-1` keeps `QualifiedOnly` and two
ordered segments:

| Segment | Behaviour revision | Status |
| --- | --- | --- |
| `1.0.30..=1.0.45` | `grok-build.catalogue.models-text-v1` | Deprecated |
| `1.0.46..=1.0.70` | `grok-build.catalogue.models-text-v2` | Maintained |

`1.0.30` stays admitted under its original revision; the label change to
Deprecated is the Contract 029 consequence of adding a newer revision, not a
membership change. `1.0.29` and `1.0.71` fail closed, and there is no inferred
interior, prerelease, or forward support. The private driver selects the decode
from the plan's assessed revision, so a stale plan cannot claim the new mapping
at an old point.

Per-hop prepared-path gates hold across the window: the exact
`--no-auto-update models` argv, the authentication preamble literals, the
`List available models and exit` grammar signal, the
`xai-grok-pager/src/models.rs` module path, the two byte-identical embedded
`default_models.json` copies (`grok-4.6`, ids `grok-4.6`/`grok-4.5`, efforts
`xhigh`/`high`/`medium`/`low`), and the row marker grammar are unchanged, and
each executable is frozen by digest. The operation remains an authenticated,
non-inference metadata process under `HarnessConfigurationPosture::Ambient`
with no retry, prompt, session, tool dispatch, or cancellation surface.

The prepared-facade proof prepares every claimed point `1.0.30..=1.0.70`
through `prepare_grok_build` and `prepare_catalogue`, feeding discovery each
hop's frozen self-reported build id. Through `1.0.46` that sweep also proves the
corpus and the discovery pin table agree, because a mismatch fails discovery;
the extracted ids reproduce the independently frozen ACP corpus at
`1.0.42..=1.0.46`. Above `1.0.46` discovery has no pin and validates only the
build-id shape and the observed version, and the vendor publishes no source
revision there (`1.0.70` self-reports `f50904eaee6c`). Pinning those ids would
narrow the ACP axis and needs its own review, so this record leaves discovery
untouched and the catalogue claim owns the version.

## Open next step

The ACP family still stops at `1.0.46` while the selected stable is `1.0.70`.
That is a separate, finite hop sweep with its own selected-surface evidence and
is not folded into this catalogue record. Queue owns dispatching it.

## Sources

- [Official npm packument for `@xai-official/grok`](https://registry.npmjs.org/@xai-official%2Fgrok)
  and [`@xai-official/grok-darwin-arm64`](https://registry.npmjs.org/@xai-official%2Fgrok-darwin-arm64);
  every compared tarball's integrity, SHA-1, SHA-256, size and publication time
  is frozen in the corpus.
- [Official `xai-org/grok-build` source](https://github.com/xai-org/grok-build)
  at `SOURCE_REV` `559751fdcec02d413e4c57c8832ab275e4f44980`:
  `crates/codegen/xai-grok-pager/src/models.rs` and
  `crates/codegen/xai-grok-shell/src/cli_models.rs`.
- Static format-piece and argument decode of the exact Darwin ARM64
  executables `1.0.31`, `1.0.46`, `1.0.62` and `1.0.70` at the file offsets and
  VMAs recorded above; no executable was run.
- Research 405 (the `1.0.46` stop and its frozen corpus), Research 316 and 306
  (the exact `1.0.30` catalogue evidence and live capsule), Contract 020 and
  Contract 029.
