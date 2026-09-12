# 305 Grok 1.0.25 Model Catalogue Evidence

Status: complete
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
tool, extension, project, update, retry, or provider-invocation flag. The
admitted driver argv is exactly `["models"]` with no optional flag, so no
prompt can be supplied through the prepared operation. `--leader-socket`
selects local leader IPC isolation, not a provider session, and is never
passed by the driver.

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
specifically; the parser therefore tolerates an unproven preamble but derives
membership, order, and default only from the strict body.

## Embedded default-model document

The executable carries two byte-identical copies of the embedded
`default_models.json` fallback (offsets 111710804 and 113348525), 2324 bytes
each, SHA-256
`c7b26d2f4a4fc6f479b4ffa6c880eaec5eca1721701faa6d970d1cb4a51d5b7a`.
Frozen bytes ship as
`crates/swallowtail-adapter-grok/src/catalogue/default_models_1_0_25.json`
and the accepted stdout grammar ships as
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/`.
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

## Field alignment ruling

The live `models` text output is authoritative for membership, order, and
default. The frozen exact-`1.0.25` document above supplies supplemental
metadata by exact id equality only:

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

## Admission verdict

Admitted as a dedicated harness catalogue under Contract 020: one ephemeral
provider-suppressed `models` process, stdin closed unwritten, bounded output,
exact `1.0.25` claim only. `QualifiedOnly` posture plus a plan validator that
requires `Qualified` on the exact catalogue behavior means every point above
or below `1.0.25` fails closed; there is no `UnverifiedNewer` catalogue path.
The strict body parser requires exactly one `Default model:` line, the
`Available models:` header after it, rows of `id` with optional leading
whitespace and an optional ` (default)` suffix, and exactly one marked row
agreeing with the header. Duplicates, empty lists, over-limit output, id
charset violations, control bytes, missing or double markers, and
header/marker disagreement fail closed with a typed diagnostic.

Consumer cancellation is not claimed: the portable `ModelCatalogRequest`
carries a deadline but no cancellation control, and dropping its future
cannot report joined cleanup (Contract 020). The driver enforces
elapsed-deadline rejection, mid-read timeout with joined stop/wait cleanup,
and typed cleanup failure instead.

## Sources

- installed `/Users/tom/.grok/bin/grok` `--help` and `models --help`
- read-only byte carves at the offsets above; no catalogue execution
- frozen `crates/swallowtail-adapter-grok/src/catalogue/default_models_1_0_25.json`
- frozen `crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/`
