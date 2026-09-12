# 305 Grok 1.0.25 Model Catalogue Evidence

Status: complete
Verdict: typed non-admission (PR 315 review findings 1–2); no catalogue claim
Owner: Tom
Date: 2026-09-12
Card: g05.052

## Question

Does exact installed Grok `1.0.25` expose an admissible non-prompt model
catalogue seam with a bounded parseable output, or must the catalogue cell
stay unavailable with a typed non-admission ruling?

## Method

Static artifact inspection only. Observed `grok --no-auto-update --version`
indirectly through the recorded version text path, `grok --help`,
`grok models --help`, and read-only byte carves of the installed executable.
The `grok models` catalogue command was never executed. No credential was
read, no provider was contacted, no session was opened, and no host install
was changed.

## Identity

| Fact | Value |
| --- | --- |
| local CLI | `grok 1.0.25 (f7e67d6988e2) [stable]` |
| source revision | `f7e67d6988e2` |
| local executable SHA-256 | `9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c` |
| local executable size | 138080336 bytes |
| executable path | `/Users/tom/.grok/bin/grok` |

## Command evidence

Root `grok --help` lists `models — List available models and exit` among the
`COMMANDS`. `grok models --help` prints:

```
List available models and exit

Usage: grok models [OPTIONS]

Options:
      --debug                 Enable debug logging
      --debug-file <FILE>     Write debug logs to FILE
  -h, --help                  Print help
      --leader-socket <PATH>  Use a custom leader socket path ...
```

The subcommand takes no `PROMPT` argument and no session, persistence,
tool, extension, project, update, retry, or provider-invocation flag.
`--leader-socket` selects local leader IPC isolation, not a provider session.
No driver or prepared operation exists on this branch: the `["models"]` argv
built during admission is recorded deficient in finding 1 below (only
`["--no-auto-update", "models"]` would have qualified), and the seam is not
admitted regardless per finding 2.

## Output grammar evidence

Read-only string carves prove the `models` printer emits a bounded text
document with these literals (offsets into the 138080336-byte executable):

| Literal | Offset | Role |
| --- | --- | --- |
| `Default model: ` | 112748856 region | default header prefix; id follows |
| `Available models:` | 112748871 region | list header; rows follow |
| ` (default)` | same region | suffix marking the default row |

The two headers are adjacent literals, so the document shape is one default
header line followed by the list header followed by id rows. Auth-status
literals (`You are not authenticated.`, `You are authenticated via
deployment key.`, `You are using XAI_API_KEY.`, `You are logged in with...`)
sit in the same region but cannot be attributed to the `models` printer
specifically; this preamble stays unproven either way, and no parser was
admitted to derive membership, order, or default from the strict body.

## Embedded default-model document

The executable carries two byte-identical copies of the embedded
`default_models.json` fallback (offsets 111710804 and 113348525), 2324 bytes
each, SHA-256
`c7b26d2f4a4fc6f479b4ffa6c880eaec5eca1721701faa6d970d1cb4a51d5b7a`.
These bytes were carved and verified but ship nowhere in tree: no driver
embeds them and no fixture directory carries them. The document is reproduced
in full below as analyzed evidence only; it authorizes nothing.
The document, in full:

```json
{
  "default": "grok-4.6",
  "web_search": "grok-4.6",
  "image_description": "grok-4.6",
  "session_summary": "grok-4.6",
  "models": [
    {
      "id": "grok-4.6",
      "model": "grok-4.6",
      "model_family": "xai",
      "name": "Grok 4.6",
      "description": "SpaceXAI's latest frontier model",
      "context_window": 500000,
      "api_backend": "responses",
      "supports_backend_search": true,
      "system_prompt_label": "Grok 4.6",
      "supports_reasoning_effort": true,
      "reasoning_effort": "high",
      "auto_compact_threshold_percent": 80,
      "compaction_at_tokens": true,
      "compactions_remaining": 1,
      "reasoning_efforts": [
        {"value": "xhigh", "label": "Extra High Effort", "description": "Highest effort and reasoning level"},
        {"value": "high", "label": "High Effort", "description": "Higher implementation quality with extensive reasoning", "default": true},
        {"value": "medium", "label": "Medium Effort", "description": "Balanced effort with standard implementation and testing"},
        {"value": "low", "label": "Low Effort", "description": "Quick, fast implementations"}
      ]
    },
    {
      "id": "grok-4.5",
      "model": "grok-4.5",
      "model_family": "xai",
      "name": "Grok 4.5",
      "context_window": 500000,
      "api_backend": "responses",
      "supports_backend_search": false,
      "system_prompt_label": "Grok 4.5",
      "supports_reasoning_effort": true,
      "reasoning_effort": "high",
      "auto_compact_threshold_percent": 80,
      "compaction_at_tokens": true,
      "compactions_remaining": 1,
      "reasoning_efforts": [
        {"value": "high", "label": "High Effort", "description": "Highest implementation quality with extensive reasoning", "default": true},
        {"value": "medium", "label": "Medium Effort", "description": "Balanced effort with standard implementation and testing"},
        {"value": "low", "label": "Low Effort", "description": "Quick, fast implementations"}
      ]
    }
  ]
}
```

Order is `grok-4.6` first, then `grok-4.5`; top-level `default` is
`grok-4.6`. Parser-adjacent validation literals in the binary
(`default_models.json: invalid JSON`, `missing 'models' array`,
`entry id= has empty 'model' field`, `'default' is '' but 'models' array
only has ...`) confirm the loader requires a non-empty `default` naming a
model entry, so the frozen `default` field is load-bearing source truth.

## Field alignment analysis (unexercised)

Had the seam been admissible, the live `models` text output would have been
authoritative for membership, order, and default, with the frozen exact-`1.0.25`
document supplying supplemental metadata by exact id equality only:

| Source field | Common projection | Rule |
| --- | --- | --- |
| `name` | `display_name` | aligned; absent name means no display name |
| `description` | `description` | aligned; `grok-4.5` omits it, so it stays `None` there |
| `context_window` | `token_limits.maximum_input_tokens` | aligned; output max is unreported, so it stays `None` |
| `reasoning_efforts[].value` + `default` flag (`reasoning_effort` fallback) | `ReasoningMetadata` modes + default | aligned as catalogue evidence; selects no mode |
| `model_family: "xai"` | nothing | a backend routing family, not a provider identity; `provider_id` stays `None` per Contract 020 identity separation |
| `web_search`, `image_description`, `session_summary` | nothing | tool-routing defaults, not model defaults |
| `api_backend`, `system_prompt_label`, compaction fields | nothing | no common meaning |

Unknown valid ids pass through with default-less metadata. Document ids
absent from the live list are ignored. Nothing is derived from an id and no
capability the source omitted is attached.

## Review findings (PR 315, reviewCommentId 5648500001)

Independent review of the admitted implementation proved two defects in the
admission itself, both confirmed by further static inspection. No catalogue
command was executed for either; all evidence below is argv-grammar probes
(`--help` exits) and read-only byte carves.

### Finding 1 — update actions were not disabled

Contract 020 requires a dedicated harness catalogue process to disable update
actions. The admitted driver ran exactly `["models"]` with no update
suppression, while this adapter's own version probe and ACP process both pass
`--no-auto-update`, and the installed root CLI documents `| --no-auto-update
| Disable update checks for this session |` (also `grok -p "..."
--no-auto-update`).

Argv probes: `grok --no-auto-update models --help` exits 0 printing the
`models` help, so the root flag is accepted in root position (as in the
probe's `["--no-auto-update", "--version"]`); `grok models --no-auto-update
--help` fails with `error: unexpected argument '--no-auto-update'`, so the
admissible argv would have been `["--no-auto-update", "models"]`, never the
flag after the subcommand. The admitted `["models"]` argv is therefore
deficient, but the deficiency is immaterial next to finding 2.

### Finding 2 — the models path can perform outbound provider I/O

The installed binary documents `[features] remote_fetch = true` as "allow
optional online model-catalog fetches (default: true; set false for
firewalled/air-gapped deployments)", pinnable by fleet
(`features.remote_fetch`, managed wins over the user file), with
`remote_config/fetch.rs` (`startup.fetch_models_blocking`, "models fetch
skipped: remote_fetch disabled") and catalog states ("model catalog:
fetching", "model catalog: fetch succeeded", "model catalog fetch timed
out", "model catalog: falling back to bundled defaults only", "model
catalog: bundled defaults in use (remote_fetch disabled)", "initial models
prefetch timed out; catalog freeze uses bundled defaults", "loaded models
from disk cache"). No CLI flag and no `GROK_*` environment override for
`remote_fetch` exists in the binary; the only switches are config-file and
fleet layers, which this task is forbidden to read or mutate — and a
user-file override would not bind managed/fleet policy anyway.

Contract 020 admits a dedicated harness catalogue process only with provider
invocation disabled. A `models` process that may fetch a remote model catalog
under default host configuration is not provider-suppressed, and no admissible
non-prompt argv, environment, or working-resource binding can make it so.
The `--no-auto-update` fix from finding 1 suppresses update checks only; it
does not touch the catalog fetch switch. A narrowed endpoint/scope statement
is unprovable: fetch behavior depends on host config, fleet policy, cache
state, and auth state, none of which static inspection can fix.

## Non-admission verdict (typed ruling)

No admissible non-prompt catalogue seam exists on exact installed Grok
`1.0.25`:

- code `swallowtail.grok.catalogue_not_admitted`
- reason `provider_invocation_not_suppressible`: the `models` command admits
  optional online model-catalog fetches (`remote_fetch`, default true) with
  no CLI or environment suppression available to a prepared operation, and
  host/fleet configuration is out of bounds for preparation to read or bind.
- reason `update_action_unsuppressed` (contributory): the as-built argv
  omitted the root `--no-auto-update` suppression the adapter itself uses.
- scope: exact `1.0.25` (`f7e67d6988e2`, SHA-256
  `9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`).
  No version range is admitted, and no `UnverifiedNewer` catalogue path is
  opened: the ruling is about the seam, not the version.
- effect: no `ModelCatalog` driver, descriptor, claim, prepared operation,
  projection row, matrix cell, baseline, or API surface is admitted. The
  feature matrix `pre_session_model_catalogue` cell for Grok stays unavailable
  under its existing provider-limitation evidence. `grok_build_acp_claim`,
  `grok_build_model_for_version`, and released `v0.5.0` are unchanged.
- re-admission gate: a future Grok release qualifies only with static proof
  of a provider-suppressed listing (fetch switch removed, or a CLI/env
  suppression the prepared operation can bind without touching host config),
  frozen the same way before any claim.

The field-alignment table above is retained as unexercised analysis: it
records what the frozen document would have supplied had the seam been
admissible. It authorizes nothing.

## Sources

- installed `/Users/tom/.grok/bin/grok` `--help`, `models --help`, and
  `--no-auto-update models --help` (argv-grammar exits only)
- read-only byte carves at the offsets above; no catalogue execution
- `config.rs`/`fetch.rs`/`cache.rs` event and config-doc literals proving the
  `remote_fetch` switch and bundled-default fallback

## Correction 2026-09-12 (g05.053)

The verdict above remains the historical g05.052 record and its argv finding 1
stands, but finding 2 and the typed non-admission are superseded. The
`GROK_CONFIG` overlay and an operation-private `$GROK_HOME/config.toml` were
tested and disproved as suppression mechanisms, and Contract 020 was corrected
on 2026-09-12: a dedicated harness catalogue may run one ephemeral
**authenticated metadata** process that performs bounded authentication or
catalogue-metadata traffic without sending a prompt, opening a model session,
invoking inference or a tool, updating the harness, retrying, or retaining
provider state. `ProviderSuppressed` is required only when a named need
requires it. The final accepted observation (`grok-4.6` default then
`grok-4.5`, exit zero, zero stderr, no session or inference, joined cleanup)
admits `grok-build.catalogue` under `HarnessConfigurationPosture::Ambient`.
Static recovery of the shipped `xai-grok-pager/src/models.rs` format pieces
also corrects this record's output grammar: exact `1.0.25` renders
`  * <id> (default)` and `  - <id>` bullet rows after an authentication
preamble. Research 306 freezes the corrected boundary and the accepted
capsule. This record is not rewritten.
