# Release Baselines

Two public-API directory roles:

- `public-api-<current>` is the working baseline. It matches
  `workspace.package.version` and absorbs additive API between tags.
  `effigy package:api` diffs the live API against this directory.
- `public-api-<previous>` is immutable. It is the newest tagged `public-api-*`
  directory older than current. A patch may not remove an item that exists
  there.

Do not recreate `public-api-unreleased`. Additive API lands in
`public-api-<current>`.

Every version-labelled file records the immutable tagged source for its own
version. Compare it with the tag named by that version, never with a moving
branch: `production-routes-0.1.1.txt` was created by the `v0.2.0` preparation,
so its owning record is `v0.2.0`; every other file here first appears in the
tag it labels. `v0.5.2`, `v0.5.3` and `v0.5.4` were tagged on `release/v0.5`,
so their directories exist here as `release/v0.5` evidence; `main` restores
them from those tags instead of regenerating them.

Route inventories (`production-routes-<version>.txt`) and internal dependency
graphs (`internal-dependencies-<version>.tsv`) follow the same current /
previous pairing. Historical freeze-audit files stay as named evidence; they
are not the working or immutable public-API roles.

The four gate scripts and `scripts/check-consumer-front-door.py` derive current
and previous from `Cargo.toml` and these directories. Release prepare does not
repoint them.
