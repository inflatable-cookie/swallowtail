# Claude Agent Prepared Integration

Use the prepared facade for the normal Claude Agent ACP path. It discovers one
host-approved executable and derives the configured instance, exact version
binding, preflight plan, ambient read-only access agreement, and open-session
request.

## Explicit Inputs

Preparation requires:

- configured-instance identity and revision
- execution host and approved executable target
- selected process environment
- maintainer-supported Anthropic public API-key profile and access evidence
- probe deadline and cancellation

Session preparation requires:

- request identity
- caller-selected model route and model
- working-resource reference
- empty `SessionOptions`

Swallowtail does not choose a model, account, credential, workspace, endpoint,
or fallback route.

Claude Agent's ACP model selector is constrained by the caller-supplied
`availableModels` set. It is caller configuration, not provider discovery, so
this route exposes no catalogue or negotiated-provider model list.

## Version Posture

Discovery records the exact Claude Agent ACP wrapper version. Qualified
milestones remain guaranteed. A newer stable release is admitted as
unverified, remains inspectable in evidence, and must identify itself as that
same exact version during ACP initialization. Excluded and older versions do
not prepare.

## Execution Boundary

The prepared plan binds:

- `acp-v1-stdio`
- ambient harness configuration
- `AmbientHost` isolation
- ambient read-only workspace access
- provider-owned durable state prohibited
- caller-selected model route

Ambient execution is not sandbox containment. The facade does not silently
select remote ACP, HTTP, or another transport. Remote ACP composition is a
separate explicit route.

`ClaudeAgentPreparedSession::open_session` executes the bound operation.
`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for inspection and advanced use.

See the compile-tested
[`prepared_claude_agent_acp` example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_acp.rs).
