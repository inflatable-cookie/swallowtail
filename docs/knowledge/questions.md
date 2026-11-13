# Questions

Questions that block or shape work. Reference them by ID from lane documents and from
briefs. An answered question keeps only its pointer to where the answer lives.

An open question can own feature-matrix `evidence_pending` cells. It must name
the gate owner, state the decision tree converting each outcome into
`producer_gap` or `provider_limitation`, and list its cells, one
`- <route> <feature>` line each, after an `Evidence gate scope:` line. The
route-matrix check enforces this.

## Q-001 — Run a live HTTP MCP honouring gate on `claude-agent.acp`?

Status: answered 2026-09-26
Answer: yes, as recommended (Tom). One attempt, harness proof first, pinned
`claude-agent-acp` 0.79.0. Lane `claude-agent-acp-http-mcp-live`.
Asked: 2026-09-25

Research 351 wired five ACP routes to emit the Contract 063 consumer HTTP MCP
entry; only `opencode.acp` is live-proven (Research 349). Recommendation: yes,
first, one attempt, harness proof against the fake SDK before the live turn,
pinned `claude-agent-acp` 0.79.0 over the host's 0.63.0. Gemini second.
Copilot, Goose and Kiro held until a consumer needs them.

Live result: Research 352. The one attempt was not accepted (typed stop
`cleanup_failed`). The cell stays No.

Diagnosis: Research 355. The stop name was the gate's own classification and
the run kept no cleanup diagnostic. The gate now keeps the typed cleanup
diagnostic (class, code, adapter stage tag) and prints it, so a second
attempt — which still needs fresh operator authority — cannot lose it again.

## Q-002 — Which command validates a fresh checkout before merge?

Status: answered 2026-09-26
Answer: no separate Queue validation for now (Tom, 2026-09-28); GitHub CI
gates merges. `AGENTS.md` "Validate". Revisit through the Queue lead.
Asked: 2026-09-26

Queue does not yet run plain pre-merge validation. The command must validate a
fresh disposable checkout, install its own dependencies and exit 0 on pass.
Candidate: `effigy qa:docs && effigy qa:routes && effigy format:check`, with
package-scoped `effigy validate:focused` named per brief. Full `effigy qa`
runs the whole workspace test suite and is likely too slow per task.

## Q-003 — How should `antigravity.headless` qualify past the `1.1.22` retry stop?

Status: answered 2026-09-26
Answer: the recommendation below (Tom): option 2 evidence first, option 1 as
the fallback. Lane `version-currentness`.
Asked: 2026-09-26

From `1.1.22` through official `1.2.11`, Antigravity retries failed model
requests itself with no published attempt bound or off switch (Research 353,
"Ruling request"). The claim stays `1.1.9..=1.1.17`. Options:

1. Accept provider-managed retry as a Contract 023 exception on a new
   milestone. Consumer-visible: only the host deadline bounds a turn.
2. Pin a finite or disabling retry control. `1.2.11` contains an
   undocumented `AGY_CLI_MODEL_API_MAX_RETRIES`; its semantics need evidence
   first, and pinning it changes the route's approved environment.
3. Keep the ceiling with a successor task.

Recommendation: option 2 first, as a bounded evidence task on the frozen
artifact against a fake model endpoint (no provider); fall back to option 1
if the control doesn't hold.

Evidence: Research 359. The control bounds every listed retryable class
and `0` disables retry, so exact `1.2.11` headless qualifies on a
retry-disabled revision with `AGY_CLI_MODEL_API_MAX_RETRIES=0` pinned
(swallowtail#069). `1.1.18..=1.2.10` still need per-point pin backfill.

## Q-004 — How should `kimi-code.local-server` qualify past `0.39.1`?

Status: answered 2026-09-26
Answer: B (Tom). Lane `version-currentness`.
Asked: 2026-09-26

`0.40.0` stopped checking the Bash tool's `cwd` argument against the
workspace; it is unchanged through official `2.1.1` (Research 354,
"Rulings"; Research 357 lands the claim). The route runs under Contract 023 `AmbientHost`, which never
claimed workspace containment, and the removed check covered only the
starting `cwd`, never what a shell command does after it starts. Options:
A pin `disabled_tools: ["Bash"]` (removes the shell tool for every consumer);
B qualify `0.40.0..=2.1.1` under `AmbientHost` without the check; C build a
new provider- or host-enforced isolation route; D keep `0.39.1` (a No
Terminal Stop exception).

Recommendation: B. Document the change in the guide, and point consumers who
want no shell at the existing `disabled_tools` control. `2.0.0` is a
same-package major-line reset with an otherwise unchanged local-server
surface; B qualifies it as the same axis.

## Q-005 — Does "release notes are behavioural authority" govern future Antigravity qualification?

Status: answered 2026-09-28
Answer: no (Tom). The ruling stays scoped to the five hops through `1.2.2`;
Contract 029 "Artifact Authority Without Public Source" governs.
Asked: 2026-09-28

On 2026-09-15 Tom ruled that official release notes were the behavioural
authority for the five Antigravity hops through `1.2.2` (Research 323; the source log is in Git history at `49b0d308`). The source
scopes it to that task. Contract 029 "Artifact Authority Without Public
Source" makes the shipped artifact the authority and a changelog discovery
evidence only.

Recommendation: keep the ruling scoped to those hops. Future Antigravity
qualification follows Contract 029: artifact evidence decides, release notes
only find candidates, as Research 353 and 359 did.

## Q-006 — Should Cline ACP expose its advertised `session/load` operation?

Status: open
Asked: 2026-10-08
Gate owner: Tom, Swallowtail operator

Research 397 confirms that Cline advertises and implements ACP `session/load`.
The qualified Swallowtail route creates a new session and sends one bounded
prompt, but has no public load or resume mapping. This qualification preserves
that surface and adds no public operation.

Decision tree:

- If exact Cline artifact evidence shows that its advertised load cannot meet
  the `load_session` cell under the approved route, classify the cell as
  `provider_limitation` and cite that frozen evidence.
- If Cline supports the required load semantics and Swallowtail lacks the
  mapping, classify the cell as `producer_gap` and give the exact Queue lane
  key that will build it.
- Keep the cell `evidence_pending` while the gate owner has not established
  which condition applies; keep this question open until the matrix cites one
  of those outcomes.

Evidence gate scope:

- `cline.acp load_session`

<a id="q-007"></a>
## Q-007 — Does the consumed Copilot no-child original initialize stand under decision e05b517f?

Status: answered 2026-10-09
Answer: Approve (Tom). Limited archive-identity exception for this one
consumed diagnostic; see
[Consumed Initialize Archive Exception](contracts/023-harness-operation-isolation-and-native-boundary.md#consumed-initialize-archive-exception).
Asked: 2026-10-09

The brief named archive `98640ca0…`; the executed pairing is executable
`df347f…` with archive `f254651a…`. The stop-rule breach is not erased.
Future identity mismatches still stop. Initialize was not reached; startup
cause unknown. Parent 119 and the exact `1.0.80` claim are unchanged.

<a id="q-008"></a>
## Q-008 — How should the frozen Copilot host-permission runner meet the Python format check?

Status: open
Asked: 2026-10-11
Gate owner: Tom (operator)

`ruff format --check` at the pinned `0.14.1` would reformat
`scripts/copilot-acp-host-permission-proof.py`. The change is formatting only:
the AST is identical (31,302 nodes). But the frozen task-173 plan, authority
and fake-pass records bind the runner's live sha256. Reformatting therefore
fails `check:copilot-acp-host-permission-task-173-binding`, and fixing that
means rewriting frozen evidence, which this lane does not do.

Options for the operator:

- Authorise a successor binding: a new plan and authority for the formatted
  runner, with historical records left unchanged. This may need new
  original-execution authority.
- Record the runner as an exempt frozen file in the Python format check, and
  leave it unformatted until its frozen chain closes.

Until answered, `check:copilot-acp-host-permission-format` fails on the
unformatted runner. Fixture detection and the other scripts are unaffected.

Planner scope clarification (2026-10-12, under Tom's existing "Go fo rit"
approval; not a new Tom ruling):

- The selector task closes on selector capability only. It does not choose
  option A (successor binding) or option B (exemption). Neither is authorised
  here.
- `scripts/copilot-acp-host-permission-proof.py` stays byte-for-byte as at
  its pre-task base (git blob `adaebe553883a464b831c2e8453811d5ed9811d9`).
  Every historical plan, authority, fake-pass, original record and consumed
  guard stays unchanged.
- `check:copilot-acp-host-permission-format` is an expected negative baseline.
  It must keep exiting nonzero and naming the runner. It is not in mandatory
  QA. No suppressed diagnostic or exclusion is added.
- No original invocation, successor execution binding or provider action is
  needed or permitted by this clarification.
- Choosing A or B, or leaving the runner as the permanent baseline, remains
  with Tom. This question stays open for that decision.

## Q-009 — How should `claude-agent.sdk` qualify past `0.3.317`?

Status: open
Asked: 2026-11-12
Gate owner: Tom (operator) for item 1; item 2 answered; item 3 closed

Research 480 freezes npm `latest` `0.3.322` (native `2.1.322`) and the hops
`0.3.318`, `0.3.319`, and `0.3.321`. Each bundled native binary changed.
Identity manifests, CLI digest bindings, and wrapper similarity do not prove
the selected native behavior. The package and native claims stay through
`0.3.317`/`2.1.317`, with the default pair, the registered-tool live point,
and the Node segment unchanged.

Decision tree:

1. Native behavior proof for the selected SDK paths on Darwin arm64 and Linux
   x64 (`2.1.318`, `2.1.319`, `2.1.321`, `2.1.322`). Either authorise a
   hermetic run of the vendor native with an isolated home and a local fake
   endpoint, or accept a static control-flow method. Without one, the cells
   stay unqualified.
2. `permissions.denyInsteadOfPrompting` on this route, from `0.3.319`.
   **Answered 2026-11-12: preserve it under a distinct SDK behavior revision.**
   Tom's decision `d5a14f90-e837-42b6-9818-9b5e74a25547` v2 chose preservation
   over refusing an affected pair; the planner relayed it and applied it as
   `task232` brief v2
   (`sha256:e6abac0dd3f10494a0a28fb379a7b183582bf87848b774fb1651bd18bf67cc1e`).
   The distinct revision is `claude-agent.sdk-v2`; every maintained claim stays
   on `claude-agent.sdk-v1` and no pair moves. The contract and its fake
   regressions are
   `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-v2/managed-denial-contract.json`.
   What this answer does not grant: original-artifact execution, a qualification
   extension, or any statement about a real native's behaviour.
3. Residuals at `0.3.321`: `promptHold`, `initializeWritten`, and the `Ki`
   telemetry callers. Planner closes these with the native proof.

Cells covered: `claude-agent.sdk` package and native rows in the route matrix
and the prepared-integration guide, from `0.3.318` through `0.3.322`.

Stage evidence (2026-11-12, Research 482):

- Item 3 is closed. The three residuals are classified from exact bytes:
  `promptHold` is unchanged and unselected (`hooks: {}` cannot arm it),
  `initializeWritten` is unchanged, and the new `0.3.321` numeric-key
  telemetry gate on the shared key sanitizer sits on the tag and
  custom-title writes, not the turn path. `0.3.321 -> 0.3.322` moves no
  selected surface. Every selected surface digest across `0.3.317`,
  `0.3.318`, `0.3.319`, `0.3.321` and `0.3.322` is in
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/selected-flow.json`.
- Item 2 is answered: preserve the managed denial under `claude-agent.sdk-v2`
  (Tom, decision `d5a14f90` v2). The distinct revision names four guarantees -
  no fabricated permission request, no admission claim for an unmediated call, a
  turn that continues until its native result, and a mediated denial
  distinguishable by its callback and
  recorded decision - and the contract fixture pins each one against fake peers.
  No claim adopts v2: the package and native segments, the pair map, the
  behavior revision used by every claim and the shipped sidecar stay as they
  were, because probe evidence must precede a claim extension.

- The source-proven half of Item 2 stands behind that answer:
  `permissions.denyInsteadOfPrompting` is an administrator-tier managed-policy
  key; `settingSources` selects only the user, project and local tiers, so
  `settingSources: []` does not exclude it; the SDK's parent tier comes only
  from `Options.managedSettings`, which the sidecar never sets, so
  `--managed-settings` is not emitted; the native's own administrator-tier
  discovery is opaque binary behavior. The direction is answered - preserve it
  under `claude-agent.sdk-v2` - and only the native evidence that a real binary
  honours it remains unproven.
- A temporary *probe-only* admission now exists (planner, 2026-11-12, question
  `e36390b98c373c94c17185abe8fcf9a5cbd14dcde0d2b720cfa8320a76caec7e`): the
  ledger `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/probe-admission.json`
  binds every proposed tuple, the selected option surface and the fake peer
  identity, admits harness-only fake runs, and carries no production admission.
  It refuses a production ledger, a foreign authority, a substituted ledger or
  peer, a non-loopback endpoint and an unjoined peer, and every record it writes
  is marked probe-only. Original-artifact mode is refused unconditionally
  because the harness has no kernel-level confinement, and the ledger names that
  missing capability. Production claims, the maintained pair map and the shipped
  sidecar are unchanged. Vendor execution is still unauthorised.
- Item 1 has its harness, not its proof.
  `probe-plan.json` fixes the artifacts, digests, argv, isolation, bounds and
  cleanup for both selected platforms, and the fake-backed harness proves the
  pipeline and its observation points on the accepted pair. Both the prepared
  driver and the sidecar admit only the maintained pair map, so every planned
  hop above `0.3.317` is refused before the SDK loads. The permission direction
  is settled; the open gates are the exact native evidence and the execution
  admission, and neither is granted by this evidence.

Confined fake continuation (task 234): the private
[probe contract](operations/claude-agent-sdk-confined-probe.md) owns the
Darwin/Linux fake enforcement boundary, executable proposal and remaining platform,
settings and interpreter-custody gates. The new launcher never enables original
mode. Existing Ubuntu/macOS Node 22.23.2 jobs supply observed fake-platform
identities; canonical administrator injection and custody remain separate gates.
Its fake observations do not close item 1, qualify Node, adopt the v2
revision in production or qualify any SDK/native pair. Linux x64 admission,
exact original-artifact authority and Nucleus proof execution remain open.

The planner's 2026-11-13 continuation requested a reviewable
[dedicated ephemeral-CI proposal](operations/claude-agent-sdk-ephemeral-ci-proposal.md)
for genuine administrator fixture setup. Its unapplied job diff and scratch-only
ownership/refusal/cleanup rehearsals grant no privileged execution, canonical
injection proof, custody ruling or native activation. The environment-setup
authority request must name the exact reviewed job/source before execution.

Tom's decision `32d31615-702b-4e3b-a972-56b4fb18bae2` v2 on 2026-11-13
chose "Dedicated fixture-only CI." The exact proposed workflow is applied;
the preceding proposal-only restriction is superseded for preparation alone.
Dispatch still needs authority naming the resulting workflow/source SHA and
GitHub workflow registration. No privileged local setup or original activation
follows from this answer.

