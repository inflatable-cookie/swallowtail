# g05.036 v0.4.4 Release Readiness

Status: ready; Card 154 merged candidate `49c9e3b2`; dependent exact-tree Desktop acceptance remains; old candidate `0673541d` remains parked
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-09
Depends on: Contract 036 with the Hosted Gate Delegation clause; immutable `v0.4.3` at `cbd4ddc8`; cards 119, 120, 121; the release playbook (card 109)
Vision tags: source release, consumer proof, Claude route

## Purpose

Get the first-turn model rejection evidence (cards 119-121) to Bovine Desktop,
which pins tags, as patch `v0.4.4`. This is the first lane run on the
simplified shape: cheap local gates only, the hosted `workflow_dispatch` run
as the heavy evidence, tag request in the same turn the candidate SHA exists,
consumer smoke on the tag. Its wall clock is the g05.034 acceptance measure.

## Operator Scope Change — 2026-09-07

The operator withdrew tag authorization for the prepared `0.4.4` candidate
(`0673541d`, run `34135201752`). The next Swallowtail release must be
covered by linked Bovine Desktop evidence across the Claude, Codex, and Grok
routes for MCP/tools, permissions, skills, model inventory/selection,
cancellation/reconciliation, and packaging, with explicit
unavailable-capability dispositions. CI and fixture parity are insufficient.
Desktop Coordinator `914728cd` is the sole integration/test owner; Swallowtail
supplies producer seams only. The candidate freeze is lifted so required
fixes can merge; the candidate may be revised and re-prepared. Card 123
waits. Reconciled 2026-09-07 (g05.035): 14/18 producer cells merged, three
more promoted (cards 125-127), Grok MCP/tools/skills withheld and blocking
the scope until card 128's probe settles the upstream question or the
operator decides the fallback. Desktop live acceptance is the release gate;
producer merges are not.

### Grok Gate Blocked On Provisioning — 2026-09-07

Isolation is provisioned and the harness is self-proved; the blocker is
account quota. Desktop extracted the exact `1.0.4` and `1.0.5` binaries into
per-version isolated homes from cached archives, both version identities
verified against frozen source `63e34641`, and all four offline fixture
capsules pass, so the probe's own four-verdict behaviour is proven before any
live turn. Those homes are empty: no host credentials were copied and no
authentication was performed. The operator's Grok account usage is exhausted,
so live rows are blocked `account_quota_exhausted` and no live provider call
was attempted.

**Quota hold lifted 2026-09-07.** The operator added top-up credit and
reported capacity restored, which is the capacity confirmation the resumption
condition named. The two bounded live probes are authorized under the existing
gates: exact `1.0.4` first, then `1.0.5`, as separate evidence segments, one
bounded prompt each, run by the existing retained probe owner through the
Desktop Coordinator with no duplicate run and no new owner. A top-up is a
capacity report, not live acceptance. An auth or quota rejection captures a
bounded result and stops; there are no repeated retries. The three-provider
release gates are unchanged.

**First live result 2026-09-07: admission proven, invocation unproven.** Both
segments ran exactly once and both returned `inconclusive`, but on each the
outbound `session/new` carried a non-empty `mcpServers` list and the
disposable echo server observed `initialize` and `tools/list`. Grok Build
accepted the client-declared server, connected to it, and enumerated its
tools; the model did not call it. That refutes `ignores_client_mcp` and
`rejects_client_mcp` at the protocol level and leaves only the invocation
question open, so the decision tree's "second inconclusive is treated as
ignores" branch must not fire. Card 133 repairs the verdict oracle, after
which Desktop runs the single authorized rerun per segment. The four cells
stay `evidence_pending` until then.

The history below stands as the record of the hold.

The operator subsequently completed isolated login for both segments, and
confirmed the account remained out of usage afterwards. **Grok account quota
was an external blocker**: it is outside the project's control, it is not a
planning question, and the scope decision is settled — the operator has not
authorized narrowing the release scope, so Grok MCP/tools stay required and
unproven rather than becoming a withheld cell. Do not re-open that choice, do
not retry, and do not request further logins.

The card 128 probe therefore has no verdict, the four `grok-build.acp` cells
stay `evidence_pending`, and `v0.4.4` cannot reach a tag request until the
gate resolves. Substituting a different version (for example `1.0.13`) is
refused because it is a different evidence segment. Bounded live Grok probes were
gated on that capacity confirmation, which has now arrived: two bounded
prompts total, one per segment, on the already self-proved harness.
Authenticated isolated homes and the offline evidence are preserved for that
run. All other producer, review, merge, and provider-free work continues.
Machine paths stay out of tracked docs; the retained probe owner holds the
location receipt.

### Evidence Policy: Prove Before Tag — 2026-09-08

Operator ruling. Both consumer gates are proven against exact-SHA
source-linked local Swallowtail main before any tag, not after it. Desktop
resolves the crates through `effigy deps link` against local source, so a
runner can be built and preflighted at an exact SHA without a tag and without
an untagged consumer pin; the committed consumer manifest stays on its
released pin and the linked lock is machine-local and never committed.

This keeps the operator's three-route scope intact rather than narrowing it,
and it inverts the pattern that cost `v0.4.2` and `v0.4.3`, where the tag came
first and the consumer defect came after.

Binding condition: the eventual tag must sit at the exact SHA the gates
proved, or at a commit with an identical tree. Otherwise the evidence does not
transfer to what is tagged. This is the same rule the release playbook already
applies to the qualifying hosted run.

Sequence: Desktop builds the runner under its own operator authority (card 305
excludes it); the Claude gate runs on the exact linked SHA; the Grok rerun
runs independently under its own fresh budget; both capsules return to
Chatterbox; then the candidate is prepared at the proved SHA and the tag
request goes to the operator.

### Grok Gate Accepted — 2026-09-08

Desktop returned exact source-linked `1.0.4` and `1.0.5` capsules, each from one
separate prompt segment, and both scored `accepts_client_mcp`. Research 295
holds the immutable identities. This closes the Card 128 live evidence gate
but does not itself complete the producer: Card 143 qualifies the already
merged Card 118 seam and settles selected-skill delivery independently. The
Claude Card 132 gate remains outstanding. No candidate, tag, or release
authority follows from the Grok return.

### Claude Gate Stopped — 2026-09-08

Desktop's single authorized primary open used the exact source-linked Card 132
tuple and returned typed
`open.failed.swallowtail.claude-agent.sdk.open_rejected`. No turn, registered
tool, control attempt, or retry ran. Research 296 freezes the capsule and
Desktop lifecycle; both Contract 061 cells remain unqualified. Card 144 owns
provider-free diagnosis and structured failed-open evidence. Another live gate
requires separate operator authorization after that result. No candidate, tag,
release, feature classification, or broader acceptance follows.

### Claude Exhausted-Credit Diagnostic Authorized — 2026-09-08

The operator confirmed the Claude account is currently out of usage credit and
authorized Card 145 to capture that state before top-up. Desktop may run one
fresh open against exact source-linked Swallowtail
`6a93f1d916945aa2b402df7329dc994570e005c8`, using Card 144's structured
receipt and `claude-sonnet-5`, the cheapest model in its already-admitted
Sonnet/Opus inventory. No prompt, tool call, control attempt, retry, or
qualification is authorized. A credited qualification suite remains a later,
separate authorization after the operator confirms top-up. No candidate, tag,
or release authority follows from this diagnostic.

### Claude MCP-Status Repair Gate — 2026-09-08

Card 145 completed through Desktop PR 172 but stopped before provider readiness
at bounded `mcp_status_invalid`; it observed no quota state. One open ran, no
turn/tool/control/retry followed, and cleanup was confirmed. Research 297
freezes the return. Card 146 now owns provider-free reconciliation of exact SDK
`0.3.259` MCP-status rows against the strict sidecar projection. Do not top up
for or run the credited qualification suite until that producer gate completes
and the operator separately confirms credit restoration and live authority.
Both Contract 061 cells remain unqualified. No candidate, tag, or release
authority follows.

### Claude Repaired Zero-Credit Diagnostic Authorized — 2026-09-08

Card 146 repaired the pre-readiness MCP-status projection mismatch and closed
at exact Swallowtail `0d120067cd260b1f5127835cab0b1a3ad020a29d`. The
operator confirmed Claude credit remains zero and authorized Card 147 to run
exactly one open with `claude-sonnet-5`. It sends no prompt and permits no
turn, tool, permission callback, control, retry, reconnect, or respawn. A
successful open closes immediately. This diagnostic does not qualify either
Contract 061 cell and grants no candidate, tag, release, top-up, or credited
suite authority.

Card 147 completed through Desktop PR 174. Open reached ready/connected state
while credit remained zero; no prompt ran. Card 148 then completed through
Desktop PR 175 and repaired the exact Contract 019 degraded-cleanup oracle
provider-free. Because the SDK initializes its query generator only on the
first prompt, Card 149 owns one final zero-credit first-turn diagnostic before
top-up. It runs one open and one cheapest-model prompt with no retry, tool, or
control action. Card 149 stopped at `mcp_server_failed` before provider
readiness or prompt submission. Card 150 now repairs the registered-MCP startup
path provider-free. The credited qualification suite remains separate, and MCP
will not be removed to bypass the failure.

Card 150 completed through Desktop PR 178. The mutable Cargo target-path race
was reproduced and repaired with an immutable content-addressed per-run courier;
24/24 registered opens passed under churn with MCP retained. The operator
confirmed Card 151 may now run exactly one zero-credit open and first prompt
through the repaired path. No retry, top-up, qualification, candidate, tag, or
release follows from that diagnostic.

Card 151 completed through Desktop PR 179: MCP authenticated, one prompt ran,
and the provider returned a typed failure. The immutable capsule proves
`api_error_status` and `terminal_reason` were present, but the sidecar discarded
their values. The operator authorized Card 152's provider-free structured-
failure projection repair and one later zero-credit diagnostic if the balance
has not been topped up. Neither task qualifies a Contract 061 cell or crosses
the release gate. Card 152 then completed through PR 297, with the structured
projection and safe classification accepted at exact-head review and merged
into `main` as `24f88fb8`; no provider run or later Desktop diagnostic occurred.

The later funded Desktop Card 318 suite passed all four independent controls
against that exact source. Research 301 freezes the accepted capsule and
review/merge/closeout chain. Card 153 is the provider-free Swallowtail
qualification and two-cell matrix reconciliation. It authorizes no additional
provider use, candidate preparation, tag, or release.

### Candidate Source Disposition — 2026-09-08

The prepared candidate `0673541d` is PARKED, not abandoned and not current.
It remains the candidate of record this roadmap and the release note name,
and the Desktop pre-release integration checkout pins it, so it is preserved
rather than deleted. It is also stale: `main` has advanced well past it with
cards 118, 125, 126, 137, 138 and more, so the eventual release will prepare
a fresh candidate at a newer SHA under card 122's successor rather than
tagging this one. Nothing is tagged from a parked candidate. Card 122 itself
is complete; its closeout thread has no remaining work.

### Final Exact-Tree Qualification — 2026-09-09

Card 153 completed the exact Claude SDK qualification, but the accepted live
capsules do not transfer to a new candidate: Grok used Swallowtail `04e9b2dd`,
Claude used `24f88fb8`, and current `main` has a third, materially different
tree. The operator authorized one consolidated final-candidate qualification
batch. Card 154 therefore freezes and merges the candidate first. The linked
Desktop gate then reruns the bounded release acceptance against that exact
merge SHA. Candidate preparation, Desktop provider execution, and tag creation
remain distinct queue and operator boundaries. No tag is inferred.

### Card 154 Candidate Merged — 2026-09-09

Card 154 completed the final-candidate preparation. PR 300 was independently
accepted at exact head `1fb5b16ceccbc1451c6b044dc2b8fabd13a194c6`, tree
`1a9db12742b68e839dfebe42f0633f5d1aa0265b`, and merged as
`49c9e3b291609c9ebf5b35a284c08302f3b8d5e3` with the same tree. Read-only
status inferred patch `0.4.4`; the one authorized prepare passed all cheap
gates; hosted run `34345060452` passed all 11 jobs; and the preparation receipt
digest was
`04a89847e14fc351bbcfdef2b48cb1d8c90282b3ea638db48043d1e8f6ab8feb`.

A later same-SHA hosted rerun, `34348374964`, failed only the Pinned MSRV
floor test twice. The qualifying run and local validation stayed green. This
nondeterministic validation failure is deferred without candidate change or a
prepare retry. The dependent Desktop gate must use merge SHA
`49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`; no tag is inferred.

## Runway

1. Card 154 prepared and merged one final candidate per the release playbook,
   with exact-tree review and qualifying hosted CI.
2. The dependent Desktop task links that exact merge SHA and runs the bounded
   final release-matrix acceptance. A failure stops without retry or tag.
3. Chatterbox presents the exact-SHA tag request only when both gates bind to
   the same candidate tree. The operator then decides whether to tag and push.
4. A successor to cancelled Card 123 runs source-consumer and Desktop repin
   evidence after the tag.

## Release Boundary

No card creates or pushes a tag. The operator authorizes the exact SHA. No
publication, GitHub Release, binaries, sidecars, installers, or consumer
mutation.

## Feature Freeze

From Card 154's planning promotion until the operator's tag decision, no other
Swallowtail implementation PR merges to `main`. Queue closeout documentation
may advance the branch, but the candidate merge SHA remains the release target.

## Batch Cards

- [154 v0.4.4 Final Candidate Preparation](batch-cards/154-v0-4-4-final-candidate-preparation.md) — ready; operator authorized 2026-09-09
- [122 v0.4.4 Candidate Preparation](batch-cards/122-v0-4-4-candidate-preparation.md) — complete; parked candidate `0673541d`
- [123 v0.4.4 Consumer Proof And Tag Capsule](batch-cards/123-v0-4-4-consumer-proof-and-tag-capsule.md) — cancelled after the scope change; successor follows an authorized tag

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.

### Card 154 Manifest

| Field | Card 154 |
| --- | --- |
| Readiness | complete; PR 300 merged candidate `49c9e3b2`; dependent Desktop exact-tree acceptance remains; no tag authority |
| Prerequisites | no active Swallowtail implementation worker; release freeze in force |
| Completion conditions | current release note and all three `0.4.4` baselines; patch status; exactly one new prepare transaction; candidate PR; exact-head independent review; qualifying hosted CI; merge SHA/tree and run ID returned; no tag |
| Owned mutable paths | the exact release/version, README, changelog, `0.4.4` baseline, release-note, and card Result surfaces named by Card 154 |
| Reserved shared closeout surfaces | roadmap, generation/index, handoff, log, and release-index closeout surfaces |
| Forbidden paths | Rust source/tests; earlier baselines; provider or Desktop work; dependency currentness; tag/publication/artifact/consumer mutation |
| Approved concurrent siblings | none in Swallowtail; Desktop Card 321 may proceed independently before the dependent live gate |
| Serial edges | dependent Desktop exact-tree acceptance follows the candidate merge; exact-SHA operator tag decision follows its accepted return |
| Worker capability class | general release-preparation worker; automatic adequate pool; no provider credentials or tag authority |
| Acceptance evidence | status JSON; prepare JSON/digest; baseline inventory; accepted head/merge/tree; hosted run ID; wall clock |
| Review oracle | one immutable tree supports release content, baselines, exact-head review, hosted CI, and the later Desktop source link |
| Stop conditions | dirty start; non-patch status; consumed prepare failure; freeze violation; review/CI tree mismatch |
| Escalation owner | operator through Chatterbox; queue coordinator for mechanical blockers |

### Card 122 Manifest

| Field | Card 122 |
| --- | --- |
| Readiness | ready on card 124's merge; the coordinator dispatches on that notice without a Chatterbox round trip; prepare authorization under the operator's standing grant |
| Prerequisites | cards 121 and 124 merged; clean canonical `main`; freeze in force |
| Completion conditions | `docs/releases/0.4.4.md` and index entry from cards 119-121 plus any merged additive tranches; patch class from the semantic API diff; read-only status inferring `0.4.4`; lock in sync; exactly one `effigy --json release prepare --yes --check-gates --version 0.4.4` on the cheap-gate table; the qualifying hosted `workflow_dispatch` run id recorded in the release note; candidate PR; review and hosted run in parallel; merge on both green; candidate SHA and run id reported to Chatterbox immediately; wall clock from dispatch to merge recorded in the Result |
| Owned mutable paths | as card 106 with `0.4.4` for `0.4.3`; no gate-script version edits (card 110 derives them) |
| Reserved shared closeout surfaces | the usual roadmap, index, generation, log, and release-index surfaces |
| Forbidden paths | every `crates/**` source and test path; every `0.4.3` and earlier baseline; contracts; guides other than the release note; claims |
| Approved concurrent siblings | none; freeze |
| Serial edges | the operator's tag decision follows; card 123 follows the tag |
| Worker capability class | release-preparation worker with the playbook; no credentials; no tag authority |
| Acceptance evidence | status output; prepare JSON; the `0.4.4` baseline files; PR head, merged SHA, qualifying run id; wall clock |
| Review oracle | one exact tree supports every release statement; the qualifying run is `workflow_dispatch` at the SHA to tag or an identical tree |
| Stop conditions | a cheap gate fails on a real defect; status infers anything but `0.4.4`; no qualifying run can be obtained |
| Escalation owner | operator via Chatterbox; coordinator for mechanical blockers |

### Card 123 Manifest

| Field | Card 123 |
| --- | --- |
| Readiness | ready when the operator-authorized `v0.4.4` tag exists |
| Prerequisites | the tag; peeled SHA verified by Chatterbox |
| Completion conditions | `effigy package:source-consumer` passes from a clean detached checkout of the tag; capsule (tag, object, peeled SHA, message, run id) relayed to the Acowtancy coordinator; release note status flipped to tagged; Contract 036 identity updated; the Desktop real-Send result with card 119's observer evidence attached when it arrives |
| Owned mutable paths | this card's `## Result`; release note status line; Contract 036 tagged identity lines; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | the usual roadmap, index, generation, and log surfaces |
| Forbidden paths | every crate; every baseline; the candidate |
| Approved concurrent siblings | held feature PRs resume merging after the tag |
| Serial edges | none |
| Worker capability class | release-evidence worker; the Desktop side runs under the Acowtancy coordinator |
| Acceptance evidence | source-consumer output at the tag; the Desktop capsule |
| Review oracle | both proofs use the exact tagged SHA |
| Stop conditions | the Desktop turn fails on a further route defect (typed code plus observer evidence; next patch, never a retag) |
| Escalation owner | operator via Chatterbox; Acowtancy coordinator for the Desktop run |

## Acceptance

`v0.4.4` tagged by the operator at an exact SHA carrying cards 119-121; the
lane's wall clock recorded against the g05.034 target of thirty minutes from
gates green to tag request.
