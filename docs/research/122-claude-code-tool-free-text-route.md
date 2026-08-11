# 122 Claude Code Tool-Free Text Route

Status: promoted
Owner: Tom
Date: 2026-08-11

## Question

Can exact Claude Code `2.1.227` provide one bounded plain-text response through
local subscription authentication with no model-visible tools, MCP servers,
working resource, retained session, continuation, callback, fallback, or
write authority?

This revises Research 121. Provider-enforced structured output is no longer a
consumer requirement. JSON-looking output remains ordinary untrusted text;
Figmatic owns extraction, deserialization, validation, compilation, gates, and
acceptance downstream.

## Exact Invocation

The live proof ran the installed payload
`/Users/tom/.local/share/claude/versions/2.1.227` from a fresh empty launch
directory with:

```text
-p
--input-format text
--output-format stream-json
--verbose
--model sonnet
--effort low
--tools ""
--safe-mode
--disable-slash-commands
--no-chrome
--prompt-suggestions false
--mcp-config {"mcpServers":{}}
--strict-mcp-config
--no-session-persistence
```

No `--json-schema`, fallback model, resume, continue, session id, working
resource, attachment, callback, or prompt argument was supplied. The prompt
was written once to stdin and stdin closed.

## Live Evidence

Authentication remained provider-supported local subscription access:

- `authMethod=claude.ai`
- `subscriptionType=max`
- `ANTHROPIC_API_KEY` absent

The prepared-facade probe cleared ambient environment and approved only
`HOME`, `USER`, and `LOGNAME`. `HOME` alone failed with the provider's
not-logged-in result; adding the two user identity variables restored the
same `claude.ai` Max subscription. The route therefore preserves those three
host-selected values and still excludes the API key.

The exact stream reported:

- one init
- `tools=[]`
- `mcp_servers=[]`
- one assistant message containing one text block
- exact initialized and assistant model `claude-sonnet-5`
- assistant `stop_reason=null` in the successful response-only envelope
- zero user messages
- zero tool-use blocks
- one terminal result
- `num_turns=1`
- `structured_output=null`
- the requested JSON-shaped bytes in ordinary `result`
- one private operation session id, with persistence disabled and no public
  binding

The process exited `0`. The fresh launch directory remained empty. Separate
Research 121 termination evidence for the same executable and safe invocation
family records `SIGTERM` exit `143`, no remaining child, and no artifact.
Swallowtail still owns the monotonic deadline, cancellation request, force
stop, wait, terminal truth, and joined task cleanup.

The separately gated prepared-facade probe then completed
`CLAUDE_RESPONSE_ONLY_LIVE_OK`, reported clean terminal and handle cleanup,
and left the repository source-status snapshot unchanged.

`num_turns=1`, the single assistant record, absence of user/tool records, and
the terminal result are the observable no-continuation and no-semantic-retry
boundary. The adapter can reject any second assistant, user, tool, or terminal
turn. Unobservable provider transport recovery is not promoted as a
Swallowtail attempt or retry capability.

## Contract Fit

The operation remains `HarnessInteraction` plus `StructuredRun`. Structured
run names the bounded one-turn runtime shape; it does not claim schema
enforcement.

The new route binds:

- plain `OperationContent` input and output
- `StructuredRun`, `StreamingEvents`, `UsageReporting`, and structured-run
  interruption capabilities
- optional exact qualified reasoning effort
- `ProviderRetentionPolicy::Prohibited`
- `HarnessConfigurationPosture::ProviderSuppressed` from exact `--safe-mode`
  evidence
- `HarnessIsolation::AmbientHost`; disabled tools do not sandbox the process
- Task, Process, and Time host services only
- no `WorkingResource`, `StructuredOutput`, tool, callback, attachment,
  output-token, session, recovery, or reattachment capability

The inherited launch directory is process context, not a working-resource
grant. The model receives no filesystem tool or callback surface. The harness
process still has ambient host authority for its own executable, OAuth/keychain
access, and provider operation, so the route does not claim provider or host
sandboxing.

## Identity Decision

Add a distinct `claude-code.response-only` driver identity in the existing
Claude adapter package.

The existing `claude-code.headless` route is exact `2.1.220`, ambient
configuration, Plan mode, read tools, and a read-only filesystem working
resource. Contract 033 binds one configuration posture per configured route.
The response-only route instead qualifies exact `2.1.227`, provider-suppressed
configuration, default permission mode, empty tools, and no working resource.
Those authority and compatibility differences are not an operation option on
the old configured instance.

Both drivers share the Claude Code family and stream-JSON transport. Shared
process, parsing, and lifecycle helpers may be reused without merging their
descriptors, compatibility axes, policies, validation, or prepared APIs.

## Failure And Output Truth

- JSON-shaped text is not validated or labelled structured output
- empty, missing, duplicate, tool-bearing, user-bearing, multi-turn,
  mismatched-model, mismatched-session, malformed, oversized, or post-terminal
  streams fail safely
- provider failure, process failure, cancellation, deadline, event-delivery,
  and cleanup outcomes remain distinct
- stable diagnostics expose no prompt, output, raw payload, path, session id,
  account data, or environment value
- no failure authorizes retry, fallback, continuation, or route change

## Decision

Proceed with a distinct exact-`2.1.227` response-only route, deterministic
corpus, prepared API, safe live probe, guide, and consumer example. Do not
change `claude-code.headless` or claim `StructuredOutput`.
