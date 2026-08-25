# 207 Kimi Code ACP Extended Effort Evidence

Status: promoted
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.060 / 167

## Question

Which exact qualified Kimi Code ACP versions project the current model's
catalogue-declared `xhigh` and `max` effort levels into a selectable
`thinking` config option, preserve the selected value through provider
application, and return effective confirmation before session readiness?

## Method And Boundary

Official Kimi Code configuration documentation and exact public GitHub / npm
identities for `@moonshot-ai/kimi-code` were inspected on 2026-08-25. Decisive
ACP adapter, agent-core, and test blobs were fetched from the peeled commits of
tags `@moonshot-ai/kimi-code@0.28.1`, `@0.29.0`, and `@0.38.0`, and compared
across every listed qualified point in `0.29.0..=0.38.0`. No Kimi install,
executable launch, OAuth/login mutation, credential or account inspection,
provider prompt, external inference, or paid work was used.

Production route evidence is the selected `@moonshot-ai/acp-adapter` path
already frozen by Research 006/086/165/179. The sibling `@moonshot-ai/acp-server`
package (agent-core-v2) remains unselected experimental surface and is cited
only as a non-authority contrast.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Configuration files](https://www.kimi.com/code/docs/en/kimi-code-cli/configuration/config-files.html) | official `support_efforts` / `default_effort` lead; example `low\|high\|max`; effort examples include `xhigh` and `max` | 2026-08-25 | `4b9d6e66f08c3a824be8c8bcf8bdf755fb6bf07969edbcdb3225327c22005d67` |
| npm `@moonshot-ai/kimi-code@0.28.1` | preceding boolean-select boundary identity | 2026-08-25 | integrity `sha512-1+GqFBdY6N0O6YBqNuclaoUY2jtKVQSKPikDBAMxF633AuB4emuSsMxDyh2KCnINH7f4ceeUdQhIjKunbS6GDA==` |
| npm `@moonshot-ai/kimi-code@0.29.0` | first declared-effort milestone identity | 2026-08-25 | integrity `sha512-cDwEubXkFAch4DsRq/Zp1RCcnkhn8+lC4fwstWmlEK62X5qgIRAeGdp8INAponIGP2ljUfcB6dU36fsAuqlumg==` |
| npm `@moonshot-ai/kimi-code@0.38.0` | current qualified ceiling; matches Research 179 | 2026-08-25 | integrity `sha512-O/z6sfjFdoDPPeTnoXzdsJ2U8IqP6K2gD3LsT+Nu8BAlHwdhCjdCQFkFTjIbLBun+aZT6x81ha5FiFt7trEilg==` |
| tag `@moonshot-ai/kimi-code@0.28.1` → commit `efacf0452d46f5dbd67499eabc053869495d5213` | boolean thinking select boundary | 2026-08-25 | annotated tag `0032545b65f95c139ecba5a48ba1b911844e1ffe` |
| tag `@moonshot-ai/kimi-code@0.29.0` → commit `8bf5bacba9e524c38fb808c0122070037ead25a8` | first extended declared-effort floor | 2026-08-25 | annotated tag `03c34eefa49513e6216390a9773326077a37f414` |
| tag `@moonshot-ai/kimi-code@0.38.0` → commit `0999454bdcb5ddd98f39bffee434dcf0a810f394` | current ceiling; Research 179 identity | 2026-08-25 | annotated tag `488fe6bb311959227c8c2602e12486e48f8b5446` |
| `packages/acp-adapter/src/config-options.ts` @ `0.29.0..=0.38.0` | thinking select construction from `supportEfforts` | 2026-08-25 | `4fb35fb760a868dff6ec0b212d050904eb48435701c42e86e3e192552b8d4567` |
| `packages/acp-adapter/src/config-options.ts` @ `0.28.1` | preceding boolean-only select | 2026-08-25 | `804e588478aa922326b7ef7f3076975a1afa5684fe5d7bd733200518f19c15e1` |
| `packages/acp-adapter/src/model-catalog.ts` @ `0.29.0..=0.38.0` | catalogue projection of `support_efforts` / default effort | 2026-08-25 | `ca27ae18254c9b7f3a2f0c3c2e3687563f21297fcad9e44a8f70b1644ce92629` |
| `packages/acp-adapter/src/session.ts` @ `0.29.0..=0.38.0` | membership validation, `setThinking`, effective status readback | 2026-08-25 | `1e4fe3cfd52b29cd4a3210099678cd486550b9cf012f078c40f66dbc0ff11e97` |
| `packages/acp-adapter/src/server.ts` @ `0.38.0` | `session/set_config_option` thinking arm and response snapshot | 2026-08-25 | `b108fd4a66bcea09d9e0f35b1bb975f118dcb27276f6401f425da7df8ed3aa14` |
| `packages/agent-core/src/agent/config/thinking.ts` @ `0.29.0..=0.38.0` | kimi-protocol preserve-or-default resolve | 2026-08-25 | `2286a08371e696e2b4400b4e733b87feec878353049b20b8b5cbe34716dbe7d0` |
| `packages/agent-core/src/agent/config/thinking.ts` @ `0.28.1` | preceding resolve boundary | 2026-08-25 | `0aaa51266183f79d7284f35dc04cc5fb7c006962497385cea40ebdea74b805d0` |
| `packages/acp-adapter/test/config-options.test.ts` @ `0.29.0..=0.38.0` | projects declared levels; stale `xhigh` collapses only when undeclared | 2026-08-25 | `cfc1608481cb5b3353f61cb0f74e5efad7bcfe5d105abe1d863ce0d5ec7811fb` |
| `packages/acp-adapter/test/model-catalog.test.ts` @ `0.29.0..=0.38.0` | Anthropic-inferred and override rows include `xhigh`/`max` | 2026-08-25 | `5254eae8d8e23697b8c7aafa6e58c3461f83b6529a6facc51ddd9dfc6cf18dd1` |
| `packages/acp-adapter/test/set-session-config-option.test.ts` @ `0.29.0..=0.38.0` | declared level set + response `currentValue`; undeclared `xhigh` rejected before SDK | 2026-08-25 | `31a25b8b5286ea9f195c6837de7b3401c5af577885ec0e6b1db710b3a24c1bd7` |
| `packages/agent-core/test/agent/config/thinking.test.ts` @ `0.38.0` | declared `max` preserved; unknown falls back only under kimi protocol | 2026-08-25 | `2bd6df4496384b8c1d7f2be25679caaecaf7e1aeb20aee97b903e7bf27418266` |
| `packages/acp-server/test/config.test.ts` @ `0.38.0` | integration coverage for declared effort rows and rejection | 2026-08-25 | `74834ccea6a81cc441695201f40638f0162b3424bcaa8e82ad13a77211f021ca` |

`config-options.ts`, `model-catalog.ts`, `session.ts`, and the three named ACP
adapter tests are byte-identical at every checked qualified point from
`0.29.0` through `0.38.0`. Official HTML is a lead only; exact adapter /
agent-core blobs own the deliver-now claims.

## Version Floor

| Boundary | Thinking option shape | Extended `support_efforts` projection |
| --- | --- | --- |
| exact `0.28.1` | boolean `off\|on` select; effort granularity hidden behind adapter default | no |
| exact `0.29.0` | `off` plus every non-blank current-model `support_efforts` entry; boolean models keep `off\|on` | yes |
| exact `0.29.0..=0.38.0` | same byte-identical ACP adapter construction / selection / confirmation path | yes |

First qualified extended-effort milestone is exact `0.29.0`. Immediately
preceding qualified boundary is exact `0.28.1` legacy boolean select
(`kimi.acp.reasoning.legacy-select-v1`). No later split inside
`0.29.0..=0.38.0` is required: the decisive ACP adapter and agent-core
thinking blobs do not change across that maintained segment.

## Option Construction

Exact `buildThinkingOption(currentEffort, supportEfforts, defaultEffort,
alwaysThinking)`:

- filters blank `supportEfforts` entries;
- effort-capable models advertise `off` plus each remaining declared string,
  or only the declared strings when `alwaysThinking`;
- boolean models (empty support list) advertise `off\|on`, or locked `on`;
- `currentValue` is `off`, an exact declared effort, or else the model
  `defaultEffort` (legacy `on` and undeclared levels collapse only in the
  rendered current value, not by inventing new rows);
- display names are capitalized effort strings and are not selection keys.

Exact `model-catalog.ts` derives `supportEfforts` from effective
`support_efforts` after override resolution and drops blanks. Default thinking
effort is declared `default_effort`, else the middle support entry, else `on`
for boolean models. Anthropic-profile inference tests project
`['low','medium','high','xhigh','max']`. Override tests project explicit
`max` rows.

There is no clamp of declared catalogue rows to `high`. Arbitrary provider
strings can appear in the snapshot when the catalogue declares them; that is
advertisement, not Swallowtail admission.

## Selection, Preservation, Confirmation

Truth layers stay distinct:

| Layer | Exact finding |
| --- | --- |
| Catalogue declaration | model `support_efforts` / inferred profile rows |
| Advertisement | session-open `thinking` select rows from current-model `supportEfforts` |
| Request | ACP `session/set_config_option` `{ configId: 'thinking', value }` |
| Dispatch | `AcpSession.setThinking` after membership resolve |
| Acceptance | ACP rejects undeclared concrete values with `invalid_params` before any SDK call |
| Effectiveness | adapter stores `getStatus().thinkingEffort` when present, else the resolved request; response rebuilds `configOptions` from that effort |
| Observation | response / `config_option_update` `currentValue` |

Exact `resolveEffortForCurrentModel`:

- `'on'` → current model `defaultThinkingEffort`;
- `'off'` → `'off'`;
- any other value must be in `entry.supportEfforts` or the request fails
  before `session.setThinking`.

Exact adapter tests prove a declared concrete level (`high`) is forwarded
unchanged and returned as response `currentValue`. Undeclared `xhigh` against
`low\|medium\|high` fails closed with no SDK call and no config update.
Always-thinking off requests may be engine-clamped; the status channel is then
the confirmation source.

Exact agent-core `resolveThinkingEffort` under kimi protocol:

- preserves a requested effort that appears in `support_efforts` (tests include
  declared `max`);
- maps unknown requested efforts to the model default;
- non-kimi protocols pass concrete values through for the backend to decide.

Because ACP membership rejects undeclared values before the SDK call, the
unknown→default agent-core fallback is not the ACP set path for hand-crafted
foreign values. Declared `xhigh` and `max` share the same preserve-when-listed
path as declared `high`; there is no separate remap table that substitutes
them when listed.

## Failure And Foreign-Value Disposition

| Case | Provider / adapter disposition | Swallowtail deliver-now disposition |
| --- | --- | --- |
| omitted reasoning | no set request | unchanged |
| `off\|on\|low\|medium\|high` | existing declared-effort behavior | preserve |
| snapshot-advertised `xhigh` / `max` | selectable; preserved when declared | admit |
| advertised foreign row beyond admitted set | projected into ACP rows when catalogue declares it | coexist in the snapshot; do not prepare; requested foreign fails unsupported |
| undeclared concrete request | ACP `invalid_params` before SDK | unsupported / fail closed |
| malformed / duplicate / missing thinking option | invalid option shape | malformed |
| ambiguous duplicate `thinking` ids | not produced by exact builder | already fail closed |
| always-thinking off | may clamp to default; confirmation shows effective | keep existing confirm rules |
| boolean model | `off\|on` only | preserve |
| load / resume / import / recovery | outside new-session selection; resume may restore stored effort without client redeclaration | no reasoning mutation; keep existing reject-before-host-effects |
| model change with stale effort | picker collapses current to new default; undeclared set still rejected | no model-selection work in this lane |
| disconnect / deadline / cancel / cleanup | unchanged host lifecycle | unchanged |

Foreign-row coexistence is required so a catalogue that advertises
`low\|medium\|high\|xhigh\|max` plus an extra future string does not make the
whole option malformed and block already-admitted values. Foreign strings never
become public selections.

## Compatibility

| Segment | Behavior revision | Extended `xhigh\|max` |
| --- | --- | --- |
| exact `0.28.1` | `kimi.acp.reasoning.legacy-select-v1` | no |
| exact `0.29.0..=0.38.0` | `kimi.acp.reasoning.declared-effort-v2` | yes, only when the current snapshot advertises the exact value |
| later stable | visible `UnverifiedNewer` | does not inherit this qualified extended-effort claim |

No new behavior revision or maintained-segment split is required. Extending
route-local validation for `xhigh` and `max` under the existing
`declared-effort-v2` revision matches the exact source floor.

## Deliver-Now Table

| Version range | Value | Snapshot gate | Set / confirm gate | Lifecycle |
| --- | --- | --- | --- | --- |
| `0.29.0..=0.38.0` | `xhigh` | current `thinking` select advertises exact `xhigh` | one `session/set_config_option`; response `currentValue` must equal `xhigh` | new-session only |
| `0.29.0..=0.38.0` | `max` | current `thinking` select advertises exact `max` | one `session/set_config_option`; response `currentValue` must equal `max` | new-session only |

Non-goals retained: arbitrary effort strings, display-label translation,
aliases, nearest-value fallback, model-name inference, load/resume mutation,
headless/local-server/Platform routes, live OAuth/prompt work, and
`UnverifiedNewer` inheritance.

## Promotion Gate

Promoted with a non-empty exact deliver-now set. Cards 168-169 may bind and
accept only the rows above under existing Contracts 011/023/029/034/037/040/
041/052. No shared contract or runtime change is required.
