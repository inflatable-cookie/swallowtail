# 158 Watchlist And Registry-Only Disposition

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 303

## Question

After the harness-route suite closed, which remaining watchlist and
ACP-registry-only leads should add, defer, reject, or revisit — without
creating packages or production rows?

## Method

Reconciled Research 143 and 153 with the 2026-08-19 ACP registry snapshot,
official docs, GitHub/npm latest metadata for the named watchlist, and the
current 47-route production matrix.

Registry: `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json`
version `1.0.0`, 38 agents, fetched 2026-08-19. Registry membership is
discovery only.

No executable was installed. No provider account, login, prompt, or live
session was used. Observed versions are not qualified claims. No production
matrix, package, or README count changed. No candidate was promoted to a new
roadmap.

## Inventory

Current source stays 40 packages and 47 production routes. Immutable `v0.3.2`
stays 30 packages and 36 routes. Watchlist names do not appear as production
route IDs.

No candidate is **add**. Owner of the next promotion decision is the
operator, not this card.

g03.087 Cline Headless is already completed. This card does not reopen it
and does not invent a successor roadmap.

## Named-candidate dispositions

| Candidate | Surface | Overlap | Authority | Maturity | Disposition | Revisit when | Next owner |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Crush | Charm `crush run` is non-interactive text. Official `crush acp` is in-tree PR `#2450`, not a released wire. Community `willbnu/crush-acp` wraps `crush run`. Not in the registry. Latest Charm release observed `v0.86.0` (2026-07-20). | text print routes | Charm first-party CLI; community ACP wrapper | official ACP unreleased | **defer** | Charm ships maintained `crush acp` or structured `crush run` events in a stable release | operator |
| Continue | npm `@continuedev/cli@1.5.47` (updated 2026-06-18). Headless `cn -p` / `--format json`; also `cn serve` and browser login. Not in the registry. | existing print / attached-server routes | Continue first-party CLI; account/platform coupling | real headless JSON, not ACP | **defer** | first-party ACP **or** a distinct attached-server identity card is justified | operator |
| MiMo Code | npm `@xiaomi-mimo/cli@0.3.0-alpha.0`, GPL-2.0. OpenCode-family TUI/CLI. Not in the registry. | `opencode.http` | Xiaomi first-party CLI | alpha | **defer** | stable non-alpha with a distinct wire from OpenCode | operator |
| Kilo | Official `kilo acp` in registry: `kilo@7.4.22`, npx `@kilocode/cli@7.4.22 acp`, first-party Kilo-Org. | `cline.acp` / OpenCode family | first-party ACP | registry-present Cline-family CLI | **defer** | identity proves material wire/lifecycle divergence from `cline.acp` **and** OpenCode | operator |
| Roo Code | No registry row. Cline-family IDE/CLI. | `cline.acp` / `cline.headless` | first-party product; no distinct machine-facing row here | IDE/CLI overlap | **defer** | first-party machine-facing surface distinct from Cline | operator |
| Amp wrapper `amp-acp` | Registry `amp-acp` `0.9.0`, binary dist from `tao12345666333/amp-acp`. Description: "ACP wrapper for Amp". | same collapse class as `pi-acp` | community wrapper, not official Amp | wrapper | **reject** | official Amp **native** ACP (new identity card; do not wrap this package) | operator |
| GLM Agent `glm-acp-agent` | Registry `glm-acp-agent` `1.6.0`, npx `glm-acp-agent@1.6.0`, author Stefan de Vogelaere, `stefandevo/glm-acp-agent`. | would-be GLM/Zhipu ACP | community wrapper, not Zhipu first-party | wrapper | **reject** | official Zhipu/GLM native ACP | operator |
| `pi-acp` | Registry `pi-acp` `0.0.33`, `svkozak/pi-acp`. | `pi.rpc` | community wrapper | already rejected by card 282 / Research 152 | **reject** (do not reopen) | official native `pi --mode acp` | operator |

Do not wrap `crush-acp`, `amp-acp`, `pi-acp`, or `glm-acp-agent`.

## Named registry-only first-party leads

These are in the 38-agent snapshot and look first-party. None have a
Swallowtail identity corpus. None are production routes. All **defer**.

| Registry id | Observed | Authors | Install shape | Revisit when |
| --- | --- | --- | --- | --- |
| `auggie` | `0.35.0` | Augment Code | npx `@augmentcode/auggie@0.35.0 --acp` | transport, authority, install, and lifecycle evidence strong enough for a **named roadmap after operator review** |
| `codebuddy-code` | `2.106.7` | Tencent Cloud | npx `@tencent-ai/codebuddy-code@2.106.7 --acp` | same named-roadmap gate |
| `cortex-code` | `1.0.73` | Snowflake | binary | same |
| `devin` | `3000.4.25` | Cognition | binary | same |
| `factory-droid` | `0.199.0` | Factory AI | npx `droid@0.199.0 exec --output-format acp-daemon` | same; ACP-daemon exec is not auto-admitted |
| `junie` | `2783.5.0` | JetBrains | binary | same |

## Other registry discovery

Not named by the 303 watchlist list, still closed here so the 38-agent
snapshot is not an open queue. All **defer** unless noted.

| Registry id | Observed | Why not add | Revisit / flatten rule |
| --- | --- | --- | --- |
| `agoragentic-acp` | `1.3.0` | Agent marketplace with USDC settlement, not an installed coding harness | **reject**; wrong shape |
| `autohand` | `0.2.1` | first-party-looking ACP CLI; no Swallowtail corpus | named roadmap after operator review |
| `corust-agent` | `0.6.0` | discovery only | same |
| `crow-cli` | `0.1.24` | discovery only | same |
| `dimcode` | `0.3.16` | discovery only | same |
| `dirac` | `0.4.37` | discovery only | same |
| `fast-agent` | `0.10.1` | registry row with empty distribution block in this snapshot | same; install evidence first |
| `harn` | `0.10.105` | discovery only | same |
| `minion-code` | `0.1.44` | discovery only | same |
| `nova` | `1.1.35` | discovery only | same |
| `poolside` | `1.0.16` | discovery only | same |
| `sigit` | `1.5.2` | discovery only | same |
| `stakpak` | `0.3.88` | discovery only | same |
| `vtcode` | `0.96.14` | discovery only | same |
| `kimi` | `1.49.0` Moonshot `kimi-cli` | distinct from Swallowtail `kimi-code.*` (`@moonshot-ai/kimi-code`) | **do not flatten** onto `kimi-code`; named roadmap only if the CLI wire is distinct |
| `codex-acp` | `1.4.0` official `@agentclientprotocol/codex-acp` | Codex already has exec and app-server | **do not flatten** onto Codex native routes; revisit only if ACP-native Codex is materially distinct from app-server |

ACP siblings of already-selected Swallowtail routes also stay deferred, not
flattened:

| Registry id | Observed | Existing Swallowtail route | Rule |
| --- | --- | --- | --- |
| `mistral-vibe` | ACP `2.24.1` | `mistral-vibe.headless` exact `2.24.2` | do not add ACP because headless was selected |
| `qoder` | ACP `0.2.14` | `qoder.headless` exact `1.1.25` | do not flatten ACP onto headless |
| `opencode` | `1.18.18` | `opencode.http` | do not flatten ACP onto HTTP |

## Already-covered registry IDs (currentness, not this card)

These registry rows correspond to existing Swallowtail routes. Version
drift is Contract 029 currentness, not a watchlist add:

| Registry id | Registry version | Swallowtail note |
| --- | --- | --- |
| `cline` | `3.0.55` | `cline.acp` / `cline.headless` |
| `cursor` | `2026.08.11` | `cursor.acp` |
| `deepagents` | `0.1.7` | production binds npm `0.1.25`; registry stale |
| `gemini` | `0.55.1` | Gemini currentness stays deferred (Research 127 / g03.085) |
| `github-copilot-cli` | `1.0.80` | `copilot-cli.acp` |
| `goose` | `1.46.0` | `goose.acp` |
| `grok-build` | `1.0.6` | qualified Grok is `1.0.4`; currentness, not 303 |
| `claude-acp` | `0.70.0` | qualified Claude Agent ACP is `0.69.0`; currentness, not 303 |
| `qwen-code` | `0.21.14` | qualified Qwen headless through `0.21.13`; currentness, not 303 |

## Overlap rules that stay closed

- Do not flatten Kilo or Roo onto `cline.*`.
- Do not flatten MiMo onto OpenCode.
- Do not flatten registry `kimi` (`kimi-cli`) onto `kimi-code.*`.
- Do not flatten `codex-acp` onto Codex exec or app-server.
- Do not treat ACP registry membership as a compatibility claim, version
  range, or implementation approval.
- `aider.headless` (g03.095 / cards 295-298) and `kiro.headless` stay
  deferred/unstarted. They are not watchlist promotions.

## Decision

Close the watchlist. **Add: none. Reject: community wrappers and the
Agoragentic marketplace. Defer: everything else, with the revisit
conditions above.**

Counts stay 40 packages / 47 production routes. No new adapter package. No
new production matrix row. No new numbered route roadmap from this card.

## Sources

- Research 143: `docs/research/143-new-harness-route-expansion-selection.md`
- Research 152: `docs/research/152-pi-acp-identity-negative.md`
- Research 153: `docs/research/153-secondary-wave-source-and-disposition.md`
- ACP registry: https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json
- Continue npm: `@continuedev/cli`
- MiMo npm: `@xiaomi-mimo/cli`
- Charm Crush releases / PR `#2450`
- Production matrix: `docs/guides/provider-route-matrix.md`
