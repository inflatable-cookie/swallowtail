# 2026-08-05 Protocol Rustdoc Closure

## Result

Both public protocol packages now document and enforce their complete exposed
surface.

The ACP documentation distinguishes transport framing from semantic activity
limits, string and integer request-ID correlation, exact session-list
capabilities, replacement and delta semantics, provider-defined unknowns,
already-terminal tool calls, partial tool refinements, and tool titles from
payload content.

The compatible-chat documentation covers codec limits, request extension
collision rules, bounded unknown-field retention, provider error-text safety,
streamed payload records, and incremental SSE completion.

Both crate roots now deny missing public documentation. The workspace warning
count falls from 5,519 to 5,182 without suppression. Four of 27 packages are
closed under the release documentation gate.

## Validation

- 103 focused protocol tests passed
- warnings-denied clippy passed for both protocol packages
- all-feature denied-missing-doc Rustdoc passed for both protocol packages
- workspace all-feature Rustdoc completed with 5,182 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with `swallowtail-testkit`, then review the core/runtime API
families before the adapter crates.
