# Handoff — g05 Card 056 Claude Agent ACP Model Options

## Objective

Implement card 056 exactly: publish negotiated model-options observation on
projected `claude-agent.acp` session open using existing runtime vocabulary.
Open one PR; do not merge it.

## Base And Authority

- planning base: `997002bb38c11d7f54ef71b8a5c6ce351b2d7d21`
- card: `docs/roadmaps/g05/batch-cards/056-claude-agent-acp-negotiated-model-options-observation.md`
- evidence: Research 279 and the ACP parity delivery gate
- precedent: Cline negotiated model parser and projected-open implementation

Verify the exact handoff blob and that the planning base is an ancestor before
editing. Work from current pushed `main`. Preserve unrelated changes.

## Worker Class

Day-to-day implementation worker. The semantics, bounded runtime type, handle
seam, projection identity, and Cline precedent already exist; remaining work is
route-local and mechanically testable. Do not promote this lane to a frontier
profile for provider availability.

## Required Shape

- Re-read card 056, Research 279, the ACP gate, Contract 061, and the Cline
  parser/projected-open tests before editing.
- Own only ACP model parsing, handle retention, projected-open contribution,
  targeted tests, and exact closeout docs.
- Preserve `open_session` behavior. Optional malformed evidence is absent on
  the preserved path; invalid evidence closes and fails only on projected open.
- Emit only the observation row from a distinct active source. No prepared,
  load/resume, catalogue, or control claim.
- Reuse `NegotiatedSessionModelOptions` and
  `InteractiveSessionHandle::negotiated_model_options`; no shared runtime/core
  or public control changes.
- Exercise all card counterexamples and run its exact validation after the
  complete batch. No live probe or provider contact.

## Parallel And Serial Edge

Card 055 runs in parallel and is the priority route, but this smaller PR lands
first. Shared adapter manifest, `lib.rs`, route matrix, guide, changelog,
milestone, card index, and log index are merge-order surfaces. Keep changes
minimal and report them so card 055 can restack cleanly.

## Closeout

Return exact base/head, PR, changed files, parser and lifecycle semantics,
provider-free falsification, validation counts, public API/god-file results,
residuals, and next move. Stop for exact-head review; do not merge.
