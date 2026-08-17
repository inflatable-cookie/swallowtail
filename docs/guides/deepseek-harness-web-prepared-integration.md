# DeepSeek Harness Web `/api` Prepared Integration

Use `swallowtail-adapter-deepseek-harness` for the installed DeepSeek Harness
Web route. The route is `deepseek-harness.local-server`; the driver ID is
`swallowtail.deepseek-harness.local-server`. It owns one `dsh web` child on a
loopback endpoint and speaks the bounded HTTP plus WebSocket `/api` surface.

This is separate from [DeepSeek Harness JSON-RPC](deepseek-harness-prepared-integration.md)
and [DeepSeek Open Platform continuation](deepseek-prepared-integration.md).
It does not qualify ACP, the browser UI, the hosted DeepSeek API, or the
configuration plane.

The current source route is additive after immutable `v0.3.2`. Consumers pin
an explicitly reviewed source commit. There is no version bump, tag, registry
publication, or harness installation in this route.

## Preparation

`prepare_deepseek_harness_web` requires:

- exact host-approved `dsh` target and `deepseek-harness.web` axis
- exact `0.1.0-rc.6` Web release evidence
- host-approved Cordis environment reference
- loopback endpoint, defaulting to `http://127.0.0.1:3080`
- local unauthenticated access evidence with no credential reference or lease
- explicit provider, model, read-only working resource, and deadline for a
  structured run

Preparation performs installed-target classification without starting `dsh web`,
opening a browser, acquiring credentials, or calling a model. The
prepared evidence retains the endpoint, protocol facade, exact method
allowlist, access status, host services, and qualified release binding.

## Supported Web subset

| Operation | Prepared boundary | Provider method | Limits and authority |
| --- | --- | --- | --- |
| structured run | `prepare_run` → `start_run` | `session.create`, `session.prompt`, mux events | one explicit provider/model, read-only cwd, bounded output and usage, native `session.cancel` |
| catalogue | `prepare_session_catalogue` → `list_sessions` / `list_page` | `session.list` | bounded pages, exact working-resource filter, opaque cursor continuation |
| search and models | prepared catalogue → `search_sessions` / `list_models` | `session.search`, `session.models` | bounded query/results; native route-local helpers |
| history | `prepare_session_history` → `page_history` / `page` | `session.history` | control-free newest-first pages; no Agent resume or interactive handle |
| fork | prepared catalogue → `prepare_fork` → `execute` | `session.fork` | native route-local operation; no invented provider-neutral fork shape or resume authority |
| archive | `prepare_archive_session` → `execute` | `workspace.archiveSession` | target-only, inactive-session, before-dispatch cancellation posture |

The Web `session.create` payload carries the working directory, not a
provider/model override. The prepared run therefore requires the ambient
Cordis host configuration to report the selected provider and model through
`host.describe`; a mismatch is rejected before session creation.

The Web method allowlist is `session.list`, `session.search`,
`session.create`, `session.history`, `session.models`, `session.prompt`,
`session.cancel`, `session.fork`, `workspace.list`,
`workspace.archiveSession`, and `host.describe`. Settings, credentials,
`llm.*`, directory pickers, export, attachments, queue, subagents, skills,
goals, commands, restore, and hard delete remain unsupported and are not
silently routed elsewhere.

History reads the provider log directly. It does not resume an Agent, mint a
continuation binding, or expose a live session handle. Provider session state
remains provider-owned; the route exposes only the qualified catalogue,
history, native fork, and archive subset.

## Process and network boundary

The structured operation starts `dsh web` with the exact `web` argument and
host-approved Cordis environment. HTTP is JSON-only POST to `/api/<method>`;
WebSocket downlinks are limited to the mux and host channels. The endpoint is
loopback-only, the host network grant is audience-bound, redirects and proxy
use are disabled, and Origin plus same-origin request headers are checked.

The process, task, HTTP workers, WebSocket worker, event stream, and cleanup
are joined on normal completion, cancellation, deadline, and failure. Native
`session.cancel` is used for active turns; cleanup then stops the owned child.
Raw prompts, tool bodies, reasoning text, credentials, private paths, and raw
wire envelopes do not enter stable diagnostics.

## Host-local live proof

Installed and live Web probes are separate Effigy selectors. They require
explicit operator inputs and may use a host-local model path such as Ollama;
that does not qualify `deepseek-official`.

```sh
export SWALLOWTAIL_DEEPSEEK_HARNESS_EXECUTABLE=/absolute/path/dsh
export SWALLOWTAIL_DEEPSEEK_HARNESS_CORDIS=/absolute/path/to/cordis-config
export SWALLOWTAIL_DEEPSEEK_HARNESS_CWD=/absolute/path/to/read-only-workspace
export SWALLOWTAIL_DEEPSEEK_HARNESS_PROVIDER=local-ollama
export SWALLOWTAIL_DEEPSEEK_HARNESS_MODEL=operator-selected-model

SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS_WEB=1 \
  effigy probe:deepseek-harness-web-installed

SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS_WEB_PROMPT=1 \
  effigy probe:deepseek-harness-web-live
```

The live prompt gate starts only the prepared Web facade. It does not start
the JSON-RPC route or browser UI and does not publish a route qualification.

## Validation

```sh
effigy validate:focused swallowtail-adapter-deepseek-harness
effigy package:verify-affected swallowtail-adapter-deepseek-harness
effigy qa:guides
effigy qa:routes
effigy qa:docs
```

The public shape is shown in
[`prepared_deepseek_harness_web`](../../crates/swallowtail-adapter-deepseek-harness/examples/prepared_deepseek_harness_web.rs).
