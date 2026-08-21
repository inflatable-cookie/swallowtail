# Gemini CLI headless `stream-json` corpus

Historical decoder specimens: `0.51.0` and `0.52.0`. The current headless
claim is qualified through `0.56.0`; the specimen corpus remains unchanged.

Source tags:

- `v0.51.0`: `8d951de3855750d5f8219d65ae22524b606133b6`
- `v0.52.0`: `d14583b926769bd98f807cdc6b1ca50e91ae26ec`

The qualified files are byte-identical at both tags:

- `packages/cli/src/config/config.ts`: `139ab5d0f1097ef57454991360ab4bda84370d5b`
- `packages/cli/src/nonInteractiveCli.ts`: `00b48be9540b0a661825c36227b67a427d0f4dfc`
- `packages/core/src/output/types.ts`: `1d4159a1f8e545ec2da2b8df413cb6778b3f79dc`
- `packages/core/src/output/stream-json-formatter.ts`: `6475e6d482239dfedad965f078e34bc23d7af0b7`

The fixtures preserve the published event shapes. Timestamps, model output,
tool values, and session IDs are deterministic test substitutions.
`<SESSION_ID>` is replaced with the exact driver-selected session ID before
execution.

Qualified command posture:

- prompt over piped stdin
- `--output-format stream-json`
- explicit model and session ID
- `--approval-mode plan`
- `--extensions none`
- empty `--allowed-mcp-server-names`
- `--skip-trust` for the already-authorized working resource
- no forced sandbox flag

The source defines exit codes `41`, `42`, `44`, `52`, `53`, `54`, `55`, and
`130` for authentication, input, sandbox, configuration, turn-limit, tool,
trust, and cancellation failures. Generic non-zero exits remain provider
failures.

`retention.json` now freezes why the separate stored-transcript delete role is
unsupported. The four selected source files are byte-identical at both
qualified tags. Delete-process exit and success text are not authoritative:
the provider can exit zero after rejection, and its storage helper catches
some unlink errors. Exact `sessions.ts` also calls summary generation before
`--list-sessions`; listing may issue a provider request and append transcript
metadata while exposing no exact terminal record.

Swallowtail therefore emits no durable-run management binding and performs no
stateful list confirmation. The opt-in operation-owned cleanup profile may
still issue its one exact delete attempt, but reports deletion as unconfirmed
and degraded. Provider output may include the first user message and never
enters stable diagnostics.
