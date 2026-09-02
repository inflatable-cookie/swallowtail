# 2026-09-02 Claude Agent SDK Foundation

Card 055 implements the provider-free `claude-agent.sdk` route: Anthropic's
official TypeScript Claude Agent SDK driven through a bounded host-owned Node
sidecar over the private `swallowtail-claude-agent-sdk-jsonl-v1` wire at
behavior revision `claude-agent.sdk-v1`. No provider session, login, package
installation, or downloaded binary execution was part of this work.

Policy and artifact were rechecked immediately before implementation, as the
card requires. The Help Center article still leads with the paused change and
the preserved statement that Agent SDK, `claude -p`, and third-party app usage
draw from the user's subscription limits. Official npm still carries `0.3.258`
on both `latest` and `next`, with the Research 278 shasum, integrity, file
count, unpacked size, and publish time unchanged. Neither stop condition
fired, and nothing was retargeted.

Five identities bind independently, each a qualified-only one-point claim: SDK
wrapper `0.3.258`, native binary `2.1.258` from the shipped manifest, Node
`22.23.2`, the private wire, and the source-tagged sidecar revision. The
wrapper and native axes are coupled but never equal, so no Claude Code or ACP
qualification transfers in either direction. The route inherits no existing
Claude claim and mints no window beyond its one point.

Credential non-custody is structural rather than incidental. The sidecar
imports the `.` entry point only, and the identity test greps the shipped
asset for `/bridge`, `/browser`, `fetchRemoteCredentials`, `createCodeSession`,
`worker_jwt`, `OAuthCredential`, `apiKeyHelper`, the cloud auth-refresh
settings, and `ANTHROPIC_API_KEY`. `Options.env` is always explicit, because
omission inherits the parent environment and would silently switch the access
profile. Open accepts only `firstParty` provenance with an `oauth` key source,
refuses account identity fields outright, and projects readiness labels only.

The lifecycle half is the reason this card was frontier work. The upstream SDK
supplies no joined stop: its cleanup races a 2000 ms timer inside a swallowed
`catch`, discards the outcome, and its own escalation is unreferenced and
reaches only the direct child. The route therefore never reads SDK cleanup as
evidence. The sidecar retains its own native child handle through
`spawnClaudeCodeProcess` and joins it independently; Rust escalates through
the execution host's descendant-tree termination authority on expiry, re-joins,
and reports exactly one of `graceful` (Clean), `escalated` (Degraded), or
`unconfirmed` (Failed). A graceful claim that carries no observation is
rejected and escalated like any other unproved join.

Descendant enrollment is proved, not asserted. Two new `swallowtail-host-local`
cases run one portable sidecar-plus-native-descendant topology on whichever
supported platform runs the suite: host tree termination reaches the
grandchild, and the counterexample shows the same topology started outside the
host's tree authority surviving a clean nearest-child join. The nearest child
is never the lifecycle boundary.

Delivered behavior is exactly layer one: one fresh read-only interactive
session with bounded correlated framing, streamed output, identity-and-
lifecycle activity, consumer-mediated `canUseTool` admission in a route-local
namespace, capability-gated interrupt receipts, and the joined close. Session
persistence is disabled, so load, resume, fork, session management, model,
effort, thinking, usage detail, checkpoints, MCP, hooks, plugins, skills,
subagents, Bash, and terminal all remain later layers or separate evidence.

Sixteen provider-free driver cases, six identity cases, and the in-crate wire,
protocol, selection, and close-state suites cover identity, framing bounds,
ordering, redaction, readiness negatives, admission capacity, interruption,
crash, disconnect, terminal records, escalation, re-join, and cleanup
ordering. The only surface outside the two card packages is the shared
provider-wide harness-activity fixture in `swallowtail-testkit`, which the
route inventory requires for any new route row.

No shared vocabulary was named, no release artifact changed, and no tag or
merge was performed.
