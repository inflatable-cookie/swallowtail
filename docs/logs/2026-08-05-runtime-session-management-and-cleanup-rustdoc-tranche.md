# 2026-08-05 Runtime Session Management And Cleanup Rustdoc Tranche

## Result

The fourth `swallowtail-runtime` public-API tranche now documents and locally
enforces durable session-resume bindings, inactive provider-session management,
typed lifecycle operations, and recovered driver-owned resource cleanup.

The review preserves three separate authorities. A resume binding permits one
exact attachment but is not management authority. A management binding permits
only the lifecycle capabilities proven for one exact inactive provider session.
A recovered-resource binding permits only cleanup of the driver-owned resources
left by one exact recovered run. Raw provider references, persisted records from
another route, and one binding type substituted for another grant no authority.

Persisted resume and cleanup records remain bounded, versioned, integrity
checked, opaque to consumers, and tied to their route or attachment
fingerprints. Archive, restore, and delete remain separate typed requests.
Cleanup effect truth and diagnostics do not imply retry, stronger deletion, or
confirmed removal after an uncertain effect boundary.

Module-local `deny(missing_docs)` covers the session binding and persistence
module, management binding and operation modules, and recovered-resource
cleanup plan and binding modules. The tranche removes 116 warnings.
`swallowtail-runtime` has 739 remaining; the workspace falls from 3,046 to
2,930 without suppression. Six of 27 packages remain closed under the release
documentation gate; runtime is not yet package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 2,930 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the remaining runtime operation-policy,
provider-import/catalogue, interactive, realtime, and direct-continuation
families.
