# 2026-09-24 Command Code 1.65.0 Identity

Research 339 froze official npm `latest` `1.65.0` and all 19 published stable
successors after `1.54.0`. The complete extracted package trees have no
removals, one additive bundled `/loop` skill, and a byte-identical
`dist/index.mjs`. Selected invocation, AgentEvent, result, usage, plan mode,
failure, and local lifecycle surfaces remain compatible.

The identity-first decision is compatible-extension: rebind the exact
`command-code.npm` point to `1.65.0` with the same
`command-code.agent-event-ndjson-v1` behavior and `QualifiedOnly` posture.
Research 330 stays bound to `1.54.0`; Research 116/118 stay bound to
`1.15.1`. Frozen records live in
`crates/swallowtail-adapter-command-code/tests/fixtures/command-code-1.65.0/`.
No prompt, login, install, host update, or downloaded-artifact execution.
