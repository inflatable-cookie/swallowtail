# 2026-07-23 Attached Runtime Records And Range Assertions

## Changed

- Added provider-neutral installed-inventory, running-inventory, and
  selected-model-detail observation scopes.
- Kept configured instance, execution host, exact runtime version, native
  model tag, route model, manifest digest, and model artifact as separate
  identities.
- Added bounded native model tags and SHA-256 manifest evidence. Public records
  carry no endpoint, path, raw manifest, prompt, output, or payload.
- Added exact attached-runtime requirements and pure preflight agreement across
  external ownership, structured direct inference, instance, topology, route
  model, runtime version, native tag, and manifest digest.
- Added explicit `RuntimeManaged` residency acceptance to request policy. It
  grants no unload, eviction, restoration, duration, or capacity authority.
- Added an assertion pack over the unchanged attached-self-hosted profile and
  reusable closed semantic-window assertions.

## Range Correction

Stable semantic segments previously admitted prerelease values that sorted
inside their boundaries. Stable ranges now reject prereleases. A prerelease
can pass only through its own exact qualified segment.

The Ollama fixture window now proves:

- baseline `0.14.0`
- latest-qualified `0.32.1`
- intermediate points `0.18.0` and `0.30.0`
- rejection below and above the window
- rejection of `0.18.0-rc.1` and `0.32.3-rc.0`
- stale compatibility-claim rejection

## Validation

- focused `swallowtail-core`, `swallowtail-runtime`, and
  `swallowtail-testkit` tests pass
- focused warnings-denied clippy passes
- `git diff --check` passes
- `effigy doctor` remains at the inherited 19 findings: seven errors and
  twelve warnings

## Next

Card 107 freezes the Ollama `0.14.0` through `0.32.1` native corpus and adds
bounded pure codecs. It adds no production network driver or live request.
