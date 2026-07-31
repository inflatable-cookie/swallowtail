# 074 Installed Harness And Protocol Maintenance Inventory

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

What installed-harness and shared-protocol compatibility claims does
Swallowtail currently maintain, where is their evidence, and which bounded
source set should receive the first g03 external currentness check?

## Method

Card 001 used repository evidence only. Card 002 added a bounded external
currentness pass on 2026-07-31. It used official Codex documentation,
maintained-project release tags and source, npm and crates.io publication
metadata, and exact tagged commits. It did not run installed executables,
authenticate, invoke a model, or change a production claim.

The inventory reconciled:

- the 27-route guide and 23-solution feature matrix
- public adapter descriptors and compatibility claims
- deterministic compatibility, protocol, lifecycle, and activity corpora
- existing promoted research through Research 073
- release and consumer handoff records
- Effigy live-probe selectors

The repository evidence cutoff varies by route and is recorded below. A value
described as current in an older research record is historical evidence, not a
2026-07-31 external-currentness claim.

## Scope And Identity

Thirteen production route ids are supplied by ten installed or attachable
harness solutions:

- eleven owned-process routes: Codex exec and app-server, Claude Agent ACP,
  Claude Code headless, Gemini ACP and headless, Grok ACP, Kimi ACP and
  headless, Pi RPC, and Qwen headless
- two attached-network routes: Kimi local server and OpenCode HTTP/SSE

The count is by route id, not provider name or crate. The same executable may
carry multiple driver claims. A shared protocol may constrain several routes
without becoming their executable compatibility axis.

## Exact Route Inventory

Every ordered claim below uses `AllowUnverified`. A valid stable version above
the named upper point may execute with the latest qualified private behavior
revision and visible unverified status. It does not extend the guarantee.

| Route | Driver and transport | Version axis | Qualified segments and milestones | Gaps or exclusions | Evidence owner and cutoff |
| --- | --- | --- | --- | --- | --- |
| `codex.exec` | `swallowtail.codex.exec`; structured CLI JSONL | `codex.cli` | deprecated `0.80.0..=0.81.0` retained boolean search; deprecated `0.84.0..=0.98.0` retained search mode; deprecated `0.99.0..=0.121.0` ephemeral ambient; maintained `0.122.0..=0.146.0` suppressed-config JSONL v1 | gap `0.82.0..=0.83.x`; exact exclusions `0.108.0`, `0.109.0` | OpenAI; Research 025-026, 064, 071; 2026-07-30 |
| `codex.app-server` | `swallowtail.codex.app-server`; JSONL RPC stdio | `codex.cli` | deprecated default stdio `0.80.0..=0.81.0` and `0.84.0..=0.99.0`; deprecated explicit stdio `0.100.0..=0.107.0`; maintained base `0.110.0..=0.130.0`; workspace roots `0.131.0..=0.146.0`; lifecycle milestones archive `0.80.0`, restore `0.92.0`, notifications `0.104.0`, descendant archive `0.123.0`, hard delete `0.140.0` | gaps `0.82.0..=0.83.x` and `0.108.0..=0.109.x` | OpenAI; Research 025-026, 037, 064, 071; 2026-07-30 |
| `claude-agent.acp` | `swallowtail.claude-agent.acp`; ACP v1 stdio | `claude-agent.acp-adapter` | baseline `0.53.0`; session configuration `0.54.0..=0.59.0`; provider capability `0.60.0`; steering metadata `0.61.0` | below-baseline `0.52.0` and unpublished `0.58.0` excluded | Agent Client Protocol project; Research 032, 038, 065, 073; 2026-07-30 |
| `claude-code.headless` | `swallowtail.claude-code.headless`; Claude stream JSON stdio | `claude-code.headless-stream-json` | exact `2.1.220` | none inside the one-point claim | Anthropic; Research 066, 071; 2026-07-30 |
| `gemini-cli.acp` | `swallowtail.gemini.acp`; ACP v1 stdio | `gemini-cli.acp-agent` | exact `0.51.0` | none inside the one-point claim | Google; Research 065; 2026-07-29 |
| `gemini-cli.headless` | `swallowtail.gemini.headless`; stream JSON stdio | `gemini-cli.headless-stream-json` | one behavior segment `0.51.0..=0.52.0` | none inside the closed range | Google; Research 045, 066; 2026-07-29 |
| `grok-build.acp` | `swallowtail.grok-build.acp`; ACP v1 stdio | `grok-build.executable` | exact stable `0.2.114` | prerelease/alpha points do not qualify | xAI; Research 070-071; 2026-07-30 |
| `kimi-code.acp` | `swallowtail.kimi.acp`; ACP v1 stdio | `kimi-code.executable` | legacy reasoning exact `0.28.1`; declared-effort behavior `0.29.0..=0.31.0` | `0.28.2..0.28.x` is outside both qualified segments | Moonshot AI; Research 006, 046, 065, 068; 2026-07-30 |
| `kimi-code.headless` | `swallowtail.kimi.headless`; stream JSON stdio | `kimi-code.executable` | one behavior segment `0.29.0..=0.31.0` | versions before `0.29.0` unsupported | Moonshot AI; Research 046, 066, 068; 2026-07-30 |
| `kimi-code.local-server` | `swallowtail.kimi.local-server`; local REST and WebSocket v2 | `kimi-code.executable`, corroborated against server metadata | baseline exact `0.28.1`; profile/tools exact `0.29.0`; global-events/catalogue-filter `0.29.1..=0.30.0`; full subagent status exact `0.31.0` | `0.28.2..0.28.x` is outside qualified segments; executable and server versions must match | Moonshot AI; Research 040-041, 046, 066, 069; 2026-07-30 |
| `opencode.http` | `swallowtail.opencode.http`; HTTP/SSE | `opencode.server` | 20 exact published segments from `1.14.48` through `1.18.10`, carrying private surfaces 01-18 | unpublished or unqualified gaps between those segments remain closed | OpenCode maintainers; Research 027, 039, 066, 071; 2026-07-30 |
| `pi.rpc` | `swallowtail.pi.rpc`; strict-LF JSONL RPC stdio | `pi.package` | exact `0.80.10` strict-LF RPC v1 | all older points incompatible; no later point qualified | Pi maintainers; Research 022, 042, 053, 066; 2026-07-29 |
| `qwen.headless` | `swallowtail.qwen.headless`; structured CLI stream JSON | `qwen-code.package` | exact `0.19.11` headless behavior | all older points incompatible; no later point qualified | Qwen maintainers; Research 017, 066; 2026-07-29 |

### OpenCode Published Segments

The OpenCode summary must not be interpreted as one continuous semver range.
The claim contains:

- `1.14.48`, `1.14.49`, `1.14.50`, `1.14.51`
- `1.15.0..=1.15.4`, `1.15.5`, `1.15.6`, `1.15.7`,
  `1.15.9..=1.15.12`, `1.15.13`
- `1.16.0`, `1.16.2`
- `1.17.0..=1.17.3`, `1.17.4`, `1.17.5..=1.17.6`,
  `1.17.7..=1.17.9`, `1.17.10`, `1.17.11`,
  `1.17.12..=1.17.20`
- `1.18.0..=1.18.10`

## Shared Protocol Axes

ACP is the only shared protocol dependency in this installed-harness set.

| Axis | Repository evidence | Routes | Boundary |
| --- | --- | --- | --- |
| ACP wire | protocol version `1` | Claude Agent, Gemini ACP, Kimi ACP, Grok ACP | Wire agreement does not qualify the harness release. |
| Stable ACP schema | shared corpus at `schema-v1.20.0`; historical harness pins remain retained | same four routes | Schema revisions are additive evidence. Each harness still binds its own executable/package behavior. |
| Claude Agent ACP dependencies | SDK `1.0.0..=1.3.0`; schema `v1.15.0..=v1.18.0` across qualified wrapper milestones | Claude Agent | Wrapper, SDK, schema, nested Claude runtime, and Agent SDK remain separate axes. |
| Gemini ACP dependencies | SDK `0.16.1`; schema `v1.19.0` at CLI `0.51.0` | Gemini ACP | Gemini headless does not inherit ACP qualification. |
| Kimi ACP dependencies | SDK `0.23.0`; schema `v1.19.1` in the retained qualified evidence; ACP package moves to `0.3.6` at Kimi `0.31.0` without selected source drift | Kimi ACP | Kimi headless and local-server claims remain separate despite sharing the executable axis. |
| Grok ACP dependencies | wire v1 observed; bundled SDK and schema are not publicly identified in the qualified artifact | Grok ACP | Missing bundled dependency identity stays explicit. Exact Grok behavior fixtures qualify only `0.2.114`. |
| Form elicitation | unstable ACP capability and method, outside the stable `schema-v1.20.0` corpus | Claude Agent only | Capability negotiation and Claude's lossless choice subset govern use. It cannot become a baseline ACP guarantee. |

Codex app-server JSONL RPC, Pi strict-LF RPC, provider-specific stream JSON,
Kimi REST/WebSocket v2, and OpenCode HTTP/SSE remain route-owned protocol
surfaces. Their superficial framing similarities do not create a shared
version axis.

## Deterministic Corpus And Probe Inventory

| Route family | Primary frozen evidence | Repeatable live probe posture |
| --- | --- | --- |
| Codex | `adapter-codex/tests/fixtures/compatibility`, activity corpus, lifecycle corpus, installed-discovery fixtures | no dedicated Effigy live selector; production discovery and deterministic target-aware probes exist |
| Claude Agent and Claude Code | adapter fixture roots `claude-agent-acp-v0.53.0-v0.61.0` and `claude-code-2.1.220`; shared Claude ACP fixtures | `probe:claude-agent-acp-managed` observes the managed wrapper package version only; no route-wide live acceptance selector |
| Gemini | ACP `0.51.0`, headless `0.51.0-0.52.0`, activity, and installed-discovery fixtures | `probe:gemini-installed` checks an explicit installed `--version` invocation but does not classify either route claim |
| Grok | shared ACP fixtures for `0.2.114`, range fixtures, deterministic discovery and ACP suites | no repeatable Effigy live selector; Research 070 records one bounded authenticated qualification |
| Kimi | ACP, headless, local-server, activity, lifecycle, and `0.30.0-0.31.0` range fixtures | `probe:kimi-installed` parses the installed version and checks the ACP claim's qualified-or-unverified posture |
| OpenCode | `opencode-v1.14.48-v1.18.10`, protocol, deletion, activity, and health/version fixtures | `probe:opencode-installed` exists but still asserts exact `1.14.48`; it is stale against the `1.18.10` upper guarantee |
| Pi | `pi-rpc-0.80.10`, protocol, lifecycle, scheduling, activity, and continuity evidence | no repeatable Effigy live selector |
| Qwen | `qwen-code-v0.19.11`, protocol, interactive-continuity, activity, and prepared-facade evidence | no repeatable Effigy live selector |
| Shared ACP | activity and lifecycle corpora at `schema-v1.20.0`; historical Claude, Gemini, Kimi, Grok, and remote-transport corpora | no protocol-only live probe; routes negotiate against their selected harness |

Live probes remain optional and separately gated. Their absence does not
weaken deterministic qualification. A selector that exists must still agree
with the current claim; the OpenCode selector currently does not.

## Consumer Exposure

| Route | Recorded application exposure | Maintenance impact |
| --- | --- | --- |
| `codex.app-server` | accepted Nucleus interactive and bounded-workspace paths | highest: a consumer release can encounter a broad six-month installed range |
| `codex.exec` | accepted Soundcheck structured-run path | highest: installed discovery and old/new JSONL behavior remain application-facing |
| `claude-agent.acp` | Nucleus child-work and typed-question adoption delegated, not yet accepted here | high near-term: ACP form semantics and wrapper range affect the pending consumer route |
| all other installed/attached harness routes | provider-wide facade, conformance, and package evidence; no accepted application-specific adoption recorded | maintenance value remains library-wide, not a claimed live consumer dependency |

Consumer exposure affects priority. It never authorizes consumer edits or
lets a consumer repository define Swallowtail compatibility truth.

## Findings

1. The route guide, feature matrix, public claims, and deterministic corpora
   reconcile to the same 13 route ids.
2. Every ordered installed-harness claim already permits visible
   unverified-newer stable execution. No latest-only or hard upper-bound policy
   needs repair.
3. Shared executable axes do not imply shared claims:
   - Codex exec and app-server have different behavior segments.
   - Kimi ACP, headless, and local-server have different baselines and
     milestones.
   - Gemini ACP and headless use separate axes despite one CLI executable.
4. Claude Agent wrapper compatibility and Claude Code headless compatibility
   are different executable/package surfaces.
5. ACP v1 is stable at the wire boundary, but harness package behavior and the
   unstable elicitation method require independent evidence.
6. The retained `0.1.0` release table is a candidate snapshot, not a current
   27-route maintenance index. It must not override later code and route
   evidence.
7. The OpenCode live selector is stale. It proves neither current attached
   compatibility nor the upper bound it now publishes.
8. Gemini's live selector establishes installation and a non-empty version,
   not compatibility classification for either CLI route.
9. Pi and Qwen have useful current-source comparisons beyond their one-point
   guarantees but no expanded production claim or repeatable live selector.
10. Claude form elicitation changed one matrix `question_exchange` cell from
    `No` to `Yes`, but its commit retained the older aggregate validator
    counts. The matrix itself is complete; the stale invariant needs only a
    validation correction from 211/273 to 212/272 `Yes`/`No` cells.
11. The Grok solution matrix and route guide labelled the version axis with
    the route id `grok-build.acp`. Repository code settles the independent
    executable axis as `grok-build.executable`; both indexes now match it.

No finding changes a durable compatibility rule. Contracts 011, 029, 032,
036, and 037 remain sufficient for this currentness pass and card 003.

## Card 002 Source Set

The first external currentness batch should remain bounded to five upstream
source families:

1. **Codex CLI** — highest accepted consumer exposure and high release cadence;
   check both route claims from the shared exact executable history.
2. **ACP plus Claude Agent wrapper** — unstable form elicitation and a wrapper
   guarantee ending at `0.61.0` despite repository source comparison through
   `0.64.0`.
3. **Gemini CLI** — ACP ends at `0.51.0`, headless at `0.52.0`, and repository
   evidence already saw `0.53.0` as unverified.
4. **Pi coding agent** — the production claim remains exact `0.80.10` while
   continuity/activity research inspected through `0.82.1`; the cwd gate
   blocks load/resume only, not general range qualification.
5. **Qwen Code** — the production claim remains exact `0.19.11` while activity
   research inspected through `0.21.1`.

Card 002 should also classify the OpenCode probe drift from current official
evidence, but it should not reopen the freshly qualified `1.18.10` range unless
the upstream stable surface has moved materially.

Kimi, Grok, OpenCode range code, and Claude Code headless were qualified or
revalidated on 2026-07-30 and have no known consumer-reproduced defect. They
remain in the inventory but outside the first full source comparison.

## Currentness Pass — 2026-07-31

Current publication and source evidence remains separate from Swallowtail's
guaranteed ranges.

| Surface | Current authoritative point | Exact comparison | Classification |
| --- | --- | --- | --- |
| Codex exec and app-server | stable npm `0.146.0`, tag `e363b08c9175ac1cbe5893615dd2cb9ddf95043b`; `0.147.0` is alpha only | current stable equals both qualified upper bounds; official docs still expose distinct app-server and non-interactive surfaces | no action |
| stable ACP | wire v1; schema `v1.20.0` at `5e89c71497fe07dd4ae633c181a17224f4a8956d`; schema crate `1.6.0`; core and remote crates `2.0.0` | every stable axis equals the frozen shared corpus; v2 remains alpha | no action |
| Claude Agent ACP | npm and tag `0.64.0` at `e56f344691a56c07e5dae2ebeb6ad2a6416f8c9d`; ACP SDK remains `1.3.0`; Agent SDK moves from `0.3.217` to `0.3.220` | `0.62.0` retains the `0.61.0` ACP mapper; `0.63.0` changes tool-progress, denial, and nested-subagent correlation; `0.64.0` adds opt-in host-owned steering fallback and the custom-answer marker | behavior milestones; qualification candidate |
| Gemini CLI ACP | stable npm and tag `0.53.0` at `decc0b46c6e11f8cad90710dcfb38fc3cdb24a96`; ACP SDK remains `0.16.1` | selected `acpSession.ts` is byte-identical to exact qualified `0.51.0` | compatible-extension candidate |
| Gemini CLI headless | same `0.53.0` executable point | selected config, non-interactive entry, stream types, and formatter are byte-identical to qualified `0.52.0`; release notes contain no selected-route change | compatible-extension candidate |
| Pi RPC | stable npm and tag `0.83.0` at `845d6ff1f6643aba440341cce877ce1c43ebbc39` | strict-LF RPC remains, but `get_available_thinking_levels`, correlated bash updates, summarization-retry events, and nested usage alter selected evidence; the published TypeBox break applies to extensions, which Swallowtail disables | behavior milestone; qualification candidate |
| Pi session continuity | same `0.83.0` package | `session-cwd.ts` is byte-identical to `0.80.10`; public switch state still cannot prove the host-leased cwd | externally blocked; no load/resume work |
| Qwen Code headless | stable npm and tag `0.21.2` at `456fc9b02d7ed69357dd87db8fe4bcd7e2e55ac1` | stream event types and error declarations remain byte-identical to `0.19.11`; safe-mode configuration, tool registry, and catalogue filtering changed across the stable interval; `0.21.2` reports no known breaking change | behavior milestone; qualification candidate |
| OpenCode HTTP/SSE | stable npm and tag remain `1.18.10` at `7902e04c3a67f7c69726bc955efb46e29214c797` | current stable equals the qualified upper bound; the optional live selector still demands exact `1.14.48` | evidence-only refresh; selector repair candidate |

No selected source shows breaking drift requiring a new exclusion or hard
upper denial. Exact stable versions above a qualified boundary retain their
existing visible `UnverifiedNewer` posture until a later implementation batch
promotes them.

## Ranked Card 003 Candidates

1. **Claude Agent `0.62.0..=0.64.0`** — highest near-term consumer exposure
   and richest milestone information. Preserve the unchanged `0.62.0` point,
   then freeze separate `0.63.0` and `0.64.0` behavior revisions.
2. **Gemini CLI `0.53.0`** — smallest useful two-route extension. The selected
   ACP and headless sources are unchanged, but each route needs its own claim,
   corpus, and conformance evidence.
3. **Qwen Code `0.19.12` through `0.21.2`** — useful installed-range closure,
   but every published stable point and the invocation-affecting config/tool
   changes need classification before one or more segments can be named.
4. **Pi RPC `0.81.0` through `0.83.0`** — valuable range coverage with real
   event and usage milestones. Keep session continuity excluded; do not let
   the unchanged cwd gate block ephemeral RPC qualification.
5. **OpenCode live-selector repair** — small validation debt that should ride
   with a meaningful range tranche rather than become an isolated roadmap.

Codex and stable ACP need no implementation. No source justifies changing the
existing baselines.

## Card 003 Selection

The first implementation tranche combines the first two ranked candidates:

- Claude Agent `0.62.0..=0.64.0`
- Gemini CLI `0.53.0` on its separate ACP and headless axes

They share ACP framing and focused validation infrastructure, but not behavior
claims. Claude provides milestone-bearing evidence; Gemini provides an
unchanged selected-source extension across two operation shapes. Roadmap
g03.002 owns the fixture-first work.

Qwen and Pi remain later provider-specific candidates. OpenCode's stale
ignored selector rides the selected tranche's acceptance card as validation
debt without reopening its production range.

## Contract Result

Contracts 011, 029, 032, 036, 037, and the existing route-specific lifecycle
and activity contracts already govern every classified delta. The evidence
introduces no new operation shape, authority, fallback, version axis, or
provider-neutral capability. Architecture and contracts therefore remain
unchanged.

Card 003 may compile fixture-first provider-specific work directly from this
ranked set. It must select a coherent tranche; it must not qualify a release
from semver, release notes, or successful discovery alone.

## Current Sources

- [Official Codex app-server documentation](https://developers.openai.com/codex/app-server/)
- [Official Codex non-interactive documentation](https://developers.openai.com/codex/noninteractive/)
- [Official Codex npm package](https://www.npmjs.com/package/@openai/codex)
- [ACP stable schema `v1.20.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.20.0)
- [Claude Agent ACP `0.64.0`](https://github.com/agentclientprotocol/claude-agent-acp/releases/tag/v0.64.0)
- [Gemini CLI `0.53.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.53.0)
- [Pi `0.83.0`](https://github.com/earendil-works/pi/releases/tag/v0.83.0)
- [Qwen Code `0.21.2`](https://github.com/QwenLM/qwen-code/releases/tag/v0.21.2)
- [OpenCode `1.18.10`](https://github.com/anomalyco/opencode/releases/tag/v1.18.10)
