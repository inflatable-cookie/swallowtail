# Claude Agent Prepared Integration

The adapter exposes two explicit local Claude routes:
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

- `claude-agent.acp` for ACP structured runs, interactive sessions, and
  provider-session delete
- `claude-code.headless` for a smaller one-prompt `claude -p` structured run
  with no bridge dependency

Neither route is an implicit fallback for the other.

Both live in `swallowtail-adapter-claude-agent`:

| Route | Driver ID and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `claude-agent.acp` | `swallowtail.claude-agent.acp`; ACP v1 over stdio | structured runs or reusable sessions with model/reasoning configuration, plan mode, activity, usage, typed questions, optional one-shot permissions, load/resume, and delete | the application cannot package the ACP sidecar or needs the smaller subscription-only read-only path |
| `claude-code.headless` | `swallowtail.claude-code.headless`; Claude Code stream JSON over stdio | one read-only plan-mode prompt using local Claude subscription state | the application needs callbacks, writes, reusable sessions, management, or API-key billing |

The host supplies the approved executable, explicit environment, configured
instance and host identity, matching access evidence, and the task, process,
time, working-resource, credential, and attachment services required by the
selected plan. Swallowtail does not install either executable, perform login,
choose a model, select billing, search `PATH`, or infer workspace authority.

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
- optional `HarnessMode::Plan` in `SessionOptions`
- optional session-wide consumer-mediated one-shot permission exchange

Structured-run preparation requires:

- request identity
- caller-selected model route and model
- one prompt
- working-resource reference
- optional deadline
- optional reasoning effort
- optional consumer-mediated one-shot permission exchange

Swallowtail does not choose a model, account, credential, workspace, endpoint,
permission result, or fallback route.

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

It also applies optional `HarnessMode::Plan` through the advertised `mode`
config option and requires exact confirmation before readiness. Plan mode is
session setup: load and resume do not redeclare it, and changing mode requires
a new prepared session. It does not alter the read-only access policy or
permission handling.

These are caller selections, not provider discovery. The route exposes no
standalone model catalogue.

ACP form elicitation is separate from permission mediation. The exact
choice-and-Other subset projects into the common typed harness-user-input
callback and is answered exactly once through the turn or run callback
exchange. Richer forms and provider option previews are declined rather than
flattened. A question is not authorization to execute a provider tool.

### Run Drain Contract

After `start_run`, take the event stream and terminal outcome and poll them
concurrently. Do not await the terminal outcome while leaving the event stream
undrained. The stream is deliberately bounded and cannot discard semantic
output, reasoning, or tool-progress events; an undrained long agentic run
therefore fails with `swallowtail.event_buffer_rejected`. A consumer that does
not surface events must still drain them and may ignore them only after they
cross the runtime boundary.

Apply the same drain rule to session turns: poll events, callbacks when
present, and the terminal outcome concurrently. Cancellation stops only the
active operation. Always close and join the turn, session or run; retain
terminal, native provider close/delete, and local cleanup as separate truth.

### ACP Permission Exchange

The default prepared run or session rejects an unexpected permission once,
cancels the turn, and reports `ProviderRequestObserved`. It exposes no callback
exchange.

Call `with_consumer_mediated_permissions` on
`ClaudeAgentRunProfileInput` or `ClaudeAgentSessionProfileInput` to opt into
the exact `acp/session/request-permission` provider extension. A run exposes
the exchange through `RunHandle::take_callbacks`; each session turn exposes it
through `TurnHandle::take_callbacks`. Applications must drain callbacks
concurrently with events and the terminal outcome.

Session mediation is fixed by the prepared session plan and applies to open,
load, resume, and every turn. It is not a per-turn switch.

Each permission callback payload is bounded JSON containing `toolCall` and
`options`. Only provider-offered `allow_once` and `reject_once` options are
included. Respond with a success payload naming one offered option:

```json
{"optionId":"allow-once"}
```

A callback failure selects the offered one-shot rejection. Wrong-turn,
unknown, duplicate, persistent, and unoffered selections fail without being
sent. Response success confirms only ACP transport acceptance; the provider
tool and terminal turn remain independently observable.

The opt-in grants response transport, not approval authority. Figmatic or
another consumer must apply its product policy or ask its operator before
choosing an allow option. Swallowtail never chooses one.

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
assistant model to match the caller selection. Its fixed `HarnessMode::Plan`
posture is present in both operation policy and immutable preflight
capabilities. It currently qualifies exact Claude Code `2.1.220`; later stable
versions remain visible `UnverifiedNewer`.

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

Consumer-mediated runs and sessions additionally bind the exact permission
extension in their immutable plan. Operations without that namespace keep the
default reject-and-stop behavior.

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

## Continuation, Retention, And Delete

ACP sessions return exact resume and management bindings. `load_session`
returns bounded ordered ACP replay before readiness; `resume_session`
reattaches without replay. Neither operation may redeclare reasoning or plan
mode. Persist bindings only through their opaque export under the same
prepared plan.

`prepare_working_state_restoration` performs attachment recovery from an
existing binding. It restores a usable session but does not reconcile the
interrupted turn. There is no provider-session catalogue or import path.

Structured runs are durable by default: native ACP close preserves history.
`ClaudeAgentRunProfileInput::with_owned_session_cleanup()` instead binds a
temporary operation-private profile that closes and then deletes that exact
session, reporting provider completion and cleanup independently. Interactive
delete is separately prepared from the opaque inactive management binding;
it reports provider-data deletion, not secure erasure. Archive and restore are
unsupported.

Claude Code headless sets `--no-session-persistence`; closing joins only the
owned run process and exposes no binding or lifecycle authority.

## Failures, Unsupported Capabilities, And Promotion

Handle preparation and runtime failures through the portable classification,
while retaining the exact `swallowtail.claude_agent.*` or
`swallowtail.claude_code.*` diagnostic for support. Never parse stderr, ACP
payloads, permission display text, or Claude prose to infer retry, auth, or
success. Terminal and cleanup outcomes remain distinct.

Claude Agent ACP has no standalone model catalogue, attachments, structured
output, output-token limit, external search, archive, restore, or
provider-session import. Claude Code headless has no callbacks, writes,
consumer tools, durable state, continuation, or management. Provider tool,
plan, task, and child observations grant no control authority.

A new capability needs exact adapter and provider-version evidence, an
immutable prepared-plan mapping, bounded projection or callback semantics,
deterministic fixtures, and route-matrix coverage. An advertised ACP method or
Claude CLI option alone does not qualify it.

## Deterministic Validation And Optional Probes

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy check:examples
```

These compile and test the prepared paths without auth or prompts. The
repository-local sidecar bootstrap and managed probe above are separately
gated operator work; authenticated Claude prompts are not required for
deterministic acceptance.
