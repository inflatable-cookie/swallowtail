# Research 409: Muse Code Signed Payload 1.4.4-R5419.1 Identity

Date: 2026-10-08

## Scope and result

This is source discovery for `muse-code.headless`, not a qualification or a
claim change. The selected opaque axis remains
`muse-code.signed-payload`; the current claim still qualifies only
`0.2.1-R1215.1` as `QualifiedOnly` (Research 131 and the current
`selection.rs`).

Meta's public `muse-stable` channel identified `1.4.4-R5419.1` as its current
stable target. Its version-addressed release manifest and the Darwin ARM64
artifact were publicly retrievable without authentication. The artifact's
SHA-256 matched the vendor manifest, and Apple's code-signature verification
identified Meta Platforms, Inc. as publisher. This establishes the exact
current artifact identity. It does not establish compatibility or a supported
range.

## Official source chain

Meta's Muse Code install documentation points to `https://dev.meta.ai/install.sh`.
That installer names `https://api.meta.ai/muse-launcher.sh` as its launcher
source and checks the downloaded launcher's SHA-256 response header. The
observed launcher source was 32,737 bytes, SHA-256
`c6db294799a190ca380da274beb3b9c0e160e0da9681a3d364ce8b0e5fa3a4bc`;
its script version is `3`. The script selects channel `muse-stable`, resolves
`https://api.meta.ai/muse-code/channels/muse-stable`, then requires the release
manifest version to match the channel version and checks the selected
platform artifact's declared size and SHA-256 before installing it as
`muse-bin-<version>`.

The channel response observed on 2026-10-08 at 16:21:49 UTC was HTTP 200. Its
raw response body SHA-256 was
`22967b2f349aba73c58f07f83d40846036e47f298ee6e47caded9fdfc464f3a9` and named
version `1.4.4-R5419.1`, state `public`, with the version-specific manifest
URL below. The channel URL is mutable discovery metadata, not the artifact
identity.

| Artifact | URL / identity | Size | SHA-256 |
| --- | --- | ---: | --- |
| Release manifest | `https://lookaside.facebook.com/lookaside/muse/download/?channel=muse&version=1.4.4-R5419.1&file=manifest.json` | — | `a56683b680f97443d46d0262c2996faa8d028c47ae40d444303e4461ad1968eb` |
| Darwin ARM64 payload (`muse-aarch64-macos`, installed as `muse-bin-1.4.4-R5419.1`) | `https://lookaside.facebook.com/lookaside/muse/download/?channel=muse&version=1.4.4-R5419.1&file=muse-aarch64-macos` | 352,745,840 bytes | `7f708414207d1665858baa6ef2ef0f5c69c727f5b3c00243fa398c2f5d5b5cc0` |

The manifest response was HTTP 200 at 16:21:50 UTC. It uses `sha256` and
declares the same size and digest as the downloaded Darwin ARM64 Mach-O. An
unauthenticated HEAD request to the artifact URL returned HTTP 200; the full
GET succeeded without credentials. `codesign --verify --strict` passed. The signature reports
identifier `muse-arm64`, Team ID `V9WTTPBFK9`, authority
`Developer ID Application: Meta Platforms, Inc. (V9WTTPBFK9)`, and signing
timestamp 2026-10-07 16:11:29 BST (2026-10-07 15:11:29 UTC). The artifact was
inspected but never executed. The vendor's version-qualified URLs identify the observed bytes;
Meta does not publish a retention or immutability guarantee for those URLs.

No public source commit or source tree was correlated to the shipped runtime.
The support identity available here is the exact signed payload binary,
publisher identity, and vendor manifest digest, consistent with Contract
029's artifact-authority rule when public source is unavailable. Research 369's
prior observation that `muse --version` reported launcher `1.0.3` remains a
separate launcher observation; this task did not execute it because the
launcher can update itself.

## Stable ordering and remaining evidence

The channel reports one current stable point. Its launcher validates the
version string's syntax but does not define comparison semantics. Meta's
public changelog currently has a `1.4.2` section and does not enumerate the
`R` build identifiers or a complete stable artifact history. Therefore the
current target is exact, but a complete ordered hop list and ordering
semantics for `muse-code.signed-payload` remain unestablished. The semantic
version component changes from major `0` to major `1` relative to qualified
`0.2.1`; treat that as a major-line identity investigation, not an inferred
compatible range.

For the next one-family qualification brief, use exact target
`1.4.4-R5419.1`, the version-specific manifest and Darwin ARM64 digest above,
and Research 112/131's frozen `0.2.1-R1215.1` evidence. Ask Meta for a complete
stable payload history or ordering statement only if a maintained range is
proposed. Compare the selected headless behavior against the frozen corpus;
do not infer behavior from changelog text or a launcher version. Preserve the
existing claim until that work is approved and proved. Any proposal that
removes an existing qualified point, needs live proof, or changes the public
route/lifecycle still needs a separate ruling.

## Attempts and boundaries

- An initial unauthenticated request to
  `https://api.meta.ai/muse-code/channels/stable` returned HTTP 404. The
  official launcher source specifies `muse-stable`; that exact channel
  returned HTTP 200.
- The release manifest and Darwin ARM64 artifact returned successfully with
  no authentication. No credentials, install, launcher invocation, host
  update, provider prompt, live catalogue/session, or binary execution was
  used.
- The only downloaded runtime artifact was the Darwin ARM64 build matching
  this inspection host. Other platform entries were not downloaded or
  signature-checked.
- The official changelog is discovery evidence only and does not supply the
  missing artifact history or source commit.

## Sources

- [Meta Muse Code installation and product documentation](https://dev.meta.ai/docs/muse-code)
- [Official install script](https://dev.meta.ai/install.sh)
- [Official Muse launcher source](https://api.meta.ai/muse-launcher.sh)
- [Official `muse-stable` channel metadata](https://api.meta.ai/muse-code/channels/muse-stable)
- [Version-specific Muse release manifest](https://lookaside.facebook.com/lookaside/muse/download/?channel=muse&version=1.4.4-R5419.1&file=manifest.json)
- [Version-specific Darwin ARM64 artifact](https://lookaside.facebook.com/lookaside/muse/download/?channel=muse&version=1.4.4-R5419.1&file=muse-aarch64-macos)
- [Meta Muse Code changelog](https://dev.meta.ai/docs/muse-code/changelog)
- [Research 369: all-route version currentness checkpoint](./369-all-route-version-currentness-checkpoint.md)
- [Research 131: Muse Code 0.2.1-R1215.1 host payload evidence](./131-muse-code-0-2-1-host-payload-drift.md)
