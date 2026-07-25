# Package And Compatibility Gates

Date: 2026-07-24

## Result

g02 card 003 realizes Contract 036 across the full 23-package family.

- workspace resolver 3 and common package metadata are explicit
- all packages target crates.io at coordinated version `0.1.0`
- 22 packages declare Rust `1.93`; Bedrock declares `1.94.1`
- all 46 internal normal dependency edges retain local paths and add ordinary
  compatible `^0.1.0` registry requirements
- release baselines freeze the internal topology, public declarations, and
  required Rust toolchains
- Effigy now routes deterministic metadata, topology, public-declaration,
  documentation, MSRV, package-content, and extracted-family checks

The public-declaration baseline is a deterministic change detector. It does not
replace maintainer classification of Rust API or guaranteed-behavior changes.

## Package Evidence

The local package gate materializes the exact tracked-plus-untracked repository
state into a temporary clean Git snapshot. It packages all 23 crates in the
accepted three-stage order without `--allow-dirty`, registry upload, or
credentials.

Each archive is checked for forbidden paths. Each generated manifest is checked
for retained path or Git dependency sources. Extracted content is scanned for
repository-local paths, private keys, and token-shaped secrets.

The complete extracted package family then passes:

- locked workspace check across all targets
- locked workspace test compilation

Temporary archive checksums prove deterministic gate output within the run but
are not retained as candidate evidence. Card 004 owns immutable checksums from
one exact clean release candidate.

## Compatibility Evidence

- metadata, package identity, and 46-edge topology gate — passed
- public-declaration baseline — passed
- package documentation for all 23 crates — passed
- 22-package all-target check on Rust `1.93.0` — passed
- Bedrock all-target check on Rust `1.94.1` — passed
- full all-target check on current stable Rust `1.97.1` — passed
- clean local package assembly and extracted-family verification — passed
- `effigy qa` — passed with 658 tests inventoried: 654 passed and four
  separately gated probes remained ignored
- `git diff --check` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings: 12
  warnings and seven errors

The toolchains were installed side by side. The user's default Rust toolchain
was not changed.

The first full QA attempt saw one OpenCode loopback deadline test fail during
session transport setup. The exact test and its complete 11-test target passed
on immediate focused rerun, then the complete repository QA passed unchanged.
No package or runtime fix was required.

## Authority

No package was uploaded. No registry account, credential, or owner was read or
changed. No tag, push, GitHub release, workflow, or consumer repository changed.

## Lane State

Card 003 is complete. Card 004 is ready to freeze one non-published candidate
and prepare exact Nucleus and Soundcheck upgrade and rollback handoffs. Actual
release mutation remains an explicit operator decision.
