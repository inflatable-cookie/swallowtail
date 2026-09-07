# 112 Integration Test Binary Consolidation

Status: complete; PR 248 merged at `169cc199`
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../034-release-lane-simplification.md`
Depends on: the g05.034 measured causes table

## Goal

Cut link time, which dominates the test suite: 370 integration test files each build a separate binary.

## Scope

1. Per crate, move `tests/*.rs` into one or a few `tests/<group>/main.rs` binaries with `mod` files, preserving every test name and module path in the output. Start with the six largest: claude-agent (32), kimi (30), codex (28), testkit (24), openai (20), gemini (20). Then opencode, protocol-acp, pi, host-local, anthropic, alibaba-model-studio.
2. Keep process-spawning tests in their own binary per crate so the `ci-process` nextest profile still selects them.
3. Record nextest binary count and wall clock before and after per crate, locally under the card 093 load method.
4. No `src` change; no test deleted or renamed.

## Acceptance Criteria

- [x] binary count per crate reduced to a handful; test count identical
- [x] `ci` and `ci-process` profiles still select the same tests
- [x] before/after timings recorded

## Validation

- `effigy validate:focused <crate>` per touched crate (up to four per run)
- `cargo nextest list --workspace` count before and after
- `git diff --check`

## Result

Implemented the test-target consolidation for the five crates that still had
large auto-discovered binary sets. The other seven named crates were already
in the target shape (two to five explicit test binaries) and remain unchanged.
The grouped targets use path-backed modules named after the original test
files. The six process-spawning targets retain their exact names and remain
outside `ci` and inside `ci-process`.

Loaded local measurement method: `sysctl -n hw.ncpu` reported 18; two
`yes >/dev/null` burners per core (36 total) ran during each
`cargo nextest list -p <package> --list-type binaries-only` measurement.
Wall-clock values are seconds from `/usr/bin/time -p`.

| Package | Test binaries before | after | Tests before | after | List wall clock before | after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `swallowtail-adapter-claude-agent` | 30 | 4 | 379 | 379 | 0.50s | 0.46s |
| `swallowtail-adapter-kimi` | 4 | 4 | 210 | 210 | 0.31s | 0.29s |
| `swallowtail-adapter-codex` | 5 | 5 | 238 | 238 | 0.27s | 0.27s |
| `swallowtail-testkit` | 2 | 2 | 97 | 97 | 0.28s | 0.30s |
| `swallowtail-adapter-openai` | 3 | 3 | 96 | 96 | 0.30s | 0.30s |
| `swallowtail-adapter-gemini` | 4 | 4 | 87 | 87 | 0.29s | 0.31s |
| `swallowtail-adapter-opencode` | 2 | 2 | 125 | 125 | 0.30s | 0.37s |
| `swallowtail-protocol-acp` | 2 | 2 | 93 | 93 | 0.29s | 0.24s |
| `swallowtail-adapter-pi` | 12 | 2 | 117 | 117 | 0.32s | 0.26s |
| `swallowtail-host-local` | 12 | 3 | 131 | 131 | 0.29s | 0.44s |
| `swallowtail-adapter-anthropic` | 12 | 1 | 107 | 107 | 0.31s | 0.29s |
| `swallowtail-adapter-alibaba-model-studio` | 12 | 1 | 45 | 45 | 0.30s | 0.27s |

Workspace evidence: `cargo nextest list --workspace --list-type
binaries-only` fell from 220 to 153 test targets; the full listed test count
stayed at 3,155. The `ci` profile selected 2,929 tests and `ci-process`
selected 226; their disjoint union is 3,155. The retained process targets are
`claude_agent_sdk_sidecar_asset`, `claude_agent_sdk_driver`, `sidecar_driver`,
`claude_code_structured_run`, `local_process`, and `watcher_service`.

Validation:

- `effigy validate:focused swallowtail-adapter-claude-agent`: 379 passed.
- `effigy validate:focused swallowtail-adapter-pi swallowtail-host-local
  swallowtail-adapter-anthropic swallowtail-adapter-alibaba-model-studio`:
  400 passed.
- `effigy package:verify-affected` passed for the same one-package and
  four-package groups.
- `cargo fmt --all -- --check` and `git diff --check` passed.

Post-Card 113 review reconciliation:

- Rebasing onto current `origin/main` (`944a4285`, which contains Card 113 at
  `d7ed04ffc30154c51af509eef00e730d384ec335`) required no change to the
  Card 113 `.config/nextest.toml` filters or its three-package macOS
  selection. The six retained process binaries still belong exactly to
  `swallowtail-adapter-claude-agent`, `swallowtail-adapter-pi`, and
  `swallowtail-host-local`; no workflow or config file is changed here.
- Removed the three dead auto-discovery roots
  `anthropic/tests/prepared_facade.rs`,
  `claude-agent/tests/consumer_route_projection.rs`, and
  `claude-agent/tests/support/discovery.rs`.
- The pre-consolidation `connection_lifecycle.rs` and `installed_probe.rs`
  explicitly used the small process-only `support/discovery.rs` fixture,
  not the broader `SharedAgent` fixture. That switch was accidental during
  grouping. Its implementation is preserved verbatim at
  `claude-agent/tests/discovery_support.rs`, included once by the compiled
  integration target, and both tests use it again. No discovery credential,
  resource, or timing semantics changed.
- A machine-readable nextest inventory counted the full `(package, test
  leaf)` multiset at 3,155 entries. The logical package/test-leaf multiset
  is unchanged from current `origin/main`; the grouped binary namespaces do
  not add or remove a test. The current profiles are `ci=2,929` and
  `ci-process=226`, with zero leaf overlap and a 3,155-entry union.
- The non-empty `ci-process` binary set is exactly six entries, each selected
  once: `claude_agent_sdk_driver`, `claude_agent_sdk_sidecar_asset`,
  `claude_code_structured_run`, `sidecar_driver`, `local_process`, and
  `watcher_service`. None is selected by `ci`.
- No autotests reachability guard was added; that belongs to Card 113/release
  mechanics.

The diff is limited to test-target manifests, test modules, and this result;
no production source, workflow, baseline, release, or script surface changed.

## Review Oracle

See the g05.034 manifest row.

## Auto-Continuation

No. Stop for exact-head review.
