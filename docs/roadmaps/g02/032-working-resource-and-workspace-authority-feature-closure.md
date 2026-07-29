# 032 Working Resource And Workspace Authority Feature Closure

Status: completed
Owner: Tom
Created: 2026-07-28
Depends on: g02.031
Vision tags: host authority, provider breadth, workspace safety
Contract refs: 009-010, 013, 015, 017, 023, 029, 037, 039, 041
Planning state: cards 107-110 complete

## Problem

The current matrix retains 31 `No` cells across two related but independent
features:

- 12 working-resource gaps
- 19 bounded-workspace-text-write gaps

These columns mix location selection, host-mediated resource access,
provider-visible filesystem access, callback-only writes, ambient harness
authority, and enforced containment. A route can use an exact working resource
without proving bounded writes. Most hosted and attached inference routes may
have no workspace operation at all.

## Goals

- [x] Revalidate every starting `No` against the exact selected route and
      current maintained evidence.
- [x] Detect false negatives and operation-shape non-applicability.
- [x] Keep working-directory selection, resource leasing, callback I/O,
      provider tools, ambient access, and containment separate.
- [x] Select only contract-ready, consumer-useful conversions.
- [x] Freeze exact version, resource, access, topology, failure, and cleanup
      corpora before production work.
- [x] Re-audit all 31 starting cells and retain honest absence.

## Non-Goals

- treating a prompt, attachment, provider file, or remote conversation as a
  working resource
- treating working-directory selection as filesystem containment
- treating `ReadWrite` resource access as permission to answer approvals or
  execute provider tools
- granting direct hosted inference ambient local filesystem access
- borrowing write capability from a sibling route or consumer integration
- adding implicit resource, endpoint, model, credential, topology, version,
  sandbox, or support-authority fallback
- consumer edits, live authentication, publication, or release mutation

## Execution Plan

### Batch 32.1 — Exact Currentness Audit

- [x] Execute card 107.
- [x] Classify all 31 starting cells exactly once.
- [x] Rank conversion candidates by consumer value and boundary coverage.

### Batch 32.2 — Contract And Corpus Gate

- [x] Execute card 108 only after card 107 selects exact routes.
- [x] Promote only evidence-required shared distinctions.
- [x] Freeze deterministic access and failure corpora.

### Batch 32.3 — Representative Implementation

- [x] Execute card 109 only for contract-ready routes.
- [x] Preserve exact resource and write authority.

### Batch 32.4 — Matrix Closeout

- [x] Execute card 110.
- [x] Prove package truth, re-audit counts, and select the next matrix family.

## Acceptance Criteria

- [x] all 31 starting cells have current exact-route dispositions
- [x] every changed cell maps to a public prepared path
- [x] working-resource support does not imply write authority
- [x] bounded writes do not imply general filesystem or shell containment
- [x] resource scope, host identity, access, and cleanup remain exact
- [x] no provider effect occurs during audit or default validation

## Decision Gates

- Ask the operator if equally valid route choices would set product policy.
- Stop when maintained evidence cannot distinguish resource selection from
  write or containment authority.
- Promote a narrow contract delta before implementation when existing host
  service and access-policy contracts do not settle an exact route.
- Keep an honest `No` when a qualifying operation exists but the selected
  route exposes no exact resource or bounded-write mechanism.

## Next Planning Checkpoint

Roadmap 033 and card 111 continue with runtime ownership and connection
rollover. Stay in g02.
