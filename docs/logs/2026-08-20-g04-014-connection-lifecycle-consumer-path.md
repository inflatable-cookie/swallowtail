# g04.014 Connection Lifecycle Consumer Path

Date: 2026-08-20
Roadmap: `../roadmaps/g04/014-connection-lifecycle-consumer-path.md`
Cards: `../roadmaps/g04/batch-cards/039-connection-lifecycle-feature-guide.md`,
`../roadmaps/g04/batch-cards/040-first-proof-route-guide-amendments.md`,
`../roadmaps/g04/batch-cards/041-connection-lifecycle-examples-and-guide-map.md`

## Result

Consumers can follow a Contract 052 path from addable catalog to the
existing prepared facade for the three first-proofs.

`docs/guides/connection-lifecycle.md` names the portable 057 records,
ordering, and forbidden inferences. Anthropic Messages, Codex app-server,
and Ollama attach route guides name addable id, topology, credential or
its absence, refresh, subject, update, overlay, and the prepare handoff.
`codex.exec` and the remaining production routes stay on the prepared
facade. Hosted interactive OAuth is not documented as realized.

Compile-only examples:

- `crates/swallowtail-adapter-anthropic/examples/connection_lifecycle.rs`
- `crates/swallowtail-adapter-codex/examples/connection_lifecycle.rs`
- `crates/swallowtail-adapter-ollama/examples/connection_lifecycle.rs`

Anthropic collects `CredentialRef`. Codex and Ollama have no credential
field. Overlay may mark `anthropic` catalogue rows. Codex and Ollama rows
stay unmarked. Stored `ConfigFieldRef` values do not feed `prepare_*`.

Portable feature token `connection_lifecycle` and a complete guide-map
family row are in place. Architecture records that the 052 path exists for
those three routes only. `public-api-0.3.3` is unchanged.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-a162a941`
Worker branch: `t3code/connection-lifecycle-consumer-path`

Validation: `effigy qa:docs`, `effigy qa:guides`, `effigy check:examples`,
`git diff --check`.

PR: https://github.com/inflatable-cookie/swallowtail/pull/12

## Next

Await review. Do not merge without operator authorisation. Hosted OAuth
stays a remaining gate. After merge, g04.010 first-proof-plus-consumer-path
can be marked completed.
