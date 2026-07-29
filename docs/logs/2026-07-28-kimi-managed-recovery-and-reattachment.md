# Kimi Managed Recovery And Reattachment

Date: 2026-07-28
Status: completed

## Changed

- made managed-recovery acceptance explicit for Kimi headless and
  local-server structured preparation
- advertised exact managed-recovery capability on qualified plans
- validated ordered retry evidence without exposing provider error text
- added maximum-one local-server active-turn cursor reattachment
- moved native cancellation control to the replacement socket
- retained one prompt, session, turn, access lease, model, and deadline
- recorded installed recovery as route-dependent and local-server recovery
  plus reattachment as supported

## Evidence

- 4 headless structured tests passed
- 6 local-server structured tests passed
- 3 Kimi corpus tests passed
- route-matrix and formatting checks passed
- the first repository QA pass found only shared-fixture dead-code lint on
  scenarios used by a different test binary; the fixture enum now carries its
  existing cross-binary allowance
- no executable, credential, account, provider request, container, or live
  model server was used

## Current State

Card 105 is complete. Card 106 remains in bounds and is ready for the 32
non-applicable matrix dispositions, package proof, and next-family selection.
