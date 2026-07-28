# Claude Agent Prepared Integration

Use the prepared facade for the normal Claude Agent ACP path. It discovers one
host-approved executable and derives the configured instance, exact version
binding, preflight plan, ambient read-only access agreement, and typed
structured-run or session operation.

## Explicit Inputs

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

## Version Posture

Discovery records the exact Claude Agent ACP wrapper version. Qualified
milestones remain guaranteed. A newer stable release is admitted as
unverified, remains inspectable in evidence, and must identify itself as that
same exact version during ACP initialization. Excluded and older versions do
not prepare.

ACP `available_commands_update` metadata is accepted whether it arrives
immediately after session creation or during a prompt. It does not become a
consumer tool or command capability.

## Execution Boundary

Both prepared plans bind:

- `acp-v1-stdio`
- ambient harness configuration
- `AmbientHost` isolation
- ambient read-only workspace access
- caller-selected model route

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

Ambient execution is not sandbox containment. The facade does not silently
select remote ACP, HTTP, or another transport. Remote ACP composition is a
separate explicit route.

`ClaudeAgentPreparedRun::start_run` and
`ClaudeAgentPreparedSession::open_session` execute the bound operations.
`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for inspection and advanced use.

See the compile-tested
[`prepared_claude_agent_acp` example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_acp.rs).
