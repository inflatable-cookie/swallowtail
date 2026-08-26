# 213 Codex Exec Model Verbosity Evidence

Status: promoted
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Card: g04.066 / 184

## Question

Which exact Codex CLI versions, providers, selected models, verbosity values,
and `codex.exec` profiles can dispatch `model_verbosity` through an adapter-
local prepared selection without ambient-config authority, silent unsupported-
model behavior, or live-provider inference?

## Method And Boundary

Evidence was collected on 2026-08-25 from current official Codex configuration
markdown and exact `rust-v0.149.1` source. No Codex install, login, credential
capture, account/catalogue inspection, provider request, prompt, or paid
operation ran. Official docs were retrieved as `.md` exports. The GitHub tag
tarball was downloaded to disposable `/tmp` only. Current `main` schema/models
paths were not used as binding evidence (`codex-rs/core/models.json` 404s on
current main; the tag stores models at `codex-rs/models-manager/models.json`).

Sibling published catalogs were retrieved only to classify version rows, not to
qualify a moving current release. Research 201 remains the `0.149.1` identity.

Route: `codex.exec`, driver `swallowtail.codex.exec`, axis `codex.cli`,
maintained behavior `codex.exec.jsonl-v1`. App-server is out of scope.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `rust-v0.149.1` tarball | exact source tree | 2026-08-25 | SHA-256 `85139f405ce455bf14ff452615cdb2572d752e31a1e0da6891ac8325915d10ce` |
| Research 201 | npm/Git identity, peeled commit `ff29a44391deccde0aba0f8390337d7f3c319ea4` | 2026-08-24 | see Research 201 |
| `codex-rs/core/config.schema.json` | closed `model_verbosity` enum | 2026-08-25 | SHA-256 `affe54cce9b9945ffd32d322415ff4cc844c62068c1190be6355580be4ca9350` |
| `codex-rs/models-manager/models.json` | exact `support_verbosity` / `default_verbosity` rows | 2026-08-25 | SHA-256 `c18214b1ba88ab9bd164753115324a7a29c0582e8d071f7b3babf749d892f549` |
| `codex-rs/core/src/client.rs` | Responses `text.verbosity` construction and unsupported ignore | 2026-08-25 | SHA-256 `7af5d4c0c15673564b455725c98d34379c4bdf579f52c4d89aecb4ceb4b190fe` |
| `codex-rs/protocol/src/config_types.rs` | `Verbosity` enum | 2026-08-25 | SHA-256 `80c1f9a5026019fe813c064ee0ec05a33772f5e6cdd2863d19dda7e4414221ba` |
| `codex-rs/exec/src/lib.rs` | `--config` parse before auth | 2026-08-25 | SHA-256 `9eec8d1f721ea843b351f817efefaf332908c16ef8247ea4ad47bf6ceaa0f950` |
| [Config reference](https://learn.chatgpt.com/docs/config-file/config-reference.md) | current docs: `low\|medium\|high`, unset uses model/preset default | 2026-08-25 | SHA-256 `6464159e3897bbda6c81871500497bf79de35c62d7dfd7af354b2a33a158f687` |
| [Config basics](https://learn.chatgpt.com/docs/config-file/config-basic.md) | CLI `--config` highest precedence | 2026-08-25 | SHA-256 `46d88f56b56542ff72e50b851d5e011dd01009815c4845137036868320c188a2` |
| [Advanced config](https://learn.chatgpt.com/docs/config-file/config-advanced.md) | `--config` TOML values; docs still mention Chat Completions ignore | 2026-08-25 | SHA-256 `6b2e6132e2eb0506231005c1d55f46fdd462e0d876de363ed3213b621756a751` |
| [Sample config](https://learn.chatgpt.com/docs/config-file/config-sample.md) | example `model_verbosity = "medium"` | 2026-08-25 | SHA-256 `9cdd6430e4449d3f4d19980a37102a3487624dcf3150e2491ab70dadd3f80c3b` |
| `rust-v0.122.0` schema | parser exists at maintained floor | 2026-08-25 | SHA-256 `8ec216678a7357751e108760491527cb4b7fcaad97bb7b64208f54636efa2713` |
| `rust-v0.122.0` models.json | mixed `support_verbosity`; not the deliver-now catalog | 2026-08-25 | SHA-256 `3faac7fdd1a34b3b43ac5071783e3e9cded577f65fbd39708a1b601470d2c2d8` |
| `rust-v0.131.0` / `rust-v0.140.0` models.json | byte-identical sibling catalogs | 2026-08-25 | SHA-256 `b21200fd39c430f750cf10030c13bb19a91fdbc07792abdbda09e0ce6479161a` |
| `rust-v0.147.0` models.json | same seven user slugs as ceiling, different file hash | 2026-08-25 | SHA-256 `384ff2e0ca67f65d2866e422e2ec7dfa5ed9e3fec7a84fe14005247a7087a302` |
| `rust-v0.148.0` models.json | same seven user slugs | 2026-08-25 | SHA-256 `b5b325b1896f25934aa39a16b810b3a357719f43fcbde337537d5d8e0b5081a9` |
| `rust-v0.149.0` models.json | byte-identical to `0.149.1` | 2026-08-25 | SHA-256 `c18214b1ba88ab9bd164753115324a7a29c0582e8d071f7b3babf749d892f549` |

Current official HTML shells were not used as the digestable corpus. Markdown
exports are the recorded docs.

## Syntax And Parser

Exact `rust-v0.149.1` schema defines `model_verbosity` as optional
`low|medium|high`. The Rust enum is:

```rust
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    #[default]
    Medium,
    High,
}
```

There is no dedicated CLI flag. Exec already passes typed `--config`
overrides. The exact dispatch form matching current Swallowtail encoding is:

```text
--config
model_verbosity="low"
```

`config_string` JSON-quotes the value, same as `model_reasoning_effort`.
Unquoted TOML `model_verbosity=low` is also accepted by `parse_overrides`;
Swallowtail must emit the quoted form for argv determinism.

`--config` parsing is local and happens in `codex exec` `run_main` before
`bootstrap_auth_config` and before provider work. Invalid override syntax
prints `Error parsing -c overrides` and exits 1. `ConfigToml` deserializes
`model_verbosity` as `Option<Verbosity>` with no unknown-variant fallback, so
an unknown value such as `loud` fails config load and exits 1 before auth.
Unknown *keys* are ignored unless `--strict-config`; Swallowtail does not pass
`--strict-config` and must not rely on unknown-key rejection.

`0.122.0` schema already carries the same closed enum. Parser presence at the
maintained floor does not qualify older catalogs.

## Request Construction

For Responses requests, `client.rs` does:

```rust
let verbosity = if model_info.support_verbosity {
    self.state.model_verbosity.or(model_info.default_verbosity)
} else {
    if self.state.model_verbosity.is_some() {
        warn!(
            "model_verbosity is set but ignored as the model does not support verbosity: {}",
            model_info.slug
        );
    }
    None
};
```

That value becomes Responses `text.verbosity`. Tests at the tag prove:

- omitted config still serializes the model `default_verbosity` (`gpt-5.4` →
  `"low"`)
- explicit `high` serializes `"high"` on a supporting model
- explicit `high` is omitted from `text` on an unsupported model

Swallowtail omission must not serialize a default. Codex may still send the
model default after spawn; that is Codex request construction, not Swallowtail
argv.

`WireApi` at `0.149.1` is Responses only. `wire_api = "chat"` deserializes as
an error. Current docs that say Chat Completions ignore verbosity are stale
relative to the exact tag. OSS/custom providers remain out of scope: the
maintained exec child does not pass `--oss` or a provider override, and
`--ignore-user-config` drops user `model_provider`.

## Catalog, Prefix Matching, And Drift

Bundled `models.json` is compiled into the binary. `OpenAiModelsManager`
starts from that catalog, then may refresh from `/models` plus
`$CODEX_HOME/models_cache.json` when the endpoint has Codex-backend or
command auth.

ChatGPT auth with at least one list-visible remote model **replaces** the
bundled catalog. Other auth merges by exact slug. Unknown slugs fall back to
`model_info_from_slug` with `support_verbosity: false`, which takes the
warn-and-ignore path.

Lookup uses longest prefix, then one namespaced suffix such as
`namespace/model`. Swallowtail must not copy that. Admission is exact slug
equality only.

`--ignore-user-config` does not pin the catalog. `--ephemeral` does not
disable model refresh. Pinning `model_catalog_json` would add a second
`--config` and a materialized file; that is out of this lane.

Live-catalog replacement can therefore still ignore a frozen-supported slug
after spawn. That remaining drift is not provider-acceptance proof and is not
a Swallowtail default. Admission uses the frozen tag table; claims stop at
selected, planned, and dispatched.

## Exact 0.149.1 Model Rows

No aliases. `supported_in_api` is true for every bundled row.

| Slug | `default_verbosity` | Visibility | Notes |
| --- | --- | --- | --- |
| `gpt-5.6-sol` | `low` | list | `minimal_client_version` `0.144.0`; `tool_mode` `code_mode_only` |
| `gpt-5.6-terra` | `low` | list | same |
| `gpt-5.6-luna` | `low` | list | same |
| `gpt-5.5` | `low` | list | `minimal_client_version` `0.124.0` |
| `gpt-5.4` | `low` | hide | picker upgrade NUX to `gpt-5.6-terra`; still in catalog |
| `gpt-5.4-mini` | `medium` | hide | picker upgrade NUX to `gpt-5.6-luna`; current exec test model |
| `gpt-5.2` | `low` | list | |
| `codex-auto-review` | `low` | hide | auto-review specialty; not an exec user profile |

Hidden visibility does not block explicit `--model`. Upgrade NUX is picker
copy, not a request-construction gate.

`0.149.0` models.json is byte-identical. `0.147.0` and `0.148.0` have the same
seven user slugs with the same `support_verbosity` / `default_verbosity`.
The exact retrieved release set is `0.147.0`, `0.148.0`, `0.149.0`, and
`0.149.1`. Claim gaps `0.82.0..=0.83.0`, `0.108.0`, and `0.109.0` do not sit
between those points. `0.147.1` and `0.148.1` were unpublished at Research 201
observation.

`0.122.0` still has models with `support_verbosity=false` (`gpt-5-codex` and
siblings). `0.131.0`/`0.140.0` share a different six-row catalog. Those
points are evidence-gated, not inferred into the deliver-now window.

## Precedence For Maintained Exec

Highest first, from official basics plus exact loader comments:

1. CLI flags and `--config`
2. project `.codex/config.toml` layers (trusted only)
3. selected `$CODEX_HOME/<name>.config.toml` profile
4. user `$CODEX_HOME/config.toml`
5. packaged defaults

Maintained `0.122.0..=0.149.1` exec already passes `--ignore-user-config` and
`--ignore-rules`, so user/project/profile layers are skipped. Ambient
verbosity cannot leak in. Older retained/ambient segments are withheld.

## Production Seam Audit

Current `prepare_structured_exec` / `exec_input::prepare`:

- one `codex exec --json --ephemeral` child on maintained points
- `--ignore-user-config --ignore-rules --skip-git-repo-check --sandbox read-only`
- explicit `--model <id>`
- `--config approval_policy="never"`
- `--config shell_environment_policy.inherit="none"`
- `--config hide_agent_reasoning=false` / `show_raw_agent_reasoning=false`
- search as `--config web_search="disabled"|"live"`
- optional `--config model_reasoning_effort="<mode>"`
- optional one `--image` and one `--output-schema`
- prompt `-`

Omitted verbosity leaves that argv byte-equivalent. The smallest adapter-local
delta is an optional `CodexModelVerbosity` on `CodexExecProfileInput`, exact
slug/version/behavior gates before spawn, and one extra `--config
model_verbosity="<value>"` pair. Not a shared capability. Not a generic
settings map. Keep `codex.exec.jsonl-v1` and `codex.exec.cli-window-2`. Do not
raise the Contract 029 ceiling.

Composition: independent `--config` key. Can sit beside absent or explicit
admitted reasoning, disabled/enabled search, JSON Schema output, and one
image. Distinct from reasoning, service tier, personality, and output-token
bounds.

## Claim Strength

| Claim | Strength |
| --- | --- |
| schema/parser accepts `low\|medium\|high` | proved at exact tag and maintained-floor schema |
| unknown values fail local config load before auth | proved from serde enum + exec bootstrap order |
| omitted Swallowtail argv lacks `model_verbosity` | proved from current exec argv |
| exact slug `support_verbosity=true` at frozen catalogs | proved from tag `models.json` |
| `--config model_verbosity` is highest-precedence vs ignored user config | proved |
| Responses request serializes configured or default verbosity | proved from source/tests at the tag |
| unsupported model warns and omits `text.verbosity` | proved; Swallowtail must reject those slugs before spawn |
| prefix/namespaced lookup | Codex does this; Swallowtail must not |
| live ChatGPT catalog replacement | proved possible; remaining post-spawn ignore drift |
| provider acceptance or effective response length | unproved; not authorized |
| account entitlement / `available_in_plans` | unproved; not authorized |

## Deliver-Now Table

| Version | Provider | Model | Values | Profile | Disposition |
| --- | --- | --- | --- | --- | --- |
| `0.147.0`, `0.148.0`, `0.149.0`, `0.149.1` | default `openai` Responses | `gpt-5.6-sol` | `low`, `medium`, `high` | maintained ephemeral suppressed exec | deliver-now |
| same | same | `gpt-5.6-terra` | same | same | deliver-now |
| same | same | `gpt-5.6-luna` | same | same | deliver-now |
| same | same | `gpt-5.5` | same | same | deliver-now |
| same | same | `gpt-5.4` | same | same | deliver-now |
| same | same | `gpt-5.4-mini` | same | same | deliver-now |
| same | same | `gpt-5.2` | same | same | deliver-now |

Omitted verbosity on those same version/model/profile rows is current
behavior, not a default serialization.

| Row | Disposition |
| --- | --- |
| `codex-auto-review` | not applicable; auto-review specialty |
| any other slug, prefix, alias, or namespaced form | reject before spawn; not deliver-now |
| maintained `0.122.0..=0.146.x` | evidence-gated; catalogs differ and include `support_verbosity=false` rows |
| retained/ambient exec segments | withheld |
| `codex.app-server` | not applicable to this lane |
| `--oss` / custom `model_provider` / Chat Completions | not applicable at the exact tag, or out of scope |
| unknown values | reject before spawn |
| live remote catalog as admission authority | withheld |
| provider-accepted / effective / observed verbosity | withheld |

Deliver-now rows: **21** explicit value rows (7 models × 3 values) on exact
published `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1` maintained exec, plus
omission.

## Adapter Binding Requirements

Cards 185-186 may run. Required binding:

- closed adapter-local `CodexModelVerbosity` with `low|medium|high` only
- optional field on `CodexExecProfileInput` only
- admit only `CodexExecBehavior::EphemeralSuppressed` and observed CLI version
  exactly `0.147.0`, `0.148.0`, `0.149.0`, or `0.149.1`
- admit only exact slugs in the deliver-now table
- emit `--config` / `model_verbosity="<value>"` using existing `config_string`
- omit the pair when unset
- reject unknown values, other models, other versions, other behaviors, and
  plan/evidence drift before process, credential, or provider effects
- do not infer from a `gpt-5` prefix or copy Codex longest-prefix lookup
- do not pin `model_catalog_json` or mutate user config
- keep `codex.exec.jsonl-v1` and `codex.exec.cli-window-2`
- no shared `Capability`, generic settings, app-server field, or ceiling bump

Docs may claim qualified dispatch against frozen tag metadata. They must not
claim provider acceptance, effective length, billing, or live-catalog support.

## Decision

Card 184 is complete with a non-empty exact table. Cards 185-186 are
authorized for that table only.
