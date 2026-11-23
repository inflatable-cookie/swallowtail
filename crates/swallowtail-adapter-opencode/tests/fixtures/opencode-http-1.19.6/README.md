# OpenCode HTTP 1.19.6 evidence

[Research 486](../../../../../docs/research/486-opencode-http-1-19-6-identity.md)
explains the stop on `opencode.server`.

- `identity.json` freezes both published hops, npm wrapper integrity, GitHub's
  verified immutable-release subjects, and the pinned GitLab package hashes.
- `native-inventory.json` contains every regular member of the 12 npm native
  packages and their matching GitHub CLI archives for each version. Asset
  digests matched both GitHub's release API and the verified release statement.
  The npm and GitHub executable hashes match for each target.
- `source-inventory.json` holds tag-archive hashes, canonical tree hashes,
  path-set hashes, the selected-file inventory, and Linux x64 literal
  observations. Canonical paths and encodings follow Research 464.
  `source-delta-1.19.6.tsv` accounts for each changed tag-tree path in the
  second hop. Research 464 retains the first hop's detailed inventory.
- `overflow-recovery.sse` is a synthetic, source-derived stream. The fixture
  server uses it to demonstrate how the existing adapter settles when it
  receives an overflow event before later usage and idle.

The ledgers describe artifact identity and static inspection. The synthetic
stream exercises Swallowtail's behavior. It supplies no upstream execution
evidence.
