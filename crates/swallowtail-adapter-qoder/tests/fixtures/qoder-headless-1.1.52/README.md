# Qoder headless 1.1.52 identity and stop evidence

Secret-free official npm identity for the `qoder.headless` route while the exact
qualified point stays at `1.1.25`. This corpus is identity evidence only: it
lands a typed stop, not a claim.

`identity.json` records the host observation, npm `dist-tags`, every published
stable point from the `1.1.25` baseline through official `1.1.52`, publication
times, tarball digests, integrity values, package metadata, the current claim at
observation, and the identity-first decision. `dist-inventory.json` records the
complete extracted package tree, per-hop added/removed/changed path sets, and
hashes for the shipped files that feed the selected route. `protocol.json`
records the selected invocation and wire presence map at every hop, the
mode-dispatch boundaries, the per-hop `--max-turns` authority classification,
and the stop boundary.

The stop is one exact counterexample: at `1.1.29..1.1.30` the CLI `--max-turns`
value stops being inert history and becomes the selected headless AgentLoop turn
ceiling. `1.1.25..=1.1.29` bind the package constant `1000` in the headless
session config and keep the fixed fallback in the drive-query expression;
`1.1.30..=1.1.52` bind the value forwarded from argv and drop that fallback.
`--max-turns` also gains a numeric `argParser` at `1.1.30`. With the selected
route argv `--max-turns 8`, a run that could reach 1000 turns at `1.1.25` now
terminates with `error_max_turns` at 8. That is a selected run-lifecycle and
bounded-limit failure change, so no exact-`1.1.52` claim lands and the operator
rules on the route's turn-binding policy first.

All 28 tarballs were retrieved from the official npm registry into `/tmp` on
2026-09-14, verified against the published SHA-1 and SHA-512 integrity values,
extracted, and inspected. No downloaded artifact was executed. No prompt, login,
credential, provider session, installation, or host update was used, and no host
`qoder` or `qodercli` exists on this machine. Research 151's `1.1.25` decoder
specimens and Research 256's empty skill-visibility disposition are unchanged.

No fixture contains a credential, host path, account identity, provider payload,
or real session id.
