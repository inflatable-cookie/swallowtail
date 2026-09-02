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

Two Contract 019 gaps are real and went to the gate rather than being invented
here: §Foreign-Language SDK Sidecars says "join the sidecar process," which the
nested native grandchild defeats, and it sets no bound or escalation path for a
join the SDK declares as bounded at roughly two seconds. `Query.close()` returns
`void` and is not a join; the joined path is `Query.return()` awaiting a bounded
`Transport.waitForExit()`.

No production code, manifest, package pin, claim, fixture, or matrix changed.
No shared vocabulary was named and no implementation card was compiled; both
remain orchestrator integration work jointly with card 054.
