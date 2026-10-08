# Pi SDK sidecar 1.1.0 qualification evidence

This fixture freezes the root npm package and every published stable hop from
`@earendil-works/pi-coding-agent@0.84.2` through `1.1.0` for the independent
`pi.sdk-sidecar` route.

## Files

- `identity.json` records the official npm `latest` and GitHub latest stable
  channel observations, exact npm tarball digests and integrity values,
  per-version package metadata, source tag commits, runtime identity, and
  source archive correlations.
- `dist-inventory.json` records each package tree's complete relative path and
  SHA-256 inventory, and the exact added, removed, changed, and identical path
  sets for each published hop.
- `protocol.json` classifies each changed compiled file and source map feeding
  selected session, event, tool, usage, attachment, configuration, model, or
  failure behavior. It freezes the facade operations, selected tool set,
  required exports, selected source hashes, and each hop's selected file set.
- `dependency-surface.json` freezes the corresponding `pi-ai` and
  `pi-agent-core` package artifact identities and the selected model row and
  lookup implementation.

## Artifact identity and execution boundary

The npm `latest` and GitHub latest non-prerelease channels agreed on `1.1.0`
on 2026-10-08. The npm package was published at
`2026-10-07T22:16:26.802Z`, with SHA-256
`09cd8a0a43dbb1d81a67346b09400b439ce71d818caba4e963ea958846a1aed4`,
SHA-512 integrity
`sha512-SeEi/4hdcHNgA9UWlefZl7ZZpm3dzi2OoxNjDHsBJ9o298LNOtbL4DGKgitlEj6uCTccvtw6f2hlCkTPVJ2RXg==`,
1,254 regular files, and complete tree SHA-256
`62fb9de5c48a64d72f8f8fb2ad7a1f7aae15ca389dc52348b7d973cc9bdf82c8`.
The matching GitHub stable tag is `v1.1.0` at
`abe508e1b89912adde45528136c3221eb69acdd7`; the npm `gitHead` is null. All
118 packaged source-map `sourcesContent` entries correlate with that tag's
source archive (SHA-256
`63b17b48b855e36e64c5013523acd48131ffcfa90ae48fe2f3e6fa9fe3d0da32`).
The previous `.84.2` npm `gitHead` equals its `v0.84.2` commit. For `.84.3`,
its npm `gitHead` differs from the release tag by two workflow/changelog-only
commits; all 220 packaged source-map sources match the tag archive (SHA-256
`00e138713eba79ca06890e507f87b0b7007fa3be38eac06ed0b3b63b16dd979a`).

Inspection used downloaded public artifacts and source archives only. The npm
artifacts were not executed; no provider prompt, session, credentials, install,
or host update was performed. The root package uses caret dependency ranges.
It includes `npm-shrinkwrap.json` through `1.0.0` and omits it from `1.0.1`
through `1.1.0`. Exact corresponding `pi-ai` and `pi-agent-core` package
artifacts are recorded, but a host-resolved transitive dependency tree is not
asserted. Provider-internal changes do not gain live-provider proof from this
static qualification.
