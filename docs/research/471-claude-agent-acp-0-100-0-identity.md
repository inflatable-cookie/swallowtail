# Research 471: Claude Agent ACP 0.100.0 Identity and Currentness

Evidence checked 2026-11-03 for `claude-agent.acp` only. Claude Code and
`claude-agent.sdk` are separate families and are unchanged here. This record
covers the final hop from `0.99.1` (Research 470) to the official current
point, and states the decision the three hop records apply together.

## Official current point

Re-probed at the start of the run. npm `latest` is `0.100.0`. npm `preview` is
`0.100.1-preview.1` and is ignored. GitHub release `v0.100.0` is not a draft
or prerelease, published 2026-11-02T14:34:15Z at
`981bf9a28ac3afef26e7877a2a638814c34f1679`. ACP registry id `claude-acp`
reports `0.100.0`. npm `gitHead` matches that tag. The exact npm integrity is
`sha512-X+f2M5RiehzSzWRJjDSf0Sbzoc8JK66l85YnYW5TAdHJX53CLQV00v2M72bfCKJ4IZ11qt+WQ/Yg9NVKcuDAhQ==`;
the downloaded tarball SHA-256 is
`bb1de8a11cb585a4e8da2fd1a0252aa613ac9cd2cdc47a23247c9847c6d132b8`.
The package pins ACP SDK `2.4.0` and Claude Agent SDK `0.3.316` — beyond the
`2.1.0` / `0.3.312` the dispatch recorded for `0.99.0` — requires Node
`>=22`, and exposes `dist/index.js` as its binary entry. The canonical
repository is `agentclientprotocol/claude-agent-acp`, the same repository
Research 460 used; the `zed-industries/claude-agent-acp` name in the dispatch
resolves to it as a redirect. Host `claude-agent-acp` was absent from `PATH`
in the observation environment; that observation belongs to that environment
and is not a claim of universal absence.

## Published stable hops

The complete published stable set from the previous qualified ceiling `0.98.0`
through current `0.100.0` is `0.99.0`, `0.99.1`, and `0.100.0`
(Research 469 and 470 carry the first two identities). `0.99.1` to `0.100.0`
adds 9 files (`dist/background-work.*`, `dist/startup-failure.*`,
`dist/v2/background.*`), removes none, and changes 53. 235 files are
unchanged, including `dist/index.js`, `dist/serve.js`, and `dist/tools.js`,
byte-identical from `0.96.0` through `0.100.0`. `deleteSessionIfStored` is
byte-identical through `0.100.0`. Preview-only patch lines in the window are
`0.98.1` and `0.99.2`; the `0.100.1-preview.1` tag is ignored. The historical
`0.52.0` and unpublished `0.58.0` exclusions remain.

The retained fixture records every integrity value, tarball SHA-256, package
dependency identity, and complete file tree in
[`claude-agent-acp-0.100.0`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.100.0/README.md).

## Selected ACP v1 review

- The v1 agent is constructed with no options: `v2` false and no background
  sink. The new background-work reporter, its `paused` tool status, the
  background terminal and tool-call cancellation methods, and resume-time
  background tool-call publication are reachable only from the v2 agent, so a
  protocolVersion 1 client never receives them. `withRunningToolCall` is a
  no-op because `registerBackgroundToolCalls` returns an empty set without the
  background option.
- `session/prompt` now validates content block types before queueing.
  Swallowtail sends text blocks only. The adapter also keeps one active turn
  per session, so the queued-prompt cancellation refinement
  (`cancelledThrough`) is unreachable for this client.
- `session/set_config_option` gains a value-type check (boolean options take
  booleans, select options take strings). Swallowtail's model, effort, and
  mode selections are strings on select options and pass. Config and mode
  changes serialize per session; the handlers, validation, and results stay.
- `session/new` and load-path `cwd` validation extends to
  `additionalDirectories` entries. Swallowtail sends none. The `session/list`
  worktree change and the `authenticate` unknown-method error are on routes
  Swallowtail does not select.
- Provider restarts and sign-out respawns now serialize per session and send a
  `config_option_update` after a query restart; both live on the provider auth
  surface Swallowtail does not exercise. `config_option_update` is an
  already-decoded session-scoped kind.
- The clear-context path after an approved ExitPlanMode gains a `usage_update`
  send beside the existing mode and config updates. `usage_update` was already
  sent at four sites in `0.98.0` and is decoded; the kind set stays.
- `available_commands_update` drops command names with whitespace and
  duplicates; the filter only removes entries.
- Startup failures (`CLAUDE_CODE_STARTUP_FAILURE_RESULTS=1`) and fork
  unknown-session errors map into typed protocol error codes; the adapter
  maps every JSON-RPC error to one rejected-request failure class.
- `session/delete` changes only its AIR archive-instead-of-delete path; the
  non-AIR `deleteSessionIfStored` body stays byte-identical and still returns
  an empty success for an absent transcript.
- Elicitation: client-facing `elicitation/create` request and response shapes
  stay. The accept-content passthrough and scalar pick filtering affect what
  the MCP server or the SDK's ask-user-question tool receives, behind the MCP
  bridge — not the ACP wire to Swallowtail.

## Decision and limits

This is a compatible extension of `claude-agent.acp.initialize-meta-extensions-v7`.
Applied together with Research 469 and 470 in one claim revision: extend the
maintained segment `0.66.0..=0.96.0` → `0.66.0..=0.100.0` through `0.100.0`.
Keep baseline `0.53.0`, claim id `claude-agent.acp.window-2`, the seven
behavior milestones, exclusions `0.52.0` and `0.58.0`, and `AllowUnverified`.
Until the claim revision lands, `0.98.0` stays the qualified ceiling and the
hops stay `UnverifiedNewer`. `0.100.1` is the synthetic `UnverifiedNewer`
point after the claim revision. Prerelease points remain incompatible without
their own exact segment.

No new public mapped operation was added. HTTP MCP live honouring remains
exact to `0.79.0` and `0.81.2` (Research 361 and 364). The root package pin
stays `0.81.2`. The pinned ACP SDK `2.4.0` and Agent SDK `0.3.316` are
recorded dependency identity on separate axes; the shared ACP schema and SDK
claims are separate families and are unchanged. No tag, publication, host
install, package install, provider prompt, live initialize, credential use, or
host mutation occurred.

Sources: [official npm package 0.100.0](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.100.0),
[official GitHub tag v0.100.0](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.100.0),
and the [ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json).
