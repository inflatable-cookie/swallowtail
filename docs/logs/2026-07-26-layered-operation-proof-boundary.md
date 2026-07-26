# Layered Operation Proof Boundary

Date: 2026-07-26

## Decision

Do not use a native consumer application as the default repetition harness for
Swallowtail lifecycle behavior.

Evidence now has three layers:

1. adapter-local deterministic scenarios execute the real prepared facade and
   public operation role against scripted process, network, SDK, time,
   cancellation, and resource fixtures
2. consumer backend scenarios exercise the command or service boundary with an
   injected deterministic Swallowtail executor
3. a small native authenticated smoke proves final wiring, installed target
   selection, access, current protocol acceptance, and product projection

Repetition belongs in layers one and two. Native repetition is justified only
when application startup, persistence, restart, or another product lifecycle
is the claim.

## Evidence

Codex already has deterministic coverage for structured search events,
attachments, schema materialization, cancellation, deadline expiry, process
join, resource release, and distinct terminal outcomes. Soundcheck's native
agent-review workflow adds database startup, plug-in discovery, evidence
collection, prompt construction, repair, ranking, and UI state before reaching
that transport seam.

Repeating the fixed workflow would mostly exercise unrelated Soundcheck
machinery. It would not improve Swallowtail's mechanism evidence enough to
justify the setup and provider effects.

`swallowtail-testkit` remains a fixture and assertion library. It does not
become a universal executor. Adapter and consumer harnesses compose its public
pieces at their own typed operation boundaries.

## Result

- Contracts 011 and 036 now state the layered boundary.
- Release and consumer-evidence architecture no longer require UI repetition
  for mechanism claims.
- Card 044 may reconcile existing Nucleus vertical evidence and Soundcheck
  integration evidence without another live workload.
- Publication remains a separate operator decision.

## Next

Execute card 044. If a later defect lacks a deterministic reproduction, add
the smallest adapter-local or consumer-backend scenario that owns it; do not
reopen a broad native workload by default.
