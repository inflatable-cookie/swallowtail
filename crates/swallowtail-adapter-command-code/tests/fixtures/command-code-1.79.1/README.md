# Command Code 1.79.1 Identity

This fixture freezes the official npm `latest` package and every published
stable hop from `1.65.0` through `1.79.1`. `dist-inventory.json` records all
72 relative file paths and SHA-256 digests for each package, then recomputes
the exact added, removed, changed, and byte-identical sets for all 32 adjacent
hops. Every changed path has a per-hop classification.

The official package contains no repository or `gitHead` identity, so the npm
tarball SRI, SHA-1, SHA-256, package manifest, and complete extracted tree
identify the shipped source artifact. The package declares Node.js `>=22`;
the separately recorded Node runtime is only the static-inspection worker and
does not transfer as consumer-host evidence.

`dist/cli.mjs` changes at every hop. Its selected JSON event serializer and
`model_request_start.model` field remain present throughout. The first
`resolveLaneModel` implementation and `feature-model:planning` setting appear
at `1.73.0`. In plan mode, Command Code may select that configured planning
model ahead of the caller model forwarded with `-m`, subject to its own
configuration and model-access checks. The adapter records the requested ID
from its prepared plan and the CLI-selected ID from that event. The latter is
the model sent into the SDK request path; it does not prove which remote
provider or backend ultimately served it. The event remains ignored by the
public runtime activity projection; only the v2 opt-in debug observation reads
its model field.

All other changes are classified as package-version metadata, changelog
discovery, bundled documentation, or the unselected VS Code extension. The
fixed command, selected event/result/usage mapping, process lifecycle, and
public operations remain unchanged. The `1.73.0+` guarantee change is
version-specific and belongs to the approved pre-1.0 minor assessment; it is
not part of the urgent `v0.5.2` SDK patch.

`plan-model-selection.json` is a deterministic conflicting-configuration
fixture. It does not come from a live Command Code session and is never sent to
a provider. Its CLI record tests the selected model decoder and debug
projection without writing settings, relocating authentication state, or
running the provider artifact.
