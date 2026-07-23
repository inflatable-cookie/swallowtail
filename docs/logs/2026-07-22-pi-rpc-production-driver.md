# Pi RPC Production Driver

Date: 2026-07-22

## Changed

- added the separate `swallowtail.pi.rpc` interactive-session descriptor and
  production driver for the qualified Pi `0.80.10` point
- added supervised bounded strict-LF JSONL with exact response correlation,
  state validation, prompt, steering, follow-up, events, and UI projection
- bound exact provider, model, executable target, delegated harness access,
  working resource, restrictive ambient policy, and disabled retry state
- added native abort for cancellation and deadlines without claiming
  downstream provider stop
- added joined process, callback, timer, resource, and credential cleanup

## Evidence

The deterministic process fixture accepts the same startup and state records as
the frozen corpus. It proves that pure request-plan validation happens before
credential or process effects, provider and model reach argv without fallback,
command acknowledgement precedes model settlement, one steering and one
follow-up message retain their scheduling classes, and dialog UI round-trips
through the callback exchange. Startup drift, cancellation, and deadline paths
all close without installing Pi, reading live harness authentication, or making
a provider request.

The route reports `AmbientHost` with read-intent tools. It makes no filesystem,
network, child-process, credential, container, or sandbox containment claim.

Callback expiry and late response, remote-authoritative topology, disconnect,
provider failure, retry drift, malformed correlation, and cleanup-failure
cross-products remain card 101 work. The driver is implemented; roadmap 035 is
not closed.

## Validation

- 8 Pi adapter tests pass
- 47 runtime tests pass
- focused runtime and Pi warnings-denied clippy passes
- workspace all-target check, docs QA, format check, and diff check pass
- `effigy doctor` returns the inherited 19 findings: 7 errors and 12 warnings,
  with no new finding

## Next

Card 101 is ready. Run the unchanged RPC profile and Contract 028 assertion pack
under local and remote-authoritative hosts, add callback-timeout and late-
response coverage, close the full failure matrix, then close roadmap 035.
