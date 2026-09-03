# 2026-09-02 Claude Agent SDK Foundation

Card 055 implements the provider-free `claude-agent.sdk` route: Anthropic's
official TypeScript Claude Agent SDK driven through a bounded host-owned Node
sidecar over the private `swallowtail-claude-agent-sdk-jsonl-v1` wire at
behavior revision `claude-agent.sdk-v1`. No provider session, login, package
installation, or downloaded binary execution was part of this work.

Policy and artifact were rechecked immediately before implementation and again
before each later round. The 2026-09-03 recheck found official npm stable had
moved, the operator selected the refresh exit, and Research 280 rebound the
exact package and native points to `0.3.259` and `2.1.259`. Policy has been
unchanged throughout: the article still leads with the paused change and the
preserved subscription-draw statement.

Five identities bind independently, each a qualified-only one-point claim: SDK
wrapper `0.3.259`, native binary `2.1.259` from the shipped manifest, Node
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

Exact-head review returned six blocking findings; five are repaired here and
the sixth is stopped on a seam. The selected model is now sent explicitly and
confirmed from `system/init` evidence, so the route cannot silently run
Claude's ambient default. Tool availability is restricted with `tools` rather
than `allowedTools`, which auto-allows without prompting; the read-only
allow-list is enforced inside the sidecar before any consumer round trip, and
an allowed decision returns the provider's own tool input unchanged while that
input never crosses the wire. Open and each turn are raced against their
caller-supplied host deadlines. Windows is declared unsupported rather than
best-effort, because the host retains a process-group owner only on Unix. A
sidecar-level falsification now runs the shipped asset under Node against a
fake SDK and fake native child, proving the option surface, the admission
contract, input intactness, and both native-join outcomes.

The lifecycle half is the reason this card was frontier work. The upstream SDK
supplies no joined stop: its cleanup races a 2000 ms timer inside a swallowed
`catch`, discards the outcome, and its own escalation is unreferenced and
reaches only the direct child. The route therefore never reads SDK cleanup as
evidence. The sidecar retains its own native child handle through
`spawnClaudeCodeProcess` and joins it independently; Rust escalates through
the execution host's descendant-tree termination authority, re-joins, and
reports `escalated` (Degraded) or `unconfirmed` (Failed). A graceful claim
that carries no observation is rejected and escalated like any other unproved
join.

`graceful` (Clean) is deliberately unreachable today. The host terminates the
tree it owns during cleanup but does not report whether anything remained, so
an observed root exit does not prove every descendant exited, and claiming
otherwise would be the Review Oracle counterexample wearing a success label.
Both shared prerequisites landed on 2026-09-03 and the route was completed
against them. Card 058's `SessionCleanupRequest` gives close one caller-selected
deadline, and card 057's `ProcessTreeCompletion` separates root exit from owned
-tree emptiness. Every public operation is now caller-bounded through the
return, not merely to the point of noticing expiry: open races startup and then
its own cleanup stages, start-turn races the correlated query response,
cancellation bounds only the receipt after always writing the interrupt, and
close runs wholly inside the caller's cleanup deadline across turn resolution,
interruption, escalation, the root join, and both lease releases.

Cleanup truth now follows host evidence exactly. `OwnedTreeEmpty` alone reports
`Clean`. Confirmed root completion after the declared descendant termination
attempt is the operator-accepted `Degraded` posture for ordinary macOS, where
card 059 proved no owned-tree observation exists under current host authority.
An observed surviving descendant or an unconfirmed root exit is `Failed`, and a
survivor outranks even an emptiness claim, because the two cannot both be true.
Windows remains unsupported: no tree owner survives the root there, so the
declared termination attempt cannot be made.

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

## Scoped-Work Relinquishment

The open guard owns the whole ordered cleanup — terminate, wait, join the pump,
release the resource lease, release the credential lease — inside one host task,
and reports completion through its own signal rather than through a join of its
handle. Process exit alone never permits a lease release or a cleanup-complete
result while the pump task is still alive.

Ownership of a handle whose task is still running at the caller's deadline was
the card's recorded stop. Card 060 closed it. The route now transfers such a
handle through `ScopedTaskService::relinquish`, naming the exact selected
execution host and the exact scope the task was spawned under, and requires
`AcceptedForReap`. That outcome is ownership transfer only: it is never
reported as a join and never strengthens a cleanup outcome, so a transferred
pump still closes `Failed` on an unconfirmed root. A refused transfer leaves
ordinary join-and-drop ownership exactly where it was. The host's own outer
reaper shutdown belongs to the execution-host lifecycle outside the task tree;
this route neither calls nor claims it.
