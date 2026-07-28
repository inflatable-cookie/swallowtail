# 089 Input And Callback Contract And Corpora

Status: planned
Owner: Tom
Created: 2026-07-28
Milestone: `../027-input-and-callback-feature-closure.md`
Depends on: card 088

## Objective

Promote the smallest missing input/callback rules and freeze the selected
route corpora.

## Scope

1. Promote only distinctions required by card 088's selected tranche.
2. Keep requested, planned, dispatched, provider-requested, consumer-resolved,
   and effective states separate.
3. Preserve exact media, size, count, schema, callback, and timeout bounds.
4. Keep provider-owned tools and search separate from consumer callbacks.
5. Freeze success, rejection, cancellation, deadline, drift, cleanup, and
   unverified-newer records without live access.
6. Make card 090's prepared evidence and conformance expectations exact.

## Acceptance Criteria

- [ ] contracts make the selected tranche deterministic
- [ ] fixtures require no live access
- [ ] authority and correlation remain exact
- [ ] unsupported and observed-only behavior remain explicit
- [ ] card 090 names exact routes and cells

## Auto-Continuation

No while planned. Card 088 must make the contract and corpus envelope exact.
