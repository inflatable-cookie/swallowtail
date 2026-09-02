# 2026-09-02 Claude Agent SDK Route Contract Gate

Owner: Tom
Card: g05 batch 053
Evidence: `../research/278-claude-agent-sdk-route-evidence.md`
Decides: one Contract 019 amendment; one Contract 029 family note
Does not decide: shared vocabulary, implementation sequencing, route claims,
or which mechanism this route uses to satisfy the invariant

## Why this exists

Research 278 found the `claude-agent.sdk` route admissible. Credential
non-custody, route identity, ambient suppression, explicit configuration, and
failure classification are all already fixed by Contracts 010, 014, 017, 019,
023, 029, 038, 041, 047, 051, and 057. No operator product, API, persistence,
or security choice is open.

One lifecycle rule is not fixed, and one evidence limit needs recording. They
are stated here as reviewable proposals because contract promotion belongs to
the orchestrator, not to this worker.

The proposal below is deliberately an **invariant**, not a mechanism. An
earlier draft of this gate required a host-supplied spawn seam and asserted
that Rust would hold the native process's PID. Both were wrong: the mechanism
is one provider-specific callback that should not bind every future sidecar,
and the PID claim is unsupported — the callback runs inside Node, returns a
Node handle, and no current wire carries that handle to Rust. The route-local
mechanism options now sit in §Implementation obligation, where they belong.

## Amendment — descendant-tree ownership and join outcome

**Contract 019 §Foreign-Language SDK Sidecars currently says:**

> Cancellation and close abort SDK work, dispose SDK/session state, drain the
> bounded wire, join the sidecar process, then release provider state,
> credentials, and host resources in contract order.

**Why that is insufficient.** It names one process, sets no bound, defines no
escalation, and requires no join outcome. Two independent facts defeat it here.

*The topology is a tree, not a child.* Rust → Node sidecar → native `claude` →
whatever that binary spawns, including Bash tool subprocesses. The npm package
is a wrapper; the agent is a 199–219 MB platform binary delivered by eight
exact-pinned `optionalDependencies` and described by the shipped
`manifest.json` (`2.1.258`, commit
`b3cd543a1f6fcdf4d8fabc0f5e5538d2ee7f38e1`). The clause is satisfied by
joining the Node process alone while the rest of the tree survives — the card
053 Review Oracle counterexample.

*A join that cannot report its outcome is not a join.* Research 278 §6 shows
the shipped SDK races `waitForExit()` against a 2 000 ms timer inside
`try{}catch{}`, discards the outcome, and returns nothing. Contract 017
already forbids hiding cleanup degradation, but nothing currently says what a
join must establish or what to do when it establishes nothing.

**Proposed amendment.** Add to §Foreign-Language SDK Sidecars:

> Where a sidecar's upstream SDK launches further provider-owned processes,
> the sidecar is not the lifecycle boundary. The execution host owns, and can
> terminate, the full descendant process tree rooted at the sidecar. The
> launch recipe proves descendant enrollment or containment on each supported
> platform; a platform where it cannot is unsupported for that route, not
> best-effort.
>
> Close joins every provider process in that tree, not only the nearest
> child. A join may be bounded, and where it is, close states the bound and on
> expiry escalates through host termination authority rather than resolving.
>
> Close returns an explicit outcome: exited gracefully, exited after
> escalation, or exit unconfirmed. A surviving descendant, or an outcome that
> cannot distinguish exit from expiry, is cleanup failure and is reported as
> such. A discarded or unobserved wait is never evidence of exit.

**Why an invariant and not a mechanism.** The clause must hold for any future
foreign-language sidecar whose SDK spawns provider processes, regardless of
whether that SDK offers a spawn callback at all. Naming
`spawnClaudeCodeProcess` in Contract 019 would bind every such route to one
vendor's API and would still not deliver tree coverage, since that callback
yields a handle to the direct child only.

**Alternative considered and rejected.** Leave the clause alone and treat tree
ownership as an implementation convention. Rejected: a convention is not
falsifiable at review time, and the counterexample the Review Oracle names is
exactly the one the current wording permits.

## Implementation obligation for this route

Not a contract matter, and not satisfiable by calling the SDK correctly.
Recorded here so the implementation card inherits it rather than rediscovering
it.

The SDK offers no joined stop — `Query.close()` returns `void`, and
`performCleanup`'s bounded race discards its outcome. The route must therefore
add sidecar behavior:

1. Retain, inside the sidecar, a process handle that can be joined
   independently of the SDK's own cleanup.
2. Join it to a declared bound; on expiry, escalate through host termination
   authority; re-join.
3. Return an explicit close state over the private wire — `graceful`,
   `escalated`, or `unconfirmed` — which must never collapse to a single
   success value. `unconfirmed` is a Contract 017 cleanup failure, not a slow
   success.
4. Do not rely on the SDK's own escalation. Research 278 §6 shows its
   SIGTERM/SIGKILL timers and its `process.on('exit')` registry are all
   `.unref()`'d and reach only the direct child, so none of it survives the
   sidecar being killed and none of it covers the tree.

**Mechanism options, both route-local and neither mandated:**

- **`Options.spawnClaudeCodeProcess`.** The callback runs inside Node and
  returns a Node `SpawnedProcess`, giving the sidecar a handle to the direct
  native child that it can then report over the wire. It does not give Rust a
  PID. Costs, both confirmed in shipped `sdk.mjs`: the built-in spawn remaps
  `exit` to an internal `sdk-exit-after-stderr-drained` event with a 200 ms
  drain grace, so a custom spawner gets plain process exit and **loses the
  stderr-tail guarantee**, which it must reconstruct if exit errors are to
  keep their stderr context; and setting the callback also suppresses the
  SDK's default `--debug-file` argument. Covers the direct child only.
- **A host-created POSIX process group or Windows job object** that captures
  descendants. Covers the tree rather than one child, and keeps termination
  authority on the Rust side, at the cost of platform-specific launch work.

Choosing between them, or combining them, is implementation work. Both must
satisfy the same invariant.

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
7. **A discarded wait is not a stop.** Neither `Query.close()` nor
   `Query.return()` may be treated as evidence that a process exited. Only an
   observed exit, or an escalation confirmed by re-join, may be reported as a
   successful close. See the implementation obligation above.
8. **No redistribution.** `LICENSE.md` is `© Anthropic PBC. All rights
   reserved`, incorporating terms by reference; the npm `license` field points
   at a README license section that does not exist. Contract 019 already puts
   provisioning on the application. Swallowtail never vendors, mirrors, pins,
   installs, upgrades, or repairs the SDK or the platform binaries.
9. **Policy recheck trigger.** Re-read the Help Center subscription article
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
- Which mechanism this route uses to hold an independently joinable process
  handle. Both options above satisfy the invariant; the choice is
  implementation work, not a contract or operator decision.
- Bash and terminal admission, which needs its own Contract 023 process
  authority and Contract 041 mediation evidence.

## Requested decision

Accept or amend the Contract 019 descendant-tree amendment and the Contract
029 family note. If accepted, promotion into
`docs/contracts/019-embedded-sdk-and-cloud-client-boundary.md` and the
Contract 029 checkpoint guidance is orchestrator integration work, jointly
with card 054, and is not part of this worker's runway.

The implementation obligation needs no decision here — it is a consequence of
the shipped SDK, and belongs to whichever implementation card the orchestrator
compiles.
