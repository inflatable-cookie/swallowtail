# 078 Antigravity CLI 1.1.9 Artifact And Catalogue Qualification

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Can one exact installed Antigravity CLI artifact support identity-safe
discovery and an authenticated model catalogue without inheriting Gemini
identity, access, or behavior claims?

## Method

This pass combined current official Antigravity documentation, the maintained
repository tags and changelog, exact installed version/help/catalogue probes,
binary hashing, and macOS signature inspection. It sent no provider prompt,
started no agent run, changed no account state, and read no credential,
keyring, or account identity.

The exact installed artifact is:

- executable: host-approved `agy`
- reported version: `1.1.9`
- platform: macOS arm64
- SHA-256: `a27bff8d7c47fe5407e6740f14ecef73e86fb65ec73fec77b0765f8849024383`
- signature authority: `Developer ID Application: Google LLC (EQHXZ8M8AV)`
- observed signing timestamp: 2026-07-31 04:26:14 local host time

The sanitized record, help, version, and model list live under
`swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.9`.

## Version Reconciliation

Official documentation still labels the CLI `1.1.8`. Repository tags `1.1.8`
and `1.1.9` both resolve to commit
`03e095ac3619462ecd0928f3f5470387dbda6a00`, whose changelog entry is headed
`1.1.8`. The installed Google-signed executable reports `1.1.9`.

Swallowtail qualifies only the observed `1.1.9` executable. It records the
shared tag commit as source and documentation evidence, not as two behavior
milestones or a continuous range. `1.1.8` is therefore not independently
permitted by the first production claim. Later stable versions remain visible
as unverified newer.

The public repository contains documentation, examples, and the changelog,
not the executable implementation source. Claims about process output are
therefore bounded by exact local probes and official documentation rather than
an invented source audit.

## Catalogue Result

`agy models` succeeds against the provider-owned cached Google session and
emits one model slug per line. The exact probe returned 11 identities spanning
Gemini, Claude, and GPT-OSS routes. Swallowtail preserves those opaque slugs
without inferring provider ownership, display labels, defaults, reasoning
support, entitlement, or invocation availability.

The catalogue parser accepts at most 512 unique ASCII model ids, 256 bytes per
line, and 512 KiB total output. Empty, duplicate, malformed, control-bearing,
or oversized responses fail closed. Non-zero exits retain status and only a
bounded sanitized stderr excerpt.

## Access And Identity

Antigravity is a separate integration family with an `agy models` stdio
transport. It is not a Gemini driver or fallback. The first access profile
represents provider-owned local Google Sign-In and subscription allowance.
Swallowtail neither acquires nor exports the keyring credential.

Enterprise project access remains a separate unimplemented posture. Gemini
API-key and enterprise routes do not satisfy the Antigravity personal profile,
and Antigravity cached sign-in does not satisfy Gemini access.

## Contract Result

No shared contract change is required. Existing integration identity,
auth-aware catalogue, exact installed-version, host-approved executable,
ambient configuration, topology, diagnostics, and forward-compatibility
contracts represent the route without flattening it into Gemini.

## Risks

- Documentation and tag numbering can drift from the distributed executable.
  Future qualification must retain exact binary evidence.
- The public repository does not expose implementation source.
- Model presence does not prove account entitlement or successful invocation.
- `agy models` depends on provider-owned cached authentication and can fail or
  change independently of executable discovery.
- Enterprise access, headless execution, continuation, permissions, and
  optional sandboxing remain outside this first claim.

## Primary Sources

- [Antigravity CLI overview](https://antigravity.google/docs/cli/overview)
- [Antigravity installation and auth](https://antigravity.google/docs/cli/install)
- [Antigravity headless mode](https://antigravity.google/docs/cli/headless)
- [Antigravity CLI repository](https://github.com/google-antigravity/antigravity-cli)
- [Antigravity CLI tags](https://github.com/google-antigravity/antigravity-cli/tags)
- exact installed Google-signed `agy` 1.1.9 artifact and sanitized probes
