# Research 316: Grok Build Catalogue 1.0.30 Identity

Status: complete; identity and catalogue-path evidence only. The production
claim change lands in the g05.067 claim commit after this record.

Observed 2026-09-14 on `grok-build.executable`, catalogue route only:

- Host `grok --no-auto-update --version`: `grok 1.0.30 (04b7ffed98c6)
  [stable]`, a symlink to `~/.grok/downloads/grok-1.0.30-macos-aarch64`,
  SHA-256
  `d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb`,
  141869568 bytes. Reproduces Research 314 exactly: brotli-decompressing the
  official `@xai-official/grok-darwin-arm64@1.0.30` package's `bin/grok.br`
  yields the same executable. The host was read, never installed, updated,
  or replaced.
- Official npm `@xai-official/grok`: `latest` `1.0.30`, `alpha` `1.0.31`.
  `1.0.31` is a channel candidate, not the stable target; the first
  unpublished stable after `latest` is `1.0.32`.
- The previous exact catalogue point is `1.0.25`, which the shipped
  `QualifiedOnly` claim
  `grok-build.catalogue.executable-1-0-25` /
  `grok-build.catalogue.models-text-v1` admits.

## Method

The `darwin-arm64` platform package tarballs for the previous exact point and
every stable through official `latest` (`1.0.25..=1.0.30`) were retrieved from
the official npm registry. Each published `sha256` was verified against the
Research 314 corpus before use: all six tarballs reproduce their frozen
digests. `package/bin/grok.br` was brotli-decompressed into `/tmp` and hashed;
the decompressed executables were never executed, and each was discarded after
probing. For every hop the probe carved:

- the shipped catalogue format literals (`Default model: `,
  `Available models:`, `  - `, `  * `, ` (default)\n`), the authentication
  preamble literals (`You are logged in with `, `You are using
  XAI_API_KEY.`, `' is using its own API key.`, `You are authenticated via
  deployment key.`, `You are not authenticated.`), the root flag
  `--no-auto-update`, the root help description `List available models and
  exit`, and the `xai-grok-pager/src/models.rs` module path, with exact
  occurrence counts;
- the embedded default-model document (`default_models.json`), identified by
  its `{\n  "default"` opening, balanced-brace matched and JSON parsed, with
  its raw SHA-256 and two byte-identical copies per executable.

The installed exact `1.0.30` executable was additionally exercised through
provider-free `--help` argv-grammar exits only: `grok --no-auto-update models
--help` exits zero, and `grok models --no-auto-update --help` fails with
`unexpected argument '--no-auto-update' found`.

No prompt, inference, ACP initialize, session, login, credential use, install,
host update, or catalogue execution occurred during identity. The single
prompt-free authenticated catalogue observation belongs to the claim stage and
is recorded separately.

## Official hop identities

Executable, brotli payload, and model-document values are SHA-256; the full
digests, published timestamps, and per-hop literal counts are frozen in the
[1.0.30 catalogue identity corpus](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.30-catalogue/identity.json).

| Version | Published | git head | Executable | Brotli | Model document |
| --- | --- | --- | --- | --- | --- |
| `1.0.25` | 2026-09-09 | `f7e67d6988e2` | `9ef4a40ad60c` | `7d13cf31755d` | `9d6924ec760a` |
| `1.0.26` | 2026-09-10 | `fadad3468632` | `081f1d861e99` | `879d6b1bc676` | `9d6924ec760a` |
| `1.0.27` | 2026-09-10 | `a538938e5720` | `6e85277b0432` | `9077bfa8ff15` | `9d6924ec760a` |
| `1.0.28` | 2026-09-10 | `cae16d2533b6` | `bfaa983f8dd4` | `ba5ad4781216` | `9d6924ec760a` |
| `1.0.29` | 2026-09-11 | `4c83f16c3e10` | `2b6b44a2e7e7` | `a6729424609e` | `9d6924ec760a` |
| `1.0.30` | 2026-09-11 | `04b7ffed98c6` | `d53b6e543e48` | `13f3c6cd5145` | `9d6924ec760a` |

Cross-checks: the `1.0.25` executable `9ef4a40ad60c…` reproduces Research 305
and the previous installed host; the `1.0.30` executable equals the current
installed host and the official `darwin-arm64` payload; the model-document
digest `9d6924ec760a…` reproduces Research 314's frozen value, which is
unchanged from `1.0.11` through `1.0.30`.

## Catalogue-specific classification

The catalogue path depends on four shipped inputs: the exact argv, the
authentication preamble, the bullet output grammar, and the supplemental
embedded default-model document. All four are stable across every hop from the
previous exact point through official stable:

- **argv and subcommand grammar.** `--no-auto-update` remains a root-position
  flag and the `models` subcommand keeps only `--debug`, `--debug-file`,
  `-h`/`--help`, and `--leader-socket`; it accepts no `PROMPT` argument. The
  installed `1.0.30` help surface is identical to the Research 306 `1.0.25`
  record, including the rejected post-subcommand placement.
- **Output grammar.** Every format and preamble literal occurs in every hop
  with byte-identical counts (`Default model: ` ×3, `Available models:` ×1,
  `  - ` ×158, `  * ` ×3, ` (default)\n` ×5, each preamble literal ×1,
  `--no-auto-update` ×4, `List available models and exit` ×1,
  `xai-grok-pager/src/models.rs` ×1). The combined literal-count and
  model-document digest is frozen as the surface digest
  `aa3ad436d5a6c8eb893d1091c02f0cadbdabe9338bce38a4205d9a04768d3e02`.
  The shipped `xai-grok-pager/src/models.rs` module path persists, so the
  Research 306 format-piece recovery remains the same source.
- **Supplemental embedded document.** The `default_models.json` document is
  byte-identical at every hop: the balanced-brace JSON document (2323 bytes)
  hashes to `9d6924ec760a…` with two byte-identical copies per executable,
  and the embedded slice including its trailing newline (2324 bytes) hashes
  to `c7b26d2f4a4f…`, which is the in-tree frozen
  `crates/swallowtail-adapter-grok/src/catalogue/default_models_1_0_30.json`.
  The default stays `grok-4.6`, the ids stay `grok-4.6` and `grok-4.5`, the
  default effort stays `high`, and the efforts stay `xhigh`, `high`,
  `medium`, `low`.
- **Membership source.** The live `models` listing remains authoritative for
  membership, order, and default; the frozen document supplements matching
  exact ids only, unknown valid ids pass through with empty metadata, and
  absent source fields stay absent.

The catalogue-specific delta across `1.0.25..=1.0.30` is therefore
zero for every changed catalogue input, while the executable bytes themselves
differ at each hop. The ACP route's mapped-window extension in Research 314 is
not used as catalogue evidence: only the catalogue-specific carve and the live
exact-`1.0.30` observation decide the catalogue.

## Contract 029 decision

`compatible extension of the exact QualifiedOnly catalogue point` on
`grok-build.executable`, catalogue route only. Keep the posture
`QualifiedOnly` and the behavior revision
`grok-build.catalogue.models-text-v1`; re-identify the exact one-point claim
as `grok-build.catalogue.executable-1-0-30` and accept exact `1.0.30`, since
the claim identifier names the exact accepted point. Every older and newer
point, including `1.0.31`, stays rejected by the qualified-only posture.

## Accepted observation

The provider-free parser, driver, fake-process, local-host, redaction, and
lifecycle gates passed before the live boundary. Exactly one prompt-free
authenticated `--no-auto-update models` observation then ran the installed
exact `1.0.30` executable with the delegated credential in an operation-private
home that reproduces only the installed channel file and the delegated
credential. It exited zero with 113 stdout bytes and zero stderr and returned
ordered `grok-4.6` default then `grok-4.5`; no prompt, model session,
inference, or tool dispatch occurred, stdin was closed, and cleanup joined.
The stdout byte count and SHA-256 reproduce the accepted Research 306
`1.0.25` capsule exactly, and no retry was performed. The redacted capsule is
frozen at
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.30-catalogue/live-capsule.json`
(schema `swallowtail.grok.catalogue.live-capsule.v4`, task `g05.067`). The live
listing owns membership, order, and default.

The Grok ACP execution window, the registered-tool courier on the accepted
live `1.0.4`/`1.0.5` capsules, the frozen `1.0.25` catalogue evidence, and
every historical corpus stay independently bounded.

## Sources

- host `grok 1.0.30 (04b7ffed98c6) [stable]` (`--no-auto-update --version`)
- npm `@xai-official/grok` and `@xai-official/grok-darwin-arm64` packuments,
  official tarballs and published integrity
- installed `1.0.30` `--help` argv-grammar exits only
- frozen
  `crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.30-catalogue/{identity.json,live-capsule.json}`
- Research 305, 306, and 314; g05.052 and g05.053
