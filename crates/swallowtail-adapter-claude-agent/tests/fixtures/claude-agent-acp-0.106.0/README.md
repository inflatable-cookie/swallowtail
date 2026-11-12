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
