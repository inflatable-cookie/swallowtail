# 2026-07-21 Claude Managed Agent Conformance And Closeout

## Changed

- added the tenth provider-neutral conformance profile for a provider-managed
  remote harness
- bound exact provider-agent identity, resource-free structured execution,
  durable retention, managed recovery, one reattachment, run callbacks, and
  per-resource deletion truth without changing the prior nine profiles
- proved blocking and child work before credential release under synthetic
  local and remote-authoritative hosts
- ran the production Anthropic Managed Agents driver through the same public
  profile under both opaque host, configured-instance, and endpoint identities
- retained provider-specific event, recovery, callback, interruption, and
  deletion behavior inside the Anthropic adapter fixtures
- closed card 079 and roadmap 025

## Evidence

- 10 provider-neutral conformance-profile tests pass
- 2 managed-harness preflight tests pass
- 10 production managed-driver tests pass across success, topology, callback,
  rescheduling, disconnect recovery, cancellation, deadline, provider failure,
  deletion ambiguity, redaction, and joined close
- focused warnings-denied clippy passes
- full `effigy qa` passes with 330 tests; three installed/live probes remain
  gated
- `effigy doctor` reports only the inherited 19 oversized-file findings: 12
  warnings and 7 errors
- `git diff --check` passes
- no live credential, provider account, external request, remote resource, or
  paid inference was used

## Remaining Risks

- Managed Agents remains a provider beta; endpoint, schema, event, retention,
  recovery, interruption, usage, rate, or deletion drift needs a dated evidence
  delta before mapping changes
- interruption acceptance remains distinct from authoritative terminal proof;
  cancellation can stay provider-unconfirmed while cleanup continues
- Cursor Cloud Agents still requires operator intent on repository and remote-
  mutation authority
- remote ACP still lacks a selected stable transport and reconnect authority

## Continuation

Return to the provider-coverage evidence checkpoint. Revalidate current
official sources, compare the remaining high-information routes, and ask for
operator intent where repository or remote-mutation authority would become
product policy. No implementation card is ready.
