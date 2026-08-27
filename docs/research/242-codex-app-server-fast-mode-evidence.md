# 242 Codex App-Server Fast-Mode Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.087 / 245

## Question

Which exact `codex.app-server` version, model, access, tier, and lifecycle rows,
if any, can bind caller-selected Fast with model membership, pre-effect
selection, confirmation, billing, persistence, restoration, and omission truth?

## Decision

No. Research 242 admits an empty deliver-now set. No app-server Fast /
`service_tier` binding is authorized on `swallowtail.codex.app-server`.

Exact tagged `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` expose a richer
typed Fast surface than Codex exec: `serviceTier` on `thread/start`,
`thread/resume`, `thread/fork`, `turn/start`, and `thread/settings/update`;
echo on `ThreadStartResponse` / `ThreadResumeResponse` / `ThreadSettings`;
and `model/list` `serviceTiers` advertising. That typed surface is real. It
still fails the card’s gates:

- unsupported catalog ids are dropped at session init without hard rejection
- mid-session settings/turn updates store a normalized tier without membership
  revalidation; later `service_tier_for_request` can omit the provider field
- `TurnStartResponse` does not echo `serviceTier`
- `ThreadMetadata` does not persist `service_tier` for cold resume
- live catalogue replacement can still rewrite membership after start echo
- `features.fast_mode` remains an ambient config gate under app-server Ambient
  posture

Research 234 remains Codex-core and exec contrast only. Do not promote exec
argv, `--config`, or Research 234’s empty-set conclusion onto this route. The
empty set here is an independent app-server qualification.

Cards that would bind app-server Fast stay blocked. No Fast feature ships from
this lane.

## Method And Boundary

Evidence was collected on 2026-08-27 from current official Codex speed
documentation, exact `openai/codex` tags through `rust-v0.149.1`, Research 201
identity, Research 234 core/exec contrast, Research 229/238 typed-field leads,
and current Swallowtail adapter production bytes. No Codex install, login,
credential capture, account/catalogue inspection, provider request, prompt, or
paid operation ran. GitHub tag tarballs were downloaded to disposable `/tmp`
only. Official docs were retrieved as `.md` exports.

Route: `codex.app-server`, driver `swallowtail.codex.app-server`, axis
`codex.cli`, maintained behavior `codex.app-server.v2.workspace-roots` from
`0.131.0`. Codex exec Fast argv stays out.

Deterministic range corpus:

`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-fast-mode-range.json`

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `rust-v0.149.1` tarball | exact source tree | 2026-08-27 | SHA-256 `85139f405ce455bf14ff452615cdb2572d752e31a1e0da6891ac8325915d10ce` |
| GitHub tag `rust-v0.129.0` tarball | first typed `ThreadStartParams.service_tier` | 2026-08-27 | SHA-256 `7c3f31ea6438af4e97c1bcfee3a899a9de955572256dddfdbd98191d4bff84a2` |
| GitHub tag `rust-v0.131.0` tarball | first bundled Fast-tier catalog rows | 2026-08-27 | SHA-256 `6e8cf81870596dac31ac164ac51f06bc94007f2c89d31e07fbe9e899751aea2d` |
| GitHub tag `rust-v0.147.0` tarball | `gpt-5.6-*` Fast-tier rows | 2026-08-27 | SHA-256 `355bde4b40d5ba6deea2e469d36f91708315729f3e84c9c69cce6b041a5ba593` |
| Research 201 | npm/Git identity, peeled commit `ff29a44391deccde0aba0f8390337d7f3c319ea4` | 2026-08-24 | see Research 201 |
| Research 234 | core gate/catalog/billing contrast; exec empty set not promoted | 2026-08-27 | see Research 234 |
| Research 229 / 238 | app-server typed-field / confirmation pattern | 2026-08-27 | see those files |
| [Speed](https://learn.chatgpt.com/docs/agent-configuration/speed.md) | Fast semantics, billing split, `/fast`, config spellings | 2026-08-27 | SHA-256 `e9de54571572cbc386e7326579e8c93c3e5a25f856359fb33112c59805fc6bbc` |
| `codex-rs/app-server/README.md` @ `0.149.1` | settings null-clear; `model/list` `serviceTiers` | 2026-08-27 | SHA-256 `664c71bc798035d7c6a91e6f45a1b6f4a5cd33745c8df0b1376717dbeed62ed5` |
| `codex-rs/core/config.schema.json` @ `0.149.1` | `service_tier` string; `[features].fast_mode` | 2026-08-27 | SHA-256 `affe54cce9b9945ffd32d322415ff4cc844c62068c1190be6355580be4ca9350` |
| `codex-rs/models-manager/models.json` @ `0.149.1` | bundled Fast membership | 2026-08-27 | SHA-256 `c18214b1ba88ab9bd164753115324a7a29c0582e8d071f7b3babf749d892f549` |
| `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json` @ `0.149.1` | generated v2 bundle | 2026-08-27 | SHA-256 `9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9` |
| `codex-rs/app-server-protocol/src/protocol/v2/thread.rs` @ `0.149.1` | start/resume/settings params and responses | 2026-08-27 | SHA-256 `27b068150d650ec6da10cd811cd176a72dd7844d701ff351989f4423032a0e07` |
| `codex-rs/app-server-protocol/src/protocol/v2/turn.rs` @ `0.149.1` | `TurnStartParams.service_tier`; response turn-only | 2026-08-27 | SHA-256 `3d76c3154f5d092eb2460fa77f1e8befdf4255447afb741724658afbf30a7704` |
| `codex-rs/app-server-protocol/src/protocol/v2/model.rs` @ `0.149.1` | `Model.service_tiers` / `default_service_tier` | 2026-08-27 | SHA-256 `3991f4a8f595767ac0de5d39a4edd7c2eea68ecaedfb007a4554e5fb1497d4e8` |
| `codex-rs/app-server-protocol/src/protocol/v2/config.rs` @ `0.149.1` | typed `config/read` `service_tier` | 2026-08-27 | SHA-256 `e67c812b34a9fddce9ec157870e338dcedebf9ff3003527e7e5b24bd51818d7e` |
| `codex-rs/protocol/src/config_types.rs` @ `0.149.1` | `ServiceTier`, legacy `fast`→`priority` | 2026-08-27 | SHA-256 `80c1f9a5026019fe813c064ee0ec05a33772f5e6cdd2863d19dda7e4414221ba` |
| `codex-rs/protocol/src/openai_models.rs` @ `0.149.1` | membership + `service_tier_for_request` | 2026-08-27 | SHA-256 `39939bf67ac473b5921d4edd0864df2c1d491edd7de5577957fc877eb3e012c3` |
| `codex-rs/core/src/session/mod.rs` @ `0.149.1` | `get_service_tier` at session init | 2026-08-27 | SHA-256 `e6218071821f7e1b3858d31c2a0aef5fa5b63dea0ab16dfbced716d825ddbcc9` |
| `codex-rs/core/src/session/session.rs` @ `0.149.1` | mid-session store without membership recheck | 2026-08-27 | SHA-256 `24ed2ac18d58621c404e8dd3c79a9fa844ade5bfc48225fd2467624e068dda89` |
| `codex-rs/core/src/client.rs` @ `0.149.1` | Responses `service_tier` request field | 2026-08-27 | SHA-256 `7af5d4c0c15673564b455725c98d34379c4bdf579f52c4d89aecb4ceb4b190fe` |
| `codex-rs/features/src/lib.rs` @ `0.149.1` | `Feature::FastMode` default enabled | 2026-08-27 | SHA-256 `791121524b5269c72254911823b77253cc98121d1dd29608663dd9d73fa7d61a` |
| `codex-rs/state/src/model/thread_metadata.rs` @ `0.149.1` | persisted resume metadata | 2026-08-27 | SHA-256 `dda4cff3a12a7631502ecc59da8cc46a3bf15f3bbbd937dc8a1f22812cec61f1` |
| `codex-rs/app-server/src/request_processors/thread_summary.rs` @ `0.149.1` | `ThreadSettings.service_tier` | 2026-08-27 | SHA-256 `8d209a9d9a6f99a4551e771cc06e12eb496bbc03af9b962fa9a4845e14a3c9f1` |
| `app-server-releases.json` / sibling fixtures | exact tag commits | 2026-08-27 | workspace fixtures |

`turn.rs` and the core Fast gate/catalog digests match Research 234/238 at
`0.149.1`. `thread.rs` and the v2 schema bundle are byte-identical at
`0.149.0` and `0.149.1`. `models.json` is byte-identical at `0.149.0` and
`0.149.1`.

## Introduction Points

| Surface | First exact tag | Notes |
| --- | --- | --- |
| typed `ThreadStartParams.service_tier` | `rust-v0.129.0` | absent on `0.128.0` v2 tree |
| bundled catalog Fast tier (`priority`) | `rust-v0.131.0` | `gpt-5.4`, `gpt-5.5` only |
| `gpt-5.6-*` Fast tier rows | `rust-v0.147.0` | adds sol/terra/luna |
| `features.fast_mode` gate | `rust-v0.118.0` | shared core; not an app-server RPC |
| `get_service_tier` request wiring | `rust-v0.125.0` | shared core session init |

`/fast`, `features.fast_mode`, and `service_tier` / `serviceTier` are not
interchangeable on this route:

- `/fast` is interactive CLI/TUI only. App-server has no `/fast` RPC method.
- `features.fast_mode` is the ambient runtime gate. When disabled at session
  init, configured tier is ignored and no request tier is sent.
- RPC `serviceTier` is an open string (double-option). Fast dispatch uses
  catalog id `priority` on the wire. Legacy `"fast"` normalizes to `"priority"`.
  Explicit `"default"` is standard-routing sentinel, not a catalog Fast id.

## App-Server Configuration Surface

| Surface | `serviceTier` present? | Swallowtail relevance |
| --- | --- | --- |
| `thread/start.params.serviceTier` | yes, string double-option | typed dispatch; start response echoes filtered value |
| `thread/resume.params.serviceTier` | yes | same; cold resume needs re-supply or ambient |
| `thread/fork.params.serviceTier` | yes | same family |
| `turn/start.params.serviceTier` | yes; “this turn and subsequent turns” | turn response is turn-only; no tier echo |
| `thread/settings/update.params.serviceTier` | yes; `null` clears to default | ack only; wait for `thread/settings/updated` |
| `ThreadSettings.serviceTier` | yes | session preference, not provider-wire proof |
| `thread/settings/updated` | yes, via `thread_settings` | same preference confirmation |
| `ThreadStartResponse` / `ThreadResumeResponse` | yes | filtered session value after init; pre-turn |
| `TurnStartResponse` | no | no turn confirmation field |
| `config/read` typed `Config` | yes | ambient/effective read; not prepared dispatch |
| `config.schema.json` / user config.toml | yes | ambient default; omission inherits it |
| `model/list` `Model.serviceTiers` | yes | advertising; live catalogue may rewrite |
| persisted `ThreadMetadata` | no | resume restores model/effort, not service tier |

JSON wire key is camelCase `serviceTier`. Rust fields remain `service_tier`.

## Model Membership

Bundled `models.json` at `0.149.1` advertises Fast under tier id `priority`
with display name `Fast`. Every Fast row has `supported_in_api: true` and
`default_service_tier: null`.

| Slug | Catalog tier id | Visibility |
| --- | --- | --- |
| `gpt-5.6-sol` | `priority` | list |
| `gpt-5.6-terra` | `priority` | list |
| `gpt-5.6-luna` | `priority` | list |
| `gpt-5.5` | `priority` | list |
| `gpt-5.4` | `priority` | hide |

`gpt-5.4-mini`, `gpt-5.2`, and `codex-auto-review` have empty `service_tiers`
arrays at the exact tag.

`model/list` maps those rows into `ModelServiceTier { id, name, description }`
plus optional `defaultServiceTier`. That is advertising. ChatGPT auth can still
replace the bundled catalogue. Static membership is evidence-only, not live
admission authority. Live catalogue/account inspection is out of scope.

## Request Construction And Truth Separation

Session init computes effective tier through shared core:

```rust
let fast_mode_enabled = config.features.enabled(Feature::FastMode);
let service_tier =
    get_service_tier(config.service_tier.clone(), fast_mode_enabled, &model_info);
```

`get_service_tier` drops unsupported configured tiers to `None` with a warning.
App-server tests assert `thread/start` with an unsupported id returns
`service_tier: None` rather than echoing the request. That is detectable soft
omit, not hard RPC rejection.

Mid-session `thread/settings/update` / `turn/start` overrides normalize legacy
`"fast"` to `"priority"` and store the value without re-running membership
checks. Later Responses construction filters again:

```rust
service_tier.filter(|tier| tier != "default" && self.supports_service_tier(tier))
```

So a settings preference echo can disagree with the provider field after a
catalogue change or an unsupported mid-session store.

Exact tagged tests prove a supported tier id on `turn/start` can appear as
Responses `service_tier` when the local catalogue still advertises it. That is
mock-server serialization proof under a frozen local cache, not live-catalogue
survival proof.

| Truth | Finding |
| --- | --- |
| requested | caller may send `serviceTier` on start/resume/turn/settings |
| RPC-dispatched | Swallowtail currently omits the field on open/resume/turn bytes |
| accepted | open string; unsupported ids soft-drop at init; mid-session store skips membership recheck |
| effective | depends on `features.fast_mode`, current model catalogue, and request-time filter |
| returned | start/resume/settings preference; turn response omits |
| persisted | not in `ThreadMetadata`; config.toml may store legacy `"fast"` spelling |
| restored | cold resume does not restore from metadata; resume param or ambient required |
| billed | ChatGPT credit multipliers vs API Priority remain documentation-only and distinct |
| observed | latency / provider acceptance unproved; not authorized |

Omitted Swallowtail app-server bytes do not serialize `serviceTier` on
`thread/start`, `thread/resume`, or `turn/start`. Omission must not claim the
ambient config default as caller-selected Fast.

## Access And Billing Profiles

Official speed documentation separates ChatGPT-credit Fast mode from API-key
billing. Do not infer one from the other.

| Profile | Config/RPC seam evidence | Billing documentation | Swallowtail disposition |
| --- | --- | --- | --- |
| ChatGPT subscription / cached login | typed RPC + frozen catalog membership | 1.5× speed; GPT-5.6/5.5 credits at 2.5× Standard; GPT-5.4 at 2× | evidence-gated seam; billing documentation only |
| API key / enterprise explicit profile | same RPC seam; no auth gate on `get_service_tier` | API token pricing; ChatGPT multipliers do not apply; API Priority at 2× Standard API rate for GPT-5.6 | evidence-gated seam; billing documentation only |

Provider acceptance, effective returned billed tier, and live entitlement stay
withheld for both profiles.

## Lifecycle Seam Audit

| Operation | Current Swallowtail bytes | Fast / service-tier seam |
| --- | --- | --- |
| new / open | `thread/start` without `serviceTier` | typed field + start echo available; soft-drop possible |
| turn / follow-up | `turn/start` without `serviceTier` | typed override available; turn response does not confirm |
| settings update | not used by current adapter | preference confirmable via `thread/settings/updated` |
| load / resume | `thread/resume` without `serviceTier` | no metadata restore; ambient or re-supply only |
| interrupted-turn reconciliation | existing turn/settings paths | preference may be live in session; not metadata-backed |
| fresh replacement | new `thread/start` | no inherited Fast claim |

App-server preflight requires `HarnessConfigurationPosture::Ambient`. Ambient
`features.fast_mode=false` or ambient `service_tier` can still affect session
init when callers omit the typed field.

## Cross-Version Finding

Decisive negative shape is stable across `0.147.0`, `0.148.0`, `0.149.0`, and
`0.149.1`:

- typed RPC and start/resume/settings preference echo exist
- turn response does not confirm
- unsupported ids soft-drop without hard reject
- mid-session store lacks membership revalidation before request-time omit
- no `ThreadMetadata` persistence
- live catalogue can move membership

Protocol typed `serviceTier` exists from `0.129.0`, but bundled Fast membership
is empty there. `0.131.0` adds `gpt-5.4`/`gpt-5.5` only. Full five-model Fast
set starts at `0.147.0`. Those earlier points are evidence-gated, not inferred
into deliver-now.

## Claim Strength

| Claim | Strength |
| --- | --- |
| typed `serviceTier` on start/resume/turn/settings | proved from v2 protocol sources |
| start/resume responses return filtered `serviceTier` | proved from protocol + thread_processor |
| `ThreadSettings` returns preference | proved from protocol and thread_summary |
| `model/list` advertises `serviceTiers` | proved from model.rs and app-server mapping |
| unsupported start ids soft-drop to `None` | proved from exact app-server tests |
| mid-session update stores without membership recheck | proved from session.rs update path |
| request-time filter can omit provider field | proved from `service_tier_for_request` |
| bundled Fast membership for five exact slugs | proved from tag `models.json` |
| mock-server turn can serialize supported `priority` | proved from exact turn_start test |
| `/fast` is not an app-server RPC | proved; docs + absence of method |
| ChatGPT credit vs API billing differ | documented; not runtime-proved |
| `priority` survives live-catalog resolution | unproved; blocks deliver-now |
| ThreadMetadata persistence/restoration | absent |
| provider acceptance / billed / latency | unproved; withheld |
| Swallowtail omission lacks `serviceTier` bytes | proved from current adapter sources |

## Empty Set Rationale

Card 245’s silent-substitution and confirmation stops block a non-empty
deliver-now table on the evidence collected here.

Coupled gaps remain after freezing the typed seam, catalog, billing split, and
omission truth:

1. **Soft unsupported drop.** Session init drops unsupported tiers to `None`
   without hard rejection. Soft omit is detectable on start response, but Codex
   still does not fail closed before session work.

2. **Preference ≠ wire after mid-session mutation.** Settings/turn can store a
   normalized tier without membership revalidation. Request construction may
   later omit the provider field while settings still echo the preference.

3. **Live catalog substitution.** ChatGPT auth can replace bundled membership
   after start echo and before provider serialization. Frozen `models.json` and
   mock-server turn tests are not live-path survival proof.

4. **No cold-resume metadata.** `ThreadMetadata` has no `service_tier`. Resume
   requires re-supply or ambient config.

5. **Ambient gate.** App-server Ambient posture leaves `features.fast_mode` and
   ambient `service_tier` in the config stack when callers omit typed fields.

Until exact tagged proof shows fail-closed pre-effect rejection, preference
matching provider serialization under live catalogue rules, and durable
restore/omission contracts, caller-selected Fast cannot bind on this route.

## Evidence-Gated Catalog Membership

Frozen bundled catalog at `0.149.1` advertises Fast tier id `priority` on these
exact slugs. Membership evidence only; not dispatch authorization:

| Version | Model | Catalog tier id | Disposition |
| --- | --- | --- | --- |
| `0.147.0`, `0.148.0`, `0.149.0`, `0.149.1` | `gpt-5.6-sol` | `priority` | evidence-gated membership only |
| same | `gpt-5.6-terra` | `priority` | evidence-gated membership only |
| same | `gpt-5.6-luna` | `priority` | evidence-gated membership only |
| same | `gpt-5.5` | `priority` | evidence-gated membership only |
| same | `gpt-5.4` | `priority` | evidence-gated membership only |

## Deliver-Now Table

No row is deliver-now.

| Row | Disposition |
| --- | --- |
| any version / model / profile Fast dispatch on `codex.app-server` | not deliver-now; soft drop, mid-session omit, live catalog, and cold-resume gaps block binding |
| frozen bundled-catalog Fast membership rows above | evidence-gated only; not dispatch authorization |
| typed `serviceTier: "priority"` without fail-closed wire proof | planned RPC only; not authorized |
| start/resume/settings preference echo alone | confirmation of session config, not provider-billed truth |
| explicit `serviceTier: "default"` | separate standard-tier control; not Fast |
| `features.fast_mode = false` | blocks all tier dispatch at session init |
| `gpt-5.4-mini`, `gpt-5.2`, `codex-auto-review` | no bundled Fast tier at exact tag |
| any other slug, prefix, alias, or namespaced form | not deliver-now |
| maintained `0.129.0..=0.146.x` | evidence-gated; catalogs differ or lack full five-model Fast set |
| ambient config.toml / config map only | not a Swallowtail prepared binding |
| `/fast` slash command | not applicable to app-server RPC |
| `codex.exec` Research 234 table or argv | not applicable to this route |
| live remote catalog as admission or confirmation authority | withheld |
| provider-accepted / returned billed / observed latency truth | withheld |

Deliver-now rows: **0**.

## Adapter Binding Requirements

No Fast/service-tier app-server binding is authorized. If a future lane closes
the soft-drop, mid-session wire, live-catalogue, and restore gaps, required
binding would need at minimum:

- closed adapter-local Fast selection encoding wire `priority`
- exact slug and version gates before open
- serialize `serviceTier` on prepared open/resume/turn only when selected
- fail closed when start/resume echo mismatches requested Fast
- fail closed when settings preference and provider serialization can diverge
- keep ChatGPT-subscription and API-key billing profiles separate
- do not use live catalog or post-prompt provider bytes as sole confirmation
- keep current omission byte-equivalent until then

Until that evidence exists, do not add Fast fields to prepared app-server
profiles.

## Decision

Card 245 is complete with an honest empty deliver-now set. Typed RPC, catalog
membership, billing split, and omission research are frozen. Fast service-tier
app-server production binding remains blocked.
