# 085 Kimi Platform Direct Driver

Status: planned
Owner: Tom
Updated: 2026-07-21
Milestone: `../028-kimi-platform-k3-direct-inference-proof.md`

## Objective

Implement the separately registered Kimi Platform catalogue and K3 structured
direct-inference driver against card 084's frozen corpus.

## Governing References

- Research 018
- Contracts 005, 006, 008-011, 014, 020, and 024
- roadmap 028
- card 084 evidence

## Scope

- separate Kimi Platform adapter package and driver identity from Kimi Code ACP
- host-approved `api.moonshot.ai` endpoint and platform API-key lease
- bounded authenticated `/v1/models` catalogue
- one text-only streaming `/v1/chat/completions` attempt for exact `kimi-k3`
- explicit output bound and `low`, `high`, or `max` reasoning selection
- ordered reasoning progress, final output, terminal usage, returned-model
  agreement, provider failure, cancellation, deadline, redaction, and cleanup
- no SDK retry, provider reconnect, state, or fallback

## Acceptance Criteria

- [ ] descriptor, instance, access, audience, route, model, reasoning, and
      output bound are exact before endpoint or credential work
- [ ] Membership, Kimi Code, subscription, and regional platform credentials
      cannot satisfy the direct access profile
- [ ] catalogue evidence remains source-scoped and does not create routes or
      prove entitlement
- [ ] one start emits one provider attempt through the shared structural codec
- [ ] returned-model mismatch, unknown semantics, provider errors, disconnect,
      cancellation, and deadline remain distinct
- [ ] every connection task is joined before awaited credential release
- [ ] stable diagnostics expose no endpoint, secret, raw payload, prompt,
      reasoning, output, or provider id

## Validation

- focused adapter and protocol tests
- focused warnings-denied clippy
- all-target workspace compile
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- implementation needs an ambient endpoint or credential
- the provider requires implicit retry, state, model alias, or fallback
- common runtime or codec APIs need Kimi identity

## Auto-Continuation

Yes. Continue to card 086 when the production driver satisfies the frozen
corpus without widening Contract 024.
