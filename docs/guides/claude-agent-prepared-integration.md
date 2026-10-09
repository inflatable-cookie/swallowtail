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
- optional consumer-supplied streamable-HTTP MCP placement

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

## Consumer-declared HTTP MCP

Contract 063 admits one consumer-supplied streamable-HTTP placement on
`claude-agent.acp`. Bind `ClaudeAgentAcpRemoteMcpPlacement` on
`ClaudeAgentSessionProfileInput`; the name is the route-owned
`swallowtail-claude-agent-acp`. A foreign name fails closed as a collision.
Omission keeps production `session/new` `mcpServers` byte-identical to today's
empty list. Load and resume still send `mcpServers: []`.

HTTP encoding emits ACP `{ type: "http", name, url, headers: [{name, value}] }`
with the consumer URL and headers verbatim. Structure is validated only:
non-empty name, absolute `http`/`https` URL, well-formed header names. URL and
header values never appear in failures, diagnostics, activity, receipts,
`Debug` output, or plan fingerprints: the public encoder returns
`ClaudeAgentAcpEncodedMcpServers`, whose `Debug` form redacts those values, and
the wire JSON stays crate-private. The provider also maps `stdio` and `sse`;
this route emits `http` only. `sse` stays modelled through
`ClaudeAgentAcpRemoteMcpPlacement::sse` and is refused. Research 351 records no
live result; Research 361 later accepted the exact `0.79.0` point.

The Contract 061 placement projection names `consumer-supplied-http` when an
HTTP entry is bound. Research 361 accepted live honouring of that HTTP entry
on exact `0.79.0`; Research 364 accepts exact `0.81.2` as well, so
`client_mcp_servers` is Yes on those two exact points. On `0.81.2`, the
persisted gate record confirms connection, tool listing, one successful tool
call, a `Completed` turn, and `Clean` cleanup (class `clean`, no code or
stage); the recorded model `claude-sonnet-4-6` is provenance. The first
`0.81.2` attempt was spent but its output record was lost when cargo captured
`eprintln`; its result is not reconstructed. The earlier exact `0.79.0`
attempt history remains in Research 352/355/361. Other window points stay
unqualified for honouring. stdio MCP live honouring is not that evidence.

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
standalone model catalogue. Projected `open_session_with_projection` publishes
exact negotiated model-options observation from the confirmed `model`
config option when that snapshot is bounded and well-formed. That row is
active-session observation only: not a catalogue, not selectable, and not
emitted from preserved `open_session`, prepared contribution, load, or resume.
Required missing `configOptions[id=model]` fails both public opens through
existing model confirmation. Snapshot-detail malformation that still confirms
`currentValue` closes the opened session and fails the projected path; the
preserved path still starts with no snapshot.

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
capabilities. It qualifies Claude Code `2.1.220..=2.1.286` on
`claude-code.headless.stream-json.v1` and
`2.1.287..=2.1.294` on private milestone `claude-code.headless.stream-json.v2`.
Both segments exclude unpublished `2.1.244`, `2.1.249`, `2.1.253` through
`2.1.256`, `2.1.262`, `2.1.264`, and `2.1.279`. Later stable versions remain
visible `UnverifiedNewer`; official `2.1.295` remains unverified after the
frozen `2.1.294` identity. Headless does not pass `--safe-mode` and already
admits ambient project instructions. The exact 2.1.294 npm wrapper, Darwin
arm64, and Linux x64 trees and every published hop after `2.1.281` remain
frozen in Research 421. Research 422 records exact-artifact hook-safety
analysis and provider-free adapter fixtures for the forked stream and denied
action cases.

The 2.1.287 stream may carry forked assistant frames with a
`parent_tool_use_id` and nullable `message.stop_reason`. A null reason is
projected as provider-unspecified activity, not a final answer; the bounded
output stream continues, and total usage is emitted from the terminal result.
Static inspection of both exact 2.1.294 platform executables shows that an
instruction-form prompt hook result of `ok: false` blocks, a rewritten
`PreToolUse` input passes through another safety and permission decision, and
a deny blocks the tool call. `Stop` and `SubagentStop` use their active-hook
state to avoid blocking their own reentry; the provider's configured block cap
remains in force. The route keeps `--permission-mode plan`, its `Read,Glob,Grep`
tool set, and user, project, and local settings. It does not disable hooks or
add a permission bypass. Research 422 records the static source review and
synthetic fake-process coverage; the fixtures are adapter tests, not captured
provider transcripts.

At the identity commit, npm `latest` and GitHub latest stable agreed on
`2.1.294`. GitHub stable `2.1.295` was published after that frozen identity;
the currentness ruling leaves `2.1.295` visible as `UnverifiedNewer` without
reopening this claim. SDK, ACP, response-only, and public lifecycle claims are
unchanged.

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
That set remains narrower than the route's qualified window. This task's
qualification through `2.1.294` does not extend the optional maximum-turn
feature; exact `2.1.282..=2.1.294` route points are not max-turn eligible under
Research 226. The feature still requires an exact probed version:

- published qualified points `2.1.242..=2.1.281` excluding unpublished
  `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`, `2.1.262`, `2.1.264`, and
  `2.1.279` were never probed for this feature
- the newly qualified `2.1.282..=2.1.294` points are not admitted by this
  exact-version feature gate; later stable points remain `UnverifiedNewer`
- no maximum-turn artifact was probed for any of those later points
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
on the route's explicit known-bad deny-list. Three supported segments share
claim id `claude-code.response-only.window-1`:

- `2.1.227` through `2.1.278` keep `claude-code.response-only.stream-json.v1`
  and remain supported as `Deprecated`.
- `2.1.280` through `2.1.281` use `claude-code.response-only.stream-json.v2`
  under the narrowed built-in-hook guarantee and remain supported as
  `Deprecated`.
- `2.1.282` through `2.1.293` use `claude-code.response-only.stream-json.v3`
  with the version-specific linked-instruction read limit; this is the
  maintained segment.

Unpublished `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`, `2.1.262`,
`2.1.264`, and `2.1.279` are denied. Later stable releases run provisionally as
`UnverifiedNewer` on the v3 behavior. The baseline, claim id, denied points,
and earlier supported segments stay in force. This is a distinct route. It
does not weaken or replace `claude-code.headless`.

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
qualification. The v1 segment ends at `2.1.278`, the v2 segment ends at
`2.1.281`, and v3 currently ends at `2.1.293`. The static deny-list is
unpublished `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`, `2.1.262`,
`2.1.264`, and `2.1.279`.

From `2.1.280`, `--safe-mode` still loads the provider's nine `@builtin`
plugins. Swallowtail guarantees what its arguments control: empty tools, empty
strict MCP, disabled slash commands and Chrome, no prompt suggestions, no
session persistence, one text-only turn. Provider built-in behaviour is
provider surface, disclosed here:

| Built-in | What it can do | What Swallowtail arguments block |
| --- | --- | --- |
| `sec-default` | Policy-only hooks can preserve managed instructions, tool policy, and settings | none; organization-seated policy is provider surface |
| `agents-md` | Reads ancestor `AGENTS.md` into `prompt.context`; `Read` can add nested instructions. Default off at `2.1.280`, on at `2.1.281`; from `2.1.282`, some symlink-linked project instructions may be omitted or require approval | empty `--tools` blocks the `Read` hook; `prompt.context` still runs, with the v3 read limit described below |
| `telemetry` | Session and engine analytics; can send first-party network requests under provider settings | none on the selected surface; `DISABLE_TELEMETRY`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, and `DO_NOT_TRACK` are proven in the frozen module but the approved environment is opaque |
| `plugin-authoring` | Invocable skill; no hook event | `--disable-slash-commands` and empty tools |
| `tips` | Session-start tip UI | none mapped onto stream-JSON |
| `mermaid` | Terminal `ui.render` rewrite | none mapped onto stream-JSON |
| `responsive-mode` | Rollout-gated prompt section/submit mutation | none; can change a turn when the provider gate is on |
| `diff` | Interactive diff UI, command, and git-state hooks | `--disable-slash-commands` and empty tools block command/tool paths |
| `claude-test` | Command/skill hooks that can launch test assets | `--disable-slash-commands` blocks invocation |

Project instruction discovery starts from the launch directory. A consumer
`Read` working resource supplies that child directory, so `AGENTS.md` joins
the existing `CLAUDE.md` discovery. Without one, v1 keeps the inherited
process cwd; v2 and v3 use an adapter-owned empty temporary working resource as
cwd. That directory is read-only and is not a filesystem containment boundary.
Frozen `instructionFiles` values can stop `agents-md` injection (`claude-md`)
or drop project/local/user kinds (`managed-only`); they are not pinned because
the `--settings` JSON path into plugin userConfig is not proven.

From `2.1.282`, upstream reduces which project instructions can be read through
repository symlinks. Its release note names traversal into macOS `/Network`,
`/.vol`-style paths, and `/home`; `2.1.284` changes external `.claude/rules`
symlinks to an approval path; and `2.1.290` blocks linked `CLAUDE.md`, rules,
and `AGENTS.md` outside working directories under its named read policies.
The v3 route does not guarantee that every symlink-linked instruction reaches
`prompt.context`. No adapter operation restores those reads or broadens
filesystem authority. The published runtime is opaque and no internal
resolver function is claimed (Research 378).

The prepared plan records `ProviderSuppressed` harness configuration and
`AmbientHost` isolation. The first says exact provider flags suppress tools
and MCP configuration. The second says those flags are not an OS sandbox.
A consumer working resource is optional project location only. The v2
adapter-owned empty directory is host-service materialization, not consumer
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

The root `package.json` keeps `@agentclientprotocol/claude-agent-acp` pinned to
exact `0.81.2` for development and live probes. Research 373 separately
qualifies the wrapper claim through `0.87.0`; the local dependency pin is not
currentness or live MCP evidence and remains unchanged. Development and live
probes use:

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
wrappers are `0.53.0..=0.87.0`, excluding unpublished `0.58.0` (Research 373).
Those milestones remain guaranteed. A newer stable release is admitted as
unverified, remains inspectable in evidence, and must identify itself as that
same exact version during ACP initialization. Excluded and older versions do
not prepare. HTTP MCP live honouring remains exact to `0.79.0` and `0.81.2`.

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
