# 2026-09-14 Qoder headless 1.1.52 identity stop

Research 318 froze official Qoder npm `1.1.25` plus all 27 published stable
successors through `1.1.52` from the official registry. Tarball SHA-256 values,
npm SHA-1/SHA-512 integrity, full extracted trees, and consecutive-hop path
deltas were recorded. The package tree holds 33 files through `1.1.46`, 35 at
`1.1.47`, 29 at `1.1.48`, and 28 from `1.1.49`; the only additions are the two
`qodersec-update` vendor scripts and the only removals are the six macOS
seatbelt profiles and `policies/sandbox-default.toml`. `bundle/qodercli.js`
changes at every hop, so all 28 points were classified directly.

The identity decision is a typed **stop**, and the production claim does not
move. Exact `qoder.package` stays at `1.1.25` under
`qoder.headless.package-window-1`, one `QualifiedOnly` maintained point with the
unchanged `qoder.headless.stdio-stream-json-v1` behavior revision. The smallest
exact counterexample is the hop `1.1.29..1.1.30`: the selected route argv
`--max-turns 8` stops being historical inert history, the CLI option gains a
numeric argParser, both headless mode entry points begin forwarding `argv.maxTurns`
into the headless session, that session begins binding the forwarded value, and
`driveQuery` loses its fixed `1000` fallback. From `1.1.30` onward the real
AgentLoop ceiling for `qodercli --print` is the argv value, so a run that could
reach 1000 turns now terminates with `error_max_turns` at 8.

Everything else on the selected surface is unchanged: selected literals at all
28 points, mode dispatch, `dont_ask` permission modes, retention, `--cwd`
working resource, one owned stdio child with join-or-kill cleanup, the required
host deadline, and the `assistant`/`result` decoder shape. The
`system`/`init` `protocol_version` string moves `1.2.0 → 1.3.0 → 1.4.0` but is
not decoded by the adapter, and the removed sandbox assets feed an argv- and
host-config-selected CLI feature the route never enables. Research 151's
`1.1.25` specimens stand; Research 256's empty deliver-now disposition and the
`g05.039`/`g05.040` gate stay independent and no skill visibility is inferred.

The evidence is frozen in
`crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.52/` with a
mutation-sensitive ledger test. No downloaded artifact was executed and no
provider operation, prompt, login, credential, installation, or host update
occurred. The route returns to the operator via Chatterbox for a ruling on the
turn-binding policy before any `1.1.30`-or-later claim lands.
