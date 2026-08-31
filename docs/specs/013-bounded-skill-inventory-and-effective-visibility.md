# 013 Bounded Skill Inventory And Effective Visibility

Status: active
Owner: Tom
Updated: 2026-08-31

## Purpose

Give consumers one bounded inventory of skills distributed through approved
global, project, and harness-specific locations. Keep that installed or
discoverable inventory distinct from Contract 058's exact selected-harness
effective roster.

## Scope

The first contract promotion must settle:

- explicit host-approved global roots
- project roots bound to the exact working resource
- adapter-declared distribution roots for one exact harness instance and
  version
- source-specific decoders rather than one assumed skill format
- positive bounds for roots, traversal depth, rows, bytes, and descriptor text
- symlink, traversal, root-escape, and unreadable-entry behavior
- immutable snapshot identity, source identity, freshness, and completeness
- duplicate names, conflicts, shadowing, and unknown precedence without silent
  effective selection
- descriptor metadata and provenance without arbitrary skill-body disclosure
- composition with Contract 058 through separate discovered and effective
  states

The initial surface is read-only. It does not install, update, enable, disable,
execute, prompt with, or mutate a skill or harness configuration. It does not
scan an ambient home or project tree, infer model visibility, or establish
provider tool authority.

## Decisions Needed

1. Name the portable inventory snapshot, source descriptor, discovered-skill
   descriptor, evidence state, and bounded failure vocabulary.
2. Assign ownership between core vocabulary, runtime traversal, adapter source
   declaration and decoding, and host root approval.
3. Fix library-owned limits and deterministic truncation or fail-closed
   behavior. Callers must not turn inventory into an unbounded scan.
4. Bind global roots to explicit host configuration, project roots to one
   normalized working resource, and harness roots to exact configured-instance
   and qualified-version evidence.
5. Preserve duplicate and conflicting rows with source provenance. Do not
   compute an effective winner unless the exact harness reports one under
   Contract 058.
6. Decide whether the first public descriptor exposes metadata only or a
   separately bounded body digest. Raw skill bodies remain out unless a later
   contract proves a need and disclosure policy.
7. Define snapshot freshness and replacement so stale roots or changed source
   declarations cannot compose silently.

## Acceptance Criteria

- global, project, and harness-distributed sources can be represented without
  flattening their authority or provenance
- every traversed root is explicit, positively bounded, and unable to escape
  through paths or symlinks
- inventory, unavailable, partial, stale, conflicting, and effective states
  remain distinct
- no discovered row claims selected-model visibility without Contract 058
- no mutation, execution, prompt, provider contact, credential, or ambient
  filesystem authority is added
- contract promotion names exact public ownership, bounds, counterexamples,
  validation, and a first provider-free proof tranche

## Promotion Targets

- dedicated Contract 062 for bounded skill inventory
- Contract 058 relationship text for the effective overlay
- architecture and privacy summaries if public ownership changes
- g05.010 implementation runway only after the contract is active
