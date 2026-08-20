# 018 Config Field Descriptors

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../006-addable-catalog-admission-and-config-fields.md`
Depends on: card 017

## Goal

Attach per-instance config-field descriptors as opaque host-owned
references.

## Scope

1. Binary path, endpoint, and environment described as host-owned field
   references.
2. Values stay host-private.
3. Public records carry no paths, URLs, or env bodies.

## Out Of Scope

- resolving those references
- sign-in or credential leases
- overlay or accent color

## Acceptance Criteria

- [x] admitted instances can name config-field descriptors
- [x] portable records expose no path, URL, or env body
- [x] host-local tests keep values behind opaque refs

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`
- `git diff --check`

## Auto-Continuation

No. g04.006 closes. g04.007 stays planned until catalog and admission exist.

## Stop Conditions

- Stop if config values leak into diagnostics or 047.

## Evidence

Admission attaches `ConfigFieldRef` values that match advertised
`ConfigFieldDescriptor` ids. JSON-file and debug records carry references,
not paths, URLs, or env bodies.
