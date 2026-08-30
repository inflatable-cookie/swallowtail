# Claude Agent Prepared Integration

The adapter exposes three explicit local Claude routes:
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

- `claude-agent.acp` for ACP structured runs, interactive sessions, and
  provider-session delete
- `claude-code.headless` for a smaller one-prompt `claude -p` structured run
  with no bridge dependency
- `claude-code.response-only` for one tool-free text response with no
  working-resource authority

Neither route is an implicit fallback for the other.

At live-proven Claude Code `2.1.228`, medium-effort response-only runs may emit
validated cumulative thinking-token estimates. Swallowtail projects them only
as content-free coalescible progress snapshots. The related empty private
thinking block and opaque signature are validated and discarded; neither is
readable reasoning, usage, or output. Unknown system and assistant shapes
still fail closed.

Both live in `swallowtail-adapter-claude-agent`:

| Route | Driver ID and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `claude-agent.acp` | `swallowtail.claude-agent.acp`; ACP v1 over stdio | structured runs or reusable sessions with model/reasoning configuration, plan mode, activity, usage, typed questions, optional one-shot permissions, load/resume, and delete | the application cannot package the ACP sidecar or needs the smaller subscription-only read-only path |
| `claude-code.headless` | `swallowtail.claude-code.headless`; Claude Code stream JSON over stdio | one read-only plan-mode prompt using local Claude subscription state | the application needs callbacks, writes, reusable sessions, management, or API-key billing |
| `claude-code.response-only` | `swallowtail.claude-code.response-only`; strict Claude Code stream JSON over stdio | one bounded assistant text response with no tools, MCP, session persistence, or working resource | the application needs schema enforcement, callbacks, filesystem authority, continuation, retry, fallback, or API-key billing |

The admitted record supplies opaque binary-path and environment refs. The host
resolves them for preparation, then supplies matching access evidence and the
task, process, time, working-resource, credential, and attachment services
required by the selected plan. Swallowtail does not install either executable,
perform login, choose a model, select billing, search `PATH`, or infer
workspace authority.

## Add The ACP Connection

Only `claude-agent.acp` currently exports an addable descriptor.
`claude-code.headless` and `claude-code.response-only` stay on the prepared
facade path below. Topology is **installed**. It is not `ExecutionLayer`.
Follow [connection lifecycle](connection-lifecycle.md) before
`prepare_claude_agent`.

1. Assemble `AddableRouteCatalog` from
   `claude_agent_acp_addable_route_descriptor`. The row is `Available` when
   the host exposes the Process service; otherwise
   `Unavailable(HostService)`. Discovery of the executable stays Contract
   008 on the selected driver.
2. `admit_instance` writes the configured instance with opaque config refs
   for `binary_path` and `environment`. Admission does not prepare.
3. There is no credential field. Local Claude subscription is inherited
   process login state. Swallowtail does not extract keychain bytes, open a
   URL, or run hosted OAuth. API-key billing stays a separate explicit
   profile, not this addable row.
4. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement stays
   independent of 047 `Ready` / `NotReady`.
5. `observe_authenticated_subject` is `Absent`. Do not scrape Claude account
   email.
6. `observe_instance_update` reuses `claude_agent_acp_claim` and optional
   Contract 032 installed-executable observation.
7. Session-negotiated ACP model rows omit `provider_id` and are not 047
   catalogue rows. Overlay keys instance plus model when a 047 catalogue
   row exists without `provider_id`. Do not invent a catalogue provider
   id. This addable snapshot has no catalogue, so overlay stays empty.
8. Build `ClaudeAgentPreparationInput::from_admitted` from the admitted record,
   then call `prepare_claude_agent`. The constructor selects the stored
   `binary_path` and `environment` refs; the host resolves them during Contract
   008 discovery.

The compile-tested
[`connection_lifecycle` example](../../crates/swallowtail-adapter-claude-agent/examples/connection_lifecycle.rs)
shows catalog through prepare for `claude-agent.acp`. The canonical
route-map example remains
[`prepared_claude_agent_acp`](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_acp.rs).

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
  [--max-turns <caller-selected-positive-integer>]
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
capabilities. It currently qualifies Claude Code `2.1.220` through `2.1.251`,
excluding unpublished `2.1.244` and `2.1.249`; later stable versions remain
visible `UnverifiedNewer`.

### Maximum Agentic Turns

`ClaudeCodeRunProfileInput::with_maximum_turns` selects one
`ClaudeCodeMaximumTurns` bound on agentic turns. This is adapter-local Claude
Code configuration. It is not a portable agent budget, an output-token limit,
a tool-call budget, a cost cap, a wall-time deadline, a context bound, or a
retry count, and it does not change `claude-code.response-only` or
`claude-agent.acp`.

One counted turn is one tool-use round trip. A final text-only response is not
counted. Research 226 proved this from the exact agent loop across every
published version in `2.1.220..=2.1.241`.

`ClaudeCodeMaximumTurns::from_u64` admits positive 32-bit integers and rejects
zero and overflow before preparation. That is deliberate: the native parser
coerces the argument with `Number` and rejects only `NaN`, so zero, negatives,
fractions, `Infinity`, exponent and hexadecimal forms, grouped digits, and the
empty string all pass Claude Code's own parsing. The native loop then guards
with a truthiness test, under which a resolved `0` disables enforcement
entirely and a negative value stops after the first tool-use turn. Only a
positive integer produces the documented bound, so only a positive integer is
selectable here.

A selection requires one of the exact Claude Code versions Research 226 probed.
That set is narrower than the route's qualified window in two ways, and both
matter:

- the compatibility claim permits later stable points as `UnverifiedNewer`, and
  no artifact for one has been probed
- the claim's segment is a semantic range that contains `2.1.230`, which was
  never published to npm, so no artifact for it exists either

Preparation fails with
`swallowtail.claude_code.headless.preparation.maximum_turns_unqualified` on any
version outside the probed set. Omission still runs on every version the route
otherwise permits.

`ClaudeCodePreparedRun::start_run` is the only surface that dispatches a bound.
`ClaudeCodePreparedRun::low_level_driver` deliberately returns an **unbound**
driver even when `maximum_turns()` is `Some`, and there is no public way to
attach a bound to a `ClaudeCodeHeadlessDriver` you built yourself.

That is deliberate rather than an omission. A bound is execution state that
only means anything alongside the exact plan and request it was prepared with,
and neither `PreflightPlan` nor `StructuredRunRequest` records one. If an
extracted driver carried a bound, a caller could hand it another prepared run's
plan and silently dispatch the wrong value — or dispatch a bound onto a run
that deliberately omitted one. Keeping the bound and its `(plan, request)` pair
together in a single path means they cannot disagree, so no comparison is
needed. Everything else about the extracted driver is unchanged; it is still
the low-level seam for callers who drive the route themselves.

Selection separates seven states that must not be conflated: requested,
prepared, dispatched, parser-accepted, natively enforced, reached, and
observed. Swallowtail proves dispatch and rejects unqualified rows; it does not
claim how many turns a given prompt will actually use.

Distinguish two things about omission:

- Omitting the selection emits no `--max-turns` argument and preserves the
  exact command and approved-environment handoff above.
- Omission is **not** a claim of unlimited execution. With the flag absent,
  `CLAUDE_CODE_MAX_TURNS` from the approved environment is authoritative on the
  host: a valid positive integer silently caps the run, and an invalid value
  aborts Claude Code at startup with exit `1` before any stream appears.
  Swallowtail does not inspect, clear, or rewrite that environment. Selecting a
  value removes the ambiguity, because explicit argv unconditionally overrides
  the environment equivalent.

When the native bound is reached, Claude Code emits one `error_max_turns`
result carrying `is_error`, `num_turns`, `stop_reason`, `usage`, and a
`Reached maximum number of turns (N)` message, with no `result` field, and the
process exits `1`. Swallowtail reports that as
`TerminalStatus::ProviderFailed` with
`swallowtail.claude_code.headless.provider_failed`, `FailureOrigin::Provider`,
no output, the usage observation still emitted, and unchanged joined cleanup.
Reaching the bound is never mapped to completion. The terminal diagnostic does
not distinguish it from other provider failure subtypes; read the exact subtype
from the stream when that distinction matters.

See the compile-tested
[`prepared_claude_code_headless` example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_headless.rs).

## Claude Code Response Only

`prepare_claude_code_response_only` accepts a host-approved stable Claude Code
executable at or above the proven `2.1.227` protocol floor, except any release
on the route's explicit known-bad deny-list. `2.1.227` through `2.1.251` are
qualified except unpublished `2.1.244` and `2.1.249`; later stable releases run
provisionally as `UnverifiedNewer` under the same runtime validation. It is a
distinct route. It does not weaken or replace `claude-code.headless`.

`ClaudeCodeResponseProfileInput::new` accepts only request identity, an exact
caller-selected model route, one prompt, and a deadline. Optional qualified
reasoning may be added with `with_reasoning_mode`. The profile has no working
resource, attachment, tool, callback, schema, output-token, session,
continuation, retry, or fallback input.

The driver writes the prompt to stdin and invokes:

```text
claude -p
  --input-format text
  --output-format stream-json
  --verbose
  --no-session-persistence
  --model <caller-selected-model>
  [--effort <caller-selected-effort>]
  --tools ""
  --safe-mode
  --disable-slash-commands
  --no-chrome
  --prompt-suggestions false
  --mcp-config {"mcpServers":{}}
  --strict-mcp-config
```

Every accepted version must emit an init event whose executable version equals
the version observed during preparation, with empty `tools` and `mcp_servers`,
one text-only assistant message, and one matching success result with
`num_turns: 1` and null or absent `structured_output`. Any tool, user, extra
assistant, second result, version/model drift, non-text block, malformed or
non-cumulative thinking estimate, usage mismatch, missing terminal frame, or
post-terminal event fails closed. The route emits exactly one matching bounded
text as ordinary `OperationContent`; JSON-shaped text carries no JSON or schema
claim.

Preparation and run-start debug observations expose the exact executable
version and its `Qualified` or `UnverifiedNewer` posture. Prepared evidence
also remains version-bound. There is no patch range that silently confers
qualification: the qualified segment ends at `2.1.251`, while newer stable
versions are provisional until evidence moves that boundary. The static
deny-list is unpublished `2.1.244` and `2.1.249`.

The prepared plan records `ProviderSuppressed` harness configuration and
`AmbientHost` isolation. The first says exact provider flags suppress tools
and MCP configuration. The second says those flags are not an OS sandbox.
No working resource is passed to the process. The execution host still needs
a launch directory, but that host process detail creates no portable
filesystem authority.

Local macOS proof required the approved environment to preserve `HOME`,
`USER`, and `LOGNAME` for Claude's OAuth/keychain lookup. It excludes
`ANTHROPIC_API_KEY`; the live auth surface reported `claude.ai` and `max`.
Do not clear required subscription state or widen the approved environment
without new evidence.

The route exposes one completion-only assistant activity and one terminal
text result. Consumers must drain events and the terminal outcome
concurrently, close the handle, and validate or parse the text themselves.
See the compile-tested
[`prepared_claude_code_response_only` example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_code_response_only.rs).

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
wrappers are `0.53.0..=0.70.0`, excluding unpublished `0.58.0`. Those
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

Both Claude Code routes set `--no-session-persistence`; closing joins only the
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
consumer tools, durable state, continuation, or management. Response only
also has no working resource or structured-output capability. Provider tool,
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

The response-only authenticated probe is separately gated:

```sh
effigy probe:claude-code-response-only
```
