# 025 Bounded Skill Inventory Contract Promotion

Status: complete; Contract 062 active; implementation unplanned
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../010-bounded-skill-inventory-and-effective-visibility.md`
Depends on: archived Spec 013; Contracts 058 and 062; operator four-track reframe

## Goal

Promote Spec 013 into dedicated Contract 062. Close the public boundary between
bounded installed/discoverable inventory and Contract 058's exact
selected-harness effective roster. Implement no Rust.

## Scope

1. Name exact portable inventory, source, descriptor, evidence, freshness,
   completeness, conflict, and bounded failure semantics.
2. Assign host root approval, runtime traversal, adapter source declaration and
   decoding, and consumer composition ownership.
3. Bind global roots explicitly, project roots to the exact working resource,
   and harness distribution roots to exact instance and qualified version.
4. Fix library-owned maxima for roots, depth, rows, bytes, and text.
5. Define symlink, traversal, root escape, unreadable entry, partial result,
   cancellation, and deadline behavior.
6. Preserve duplicates and conflicts with provenance. Do not select an
   effective winner from inventory.
7. Keep initial disclosure descriptor-only and read-only.
8. Update Contract 058, architecture, indexes, and g05.010 only as needed to
   agree with the promoted boundary.

## Out Of Scope

- Rust implementation, filesystem traversal, provider contact, or live probes
- ambient home/project scans or caller-selected unbounded roots
- skill bodies, prompts, execution, install, update, enable, disable, or config
  mutation
- model-effective visibility without exact Contract 058 evidence
- watcher, feature-façade, version-currentness, papercut, or release work

## Acceptance Criteria

- [x] Contract 062 is active and Spec 013 is archived
- [x] ownership, bounds, snapshot identity, freshness, traversal, conflicts,
  privacy, and fail-closed behavior are exact enough to test
- [x] global, project, and harness distribution sources remain distinguishable
- [x] discovered and selected-harness effective truth cannot be silently composed
- [x] a provider-free first implementation tranche can be compiled without a new
  product-policy decision

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Stop after contract promotion for an implementation-readiness review.

## Result

Contract 062 now owns explicit root authority, deterministic bounded
traversal, descriptor-only decoding, completeness, freshness, duplicates,
conflicts, cancellation, joined cleanup, and fail-closed composition with
Contract 058. Spec 013 is archived. No Rust or production claim changed.
