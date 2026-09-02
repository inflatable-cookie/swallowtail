# 2026-09-02 Claude Agent SDK Route Gate

Card 053 froze the native `claude-agent.sdk` evidence and contract gate.
Research 278 records it; the contract proposal is in
`../triage/2026-09-02-claude-agent-sdk-route-contract-gate.md`.

Official subscription guidance was rechecked immediately before artifact
freeze and is unchanged: Agent SDK, `claude -p`, and third-party app usage
still draw from the user's subscription limits, with advance notice promised
before any change. Provisional, with a named recheck trigger.

Selected artifact is official npm `@anthropic-ai/claude-agent-sdk` `0.3.258`,
tarball SHA-256
`656cf237bc567cb172a007a0fd5b3958cf960d154c03ab390a755d2c3bdbb398`, 15 files
all digested, Node `>=18.0.0`, `claudeCodeVersion` `2.1.258`. Nothing
downloaded was executed and no authenticated probe ran.

Three findings shaped the gate. The npm package is a wrapper: the agent is a
199–219 MB native binary delivered by eight pinned platform packages, so the
route is two processes deep. The artifact cannot be corroborated against
public source — npm carries no `gitHead`, the tarball is staged from a private
monorepo, and the public GitHub repository holds no SDK source — so the npm
digest is the sole identity for this family. And the shipped `manifest.json`
declares tested wrapper versions topping out at `0.3.227` inside the wrapper
published as `0.3.258`, the clearest instance of declaration diverging from
shipping artifact.

Credential non-custody holds and needs no new contract: Contracts 010, 017,
047, and 057 already cover harness-owned delegated authentication and the
disclosed subject. The default entry point declares no token material across a
ten-pattern search, and there is no exported login function at all. The real
risk is import choice, not the package — `/bridge` and `/browser` declare raw
`accessToken`, `worker_jwt`, and `OAuthCredential`. That reduces to one
mechanically checkable rule.

Route identity is additive. `claude-agent.sdk` cannot reuse `claude-agent.acp`
— different wire, package, and version axis — and reusing it would retroactively
invalidate Research 272's correct classification of the ACP bridge's Agent SDK
pin as unmapped. It cannot reuse `claude-code.headless` or `.response-only`,
which sit on the Claude Code axis.

The lifecycle finding is sharper than the declarations suggest, and reading
shipped `sdk.mjs` was what settled it. The SDK exposes **no joined stop at
all**. `Query.close()` returns `void`; `performCleanup` races
`waitForExit()` against a 2 000 ms timer inside `try{}catch{}` and discards the
outcome, so it can resolve silently while the native child is still alive with
nothing distinguishing exit from expiry. Its own SIGTERM/SIGKILL escalation and
exit registry are all `.unref()`'d and reach only the direct child.

One Contract 019 gap went to the gate, stated as a provider-neutral invariant
rather than a mechanism: the execution host owns and can terminate the full
descendant tree, close joins the tree rather than the nearest child, and close
returns an explicit graceful / escalated / unconfirmed outcome, with a
discarded wait never counting as evidence of exit. Naming
`spawnClaudeCodeProcess` in the contract was rejected — it would bind every
future sidecar to one vendor callback that covers only the direct child.

Because the SDK supplies no join, the route carries a matching implementation
obligation: hold an independently joinable process handle in the sidecar and
return the three-valued close state over the private wire. That is layer 1, not
a later hardening step. `spawnClaudeCodeProcess` is recorded as one route-local
option, with its costs — a custom spawner loses the SDK's stderr-tail drain and
suppresses its default `--debug-file` — alongside a host-created process group
or Windows job object that covers the tree.

No production code, manifest, package pin, claim, fixture, or matrix changed.
No shared vocabulary was named and no implementation card was compiled; both
remain orchestrator integration work jointly with card 054.
