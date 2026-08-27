# 234 Codex Exec Fast Service-Tier Evidence

Status: promoted
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.083 / 233

## Question

Which exact `codex.exec` version, model, access profile, feature-gate,
`service_tier`, and lifecycle rows can bind caller-selected Fast without live
catalogue authority, ambiguous config precedence, or unconfirmed substitution?

## Method And Boundary

Evidence was collected on 2026-08-27 from current official Codex speed
documentation and exact tagged `rust-v0.149.1` source plus sibling tags that
pin introduction points. No Codex install, login, credential capture,
account/catalogue inspection, provider request, prompt, or paid operation ran.
Official docs were retrieved as `.md` exports. GitHub tag tarballs were
downloaded to disposable `/tmp` only.

Research 201 remains the `0.149.1` identity anchor. Research 213 remains the
exec configuration, verbosity, and lifecycle baseline. Research 229 is a
sibling-route stop and cannot settle this exec question.

Route: `codex.exec`, driver `swallowtail.codex.exec`, axis `codex.cli`,
maintained behavior `codex.exec.jsonl-v1`. App-server is out of scope.

Frozen corpus:
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/exec-fast-service-tier-range.json`.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `rust-v0.149.1` tarball | exact source tree | 2026-08-27 | SHA-256 `85139f405ce455bf14ff452615cdb2572d752e31a1e0da6891ac8325915d10ce` |
| GitHub tag `rust-v0.118.0` tarball | first `fast_mode` feature gate | 2026-08-27 | SHA-256 `7be18a18f604ad877b47c08d9db00018bd1dbbbd774dc6e1e7ea52dc492d7138` |
| GitHub tag `rust-v0.125.0` tarball | first `get_service_tier` wiring | 2026-08-27 | SHA-256 `1e20b221eaedc20135a76b897ff536bc8fa0021d5b9ff13e6bf7543060a91766` |
| GitHub tag `rust-v0.131.0` tarball | first bundled Fast-tier catalog rows | 2026-08-27 | SHA-256 `6e8cf81870596dac31ac164ac51f06bc94007f2c89d31e07fbe9e899751aea2d` |
| Research 201 | npm/Git identity, peeled commit `ff29a44391deccde0aba0f8390337d7f3c319ea4` | 2026-08-24 | see Research 201 |
| Research 213 | exec `--config`, verbosity omission, maintained exec argv | 2026-08-25 | see Research 213 |
| `codex-rs/core/config.schema.json` | `service_tier` string; `[features].fast_mode` boolean | 2026-08-27 | SHA-256 `affe54cce9b9945ffd32d322415ff4cc844c62068c1190be6355580be4ca9350` |
| `codex-rs/models-manager/models.json` | `service_tiers` / `default_service_tier` rows | 2026-08-27 | SHA-256 `c18214b1ba88ab9bd164753115324a7a29c0582e8d071f7b3babf749d892f549` |
| `codex-rs/protocol/src/config_types.rs` | `ServiceTier`, `SERVICE_TIER_DEFAULT_REQUEST_VALUE` | 2026-08-27 | SHA-256 `80c1f9a5026019fe813c064ee0ec05a33772f5e6cdd2863d19dda7e4414221ba` |
| `codex-rs/protocol/src/openai_models.rs` | `service_tier_for_request`, catalog membership | 2026-08-27 | SHA-256 `39939bf67ac473b5921d4edd0864df2c1d491edd7de5577957fc877eb3e012c3` |
| `codex-rs/core/src/session/mod.rs` | `get_service_tier`, gate composition | 2026-08-27 | SHA-256 `e6218071821f7e1b3858d31c2a0aef5fa5b63dea0ab16dfbced716d825ddbcc9` |
| `codex-rs/core/src/client.rs` | Responses `service_tier` request field | 2026-08-27 | SHA-256 `7af5d4c0c15673564b455725c98d34379c4bdf579f52c4d89aecb4ceb4b190fe` |
| `codex-rs/exec/src/lib.rs` | exec harness override posture, session bootstrap | 2026-08-27 | SHA-256 `9eec8d1f721ea843b351f817efefaf332908c16ef8247ea4ad47bf6ceaa0f950` |
| `codex-rs/features/src/lib.rs` | `fast_mode` stable gate default | 2026-08-27 | SHA-256 `791121524b5269c72254911823b77253cc98121d1dd29608663dd9d73fa7d61a` |
| [Speed](https://learn.chatgpt.com/docs/agent-configuration/speed.md) | Fast semantics, billing split, `/fast`, config spellings | 2026-08-27 | SHA-256 `e9de54571572cbc386e7326579e8c93c3e5a25f856359fb33112c59805fc6bbc` |

## Introduction Points

| Surface | First exact tag | Notes |
| --- | --- | --- |
| `service_tier` config key | `rust-v0.115.0` | predates the Fast gate |
| `features.fast_mode` gate | `rust-v0.118.0` | `Stage::Stable`, `default_enabled: true` |
| `get_service_tier` request wiring | `rust-v0.125.0` | gate + config + model membership |
| bundled catalog Fast tier (`priority`) | `rust-v0.131.0` | `gpt-5.4`, `gpt-5.5` only |
| `gpt-5.6-*` Fast tier rows | `rust-v0.147.0` | adds `gpt-5.6-sol`, `gpt-5.6-terra`, `gpt-5.6-luna` |

`/fast`, `features.fast_mode`, and `service_tier = "fast"` are not
interchangeable:

- `/fast` is an interactive TUI slash command and keybinding. Exec structured
  JSONL has no `/fast` flag or argv.
- `features.fast_mode` is the runtime gate. When disabled, configured
  `service_tier` is ignored and no request tier is sent.
- `service_tier` is the selected tier value. Config accepts arbitrary strings,
  but Fast dispatch uses catalog-backed `priority` on the wire. Legacy config
  spellings `fast` and `priority` normalize to `priority`. The explicit
  sentinel `default` means caller-selected standard routing and is not a
  catalog tier id.

Catalog rows advertise Fast under tier id `priority` with display name `Fast`.
That id is not the same string as config/TOML `service_tier = "fast"`, though
`from_request_value` maps both legacy `fast` and `priority` to the Fast enum.

## Syntax, Gate, And Precedence

Exact `rust-v0.149.1` schema:

- top-level `service_tier`: optional string
- `[features].fast_mode`: optional boolean

There is no dedicated CLI flag. Exec already passes typed `--config` overrides.
The dispatch forms matching current Swallowtail `config_string` encoding are:

```text
--config
service_tier="priority"
```

Legacy config alias also accepted:

```text
--config
service_tier="fast"
```

Gate control when default is insufficient:

```text
--config
features.fast_mode=true
```

```text
--config
features.fast_mode=false
```

`features.fast_mode` defaults to enabled at the exact tag. Disabling the gate
blocks all tier dispatch even when `service_tier` is set.

Config load precedence for maintained exec matches Research 213:

1. CLI flags and `--config`
2. project layers (skipped by `--ignore-user-config` on suppressed exec)
3. profile file (skipped)
4. user config (skipped)
5. packaged defaults

Maintained suppressed exec already passes `--ignore-user-config` and
`--ignore-rules`, so ambient user/project/profile `service_tier` or
`features.fast_mode` cannot leak in.

`--config` parsing is local and happens in `codex exec` `run_main` before
`bootstrap_auth_config` and before provider work, same as verbosity.

At config merge, legacy `service_tier = "fast"` is normalized to internal
`priority` when `features.fast_mode` is enabled. Saving back to TOML may
display `service_tier = "fast"` for the Fast tier; that is persistence UX, not
the wire id.

## Request Construction

Session bootstrap computes:

```rust
let fast_mode_enabled = config.features.enabled(Feature::FastMode);
let service_tier =
    get_service_tier(config.service_tier.clone(), fast_mode_enabled, &model_info);
```

`get_service_tier` rules at the exact tag:

- gate disabled → `None`
- gate enabled, no configured tier → `None` (catalog `default_service_tier`
  is not auto-applied)
- gate enabled, tier `default` → keep `default` sentinel
- gate enabled, tier matches catalog `service_tier.id` → keep tier
- gate enabled, unsupported tier → drop to `None` with warning

`service_tier_for_request` then omits explicit `default` and unsupported ids
before serializing the Responses request:

```rust
service_tier.filter(|tier| tier != "default" && self.supports_service_tier(tier))
```

For Fast rows the wire value is `priority`.

Unsupported configured tiers warn and are omitted from the provider request.
That is a post-config downgrade, not a pre-spawn hard error. Swallowtail must
reject unsupported slugs and tiers before spawn.

Swallowtail omission must not serialize `service_tier` or `features.fast_mode`.
Codex may still send no tier after spawn when both are omitted; that is
provider-side behavior, not Swallowtail argv proof.

Exec harness sets `service_tier: None` in `ConfigOverrides`, which does not
block CLI `--config` overrides. Exec does not pass `service_tier` on
`thread/start` RPC params; tier truth lives in the built `Config` object used
by the in-process app-server path.

Returned `service_tier` may appear on thread start/resume responses and in
`SessionConfiguredEvent`, but current Swallowtail exec decoding does not
observe it.

## Catalog And Model Membership

Bundled `models.json` at `0.149.1` is compiled into the binary. `OpenAiModelsManager`
may refresh from `/models` plus cache when the endpoint has Codex-backend or
command auth. ChatGPT auth with at least one list-visible remote model can
replace the bundled catalog. That live replacement can remove or alter Fast
membership after spawn. Admission uses the frozen tag table; claims stop at
selected, planned, and config-encoded truth. Provider-field serialization after
catalog resolution is not proved on the exec path.

Lookup inside Codex uses longest prefix and namespaced suffixes. Swallowtail
must not copy that. Admission is exact slug equality only.

Exact `0.149.1` bundled Fast-tier rows. Tier id is `priority`; display name is
`Fast`. Every row has `supported_in_api: true` and `default_service_tier: null`.

| Slug | Catalog tier id | Visibility |
| --- | --- | --- |
| `gpt-5.6-sol` | `priority` | list |
| `gpt-5.6-terra` | `priority` | list |
| `gpt-5.6-luna` | `priority` | list |
| `gpt-5.5` | `priority` | list |
| `gpt-5.4` | `priority` | hide |

`gpt-5.4-mini`, `gpt-5.2`, and `codex-auto-review` have empty `service_tiers`
arrays at the exact tag and are not Fast deliver-now rows.

`0.131.0` and `0.140.0` catalogs advertise Fast only on `gpt-5.4` and
`gpt-5.5`. `0.122.0` has the `fast_mode` gate but zero bundled Fast-tier
models. Those points are evidence-gated, not inferred into the deliver-now
window below.

## Access And Billing Profiles

Official speed documentation separates ChatGPT-credit Fast mode from API-key
billing. Do not infer one from the other.

| Profile | Config-seam evidence | Billing documentation | Swallowtail disposition |
| --- | --- | --- | --- |
| ChatGPT subscription / cached login | gate + frozen catalog membership + `--config` can encode tier | 1.5× speed; GPT-5.6/5.5 credits at 2.5× Standard; GPT-5.4 at 2× | evidence-gated config seam; billing documentation only |
| API key / enterprise explicit profile | same config seam; no auth gate on `get_service_tier` | API token pricing; ChatGPT credit multipliers do not apply; API Priority at 2× Standard API rate for GPT-5.6 | evidence-gated config seam; billing documentation only |

Provider acceptance, effective returned tier, observed latency, and live
entitlement are not proved and stay withheld for both profiles.

## Production Seam Audit

Current `prepare_structured_exec` / `exec_input::prepare` omits Fast controls.
Research 213 argv remains byte-equivalent on omission:

- one `codex exec --json --ephemeral` child on maintained points
- `--ignore-user-config --ignore-rules --skip-git-repo-check --sandbox read-only`
- explicit `--model <id>`
- fixed approval, shell, reasoning, search, and verbosity pairs as already bound
- no `service_tier` or `features.fast_mode` pair

The hypothetical adapter-local delta for a future qualified lane would add an
optional closed Fast selection on `CodexExecProfileInput`, exact
slug/version/behavior gates before spawn, and:

```text
--config
service_tier="priority"
```

Optional explicit gate reinforcement:

```text
--config
features.fast_mode=true
```

Omit both pairs when unset. Do not emit `service_tier="fast"` if a single wire
canonical form is desired; both normalize, but `priority` matches the request
id.

Composition: independent `--config` keys beside absent or explicit admitted
reasoning, verbosity, search, JSON Schema output, and one image. Distinct
from reasoning, verbosity, personality, and output-token bounds.

## Claim Strength

| Claim | Strength |
| --- | --- |
| `fast_mode` gate exists and defaults enabled at exact tag | proved |
| `service_tier` accepts string config; legacy `fast` normalizes to `priority` | proved |
| gate disabled ignores configured tier | proved from `get_service_tier` tests |
| omitted tier sends no request field; catalog default not auto-applied | proved |
| bundled catalog Fast membership for five exact slugs | proved from tag `models.json` |
| wire `priority` when catalog still supports tier at request time | proved from source/tests at exact tag |
| exec `--config` highest precedence with ignored user config | proved; same as Research 213 |
| unsupported tier warns and omits provider field | proved; silent downgrade inside Codex |
| `/fast` is TUI-only; exec has no `/fast` argv | proved |
| ChatGPT credit vs API billing differ | documented; not runtime-proved |
| `priority` survives live-catalog resolution on exec path | unproved; blocks deliver-now |
| pre-prompt exec observation of effective tier | absent; current decoder ignores returned tier |
| provider acceptance / returned effective tier / latency | unproved; withheld |
| live ChatGPT catalog replacement | proved possible; not admission authority |

## Empty Set Rationale

Card 233’s silent-substitution stop condition blocks a non-empty deliver-now
table on the evidence collected here.

Three coupled gaps remain after freezing gate, config, catalog, billing, and
omission truth:

1. **Live catalog substitution.** ChatGPT auth can replace the bundled catalog
   after spawn and remove or alter Fast membership before request construction.
   Frozen `models.json` membership is catalogue evidence only. It is not proof
   that `priority` still serializes on the exec path for the selected exact
   model at request time.

2. **Silent unsupported downgrade.** When the effective catalog no longer
   advertises the configured tier, `get_service_tier` drops the tier to `None`
   and request construction omits the provider field. That is a warn-and-omit
   downgrade inside Codex, not a pre-effect hard error. Static Swallowtail slug
   checks plus `--config service_tier="priority"` cannot prove the provider
   field survived catalog resolution.

3. **No pre-prompt confirmation seam.** Current `codex.exec` decoding does not
   observe returned or effective `service_tier` before provider work starts. A
   post-prompt provider response would not satisfy the pre-effect gate.

Until exact tagged proof shows `priority` surviving catalog resolution on the
bounded exec path—or a future binding adds fail-closed pre-prompt
confirmation—caller-selected Fast cannot bind without unconfirmed substitution.

## Evidence-Gated Catalog Membership

Frozen bundled catalog at `0.149.1` advertises Fast tier id `priority` on
these exact slugs. This is membership evidence, not a deliver-now dispatch
authorization:

| Version | Model | Catalog tier id | Disposition |
| --- | --- | --- | --- |
| `0.147.0`, `0.148.0`, `0.149.0`, `0.149.1` | `gpt-5.6-sol` | `priority` | evidence-gated membership only |
| same | `gpt-5.6-terra` | `priority` | evidence-gated membership only |
| same | `gpt-5.6-luna` | `priority` | evidence-gated membership only |
| same | `gpt-5.5` | `priority` | evidence-gated membership only |
| same | `gpt-5.4` | `priority` | evidence-gated membership only |

Omitted `service_tier` and `features.fast_mode` on current exec argv is
current behavior, not default Fast serialization.

## Deliver-Now Table

No row is deliver-now.

| Row | Disposition |
| --- | --- |
| any version / model / profile Fast dispatch on `codex.exec` | not deliver-now; live-catalog substitution and silent downgrade block binding |
| frozen bundled-catalog Fast membership rows above | evidence-gated only; not dispatch authorization |
| `--config service_tier="priority"` without pre-prompt confirmation | planned argv only; not proved provider serialization |
| explicit `service_tier = "default"` | separate standard-tier control; not Fast |
| `features.fast_mode = false` | blocks all tier dispatch |
| `gpt-5.4-mini`, `gpt-5.2`, `codex-auto-review` | no bundled Fast tier at exact tag |
| any other slug, prefix, alias, or namespaced form | not deliver-now |
| maintained `0.122.0..=0.146.x` | evidence-gated; catalogs differ or lack full five-model Fast set |
| retained/ambient exec segments | withheld |
| `codex.app-server` `/fast` or typed RPC tier | not applicable to this lane |
| live remote catalog as admission or confirmation authority | withheld |
| provider-accepted / returned / billed / observed latency truth | withheld |
| Research 213 verbosity table | not applicable to Fast tier |

Deliver-now rows: **0**.

## Adapter Binding Requirements

No Fast/service-tier exec binding is authorized. If a future lane closes the
pre-prompt confirmation gap, required binding would need at minimum:

- closed adapter-local Fast selection encoding wire `priority`
- optional field on `CodexExecProfileInput` only
- exact slug and version gates before spawn
- emit `--config service_tier="priority"` using existing `config_string`
- fail closed when pre-prompt effective tier mismatches or is absent
- keep ChatGPT-subscription and API-key billing profiles separate
- do not use live catalog or post-prompt provider bytes as sole confirmation
- keep `codex.exec.jsonl-v1` and `codex.exec.cli-window-2`

Until that evidence exists, do not add Fast fields to prepared exec profiles.

## Decision

Card 233 is complete with an honest empty deliver-now set. Gate, config,
catalog membership, billing split, and omission research are frozen. Fast
service-tier exec production binding remains blocked.
