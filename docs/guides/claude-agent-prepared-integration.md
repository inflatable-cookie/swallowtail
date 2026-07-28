# Claude Agent Prepared Integration

The adapter exposes two explicit local Claude routes:

- `claude-agent.acp` for ACP structured runs, interactive sessions, and
  provider-session delete
- `claude-code.headless` for a smaller one-prompt `claude -p` structured run
  with no bridge dependency

Neither route is an implicit fallback for the other.

## Claude Agent ACP Inputs

Preparation requires:

- configured-instance identity and revision
- execution host and approved executable target
- selected process environment
- one explicit Anthropic access profile and matching evidence:
  - `ApiKey`, `PayAsYouGo`, one credential reference, and `Ready`
  - `LocalUnauthenticated`, `SubscriptionAllowance`, no credential reference,
    and `NotRequired`
- probe deadline and cancellation

Session preparation requires:

- request identity
- caller-selected model route and model
- working-resource reference
- optional reasoning effort in `SessionOptions`

Structured-run preparation requires:

- request identity
- caller-selected model route and model
- one prompt
- working-resource reference
- optional deadline
- optional reasoning effort

Swallowtail does not choose a model, account, credential, workspace, endpoint,
or fallback route.

Local subscription access means the approved ACP process inherits the selected
environment and uses authentication already held by the local Claude
installation. Swallowtail does not read, copy, lease, or reclassify that
session as an API credential. API-key access remains a separate profile and
still acquires one scope- and audience-bound secret lease.

From adapter `0.54.0`, Swallowtail applies the caller-selected model through
ACP `session/set_config_option` after session creation and requires the
returned model value to match the preflight route. It does not accept the
bridge's initial `default` value as evidence that the requested model is
active. `0.53.0` retains its qualified legacy session-creation binding.

The same config exchange applies optional reasoning effort after model
selection. Swallowtail accepts only a value advertised by that session's
model-specific `effort` option and requires exact confirmation before the
first prompt. Supported provider values are `default`, `low`, `medium`,
`high`, `xhigh`, and `max`; a given model may advertise only a subset.

These are caller selections, not provider discovery. The route exposes no
standalone model catalogue.

## Claude Code Headless

`prepare_claude_code_headless` discovers one host-approved `claude`
executable, then binds a provider-supported local subscription profile. This
route accepts no API credential or pay-as-you-go profile.

The driver writes the prompt to stdin and invokes:

```text
claude -p
  --input-format text
  --output-format stream-json
  --verbose
  --no-session-persistence
  --model <caller-selected-model>
  [--effort <caller-selected-effort>]
  --permission-mode plan
  --tools Read,Glob,Grep
  --setting-sources user,project,local
  --mcp-config {"mcpServers":{}}
  --strict-mcp-config
```

The selected process environment must preserve the local Claude login. For a
local macOS host this normally includes `HOME`, because Claude Code reads OAuth
state through the user's keychain. An alternate Claude profile may also need
`CLAUDE_CONFIG_DIR`. Do not select `--bare`: current Claude Code disables OAuth
and keychain reads in that mode. Excluding `ANTHROPIC_API_KEY` from the approved
environment keeps this route subscription-only.

The headless route is read-only, disables session persistence, emits bounded
stream-JSON output and usage, supports `default`, `low`, `medium`, `high`,
`xhigh`, and `max` reasoning selections, and requires the initialized and
assistant model to match the caller selection. It currently qualifies exact
Claude Code `2.1.220`; later stable versions remain visible
`UnverifiedNewer`.

See the compile-tested
[`prepared_claude_code_headless` example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_headless.rs).

## Repo-Local ACP Sidecar

Swallowtail pins `@agentclientprotocol/claude-agent-acp` in the root
`package.json`. Development and live probes use:

```sh
effigy bootstrap:claude-agent-acp
effigy probe:claude-agent-acp-managed
```

The approved executable target is then the repository-local
`./node_modules/.bin/claude-agent-acp`, not a global installation. This removes
the global-package requirement but still requires Node 22 or later.

The dependency is application-owned. A Rust library cannot carry its checkout's
`node_modules` into a downstream executable, so Figmatic or another packaged
consumer must pin the same package in its own application package and resolve
its own local `.bin` target. This follows the application-level dependency
posture used by [T3 Code](https://github.com/pingdotgg/t3code) for its Claude
integration.

## ACP Version Posture

Discovery records the exact Claude Agent ACP wrapper version. Qualified
milestones remain guaranteed. A newer stable release is admitted as
unverified, remains inspectable in evidence, and must identify itself as that
same exact version during ACP initialization. Excluded and older versions do
not prepare.

ACP `available_commands_update`, `config_option_update`, and
`current_mode_update` metadata is accepted whether it arrives between session
creation and the first turn or during a prompt. These session-scoped updates
do not become consumer tools, commands, or turn output. Any other
`session/update` without an active turn remains a protocol failure.

The Claude Agent route accepts one ACP receive frame up to 4 MiB and keeps at
most 8 MiB in its receive decoder. This adapter-specific bound admits bridge
tool-result updates that echo file content. The shared ACP default remains 64
KiB per frame with a 256 KiB buffer; Gemini and Kimi retain that default.

## ACP Execution Boundary

Both prepared plans bind:

- `acp-v1-stdio`
- ambient harness configuration
- `AmbientHost` isolation
- caller-selected model route

The session plan binds ambient read-only workspace access and exposes only
`Read`, `Glob`, and `Grep`. The structured-run plan instead binds ambient
read-write access, resolves a matching `ReadWrite` filesystem lease, exposes
`Read`, `Glob`, `Grep`, `Edit`, and `Write`, and selects the advertised
`acceptEdits` mode before its one prompt. It does not enable shell or broader
provider tools.

Local subscription plans omit the credential host service. API-key plans
require it. Both retain the exact `api.anthropic.com` audience and access
evidence.

Interactive sessions prohibit a reusable provider-state binding.
`ClaudeAgentPreparedRun` explicitly accepts durable transcript retention
because native ACP close preserves history. The run creates one
operation-private session, executes one prompt, closes natively at qualified
versions, joins process, resource, optional credential, turn, and deadline
work, and exposes no reusable session or management binding. Close is not
deletion.

Ambient execution is not sandbox containment. The resolved working resource
selects the working directory and lease authority; it does not prevent the
harness from reaching other paths allowed to the execution-host user. The
facade does not silently select remote ACP, HTTP, or another transport. Remote
ACP composition is a separate explicit route.

`ClaudeAgentPreparedRun::start_run` and
`ClaudeAgentPreparedSession::open_session` execute the bound operations.
`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for inspection and advanced use.

See the compile-tested
[`prepared_claude_agent_acp` example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_acp.rs).
