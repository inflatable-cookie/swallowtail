# g05.029 Claude SDK Interactive Parity

Status: stopped; honest evidence stop — consumer multi-turn editing session unproven (clause 1 fail); permission modes, default preservation, landing order, and v0.4.1 carrier pass; audit PR 307 merged as `55595c38` after independent exact-head review `5607443298` with no follow-ups
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-09
Depends on: Contracts 017, 019, 023, 029, 036, 041; Research 278 and 280; completed g05.022 and g05.023; tagged `v0.4.0`
Vision tags: Claude route, consumer parity, interactive session, discovery

## Purpose

Make `claude-agent.sdk` a full editing harness for consumers. The Bovine
Desktop requirement of 2026-09-04 (operator-confirmed) is that Claude support
must match what Paseo and T3 Code offer. Today the sidecar hard-codes
`Read`, `Glob`, `Grep`, `permissionMode: "default"`, empty MCP servers, and
rejects resume; ACP sessions bind read-only. No Claude route can drive an
editing chat.

Research 278 layered the route exactly this way: layer 1 shipped in
`v0.4.0`; layer 2 is permission modes, model and effort, resume and fork;
layer 3 is MCP; layer 4 is Bash with Contract 023 and 041 evidence. This
roadmap runs those layers in the consumer's priority order.

## Posture

Every write tool runs under an explicit `AmbientHost` posture with
consumer-mediated per-call admission (Contract 017: an ambient harness may
run write tools without a bounded filesystem claim; Contract 023: tool
allowlists do not contain the process). The read-write lease is a location
and callback scope, not containment. The default remains read-only; the
consumer opts in per prepared profile. `bypassPermissions` is never
admitted. In `acceptEdits` mode the SDK auto-approves edits, so per-call
visibility holds only in `default` and `plan`; the guide states this.

## Runway (priority order from the consumer requirement)

1. Read-write session (delivered as card 080): (`Edit`, `Write`, `MultiEdit`) through
   per-call admission on a read-write lease, `permissionMode` at open, and
   `setPermissionMode` mid-session. Carrier for `v0.4.1`.
2. Bash under mediation with intact tool input (delivered as card 081 through PR 233 at `97f37e4d`).
3. Mid-session model and effort change with confirmed values (delivered as card 082).
4. Resume, `resumeSessionAt`, and session listing; fork optional (delivered as card 083: resume as Contract 017 resume with lease cwd and account checks).
5. Client MCP servers on open (delivered as card 084: declared MCP servers, mediated MCP tools, per-server status).
6. Grok `grok-build.acp` answerable permission requests, or an
   explicit labelled activity-only posture (delivered as card 085; stable matrix disposition remains `No`).
7. Identity research on discovered native Claude Code and
   discovered or minimally bundled Node against the SDK wrapper (delivered as card 086);
   then the five SDK axes move from qualified-only exact pins to Codex-style
   qualified ranges with stable-newer allowed (held at the stopped card 087 gate: Research 287 admits no range).
8. Install guidance surfaced through discovery diagnostics for
   Claude Code, Codex, and Grok, after one provider-neutral vocabulary
   amendment (delivered as card 088).

Out of scope for the whole roadmap: hosted OAuth in the application, API-key
routes, Bedrock.

## Release Edge

Items 1 and 2 are additive under Contract 036 (default unchanged, opt-in per
profile, no public item removed), so they ship as patch `v0.4.1`. When card
080 merged, Chatterbox compiled the `v0.4.1` release-readiness roadmap (g05.030) on the
`v0.4.0` precedent: exact-head review, the eleven local gates, exact-SHA CI,
the external source consumer, and one operator-authorized Bovine smoke.
Cards 081-084 targeted `v0.4.2`; 085-088 followed their evidence. Nothing here
forces a minor.

## Folded Card Evidence (g05.038)

- 080 — delivered; PR 221 permission policy and PR 224 ambient read-write editing.
- 081 — complete; bounded command view on the callback; PR 233 merged at `97f37e4d`.
- 082 — complete; PR 239 merged at `c8512290`.
- 083 — complete; PR 244 merged at `7cb08b1f`.
- 084 — complete; PR 255 merged at `8a377c1b`.
- 085 — complete; PR 243 merged at `8bbeceb4`; stable matrix disposition remains `No`.
- 086 — complete; PR 241 merged at `d127837f`; no claim change.
- 087 — stopped; Research 287 admits no range; five exact QualifiedOnly pins unchanged.
- 088 — complete; PR 242 merged at `80e004b4`.
- 089 — complete; bounded-profile exclusion retained; PR 223 merged as `e4399790`.
- 105 — complete; PR 245 merged at `217de072`; consumer-critical terminal code and result fields.
- 108 — complete; PR 289 merged at `8e6669c8`; optional read-only working resource for the response-only child.
- 119 — complete; PR 268 merged at `cfb0b106`; bounded observer evidence for the first-turn model rejection.
- 120 — complete; PR 269 merged at `4a27676d`; loaded SDK version verified at open.
- 121 — complete; PR 270 merged at `8cf1fe9f`; terminal state after first-turn rejection.
- 124 — complete; PR 271 merged at `02003ecb`; bounded manifest lookup and one model-id predicate.

## Retired Card Dispatch (g05.038)

Promoted planning commit: the `main` commit that introduced this file.
Card 080 was approved concurrent with g05.009 cards 074, 075, 076, and 079 (all complete).
Card-level dispatch is retired: every card above is complete or stopped, and
serial edges are now owned by this task. The per-card manifests below are
folded into the evidence ledger above; only the stopped 087 gate keeps future
scope, recorded as a held gate at the end of this file.

## Acceptance

The folded cards ran. The original five-clause target is not fully proven.
See Result.

## Dispatch

| Field | g05.029 acceptance audit |
| --- | --- |
| Readiness | ready; every implementation item is complete or stopped and the exact card, release, live-capsule, PR, and Git history can score the parent task without new provider work |
| Prerequisites | Contracts 017, 019, 023, 029, 036, and 041; Research 278, 280, 286, 287, and 301; folded cards 080-089, 105, 108, 119-121, 124, and 153; tags `v0.4.0..v0.4.4`; Desktop Cards 318 and 323 plus their immutable capsules |
| Completion conditions | one source-linked ledger scores all five Acceptance clauses independently; distinguish provider-free capability/mediation proof from an actual consumer-driven multi-turn editing session; prove open and mid-session permission selection plus bypass rejection; compare the default profile and guaranteed behavior at `v0.4.0`, `v0.4.1`, and current main; reconstruct launch, PR-merge, and dependency order for later runway items and identify any reopening of the Card 080 seam; prove the exact Card 080 content and gates present at the `v0.4.1` tag; update this task and one log |
| Owned mutable paths | this task; one new `docs/logs/2026-09-09-g05-029-*.md`; one `docs/logs/README.md` index line; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`; queue coordinator edits these after merge |
| Forbidden paths | Rust and tests; Cargo manifests and lockfile; sidecar assets; baselines; guides and matrices; contracts, architecture, specs, research, and release notes; tags; every other task body; all Desktop paths |
| Approved concurrent siblings | none; this is the last ready generation task and owns task-level closeout |
| Worker capability class | evidence-only cross-history acceptance audit; Git, GitHub, queue, tagged-source, and immutable consumer-capsule correlation; documentation only |
| Acceptance evidence | exact tag peels and trees; PR heads, merges, reviews, and ancestry; provider-free mediation and permission-mode tests already recorded at accepted heads; release baselines and notes; Desktop capsule prompts/verdicts; current guide/matrix claims as corroboration only |
| Review oracle | “a consumer drives” requires an observed consumer operation, not a fake SDK or compile test; multi-turn, editing, and pre-effect mediation must coexist in one admissible evidence chain; delivery order is scored from durable dependency and merge chronology rather than card-number prose; default preservation compares behavior and selection, not only API shape; Card 087's stopped no-range result is preserved and does not silently become acceptance or new scope |
| Stop conditions | source identities disagree; a clause cannot be proved without a new provider call, consumer mutation, runtime change, release mutation, or reopening Card 087; consumer evidence lacks the exact Swallowtail source identity or required prompt/tool/turn observations |
| Escalation owner | Chatterbox for acceptance semantics or follow-up planning; operator for any new live-provider, consumer, compatibility-range, or release authority |

## Result

Honest evidence stop. Log:
`docs/logs/2026-09-09-g05-029-claude-sdk-interactive-parity-acceptance-audit.md`.

| Clause | Verdict |
| --- | --- |
| A consumer drives a multi-turn editing session on the SDK route with every tool call mediated before it runs | fail. Provider-free tests prove write mediation. Live Card 318/323 capsules are four independent one-turn registered-tool controls on `desktop/reconcile`, not `Edit`/`Write`/`MultiEdit`, and not two user turns in one session. `v0.4.1` had no pre-tag application drive; the first Desktop open failed; the `v0.4.2` smoke never reached Chat |
| Permission mode is selectable at open and changeable mid-session | pass. PR 221 provider-free tests at accepted head `16ad903b` / merge `80d69dc6`: selected mode on open, confirmed mid-session `plan` then `default`, bypass/`auto`/`dontAsk` rejected. Live capsules used `permissionMode: default` only |
| The read-only default and `v0.4.0` behaviour are unchanged | pass. Default constructor remains `read_only()`: `Read`/`Glob`/`Grep`, `default` mode, read lease across `v0.4.0`, `v0.4.1`, and current main. Later additive fields stay omitted unless opted in |
| Later scope lands in priority order without reopening the card-080 seam | pass. Merge ancestry respects promoted serial edges (`0acba239`). Concurrent 085/086/088 PR numbers are not landing order. Later profile/session edits are additive; `read_write()` still excludes Bash |
| `v0.4.1` carried card 080's items on the Contract 036 gates | pass. PRs 221 and 224 are ancestors of tagged `c3cce750`. Research 286 patch class; PR 229; workflow-dispatch `33969131592` six jobs; source consumer. Operator compression dropped the Bovine editing smoke; the release note states no application drove the candidate |

Card 087 remains a stopped no-range gate (PR 256 `54153c68`; five exact
`QualifiedOnly` pins). Coordinator closeout 2026-09-09 sets `Status:
stopped`: PR 307 merged as `55595c38` after independent exact-head review
`5607443298` (ready to merge, no follow-ups). No repair, live probe,
range widening, release, or tag is opened here.

## Card 080 Stop And Ruling

Card 080's first PR (221) hit the card's stop: shared preflight refused
`ReadWrite` with `Capability::ToolCalls` for every policy, although Contract
013 excludes consumer tools only in the bounded profile. On 2026-09-04 the
operator ruled to narrow the guard to the boundary claim. Contract 013 now
says so explicitly. Card 089 changes core and testkit; card 080 completes in
two PRs: PR 221 merges as it stands (permission modes, typed write refusal,
proved write mediation), then the same worker lifts the refusal after card
089 merges. Both ride `v0.4.1`.

### Held Gate 087 — Qualified Ranges (from stopped card 087)

Readiness was planned; ready after cards 084 and 086 (both complete).
Prerequisites were card 086 research promoted and card 084 merged. If Chatterbox
reopens this scope, the completion conditions are: five axes on the Codex range
shape with cited endpoints and explicit gaps; boundary identity tests; guide,
matrix, changelog, additive baseline; one PR. Owned paths would be
`crates/swallowtail-adapter-claude-agent/src/sdk/selection.rs`,
`src/sdk/asset.rs`, `tests/claude_agent_sdk_*identity*.rs`, the additive
baseline, guide and matrix version cells, and `CHANGELOG.md [Unreleased]`.
Acceptance evidence is identity tests at each boundary citing card 086 lines.
Stop if card 086 shows a surface change inside the range (return to Chatterbox).
