# 2026-09-02 Claude Agent SDK Route Contract Gate

Owner: Tom
Card: g05 batch 053
Evidence: `../research/278-claude-agent-sdk-route-evidence.md`
Decides: two Contract 019 amendments; one Contract 029 family note
Does not decide: shared vocabulary, implementation sequencing, route claims

## Why this exists

Research 278 found the `claude-agent.sdk` route admissible. Credential
non-custody, route identity, ambient suppression, explicit configuration, and
failure classification are all already fixed by Contracts 010, 014, 017, 019,
023, 029, 038, 041, 047, 051, and 057. No operator product, API, persistence,
or security choice is open.

Two lifecycle rules are not fixed, and one evidence limit needs recording.
They are stated here as reviewable proposals because contract promotion
belongs to the orchestrator, not to this worker.

## Gap 1 — nested provider process ownership

**Contract 019 §Foreign-Language SDK Sidecars currently says:**

> Cancellation and close abort SDK work, dispose SDK/session state, drain the
> bounded wire, join the sidecar process, then release provider state,
> credentials, and host resources in contract order.

**Why that is insufficient here.** The route is two processes deep: Rust →
Node sidecar → native `claude`. The npm package is a wrapper; the agent is a
199–219 MB platform binary delivered by eight exact-pinned
`optionalDependencies` and described by the shipped `manifest.json`
(`2.1.258`, commit `b3cd543a1f6fcdf4d8fabc0f5e5538d2ee7f38e1`).

The grandchild is the process that holds provider state, owns the credential
reach, and spawns Bash. The clause as written is satisfied by joining the Node
process alone, while the native child survives. That is exactly the card 053
Review Oracle counterexample.

**Proposed amendment.** Add to §Foreign-Language SDK Sidecars:

> Where the upstream SDK itself launches a further provider-owned process, the
> sidecar is not the lifecycle boundary. The launch recipe binds every process
> in the chain: the host declares the process group, the sidecar delegates
> creation to a host-supplied spawn seam rather than letting the SDK spawn
> ambiently, and the host holds independent termination authority over the
> provider process. Close joins the chain, not the nearest child. A provider
> process that outlives the sidecar is a cleanup failure, not a detached
> background task.

**Mechanism this route already has.** `Options.spawnClaudeCodeProcess?:
(options: SpawnOptions) => SpawnedProcess` is the declared seam.
`SpawnedProcess` exposes `kill(signal)`, `killed`, `exitCode`, `signalCode?`,
and `on('exit')`. Declared cost, which the implementation card must carry:
the SDK's built-in spawn delivers `exit` only after stderr closes, whereas
"custom `spawnClaudeCodeProcess` implementations emit plain process exit" — a
host spawner loses the stderr-tail guarantee and must reconstruct it.

**Alternative considered and rejected.** Leave the clause alone and treat
group ownership as an implementation convention. Rejected: a convention is not
falsifiable at review time, and the counterexample the oracle names is
precisely the one the current wording permits.

## Gap 2 — bounded join versus guaranteed join

**Why it is not fixed.** Contract 019 says "join the sidecar process" with no
time bound and no escalation path. The SDK's join is explicitly bounded:

- `Query.close(): void` — declared to terminate the subprocess, but returns
  `void`. Not awaitable, therefore not a join.
- `Query.return()` / `AsyncDisposable` — runs `performCleanup()`, which awaits
  `Transport.waitForExit()` **bounded**, so cleanup "don't resolve while the
  child is still draining the stdin EOF that `close()` just sent."
- `SpawnOptions.signal` is a forwarded signal that fires only after stdin EOF
  plus a declared `GRACEFUL_EXIT_TIMEOUT_MS` grace of roughly two seconds.

A bounded join can expire. Contract 017 already forbids hiding the result
("provider completion never hides cleanup degradation or failure"), but the
required sequence is stated nowhere.

**Proposed amendment.** Add to §Foreign-Language SDK Sidecars:

> A sidecar join may be bounded. Where it is, close states the bound, and on
> expiry escalates to host-owned termination of the declared process group
> rather than resolving. Cleanup that completed only by escalation, or that
> could not confirm exit, is reported as degraded. A bounded join is never
> reported as a clean close.

**Consequent close order for this route**, recorded so review has something
exact to test: `interrupt()` if a turn is live → `Query.return()` /
asyncDispose → await bounded `waitForExit()` → on expiry, host-owned
process-group termination → report degradation → release provider state,
credentials, and host resources in Contract 017 owner order.

## Note — Contract 029 evidence limit for this family

Not an amendment. A family-scoped fact any future checkpoint must carry, so a
later reader does not assume the ACP family's evidence method transfers.

The `@anthropic-ai/claude-agent-sdk` npm artifact **cannot** be corroborated
against public source. Three independent facts, each verified in Research 278:

1. npm metadata for `0.3.258` carries no `gitHead`. The
   `@agentclientprotocol/claude-agent-acp` family does, and Research 272 used
   it to match npm publishes to GitHub tags.
2. `_resolved` shows the tarball staged from the private
   `claude-cli-internal` monorepo.
3. The public `anthropics/claude-agent-sdk-typescript` tree at tag `v0.3.258`
   contains no SDK source — only `CHANGELOG.md`, `LICENSE.md`, `README.md`,
   `examples/session-stores/`, `scripts/`, and workflows.

**Consequence.** For this family the npm tarball digest is the sole artifact
identity. A checkpoint must not diff GitHub tags, must not treat that
repository's `CHANGELOG.md` as a shipped-behavior oracle, and must state the
narrower proof explicitly rather than implying parity with the ACP family's
method. Publication cadence is roughly daily (285 versions), which makes the
per-version qualification cost a real planning input.

## Rules this gate fixes without a contract change

Recorded here so the implementation card inherits them as review-testable
rules rather than re-deriving them.

1. **Entry point.** The route imports the `.` entry point only. Importing
   `@anthropic-ai/claude-agent-sdk/bridge` or `/browser` is a violation —
   both declare raw credential parameters (`accessToken: string`,
   `RemoteCredentials.worker_jwt`, `OAuthCredential.token`). Mechanically
   checkable: grep for `claude-agent-sdk/bridge`, `claude-agent-sdk/browser`,
   `bridge.mjs`, `browser-sdk.js`, `fetchRemoteCredentials`,
   `createCodeSession`, `worker_jwt`, `OAuthCredential`.
2. **Never a credential producer.** `Settings.apiKeyHelper`, `awsAuthRefresh`,
   and `gcpAuthRefresh` are never set.
3. **`Options.env` always explicit.** Omission inherits `process.env` by
   declaration, which would both violate Contract 019 §Explicit SDK
   Configuration and silently switch the access profile if
   `ANTHROPIC_API_KEY` is present. Require `AccountInfo.apiProvider ===
   'firstParty'`.
4. **Ambient suppression is opt-out, not default.** `settingSources` loads all
   sources when omitted — pass `[]`. Omitting `skills` is declared *not* to be
   "skills off" — pass an explicit list. `persistSession` defaults to `true`.
5. **Declarations are not runtime.** Only `system/init` `capabilities` may be
   treated as runtime evidence. The shipped `manifest.json`
   `sdkCompat.testedWrapperVersions` tops out at `0.3.227` inside the wrapper
   published as `0.3.258` — a stale declaration in the shipping artifact.
   `sdk.d.ts` also has stripped members, so "not declared" never proves "not
   in the protocol."
6. **Resume rebinding.** Re-read `SDKSystemMessage.cwd` and `AccountInfo`
   after resume; fail closed on cwd or account mismatch.
7. **No redistribution.** `LICENSE.md` is `© Anthropic PBC. All rights
   reserved`, incorporating terms by reference; the npm `license` field points
   at a README license section that does not exist. Contract 019 already puts
   provisioning on the application. Swallowtail never vendors, mirrors, pins,
   installs, upgrades, or repairs the SDK or the platform binaries.
8. **Policy recheck trigger.** Re-read the Help Center subscription article
   and re-freeze before any support claim, any release first shipping the
   route, or any Agent SDK Contract 029 checkpoint. A changed statement is a
   stop. The policy is unversioned prose, so there is no digest to pin and
   this trigger is the only available mechanism.

## Explicitly not decided here

- Shared provider-neutral vocabulary. Research 278 §7 partitions portable
  candidates from route-local surfaces but names none as shared. That
  reconciliation is orchestrator work after card 054.
- Any `claude-agent.sdk` claim id, behavior revision, support window, or
  qualified range. None is minted.
- Implementation sequencing. Research 278 §8 splits four layers; compiling
  them into cards is orchestrator work.
- Bash and terminal admission, which needs its own Contract 023 process
  authority and Contract 041 mediation evidence.

## Requested decision

Accept or amend the two Contract 019 clauses and the Contract 029 family note.
If accepted, promotion into `docs/contracts/019-embedded-sdk-and-cloud-client-boundary.md`
and the Contract 029 checkpoint guidance is orchestrator integration work,
jointly with card 054, and is not part of this worker's runway.
