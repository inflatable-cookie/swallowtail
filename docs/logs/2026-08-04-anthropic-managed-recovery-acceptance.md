# 2026-08-04 Anthropic Managed Recovery Acceptance

Roadmap: `../roadmaps/g03/033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md`
Card: `../roadmaps/g03/batch-cards/086-anthropic-recovery-prepared-and-package-acceptance.md`

## Changed

- documented ordinary delete-on-close and opt-in cross-process recovery as
  separate prepared Managed Agents profiles
- documented opaque checkpoint and cleanup-binding persistence, exact prepared
  route restoration, bounded observation, callback non-authority, and explicit
  inactive cleanup
- exposed all four preparation paths in the compile-tested public example:
  ordinary run, recoverable run, run reconciliation, and recovered cleanup
- promoted Anthropic recovery from pending to realized in architecture and
  Contract 048
- updated the route matrix with both recovery roles and exact prepared paths
- retained the feature CSV's combined capability shape: its existing
  provider-managed recovery and owned cleanup values were already `Yes`
- split recovery-specific prepared types, parsers, fixtures, and interruption
  proof out of five files which had crossed the structural warning threshold

## Validation

- `effigy validate:focused swallowtail-adapter-anthropic swallowtail-runtime`
  — 188 tests passed; focused package check passed
- `effigy package:verify-affected swallowtail-adapter-anthropic swallowtail-runtime`
  — both independently extracted packages compiled
- `effigy qa:docs` — passed
- `cargo fmt --all -- --check`
- `git diff --check`

`effigy doctor` still fails on known repository-wide structural debt: 206
findings, comprising 187 warnings and 19 errors. None of the files split by
this card remains in that report.

No authenticated provider work, paid inference, consumer edit, live resource
creation, or live deletion ran.

## Next Move

Hold at the g03 evidence gate until a consumer-reproduced defect, material
non-deferred provider/interface drift, or explicit operator selection supplies
the next roadmap input.
