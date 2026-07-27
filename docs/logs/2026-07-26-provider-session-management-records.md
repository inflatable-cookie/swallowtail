# Provider Session Management Records

Date: 2026-07-26

## Change

Card 046 realizes the provider-neutral record layer for Contract 038.

`swallowtail-core` now distinguishes:

- archive, restore, delete, and provider-native active close capabilities
- reversible lifecycle state from runtime attachment state
- history removal, provider data deletion, and provider hard deletion
- target-only and provider-defined descendant scope
- applied, already-in-state, already-absent, before-effect failure, and
  after-effect uncertainty
- exact interface bindings and their compatibility assessments

`swallowtail-runtime` now exposes one redacted
`ProviderSessionManagementBinding`. Construction requires the provider
session reference plus the exact driver descriptor, configured instance,
access evidence, binding origin, and optional working-resource scope. It
derives and validates driver, transport, facade, instance revision, host,
target, interface compatibility, and lifecycle capabilities.

## Boundary

- raw provider references remain insufficient on their own
- load and resume are not required
- provider-native close remains separate from archive, restore, and delete
- consumer thread state and destructive authorization remain downstream
- driver-owned remote-resource deletion is unchanged
- no runtime role, provider request, retry, fallback, registry, or provider
  adapter was added

## Validation

- `cargo test -p swallowtail-core -p swallowtail-runtime`: 112 passed
- focused core/runtime Clippy with warnings denied: passed
- `effigy check:rust`: all 23 workspace crates and all targets pass
- `effigy format:check`: passed
- `git diff --check`: passed

## Next

Card 047 adds the immutable management plan and scoped low-level runtime role.
