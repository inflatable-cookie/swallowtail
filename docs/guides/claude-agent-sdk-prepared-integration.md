# Claude Agent SDK Prepared Integration

Use the prepared facade for the Claude Agent SDK sidecar route: Anthropic's
official TypeScript Claude Agent SDK running inside a host-owned Node sidecar
over the private strict LF-JSON wire
`swallowtail-claude-agent-sdk-jsonl-v1`. The application provisions the exact
approved Node runtime, the source-tagged sidecar entry point, the exact SDK
package with its peer dependencies, and the platform package carrying the
native binary through a host-approved interpreted-script launch recipe;
preparation binds the configured instance, exact version bindings, restrictive
policy, preflight plan, and session request.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `claude-agent.sdk` in `swallowtail-adapter-claude-agent`, with
driver ID `swallowtail.claude-agent.sdk`. It is Unix-only: see
[Supported Platforms](#supported-platforms). Choose it for one fresh read-only
interactive session on the user's own Claude subscription, with streamed
output, identity-and-lifecycle activity, consumer-mediated tool admission,
interrupt, and a host-owned descendant-tree close. Reject it when the
application cannot provision the Node runtime, sidecar asset, SDK package, and
platform binary, or needs a model catalogue, structured runs, resume, fork,
session management, usage detail, model/effort/thinking control, MCP, hooks,
plugins, skills, subagents, checkpoints, writes, or Bash and terminal
execution. Those are later layers or separate routes, not withheld defaults.

All four Claude routes remain distinct. `claude-agent.acp` speaks ACP v1 over
stdio through a third-party bridge and is versioned on its own adapter axis.
`claude-code.headless` and `claude-code.response-only` drive the `claude`
CLI's stream-JSON interface directly and are versioned on the Claude Code
axis. `claude-agent.sdk` reaches the same native binary only through the
official SDK wrapper and is versioned on the SDK axis. The axes are coupled
but never equal: `0.3.258` declares native `2.1.258`, so a Claude Code
qualification never transfers to this route and this route's qualification
never transfers back.

## Subscription Authority

Usage draws from the user's own Claude subscription limits. Anthropic's
Help Center article "Use the Claude Agent SDK with your Claude plan" is the
first-party authority, and it explicitly names third-party applications. That
statement is current but provisional: it was rechecked on `2026-09-02`, it
pauses a previously announced change, and Anthropic promises notice before
anything takes effect. Re-read and re-freeze it before publishing a support
claim, before a release that first ships this route, and before any Agent SDK
Contract 029 checkpoint. A changed statement is a stop, not a downgrade.

## Credential Non-Custody

Swallowtail never holds the subscription credential.

1. The user runs the official Claude Code login out of band. Swallowtail does
   not perform, wrap, or drive it, and the SDK exposes no login function.
2. Credentials stay in the official Claude credential store, reachable only by
   the native binary, which authenticates itself.
3. Swallowtail leases a delegated credential reference that exposes no secret,
   and passes no credential over the sidecar wire.
4. Open observes typed readiness only: `apiProvider` must be `firstParty` and
   `apiKeySource` must be `oauth`. An API-key or delegated-cloud provenance
   label fails closed rather than silently running on a different profile.
   Account identity fields are refused, not redacted after the fact.

Two mechanical rules make this checkable rather than reviewable. The sidecar
imports the `.` SDK entry point only — the `/bridge` and `/browser` subpaths
declare raw access tokens, minted worker credentials, and OAuth credential
messages, and any reference to them fails the build's identity test. And
`Options.env` is always set explicitly, because omitting it inherits the
parent environment and would silently switch the access profile if
`ANTHROPIC_API_KEY` were present.

## Supported Platforms

Contract 019 requires the launch recipe to state exactly whether the host can
attest tree emptiness or only root completion, and permits the latter only as an
explicit platform-qualified degraded boundary.

The execution host retains a process-group owner on Unix, so the declared
descendant termination attempt reaches the tree for as long as the session
lives. Windows terminates a tree by request without retaining ownership of it,
so once the Node root exits there is no owner left to make that attempt
through. This route therefore declares Windows unsupported: the addable row
reports `Unsupported` and `open_session` refuses before any process starts.

Unix is supported under the accepted root-only degraded posture: the host can
make the termination attempt and observe root completion, but cannot attest
owned-tree emptiness, so close reports `Degraded` rather than `Clean`. Neither
posture is inferred from a successful close; both are stated up front.

## Explicit Inputs

Admission requires an admitted instance record for the `claude-agent.sdk`
addable route carrying opaque host-owned references only:

- a launch-recipe reference binding the approved Node runtime and sidecar
  entry point (`LocalExecutableLaunch::interpreted_script`)
- an environment reference whose approved body carries
  `CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE` (the exact `.` entry path),
  `CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY`, and
  `CLAUDE_AGENT_SDK_SIDECAR_MANIFEST`
- a delegated subscription credential reference
- an open `Deadline`, which bounds open, startup, and readiness against the
  host clock

`ClaudeAgentSdkSessionPreparation::from_admitted` lifts those references into
the explicit preparation input without exposing paths, environment values, or
credential bytes. Direct construction takes the same pieces explicitly:
configured-instance identity and revision, execution host, launch target,
environment, credential, access profile, model route, model, working-resource
reference, request identity, and the open deadline. This layer admits no
session options.

Swallowtail does not choose the model, account, credential, workspace, Node
runtime, SDK package, native binary, or fallback route, and never installs,
vendors, updates, repairs, or redistributes any of them.

The host binds task, process, time, credential, and working-resource services.
The working resource is read-only, and `ProviderSuppressed` configuration is
not a sandbox.

## Version Posture

Five separate axes carry qualified-only one-point claims; none admits an
unverified-newer point:

- `claude-agent.sdk.package`: exact `@anthropic-ai/claude-agent-sdk@0.3.258`
- `claude-agent.sdk.native`: exact native `2.1.258`, as the shipped
  `manifest.json` declares it
- `claude-agent.sdk.node`: exact Node `22.23.2` (satisfying the upstream
  `>=18.0.0` requirement)
- `claude-agent.sdk.wire`: exact `swallowtail-claude-agent-sdk-jsonl-v1`
  (opaque)
- `claude-agent.sdk.sidecar`: the exact source-tagged sidecar revision
  (opaque)

Two evidence limits belong to this family and must not be papered over. The
npm tarball digest is the sole artifact identity: npm carries no `gitHead`,
the tarball is staged from a private monorepo, and the public GitHub
repository holds no SDK source, so a future checkpoint cannot diff tags or
read that repository's changelog as a shipped-behavior oracle. And shipped
declarations are not runtime evidence — the shipped `manifest.json` declares
tested wrapper versions topping out at `0.3.227` inside the wrapper published
as `0.3.258`. Only the runtime `capabilities` observed at open may be treated
as behavior.

## What Open Verifies

Open runs before any provider work and fails closed on any mismatch: wire,
behavior revision, SDK package and version, native version, Node version, the
host-leased working directory, the effective model, the read-only
`Read`/`Glob`/`Grep` tool set, and first-party subscription readiness.

The selected model is sent as `options.model` and then confirmed from the
runtime's own `system/init` evidence. A session that silently ran Claude's
ambient default instead of the plan's model is a substitution, not a
convenience, and fails closed.

Every public operation is bounded by a caller-supplied host deadline, and the
bound covers the return, not merely the noticing.

- `open_session` races startup against the open deadline. On expiry it makes
  the descendant termination request first, then races each cleanup stage
  against the same bound, and reports `open_cleanup_unconfirmed` rather than
  implying cleanup finished.
- `start_turn` races the correlated query response against the turn deadline,
  so a sidecar that stops answering cannot hold the public future open.
- Cancellation always writes the interrupt; only the receipt is bounded, and an
  unanswered receipt still returns `Requested`, which never claims provider
  truth.
- `close` takes the caller's `SessionCleanupRequest`. One deadline covers turn
  resolution, interruption, the close command, host escalation, the pump join,
  and both lease releases. No stage restarts it, and expiry returns failure. Ambient behavior is suppressed by
construction rather than by omission: setting sources are empty, skills are an
explicit empty list (omission is documented *not* to mean "skills off"),
session persistence is disabled, and MCP servers, plugins, hooks, subagents,
and system prompts are all set explicitly.

Runtime-advertised capabilities are recorded and then enforced. An interrupt
receipt is admissible only where the runtime advertised
`interrupt_receipt_v1`; a receipt without that advertisement is a failure, not
a bonus.

## Tool Admission

Read-only tool use is admitted by the consumer, never inferred by the sidecar.

Availability is restricted with `Options.tools`. `Options.allowedTools` is
never set: it auto-allows without prompting, which would bypass per-use
admission entirely. The exact read-only allow-list is enforced inside the
sidecar before any consumer round trip, so an unknown tool is denied without
ever being offered, and the Rust side rejects an out-of-set request as a
transport failure rather than delegating it.

Each admitted request crosses the wire as a bounded correlated callback in the
route-local `claude-agent-sdk/can-use-tool` namespace carrying the tool name
and nothing else. The tool's own input never crosses the wire: the sidecar
retains it privately and returns it unchanged as `updatedInput` on allow,
because `updatedInput` replaces what the provider would otherwise use — an
empty object would silently destroy the path or pattern the tool needs. A
consumer failure, an abandoned turn, or a closed exchange all deny. The namespace is deliberately route-local: shared
permission vocabulary is orchestrator work once a second provider proves the
same semantics.

Bash, terminal, write tools, and every non-read tool are outside this route.
A capability advertisement is not admission; those need their own Contract 023
process authority and Contract 041 mediation evidence.

## Close And The Descendant Tree

This is the part that differs from every single-process sidecar. The route is
two processes deep — Rust → Node sidecar → native `claude` → whatever that
binary spawns — and the upstream SDK supplies no joined stop. Its cleanup
races a bounded timer inside a swallowed `catch` and discards the outcome, and
its own escalation timers are unreferenced and reach only the direct child.
None of that may be read as evidence that a process exited.

Close runs in this order, entirely inside the caller's one cleanup deadline:

1. resolve any live turn, then join its host-deadline task
2. interrupt a live turn
3. end sidecar input through the explicit `close` command
4. the sidecar joins its own independently retained native child handle to the
   declared 2000 ms bound, which it starts before any other close await so no
   SDK-side drain can consume the bound
5. make the declared descendant termination attempt through host authority
6. re-join the root, then release resource and credential leases in contract
   order

The outcome comes from evidence, never from hope:

| Evidence | `CleanupOutcome` | Meaning |
| --- | --- | --- |
| host attests `OwnedTreeEmpty` | `Clean` | no member of the owned tree remains |
| host attests root completion only, root exit confirmed | `Degraded` | the root exited after the declared termination attempt; descendants stay unconfirmed |
| sidecar observed its native child still running | `Failed` | a descendant survived |
| root exit never observed | `Failed` | cleanup could not be established |

Three rules hold this together.

The sidecar reports only what it observed of its own direct native child. A
claimed exit that carries no observation is discarded, and a handle still
showing a live child is a positive survivor observation, not an absence of
news. A survivor outranks even an emptiness claim: the two cannot both be true.

Only `ProcessTreeCompletion::OwnedTreeEmpty` from the execution host may support
`Clean`. Root exit is not tree completion, and this route never promotes one to
the other, caches root-only evidence as tree-empty, or widens it to another
route or platform.

**On ordinary macOS, `Degraded` is the normal successful close.** No host in
this repository can observe owned-tree emptiness (card 059), so the operator
accepted this bounded root-only posture on 2026-09-03. Applications that cannot
accept it should reject the route at selection rather than treat `Degraded` as
noise.

## Failure Diagnostics

Every driver failure is a safe `swallowtail.claude-agent.sdk.*` diagnostic.
Sidecar records are bounded and redacted, stderr is dropped without
inspection, and no provider payload, credential, path, or raw SDK value
reaches a public record. Do not parse diagnostic text; classify on the code.

## Normal Path

See the [prepared SDK sidecar
example](../../crates/swallowtail-adapter-claude-agent/examples/prepared_claude_agent_sdk.rs)
for the exact call sequence: prepare, open, start one turn, admit tool use,
observe the terminal outcome, then close and read the cleanup outcome.
