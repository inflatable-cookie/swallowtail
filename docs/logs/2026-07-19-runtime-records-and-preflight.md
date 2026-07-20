# Runtime Records and Preflight

Date: 2026-07-19
Status: recorded

## Result

g01 roadmap 004 is complete.

- `swallowtail-core` now represents driver, integration, transport, instance,
  host, model-route, access-profile, ownership, protocol-facade, and operation
  requirement dimensions separately.
- Access status exposes credential, entitlement, endpoint authorization,
  runtime readiness, and support authority without a collapsed readiness flag.
- Capability profiles carry exact named constraints; unsupported or unknown
  constraints fail explicitly.
- Preflight has no side-effect-capable inputs. It returns a dimensional safe
  failure or an immutable plan bound to the exact selected records.
- `swallowtail-testkit` covers ten rejection cases, exact successful binding,
  and stale instance revision with a provider-side-effect recorder.
- Fifteen tests, formatting, checking, clippy, docs QA, links, and diff hygiene
  pass.

## Boundary

No async traits, executor, process or network service, host-reference
resolution, provider transport, credential value, or consumer record was
added. Those remain owned by roadmap 005 or later roadmaps.

## Next Lane

g01 roadmap 005 begins with the minimal executor-neutral runtime crate and
separate dynamic role traits. The pure preflight plan remains the mandatory
gate before any role call.
