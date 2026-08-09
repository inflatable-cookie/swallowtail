# 192 Idiom Registry Client And Package Acceptance

Status: ready
Owner: Tom
Updated: 2026-08-09

## Goal

Realize the registry-client merge surface without transport authority, then
close package acceptance for the lane under Contracts 036 and 055.

## Scope

- portable registry records: package references and namespaces
- pull and push merge semantics following the confidence merge outcomes
- bounded typed responses
- Contract 036 architecture/package review for `swallowtail-idioms` entry
  into the workspace release set
- guide, example, route matrix, and architecture notes; release-baseline
  handling

## Out Of Scope

- HTTP client, transport, or registry service
- learned backend and the Soundcheck correction-loop proxy (later
  checkpoint)
- version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- registry merge fixtures pass without transport authority
- Contract 036 package review passes for the new package
- guide, example, matrix, and architecture stay mutually honest
- focused and extracted-package validation pass

## Validation

- `effigy validate:focused swallowtail-idioms`
- `effigy package:verify-affected swallowtail-idioms`
