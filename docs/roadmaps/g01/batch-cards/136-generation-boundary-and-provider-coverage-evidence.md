# 136 Generation Boundary And Provider Coverage Evidence

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../046-generation-boundary-and-provider-coverage-checkpoint.md`

## Objective

Reassess Swallowtail's realized coverage after remote ACP and decide both the
next high-information proof and the correct generation container.

## Scope

- inventory production descriptors, shared transports, conformance profiles,
  operation shapes, authority boundaries, and maintained version ranges
- identify remaining lifecycle, credential, catalogue, protocol, topology, and
  compatibility-maintenance gaps
- revalidate leading candidates against current official provider or
  maintained-project evidence
- compare one materially new boundary with maintenance depth for existing
  supported ranges
- account for g01's 46-roadmap size before compiling another lane
- recommend continuing g01 or deliberately starting g02
- compile only the selected planning lane; no implementation

## Acceptance Criteria

- [x] realized coverage and remaining gaps are explicit
- [x] candidate evidence is current and authoritative
- [x] guaranteed support and unverified-newer execution remain separate
- [x] the recommendation adds architectural or maintenance information
- [x] the generation choice follows the documented 30-50 range
- [x] unresolved product policy is returned to the operator
- [x] one sole next task remains

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- provider, authentication, support floor, topology, or generation choice
  would establish unsettled product policy
- current evidence does not distinguish the leading candidates

## Auto-Continuation

No. The checkpoint may recommend a lane, but must stop for any unsettled
provider or generation policy.

## Outcome

Research 030 retains the 21 production descriptors and thirteen common
profiles, then records three material evidence changes:

- ACP v1 has more stabilized optional lifecycle features while ACP v2 remains
  Draft
- Rust SDK `2.0.0` is a compile-time SDK API major and explicitly leaves
  stable ACP v1 wire unchanged
- the stabilized ACP registry distributes local agents but supplies no
  provider-hosted remote endpoint

A provider-specific remote ACP adapter and interactive auth lifecycle therefore
remain evidence-gated. Grok Build is selected instead. It is a first-party xAI
harness over ACP stdio with exact version observation, update suppression,
provider permissions separate from sandboxing, and 111 published stable
`0.2.x` points. That release stream exercises Contract 029 without inventing a
continuous range.

Existing contracts cover the candidate restrictive read-only,
pre-authenticated, ambient-host subset. Roadmap 047 and cards 137-140 compile
the artifact corpus, discovery/dispatch, production driver, and cross-topology
closeout. Only card 137 is ready; later cards remain planned until it proves an
exact safe lifecycle.

g01 stays active at 47 roadmaps. No g02 is created or implied.

## Evidence

- Research 030
- roadmap 047 and cards 137-140
- live ACP registry schema `1.0.0`, 38 entries
- Grok registry entry and npm `0.2.0..=0.2.111` candidate envelope with
  unpublished `0.2.48`
- official ACP and xAI documentation
- exact Rust SDK `2.0.0` package changelog
- `effigy qa:docs` and `effigy qa:northstar` pass
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings:
  12 warnings and 7 errors
