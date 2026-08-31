# 013 Bounded Skill Inventory And Effective Visibility

Status: promoted
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Roadmap: g05.010
Promoted to: Contract 062

## Purpose

Give consumers one bounded inventory of skills distributed through approved
global, project, and harness-specific locations. Keep that installed or
discoverable inventory distinct from Contract 058's exact selected-harness
effective roster.

## Scope

The contract promotion settled:

- explicit host-approved global roots
- project roots bound to the exact working resource
- adapter-declared distribution roots for one exact harness instance and
  qualified version
- source-specific decoders rather than one assumed skill format
- positive bounds for roots, traversal depth, rows, bytes, and descriptor text
- symlink, traversal, root-escape, and unreadable-entry behavior
- immutable snapshot identity, source identity, freshness, and completeness
- duplicate names, conflicts, shadowing, and unknown precedence without silent
  effective selection
- descriptor metadata and provenance without arbitrary skill-body disclosure
- composition with Contract 058 through separate discovered and effective
  states

The surface is read-only. It does not install, update, enable, disable,
execute, prompt with, or mutate a skill or harness configuration. It does not
scan an ambient home or project tree, infer model visibility, or establish
provider tool authority.

## Settled Decisions

1. Contract 062 owns a provider-neutral descriptor inventory. Contract 058
   continues to own the stronger selected-harness effective roster.
2. The execution host approves and traverses opaque roots. Runtime owns
   portable records and pure assembly. Adapters declare exact distribution
   sources and decoders. Consumers choose global and project sources.
3. Global, exact-working-resource project, and exact-version
   harness-distribution sources remain distinct. No ambient root exists.
4. Library-owned maxima bound roots, depth, visited entries, candidates, bytes,
   rows, failure notes, and every public text field. Limits produce explicit
   partial truth, never silent complete truncation.
5. Canonical containment, same-root symlink following, cycle detection, and
   deterministic entry ordering are mandatory host behavior.
6. Snapshots are immutable point-in-time observations. Changes to roots,
   resources, configured instances, qualified versions, decoders, or source
   observations make old evidence stale for composition.
7. Duplicate display names remain separate. Conflicts preserve provenance and
   do not select an effective winner.
8. Initial disclosure is metadata-only. Raw bodies and a public body digest
   remain out.
9. Inventory composes with Contract 058 only through an exact same-context
   adapter mapping. Partial or unavailable roster absence remains unknown.
10. Cancellation, deadline, host failure, or cleanup failure publishes no
    successful snapshot. All host work joins.

## Review Oracle

Invariant: bounded discovery preserves exact source and observation truth
without acquiring ambient filesystem or model-effective authority.

Smallest adversarial counterexamples:

1. Supply a raw home or project path without a host-approved root binding. The
   operation must reject it before traversal.
2. Follow a symlink outside its approved root. The entry must be omitted and
   the source marked partial without disclosing the path.
3. Reach a fixed row or byte maximum and report the retained prefix complete.
   The source must instead remain partial with bounded limit evidence.
4. Deduplicate two equal display names or select project over global by source
   order. Both rows must remain and precedence must stay unknown.
5. Treat a discovered row as model-visible because an incomplete effective
   roster lacks it. The overlay must report unknown.
6. Combine inventory and effective snapshots across resource, instance,
   version, decoder, or observation revisions. Composition must fail closed.

## Acceptance Criteria

- [x] global, project, and harness-distributed sources can be represented
      without flattening their authority or provenance
- [x] every traversed root is explicit, positively bounded, and unable to
      escape through paths or symlinks
- [x] inventory, unavailable, partial, stale, conflicting, and effective states
      remain distinct
- [x] no discovered row claims selected-model visibility without Contract 058
- [x] no mutation, execution, prompt, provider contact, credential, or ambient
      filesystem authority is added
- [x] Contract 062 names exact public ownership, bounds, counterexamples,
      validation, and a first provider-free proof tranche

## Promotion Targets

- [Contract 062 Bounded Skill Inventory](../../contracts/062-bounded-skill-inventory.md)
  is active
- Contract 058, contract indexes, roadmap state, and the sole Next Task agree
- implementation remains unplanned pending review
