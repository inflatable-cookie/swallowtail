# OpenCode 1.18.10 Range Extension

Date: 2026-07-30

Card 148 extends the OpenCode HTTP/SSE guaranteed window from
`1.14.48..=1.18.4` to `1.14.48..=1.18.10`.

## Evidence

- six exact stable releases add npm publication dates, tag commits, and full
  OpenAPI hashes
- selected execution, lifecycle, deletion, continuity, callback, usage,
  generation-control, and activity schemas remain unchanged
- all selected routes continue to dispatch through `surface-18`, `delete-02`,
  and `runtime-02`
- `1.18.8` adds one optional field to an unrelated OAuth callback artifact;
  `1.18.9` removes it
- the corpus records that full-artifact delta without claiming a selected
  protocol milestone
- exact stable points above `1.18.10` remain permitted as visibly unverified
  newer; prereleases remain incompatible

## Validation

- `cargo test -p swallowtail-adapter-opencode`: 82 passed
- `cargo clippy -p swallowtail-adapter-opencode --all-targets -- -D warnings`
- provider route, lifecycle, feature, and activity matrix checks
- docs QA
- `git diff --check`

No live OpenCode prompt, server mutation, installation, consumer edit, or
publication ran.

## Next

Card 149 closes the two-adapter maintenance tranche through focused cross-host,
package, public-API, and public-truth evidence.
