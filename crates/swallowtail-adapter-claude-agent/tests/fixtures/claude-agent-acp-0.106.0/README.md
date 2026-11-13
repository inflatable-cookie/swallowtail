# Claude Agent ACP 0.106.0 inventory in progress

This preliminary inventory records the npm package trees from 0.100.0 through
0.106.0 and the complete per-hop path and digest changes. The registry record
includes npm metadata, downloaded tarball SHA-256 values and matching upstream
Git tag commits. No package was installed or executed.

This is identity work in progress, not qualification. Integrity verification,
selected-path classification, dependency tracing, behavioral proof and
mutation-sensitive assertions remain before changing the compatibility claim.

The ACP tarballs' SHA-512 integrity values and SHA-256 values were verified
against npm. `permission-boundary.json` records a separate Linux x64 static
observation: the SDK 0.3.319 and 0.3.322 native artifacts contain a setting
that can suppress `canUseTool` prompts. ACP enables user settings and routes
permission requests through that callback. This needs a route-specific
adaptation decision and proof; it does not establish a revised claim. The
project and local settings copies of this key are ignored by the native
setting; user and managed settings remain relevant.

`dependencies.json` freezes registry integrity, per-file hashes, adjacent
version deltas and the bundled Agent SDK native manifests. Those manifests
are package evidence, not qualification of the separate SDK/native axes.
`static-closure.json` records a conservative literal-import traversal from
v1 entry points and assigns each changed path an explicit structural class and
reachable modules a surface role. Dynamic resolution and semantic effects
remain separate review obligations; a reachable label is not proof of use,
and an unreached label is not proof of absence from all dynamic paths.

`idle-compaction-boundary.json` records a second selected process boundary:
from 0.101.0 the v1 session can initiate `/compact` without a client prompt
when the recorded idle, cache and token conditions hold. This observation does
not authorize provider execution or qualify a new version.

`relocations.json` pairs the v1 paths moved in 0.103.0 and the session-index
paths moved again in 0.105.0. Each pair has both hashes; nonidentical pairs
remain selected review work rather than being classified as unchanged.

`native-permission-control.json` pins bounded static no-prompt function ranges
for Linux x64 and Darwin arm64 SDK 0.3.319, 0.3.321 and 0.3.322. The ranges
are not a complete native behavior proof. `idle-control-trace.json` binds the
v1 timer, settings and teardown trace to exact artifact source ranges and
records the in-flight timer race and the route's host-supplied environment.

At 0.101.0 the timer reads the ambient settings manager directly, so an inline
SDK settings override does not establish idle suppression. At 0.106.0 inline
compaction settings are considered, but managed values take precedence. The
underlying files are pinned in `dist-inventory.json`; see Research 482 for
the remaining route-authority and race proof.

`guard-import-edges.json` binds each observed v1 caller and idle module to
the registry tarball and complete inventory. It records only static class
shape and the literal import; it does not attest Node loading or native code.
