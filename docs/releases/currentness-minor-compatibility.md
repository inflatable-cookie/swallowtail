# Currentness Minor Source And Compatibility Ledger

Status: reviewed source and compatibility assessment at planning `main`,
followed by one prepared source-only `0.6.0` candidate under merge hold. It is
not a tag, claim change, publication, or release authorization.

Assessment head: `b5e4760c9c330da1e0b33cb769218e20ca7ed239`. Released line:
annotated tag `v0.5.4`, peeled commit `bb53587a3f6fe28d00e9d07fa3c8ba3f179e1451`,
tag object `3c0363430726febaffbc32726a56a7c4016da2db`. The assessment head
carried workspace version `0.5.1`; the candidate below carries the coordinated
`0.6.0` version and appears in [its own release note](0.6.0.md).

This ledger classifies what current `main` already contains against Contract
036 patch-versus-minor and Contract 029 qualification evidence. Queue owns
task sequencing. It does not authorize a release, rewrite matrices, or expand
provider authority.

## Candidate Preparation Record

The prepared candidate implements this ledger's finite scope. It is a source
candidate only: merge stays on hold until planner milestone QA and a
qualifying hosted release and MSRV run cover the exact reviewed head, and tag
creation and push need separate operator authority naming the exact SHA.

- **Baselines.** Every version-labelled baseline was restored byte-for-byte
  from the tag that owns it, and the `release/v0.5` baselines absent from
  `main` were added from `v0.5.2`, `v0.5.3` and `v0.5.4`, so the newest tagged
  baseline older than the candidate is `v0.5.4`. All 572 version-labelled
  files then matched the earliest tag containing their path. The `0.6.0`
  public-API, production-route and internal-dependency inventories were
  generated from the candidate afterwards.
- **Lease API.** All seven `v0.5.2`/`v0.5.4` lease methods are ordinary public
  API again with their released signatures, docs and released test
  assertions; runtime policy and lifecycle are unchanged. The
  `swallowtail-runtime` public API is identical to `v0.5.4`, and its source
  still differs from that tag only in `prepared_operation/tests.rs`,
  `prepared_operation/tests/support.rs` and `session_cleanup.rs`, which come
  from main-line work after the tag.
- **Security delta.** The candidate carries the approved `rustls` lock
  movement for RUSTSEC-2026-0346, -0347, -0381, -0382 and -0383 and the exact
  emSign distrust patches approved under Contract 023. They arrive after this
  assessment was written, change no public API, provider pin or MSRV, and do
  not move a qualification claim. Released tags keep their published trust
  state; no advisory is ignored.
- **Courier corrections.** The already-forward-ported authenticated courier
  idle and approval-wait corrections stay included; no second port exists.
- **Compatibility.** The candidate's semantic API is additive against
  `v0.5.4` with no removals, and the route inventory is the tagged 50 plus
  additive `opencode.acp`. The minor is forced by the consumer-visible
  guarantee changes this ledger classifies, not by an API removal.

## Frozen Comparison

`v0.5.4` is a courier idle patch on `release/v0.5` over immutable `v0.5.3`.
Its 40-package set and 50 production routes match `v0.5.1` through `v0.5.3`.
SDK, native, Node, sidecar wire, and courier pins stay
`0.3.259` / `2.1.259` / `22.23.2` / `swallowtail-claude-agent-sdk-jsonl-v1`.
Merge-base of this `main` and `v0.5.4` is tagged `v0.5.1`
(`e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27`), so the currentness sweep and the
released-line patches are not a linear hop.

Current `main` already carries the courier idle correction as a forward-port,
plus the currentness sweep, Opaque constructor narrowing, additive public API,
and one extra production route (`opencode.acp`). Before the candidate restored
tagged bytes, the working `release-baselines/production-routes-0.5.1.txt` on
the assessment head listed 51 routes while tagged `v0.5.1` and `v0.5.4`
inventories list 50. That temporary drift is now corrected: `0.5.1` records
its tagged 50 routes and the `0.6.0` candidate baseline holds 51.

## Why The Next Coordinated Release Is A Minor

Contract 036: compatible public API and guaranteed behavior are patch;
breaking public API or guaranteed behavior are minor. Provider qualification
is a separate Contract 029 axis and does not by itself pick the version.

These current-`main` changes are independently classified as minor or as
breaking-if-shipped:

| Change | Contract | Classification | Evidence |
| --- | --- | --- | --- |
| Opaque constructor refuses more than 32 raw exclusion yields and Opaque version, exclusion, or behavior-revision text over 256 UTF-8 bytes | 036, 029, Spec 015 | pre-1.0 minor; signatures stay, accepted constructor inputs shrink | Contract 036 Coordinated Pre-1.0 Version; Spec 015 |
| Antigravity child projection `SubagentStatus::Unknown` when the stream has identity and no child status | 036, 023, 045 | breaking lifecycle if shipped; inferred `Completed` no longer applies | Research 424; Contract 023 Antigravity Headless Denial And Child Status |
| Command Code plan-mode model selection from `1.73.0` (`featureModels.planning` may precede `-m`) | 036, 029 | guaranteed-behavior change; private observation only | Research 402; CHANGELOG Unreleased |
| Claude Agent SDK default pair moves from released exact `0.3.259`/`2.1.259` to `0.3.293`/`2.1.293`; maintained window is `0.3.284..=0.3.295` | 036, 037, 029 | shrinking the released pin out of the production window is a guaranteed-range shrink for upgraders; additive pair enum is source-compatible for new callers | Research 416, 420; Contract 037; `ClaudeAgentSdkPackageNativePair::DEFAULT` |
| Temporary mutation of version-labelled `public-api-0.5.1` dumps after the `v0.5.1` tag | 036 | candidate restored tagged bytes then generated the `0.6.0` baseline (resolved) | Contract 036 Public API And Documentation; Contract 037 pair-selection exception |
| Seven `v0.5.2`/`v0.5.4` public lease methods were `pub(crate)` or `#[doc(hidden)]` on the assessment head | 036 | removal versus `v0.5.4`; the candidate restores all seven as ordinary public API (resolved) | tagged `public-api-0.5.4/swallowtail-runtime.txt`; `check:lease-api-fixture` |
| Gemini ACP exact `0.63.0` permission and read-boundary restrictions (`.gemini` writes and redirection ASK_USER reject-and-cancel, real-path reads, protected `.env.*` except example/sample/template/dist, 64 KiB tool-output cap) | 036, 029 | consumer-visible restriction; pre-1.0 minor. Exact `0.62.0` tool-update mapping stays additive. HTTP MCP live honouring stays exact `0.61.0` | Research 415; behavior `gemini-cli.acp.v0.63.0-restricted-files` |
| Codex app-server `0.159.0..=0.161.0` read-only protection for an existing top-level `.aws` directory inside the approved writable root | 036, 023 | consumer-visible workspace-write narrowing; pre-1.0 minor. Contract 023 accepted the limitation; this ledger classifies the 036 consequence. Earlier app-server segments stay their prior revisions | Research 388; Contract 023 Codex App-Server Protected AWS Directory; behavior `codex.app-server.v2.protected-aws-workspace-roots` |

Additive qualifications, extra route `opencode.acp`, and additive public items
are patch-shaped on their own. They ride the same coordinated minor because
the breaking items and consumer-visible restrictions already force it.

Urgent consumer adoption of `v0.5.2`–`v0.5.4` stays on the released line. This
assessment does not move those tags.

## Restore Obligations And Their Resolution

Contract 036 and Spec 015 require the candidate to restore every older
version-labelled baseline byte-for-byte from its tag, prove that restoration,
then generate the new version's inventories. Before the candidate, the
assessment head carried a temporary current-source `public-api-0.5.1`
inventory, and these nine dumps differed from tagged `v0.5.1`:

- `swallowtail-core.txt` — additive `InterfaceCompatibilityClaim::has_version_interval`
- `swallowtail-adapter-claude-agent.txt` — 12-pair `ClaudeAgentSdkPackageNativePair` (`0.3.284`–`0.3.295`), `DEFAULT` `.293`, `with_package_native_pair`
- `swallowtail-adapter-antigravity.txt` — `ANTIGRAVITY_HEADLESS_RETRY_PIN_{NAME,VALUE}`, `with_effort`
- `swallowtail-adapter-gemini.txt` — `with_default_approval`, `with_permission_allow_once`
- `swallowtail-adapter-opencode.txt` — full `opencode.acp` prepared facade and HTTP MCP placement
- `swallowtail-adapter-copilot-cli.txt`, `swallowtail-adapter-goose.txt`, `swallowtail-adapter-kiro.txt` — HTTP MCP placement types
- `swallowtail-testkit.txt` — consumer-route ledger helpers

`swallowtail-runtime.txt` on the assessment head matched tagged `v0.5.1` (no
lease methods). Tagged `v0.5.4` adds seven public methods:

- `PreparedRegisteredToolBinding::lease_deadline`
- `RegisteredToolBridgeLease::{lease_deadline,next_call_deadline}`
- `RegisteredToolOpenRequest::{lease_deadline,with_lease_deadline}`
- `RegisteredToolOperationKernel::next_call_deadline`
- `RegisteredToolPreparation::prepare_with_lease_deadline`

On the assessment head, `prepare_with_lease_deadline` was `#[doc(hidden)]
pub`, and `lease_deadline` / `with_lease_deadline` on the open-request and
prepared binding were `pub(crate)`; a minor cut from that tree would have been
a breaking removal for `v0.5.4` consumers.

Both obligations are resolved. The restored `0.5.1` dumps again record tagged
bytes, and the additive entries above appear only in `public-api-0.6.0`. All
seven lease methods are ordinary public API again with their released test
assertions, the `swallowtail-runtime` public API is identical to `v0.5.4`, and
`check:lease-api-fixture` proves the released signatures from one external
consumer plus their presence in the generated `0.6.0` dump. This ledger still
does not rewrite tagged evidence.

## Included At Planning Main

Each row is already implemented and claimed on this head, with cited research
or contract evidence. Inclusion here is a compatibility classification, not a
new qualification.

### Core, runtime, and authority

| Item | Kind | Evidence |
| --- | --- | --- |
| Bounded finite Opaque construction: 1..=32 exact members, 256-byte text bounds, equality classify, `has_version_interval` | minor constructor narrowing plus additive introspection | Spec 015; Contract 029 Finite Exact Opaque-Set; `claim.rs` |
| First production multi-member Opaque set: `llama-cpp.attached` exact `b9910-f5525f7e7` and `b11429-d81235049`, QualifiedOnly | Contract 029 qualification on the implemented Opaque set | Research 421; route matrix |
| Courier idle after authenticate (Contract 063 listener) | patch-compatible; already tagged `v0.5.4` and forward-ported | [0.5.4 notes](0.5.4.md); CHANGELOG |
| SDK registered-tool lease split and per-turn usage | patch-compatible on the released line (`v0.5.2`); the candidate restores the seven public lease methods main had hidden | [0.5.2 notes](0.5.2.md); [SDK backport proof](sdk-v0.5.2-backport.md) |
| Antigravity identity-only child as `SubagentStatus::Unknown` | breaking lifecycle if shipped | Research 424; Contract 023 |
| Command Code private model-selection observation from `1.73.0` | minor guarantee; no public `model_request_start` activity | Research 402 |
| Shared ACP schema `schema-v1.20.0` through `schema-v1.24.1`; wire `protocolVersion` remains `1` | additive qualification | Research 406 |
| Node 22.23.2..=22.23.3 on Claude SDK and Pi sidecar Node axes | additive; preserves 22.23.2 | Research 387, 396 |

### Route families with evidence-backed current claims

Posture and ceilings below are current `main` truth from the route matrix,
feature matrix, CHANGELOG Unreleased, and the cited research. Old consumers
on `v0.5.4` keep that tag's pins.

| Family | Current claim on `main` | Classification vs `v0.5.4` | Evidence |
| --- | --- | --- | --- |
| `claude-agent.sdk` | QualifiedOnly package `0.3.284..=0.3.295` coupled to native `2.1.284..=2.1.295`; default pair `.293`; live registered-tool remains exact `.259` | minor: default and window leave released `.259`; additive pair selector is source-compatible for new callers | Research 420, 416, 301; Contract 037 |
| `claude-agent.acp` | `0.53.0`–`0.87.0` excluding `0.58.0`; HTTP MCP live honouring exact `0.79.0` and `0.81.2` | additive qualification; live honouring does not transfer | Research 373, 361, 364 |
| `claude-code.headless` | Deprecated v1 through `2.1.286`; maintained v2 `2.1.287..=2.1.294`; `2.1.295` UnverifiedNewer | additive private milestone | Research 423 |
| `claude-code.response-only` | v3 maintained `2.1.282..=2.1.293` | additive private milestone | Research 378 |
| `llama-cpp.attached` | exact Opaque set `b9910` and `b11429`; `v0.2.0`–`v0.5.0` gaps; semantic `v0.6.0` is source evidence for `b11429` | additive Opaque membership | Research 421 |
| `llama-cpp.owned` | exact Opaque `b10069` / `178a6c449` | unchanged; `v0.6.0`/`b11429` unqualified | Research 407 |
| `ollama.attached` | preserved through `0.35.1`; maintained `0.40.0..=0.40.11` on `ollama.native-text-v1.manifest-list-runner` | additive private milestone; fail-closed identity drift | Research 418, 448 |
| `gemini-cli.acp` | `0.51.0..=0.61.0` plus exact `0.62.0` (`v0.62.0-tool-updates`) and exact `0.63.0` (`v0.63.0-restricted-files`); HTTP MCP live exact `0.61.0` | `0.63.0` permission/read restrictions are minor; `0.62.0` is additive; live honouring stays exact `0.61.0` | Research 415, 362 |
| `gemini-cli.headless` | ceiling `0.61.0`; `0.62.0`/`0.63.0` UnverifiedNewer | stop, not inclusion | Research 417 |
| `deepseek-harness.local-server` | axis `deepseek-harness.web`; exact RCs `0.1.0-rc.6` through `0.1.1-rc.2`; `--no-open` from `0.1.0-rc.8` | additive exact RCs | Research 411 |
| `deepseek-harness.jsonrpc` | exact `0.1.0rc6` | unchanged | Research 412 (study only) |
| `copilot-cli.acp` | exact QualifiedOnly `1.0.80` | unchanged; later stables remain unqualified | Research 440, 441 |
| `kimi-code.acp` | QualifiedOnly through `0.38.0` | stop, not inclusion | Research 403 |
| `kimi-code.headless` | v2 through `2.1.1` excluding exact `2.1.0` | additive; Q-004 does not apply to this family | Research 413 |
| `kimi-code.local-server` | through `2.1.1` under AmbientHost (Q-004 B) | additive | Research 357; Q-004 |
| `grok-build.acp` | through `1.0.46`; `1.0.47`–`1.0.70` UnverifiedNewer | additive; ceiling unchanged | Research 385, 443 |
| `grok-build.catalogue` | `QualifiedOnly` window `1.0.30..=1.0.70`: deprecated `1.0.30..=1.0.45` on `models-text-v1`, maintained `1.0.46..=1.0.70` on `models-text-v2` | additive qualification with a new private behavior milestone; `1.0.30` stays admitted and keeps its revision | Research 443, 405 |
| `zcode.app-server` | existing identity; runtime `0.16.9` inside ZCode `3.14.4` is discovery only | stop | Research 408 |
| `muse-code.headless` | signed payload pin remains `0.2.1-R1215.1`; `1.4.4-R5419.1` is identity only | stop | Research 409 |
| `kiro.acp` | exact `2.21.4`; HTTP MCP live honouring exact that point | stop for later stables | Research 419, 394 |
| `cline.acp` | route table `3.0.55..=3.0.70` except unpublished `3.0.59`; `session/load` unmapped | additive qualification; Q-006 still open | Research 397 |
| `cline.headless` | `3.0.55`–`3.0.70` excluding `3.0.59` | additive | Research 390 |
| `goose.acp` | exact `1.50.1`\|`1.51.0`\|`1.52.0`\|`1.53.0` | additive | Research 392 |
| `codex.exec` | through `0.161.0` | additive | Research 410 |
| `codex.app-server` | through `0.161.0`; maintained `0.159.0..=0.161.0` on `codex.app-server.v2.protected-aws-workspace-roots` | `0.159.0..=0.161.0` `.aws` write narrowing is minor; earlier post-`0.155.1` segments stay their accepted revisions | Research 388; Contract 023 |
| `cursor-agent.{catalogue,acp,headless}` | through `2026.10.01-14929f9` | additive | Research 393, 395, 399 |
| `pi.rpc` | through `1.1.0` | additive | Research 383 |
| `pi.sdk-sidecar` | exact package points through `1.1.0`; Node `22.23.2..=22.23.3` | additive | Research 400, 396 |
| `opencode.http` | through `1.18.35` | additive | Research 376 |
| `opencode.acp` | through `1.18.35`; extra production route versus tagged 50 | additive new route on `main` | Research 377 |
| `antigravity.catalogue` | maintained through `1.3.1` except unpublished `1.2.18`; `1.3.2` UnverifiedNewer | additive | Research 380 |
| `antigravity.headless` | deprecated `1.1.9..=1.1.17` plus exact `1.2.11` with `AGY_CLI_MODEL_API_MAX_RETRIES=0` and required effort; `1.1.18..=1.2.10` hole | included at `1.2.11`; later points stopped | Research 359, 424; Q-003 |
| `bedrock.runtime` | SDK through `1.148.0` | additive | Research 381 |
| `bedrock.catalogue` | SDK through `1.161.0` | additive | Research 386 |
| `qwen.headless` | through `0.25.0` | additive | Research 375 |
| `qoder.headless` | through `1.1.65` | additive | Research 389 |
| `mistral-vibe.headless` | through `2.26.0` | additive | Research 384 |
| `deepagents.acp` | `0.1.30..=0.1.34` | additive | Research 391 |
| `oh-my-pi.rpc` | through `18.8.3`; `18.8.4` UnverifiedNewer | additive | Research 382 |
| `command-code.headless` | 33 exact points through `1.79.1`; model-selection v2 from `1.73.0` | minor guarantee plus additive points | Research 402 |

### Additive public API already on `main`

These are additions relative to tagged `v0.5.1` dumps. None remove a tagged
`v0.5.1` item. They are patch-shaped API growth on a minor that is already
required for constructor and lifecycle narrowing.

- Opaque `has_version_interval`
- Claude SDK pair enum and `with_package_native_pair` (default `.293`)
- Antigravity retry-pin constants and `with_effort`
- Gemini ACP approval builders used by the exact `0.61.0` HTTP MCP path
- `opencode.acp` prepared facade
- Copilot, Goose, and Kiro HTTP MCP placement types (emission is not live
  honouring)
- Testkit consumer-route ledger helpers

## Compatibility Versus `v0.5.4` Consumers

Preserve the released line as the home of old consumers. The minor does not
rewrite `v0.5.4`.

Required for an upgrade from `v0.5.4` onto the minor:

1. The seven lease methods are restored as ordinary public API; a silent
   hide would have been a breaking removal.
2. Name the SDK default change: callers who previously got implicit
   `0.3.259`/`2.1.259` get `0.3.293`/`2.1.293`. The maintained enum has no
   `.259` variant. Research 301 live evidence stays bound to `.259` and does
   not transfer. Putting `.259` back into the selectable set is a product
   ruling, not a silent candidate choice.
3. Keep Node `22.23.2` in the qualified Node segments.
4. Keep wire `swallowtail-claude-agent-sdk-jsonl-v1`.
5. Treat `opencode.acp` as an additive route, not a replacement of
   `opencode.http`.
6. Restore tagged `public-api-0.5.1` bytes before generating the candidate
   baseline, then record the additive and breaking deltas against `v0.5.4`
   as well as `v0.5.1`.

`v0.5.4` courier behavior is already on this `main`. No second courier port
is required.

## Unresolved Qualification Gates

Each row names a finite next proof, design, or implementation and the
existing Queue lead. Do not duplicate these tasks. The Grok Build catalogue
default-not-in-list header gate closed in Research 443 with a same-contract
private mapping and a two-segment window claim.

| Gate | Next bounded work | Existing lead |
| --- | --- | --- |
| Antigravity headless past `1.2.11` (Mac resource/RPC/permission, retry-budget, approved-environment auth, and Linux x64 selected control flow; child Unknown already adapted) | Close those non-Windows evidence gaps, including the unmapped Linux x64 selected control-flow proof (Research 424 remaining gate 5). Keep exact `1.2.11` until that evidence supports movement; `1.3.2` UnverifiedNewer. Windows stays on the later sweep. | `eca55612-015c-46ed-a38b-52a85b30f15e` |
| Antigravity `1.1.18..=1.2.10` per-point pin backfill | Not now | `2ad46d4a` |
| Grok Build ACP past `1.0.46` | Separate per-hop selected-surface sweep to official `1.0.70`; the ACP ceiling stays `1.0.46` and only gains unverified-newer reach (Research 443) | Queue dispatch; no ACP qualification lead yet |
| ZCode runtime hops and selected app-server behavior after identity `0.16.9` | Prove hops; missing `3.14.4` source tag remains a hole | `b5f780ff` |
| Kimi ACP local-spawn process authority through official `2.1.1` | Review host-mediated process design; ceiling stays `0.38.0` | `e21bdd6e` |
| Kiro ACP owner-sweep reachability after `2.21.4` | Startup reachability in the isolated owner-state harness | `2ec033e3` |
| Gemini headless Plan Mode `exit_plan_mode` → YOLO at `0.63.0` | Separate enforcement design and authority ruling; PR 478 is a design | no implementation lead; Research 417 |
| `deepseek-harness.local-server` past `0.1.1-rc.2` / `0.2.0-rc.2` browser auth | Browser-session design (PR 479 / Research 427) then implementation | no implementation lead |
| DeepSeek SDK JSON-RPC profile | Design (PR 476 / Research 412 / Contract 023); new route, not a milestone on `0.1.0rc6` | no implementation lead |
| llama.cpp owned past `b10069` | Claim-model and owned-runtime selection adaptation | Research 407; no separate open currentness lead beyond the candidate |
| Muse signed payload `1.4.4-R5419.1` | Qualification after public hop/source identity exists | Research 409 |
| Copilot CLI later than exact `1.0.80` | Complete qualification evidence; diagnostics through Research 441 do not grant support | no open qualification lead; diagnostic host-identity `e83989a1`; parent currentness stays exact `1.0.80` |
| HTTP MCP live honouring on Copilot, Goose, Kiro | Not now | `f908ce90` |
| Further Gemini HTTP MCP live attempts | Not now | `d84c5632` |
| Cline ACP `session/load` (Q-006) | Operator picks `provider_limitation` or `producer_gap` from frozen Research 397 | Q-006; no duplicate task |
| Windows | Whole-codebase sweep when Tom selects timing | `17518642` |

Named incompatibilities stay named. Do not relabel them as terminal
abandonment (Contract 029 No Terminal Stop).

## Explicit Deferrals

- Windows, including Antigravity Windows ARM64 sandbox proof (Research 414)
  and the later whole-codebase sweep (`17518642`). Linux x64 Antigravity
  selected control-flow proof is not deferred; it stays on `eca55612`.
- Designs in PR 476, 478, and 479: designs, not implementation approval.
- Copilot exact `1.0.80` until current claims and complete qualification
  evidence establish otherwise.
- HTTP MCP live honouring beyond the exact accepted points
  (`opencode.acp`; `claude-agent.acp` `0.79.0`/`0.81.2`; `gemini-cli.acp`
  `0.61.0`; `kiro.acp` `2.21.4` emission without live honouring).
- Opaque production claims other than the reviewed `llama-cpp.attached`
  two-member set remain one exact point until their own route evidence is
  reviewed (Spec 015).
- Queue pre-merge validation (`e24d3751`) and other Not-now product work
  outside this sweep.

## Consumer Cells And Matrix Hygiene

The candidate reconciled the two hygiene cells and left claim truth alone:

| Cell | Candidate state | Note |
| --- | --- | --- |
| `cline.acp` version | Feature-matrix version column reads `Published 3.0.55-3.0.70 excluding 3.0.59`, matching the route matrix and Research 397 | stale metadata corrected; no claim change |
| `cline.acp load_session` | Feature matrix `No` with cross kind `evidence_pending` citing open Q-006 | exposed, not silently reclassified; the cell awaits the operator's provider-limitation or producer-gap choice |
| `copilot-cli.acp client_mcp_servers` | `producer_gap` | emission is not honouring; live gate Not now (`f908ce90`) |
| `goose.acp client_mcp_servers` | `producer_gap` | same |
| `kiro.acp client_mcp_servers` | `producer_gap` | same; HTTP MCP live remains exact `2.21.4` unproven |
| `claude-agent.sdk` pair selection | DEFAULT `.293`; live registered-tool `.259` | documented in the candidate notes; live proof does not transfer |
| HTTP MCP honouring | exact points only; later window members do not inherit | Contract 063 placement vs live gate |

Q-006 remains open and still owns the `cline.acp load_session` decision tree;
the candidate only makes the pending state visible.

## Product, Security, And Consumer Rulings Exposed

Complete independent assessment without selecting these:

1. **SDK `.259` on the minor.** Keep it only on `v0.5.4`, or add it as a
   selectable pair on the minor. Current source has no `.259` variant.
2. **Gemini Plan enforcement** that can keep the no-automatic-implementation
   contract (PR 478). That expands headless authority.
3. **DeepSeek browser-session / SDK JSON-RPC** as new routes or access types
   (PR 476, 479). New credential or persistence authority.
4. **Grok catalogue default-not-in-list** if `ModelCatalog` cannot represent
   it without narrowing (`8d166e76`).
5. **Q-006** Cline `load_session` classification.
6. **Public lease methods.** Settled for this candidate: restored as ordinary
   public API rather than a documented removal versus `v0.5.4`.
7. **Antigravity `SubagentStatus::Unknown`** already in source; shipping it
   is the breaking lifecycle that forces the minor. Withholding it is a
   candidate-content choice, not a silent revert in this ledger.

## Candidate Scope

Finite content this prepared candidate realises; it is still not a tag:

**Include** current `main` currentness qualifications listed above that
already have claim plus research evidence; courier idle already on `main`;
Opaque constructor narrowing; Antigravity Unknown child projection;
Command Code `1.73.0` model-selection guarantee; Gemini ACP `0.63.0`
restricted-files behavior; Codex app-server `0.159.0..=0.161.0` protected-`.aws`
write narrowing; additive public API; additive route `opencode.acp`; Node
`22.23.3`; ACP schema through `v1.24.1`; restore `v0.5.4` public lease
methods; restore tagged `0.5.1` baselines then generate the new minor
inventories.

**Exclude from claim movement** Copilot later than `1.0.80`; Kimi ACP past
`0.38.0`; Kiro past `2.21.4`; Grok catalogue past `1.0.30`; ZCode / Muse
identity-only targets; Gemini headless past `0.61.0`;
`deepseek-harness.local-server` past `0.1.1-rc.2` and JSON-RPC past
`0.1.0rc6`; llama.cpp owned past `b10069`;
Antigravity headless past exact `1.2.11` except the already-adapted Unknown
projection; Windows; PR 476/478/479 implementations; further HTTP MCP live
attempts; Q-006 mapping.

**Old consumers** stay on `v0.5.4` for the released SDK/native/Node/wire
pins. The minor's SDK window is the currentness window, with the `.259`
upgrade question exposed above.

## Dependency Ordering

Queue-owned; this list is a recommendation, not a repository status mirror.

1. This ledger, independently reviewed.
2. Matrix hygiene that does not change claim truth (Cline version column;
   Q-006 cell once Tom answers). Prefer a candidate-prep docs pass over a
   separate currentness family.
3. Parallel family adaptations on existing leads: Antigravity non-Windows
   including Linux x64 selected control flow (`eca55612`), Grok catalogue
   (`8d166e76`), ZCode (`b5f780ff`), Kimi ACP
   design (`e21bdd6e`), Kiro reachability (`2ec033e3`). Copilot later-than-
   `1.0.80` needs a planner brief; do not spawn a duplicate from this task.
4. Product rulings 1–7 only if the planner wants those inclusions.
5. Candidate preparation: restore baselines and public lease API, generate
   minor inventories, write candidate notes. Complete as one prepared
   candidate; no tag until the operator names a SHA.

## Validation For This Document

The assessment alone was docs-only. The candidate changes matrix truth in two
reconciled cells, so covering checks are `effigy qa:docs`,
`effigy qa:routes`, and
`effigy skill run northstar/retired-concepts`.
