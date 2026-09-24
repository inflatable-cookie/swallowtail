# Command Code 1.65.0 identity corpus

Secret-free official npm identity for the `command-code.headless` route before
the exact claim moves from `1.54.0` to `1.65.0`.

`identity.json` records the host observation, npm dist-tag and integrity
records, every published stable hop after `1.54.0`, and the identity-first
decision. `dist-inventory.json` records the complete extracted package tree
and exact per-hop file deltas. `protocol.json` records the selected
invocation, AgentEvent, result, usage, failure, retention, and continuation
subset plus the explicitly bounded unselected additions.

All artifacts were retrieved from the official npm registry into `/tmp` on
2026-09-24. Downloaded artifacts were inspected but never executed. No prompt,
login, credential, provider session, installation, or host update was used.
Research 116 and 118 remain bound to `1.15.1`. Research 330 remains bound to
`1.54.0`.
