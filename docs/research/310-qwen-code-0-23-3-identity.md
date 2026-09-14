# Research 310: Qwen Code 0.23.3 Identity

Status: complete; identity evidence only. Production claim changes land in
the g05.060 claim batch after this record.

Observed 2026-09-14 on the `qwen-code.package` axis:

- Host `qwen --version`: `0.21.2`; executable SHA-256
  `08d28a806f88eb00351fd32f32f891ae5c17d39d28c2538a57a8f8b931684a13`,
  71 bytes. The host was not installed, updated, or otherwise mutated.
- `npm view @qwen-code/qwen-code version`: `0.23.3`.
- Official npm stable hops after the prior `0.22.3` ceiling are exactly
  `0.23.0`, `0.23.1`, `0.23.2`, and `0.23.3`. Preview and nightly channels
  were excluded. `0.20.2`, `0.21.16`, `0.22.4`, and `0.23.4` were not
  published stable points at observation.
- The family release channel is the direct QwenLM/qwen-code tags and releases
  `v0.23.0` through `v0.23.3`; the repository-wide generic `latest` release
  endpoint resolves an unrelated SDK release and was not used as family
  evidence.

## Official hop identities

| Version | npm published | GitHub tag commit | npm shasum | extracted package files |
| --- | --- | --- | --- | ---: |
| `0.22.3` | `2026-08-28T17:30:35.909Z` | `09825973e7d3c3fd07e17909c396aa62f48ce51f` | `1529461d6c79c273b184849e6777e54ac98ae860` | 1005 |
| `0.23.0` | `2026-09-03T11:53:52.432Z` | `98a9c964158697dd5631d15a62174684ff7bbb53` | `01a83323ddf99f2b9b0e76e87c0ea42fab1d24c2` | 1023 |
| `0.23.1` | `2026-09-08T16:17:55.104Z` | `d9331e6fcf7f8f8225a16a9ec6973f8f38f56747` | `d15f0dd9ecc8325d88f5b79439849e31d55d581c` | 1033 |
| `0.23.2` | `2026-09-09T11:08:10.043Z` | `f56de980b316cd5410f067fbb62357481ebd66b8` | `98c144d1ec89a66e6f0cb2d6685efda2e1e9f6cd` | 1034 |
| `0.23.3` | `2026-09-10T15:20:05.693Z` | `b695664b8df06d06c625db3e30b97045d82092c7` | `3336056e130b9199bf365e0c2a34416523240e18` | 1050 |

The official `0.23.3` npm integrity is
`sha512-QaBNg5y/nfKJCMdVYjZmEx9n0StGTvg3NIYRExHIy9OWgBQ4wE7MQc7YrAMvy13pUmqJhYlpwOtvNsGDAgvYdA==`.
Its tarball SHA-256 is
`1f791a136e326f79f317784d1244c79b020a0058eb9458dcf6e1594ae67fb5e5`.
The extracted `cli.js` is 14,550 bytes with SHA-256
`d2556c3877e4f697c14e1fb8f532561b583ae99f638b07b9412c6ed491e315be`.

The complete package path/hash inventory and per-hop added, removed, changed,
and identical sets are frozen in the
[0.23.3 dist inventory](../../crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.23.3/dist-inventory.json).
The per-hop category counts are `402/384/37/584`, `378/368/37/618`,
`261/260/14/759`, and `345/329/19/686` respectively.

## Selected-surface classification

The selected non-interactive types and system controller are byte-identical
across all five compared points. The shared help-option refactor in `0.23.0`
moves definitions into `top-level-options.ts`, but the selected flags,
approval values, input/output formats, and selected read tools remain present
and stable through `0.23.3`. The static image-only catalogue filter and the
`get_available_models`/`set_effort` control names remain unchanged.

The selected mapped stream types retain `system`, `stream_event`, `assistant`,
and `result`, with nested `goal_state` left unmapped. Session changes in
`0.23.1` and `0.23.2` are import/refactor and resumed-transcript prompt-ID
bookkeeping; they do not change the selected provider session ID or resume
wire. The exact `0.21.15` reasoning control and caller-decreasing budget
boundaries remain unchanged and are not widened.

The Plan policy, permission-flow selected inputs, and Plan tool source are
byte-identical where compared, except for the `0.23.2` Plan execution-mode
confirmation change. That direct counterexample keeps the separately proven
exact Plan set bounded at `0.22.3`; reasoning and budgets remain exact
`0.21.15` only.

Changed upstream files feeding goals, retries, generic/GPT or Alibaba
reasoning metadata, provider model-discovery ordering, permission denial
presentation, pre-aborted tool scheduling, Web Shell, daemon, ACP, session
source registry, transcript export, and UI/workflow surfaces are named and
bounded in `protocol.json` and remain unmapped. None changes Qwen headless
execution authority or justifies flattening into Model Studio or Qwen ACP.

## Contract 029 decision

The family is a `compatible-extension` of
`qwen-code.headless.v0.21.15-reasoning-control`. Keep the deprecated
`0.19.11..=0.20.1` and `0.21.0..=0.21.14` windows, exact `0.21.15`, and
maintained `0.22.0..=0.23.3`. Qualify every published hop. Keep the
unpublished gaps and `AllowUnverified`; after qualification, `0.23.4` is the
first unverified newer stable candidate. No provider prompt, live catalogue,
login, installation, host update, or credential was used.
