# Bounded Workspace Session Contract

Date: 2026-07-20
Lane: g01 roadmap 010, card 031

## Outcome

- Contract 013 makes interactive resource access, filesystem scope, approval
  posture, provider network, search, provider-request handling, deadline, and
  cleanup independent policy dimensions.
- Read-only interactive remains the default and retains its current exact
  semantics.
- The first bounded task profile permits one host-resolved read/write working
  resource, denies provider network and search, denies approval, and may only
  observe-and-stop for declared Codex approval or user-input extensions.
- Swallowtail owns portable policy and enforcement. Nucleus owns the selected
  host/resource, tasks, work items, mandates, evidence, review, receipts, and
  outcome projection.
- Local execution of a remote-authoritative resource remains blocked.

## Next Gate

Card 032 must realize typed policy and preflight records before the Codex
driver or host implementation changes.
