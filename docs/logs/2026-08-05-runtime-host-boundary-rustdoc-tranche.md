# 2026-08-05 Runtime Host Boundary Rustdoc Tranche

## Result

The first `swallowtail-runtime` public-API tranche now documents and locally
enforces role requests, driver registration, host-service ports, the exact
host-service registry, and operation lifecycle handles.

The review keeps authority explicit. A role request carries only caller and
preflight inputs. A registration rejects roles absent from its descriptor.
Host services remain optional, execution-host-bound effect ports without
fallback inference. Handle streams and outcome receivers are take-once views;
closing still joins scoped work and releases resources at the exact ownership
strength exposed by each handle.

Module-local `deny(missing_docs)` covers `roles`, `registration`,
`host_traits`, `host_registry`, and `handles`. The tranche removes 260
warnings. `swallowtail-runtime` has 1,407 remaining; the workspace falls from
3,858 to 3,598 without suppression. Six of 27 packages remain closed under the
release documentation gate; runtime is not yet package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- the 27-package semantic public API baseline remained unchanged
- workspace all-feature Rustdoc completed with 3,598 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the runtime activity, event, outcome, and callback
families before moving into provider-session and recovery records.
