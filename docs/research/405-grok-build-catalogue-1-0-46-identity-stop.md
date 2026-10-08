# Research 405: Grok Build Catalogue 1.0.46 Identity Stop

Status: exact identity and static-path evidence; no claim change. The current
official stable is `1.0.46`, but its Darwin ARM64 executable adds a catalogue
default header that the current `ModelCatalog` mapping does not represent.

Observed 2026-10-08 on `grok-build.executable`, `grok-build.catalogue` only.
The prior exact claim remains
`grok-build.catalogue.executable-1-0-30` /
`grok-build.catalogue.models-text-v1`, with `QualifiedOnly` posture and the
single exact `1.0.30` point.

## Official channel and artifacts

The official npm registry reports `@xai-official/grok` `latest` `1.0.46` and
`alpha` `1.0.50`. Published stable package versions `1.0.47`, `1.0.48`, and
`1.0.49` have no current dist-tag; they are not inferred from `latest`.
The selected-channel ledger is every published stable hop from `1.0.31`
through `1.0.46` after the exact `1.0.30` ceiling. `1.0.50` remains excluded
as alpha.

The [identity corpus](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.46-catalogue/identity.json)
freezes the wrapper and Darwin ARM64 platform package identities, publication
times, source commit identifiers where published, npm SRI and SHA-1 values,
and locally computed tarball SHA-256 values for all seventeen compared
versions. Every downloaded wrapper and platform tarball was verified against
the registry's published integrity and SHA-1. The selected runtime target is
Darwin ARM64 because of the worker environment; the installed host executable
was not invoked or rechecked.

The [complete shipped-file inventory](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.46-catalogue/dist-inventory.json)
shows five wrapper files and four Darwin ARM64 package files at every point.
For each hop, only wrapper `package.json` changes; its launcher and install
scripts are byte-identical. The platform package changes `package.json` and
`bin/grok.br`; its README and notices are identical. There are no added or
removed paths. The inventory records every per-file size and SHA-256, and the
exact changed/identical path sets.

## Catalogue-path evidence

The [runtime surface ledger](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.46-catalogue/runtime-surface.json)
records each decompressed executable's digest and size, selected static
catalogue and authentication signals, and both embedded `default_models.json`
documents. The two embedded documents stay byte-identical at every compared
point, with default `grok-4.6` and ids `grok-4.6`, `grok-4.5`.

The catalogue grammar literals and authentication preamble remain present.
However, the `1.0.46` executable contains one occurrence of
`Default model:  (not in the list below)`; it occurs in none of `1.0.30` through
`1.0.45`. In that output form, the header is followed by `Available models:`.
The current parser strips `Default model: ` and requires a valid model id,
then requires that default to match a listed row. It therefore fails closed
on this form. A unit regression pins that rejection. No live catalogue was
run, so the corpus does not claim that a particular account reaches this
branch.

The Darwin executable changes at every hop. Its exact digest and selected
static signals are frozen, but those observations do not characterize other
internal binary changes or establish general compatibility. The new selected
header is sufficient to stop the claim update. No provider command, prompt,
credential, install, host mutation, or downloaded artifact execution occurred.

## Decision and next adaptation

Keep the catalogue claim at exact `1.0.30`; reject `1.0.31` through `1.0.46`
under the existing `QualifiedOnly` claim. Preserve the ACP claim and its
independent `1.0.46` ceiling. Do not remove default metadata or promote an
unlisted model by guesswork. The next adaptation must define and prove a
same-contract mapping for the default-not-in-list form, including its listing
and failure semantics. If the public `ModelCatalog` result cannot honestly
represent that provider state without consumer-visible narrowing or a new
operation, obtain an operator ruling before changing the contract.

## Sources

- [Official npm metadata for `@xai-official/grok`](https://registry.npmjs.org/@xai-official%2Fgrok)
- [Official npm metadata for `@xai-official/grok-darwin-arm64`](https://registry.npmjs.org/@xai-official%2Fgrok-darwin-arm64)
- Official wrapper and Darwin ARM64 tarballs for every compared version; the
  registry URLs, integrity, SHA-1, SHA-256, and publication times are frozen
  in `identity.json`.
- Current parser and fail-closed regression in `src/catalogue.rs`.
