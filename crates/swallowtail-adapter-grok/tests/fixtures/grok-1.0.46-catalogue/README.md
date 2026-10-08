# Grok Build catalogue 1.0.46 identity stop

This secret-free corpus freezes the official `@xai-official/grok` package
identity and the matching Darwin ARM64 runtime artifacts from `1.0.30`
through `1.0.46`. Registry SRI and SHA-1 values were verified against all
downloaded tarballs; the corpus also records locally computed SHA-256 digests.
No package script or executable was run, and no binary is stored here.

- `identity.json` records registry channel state, exact wrapper and platform
  package identities, every compared release, and the exact qualification
  stop.
- `dist-inventory.json` records every shipped path and digest plus added,
  removed, changed, and identical paths for each of the sixteen hops.
- `runtime-surface.json` records decompressed executable digests, sizes,
  selected static catalogue/authentication signals, and the embedded default
  model document at each point. It does not claim to characterize the binary's
  other internal changes.

The `1.0.46` executable contains one new `Default model:  (not in the list
below)` marker. The existing adapter parser rejects that value. Research 405
records why the catalogue claim remains at exact `1.0.30` and names the
same-contract adaptation or operator ruling needed to continue.
