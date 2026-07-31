# Pi RPC Session Continuity

Status: deferred
Source: g02.029 and cards 096-098

## Gap

Pi RPC exposes persisted-session switching, ordered message reads, and
append-order entry reads. Its maintained public interface does not let
Swallowtail attach the session with the exact host-leased working directory
and corroborate that binding before use.

Without that surface, a copied session path or id could appear to authorize a
different working resource. Swallowtail therefore cannot honestly expose
prepared load or replay-free resume while preserving its immutable resource
and host plan.

## Preserved Evidence

- [source roadmap g02.029](../g02/029-pi-rpc-session-continuity.md)
- [card 096 currentness and corpus](../g02/batch-cards/096-pi-rpc-session-continuity-currentness-and-corpus.md)
- [card 097 implementation](../g02/batch-cards/097-pi-rpc-session-continuity-implementation.md)
- [card 098 closeout](../g02/batch-cards/098-pi-rpc-session-continuity-closeout.md)
- Research 053
- Contracts 009, 017, and 038

## Promotion Gate

Promote only after a maintained public Pi interface can:

1. attach or load a persisted session with the exact caller-bound working
   directory
2. return enough public evidence to corroborate that resource binding before
   a usable handle exists
3. preserve complete ordered replay for load and replay-free resume
4. retain exact host, configured instance, access, model, provider-state,
   cancellation, and cleanup bindings

Promotion requires a currentness refresh. Do not infer support from private
state, copied identifiers, process cwd alone, or a newer version number.

## Retained Boundary

The current ephemeral Pi interactive route and `--no-session` structured-run
route remain unchanged. This backlog item grants no native close, archive,
restore, delete, containment, or provider-session binding persistence claim.

