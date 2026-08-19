# 150 Mistral Vibe Headless 2.24.2 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 274

## Question

Is official Mistral Vibe `v2.24.2` programmatic CLI a distinct bounded
print wire that can freeze one `vibe --prompt` run without flattening
onto `vibe-acp`, the TUI, `--continue`/`--resume`, teleport `&`,
`--max-price` as policy, or `--auto-approve`/`--yolo`?

## Method

Reconciled Research 144/145 with official CLI docs, GitHub
`mistralai/mistral-vibe` tag `v2.24.2`, PyPI `mistral-vibe==2.24.2`, the
ACP registry entry `mistral-vibe` `2.24.1`, and tagged sources from the
PyPI sdist (source review only).

Did not install Vibe. Did not extract a platform `vibe-*` archive. Did
not log in, run `--setup`, or send a live `--prompt`. Host PATH has no
`vibe`.

Observed versions are not qualified claims. No headless compatibility
claim in this record.

## Identity

| Surface | Value |
| --- | --- |
| Route | `mistral-vibe.headless` |
| Axis (provisional) | `mistral-vibe.release` |
| Package (provisional) | `swallowtail-adapter-mistral-vibe` |
| GitHub tag | `v2.24.2` lightweight commit `5e6aa0f6beb3454454f4c1de74a7652ba577ab05` |
| GitHub commit date | 2026-08-18T14:12:06Z |
| GitHub release | published 2026-08-18T14:13:33Z, not prerelease |
| PyPI | `mistral-vibe==2.24.2`, not yanked, Production/Stable |
| PyPI sdist SHA-256 | `be62b3148a9640ab2d72ab9849a40499d1680aa59589b01deb62c5eb08df269d` |
| PyPI wheel SHA-256 | `52536350059bcbffa3fd7a2e7ce50b3087b2839f26b9170c6bd3a63501e19e30` |
| Console script | `vibe = vibe.cli.entrypoint:main` |
| ACP registry | `mistral-vibe` `2.24.1`, `cmd: ./vibe-acp` (lagged; discovery only) |
| Host | absent |

The GitHub release ships two PyInstaller families: `vibe-*` (this route)
and `vibe-acp-*` (not this route). Example host archive, not extracted:
`vibe-darwin-aarch64-2.24.2.zip` SHA-256
`924069bab01afbf697981151f4c29e38e594506cf18c44e94b4a5185f6704125`.

Swallowtail binds a host-approved `vibe` executable (GitHub zip or matching
pip install). It does not wrap Python, PyInstaller internals, or
`vibe-acp`.

PyPI `2.24.2` matches the GitHub tag. Axis stays `mistral-vibe.release`.
Registry lag at `2.24.1` is not a claim.

## Selected wire

Entrypoint:

```
vibe --prompt <one prompt> --output streaming --max-turns <positive bound> --trust --agent plan --workdir <opaque working resource>
```

`--output streaming` is the selected decode wire: one completed public
history entry per stdout line, camelCase JSON from
`model_dump(mode="json", by_alias=True)`. Only
`generationStatus == completed` is emitted; in-progress updates are
deduped by `entry.id` and dropped.

`--output json` is a dump-at-end sibling: indent-2 array of history, or
`{history, teleportUrl}` if teleport ran. Do not mix that array with the
streaming decoder. `--output text` is the CLI default and is unselected.

`--prompt` / `-p` is the programmatic flag. Empty `--prompt` without TEXT
falls through to piped stdin. Swallowtail always passes `--prompt` with
TEXT. The interactive positional `PROMPT` is TUI, not this route.

First useful op:

1. spawn the selected argv
2. drain completed public-history NDJSON
3. map process exit 0 → `end_turn`; host kill → `cancelled`;
   `ProgrammaticLimitError` (stderr + exit 1) → bounded limit, not
   `end_turn`
4. join or kill the child

Tagged streaming `type` discriminator: `message`, `reasoning`, `effect`,
`callback`, `checkpoint`, `notice`. Message fields used on the first
corpus: `role`, `content` text blocks. Programmatic tests emit user then
assistant messages; system messages are not required.

## Authority

`--agent` argparse default is unset; the process then inherits config
`default_agent`. Tagged schema default is `accept-edits` (auto-approves
file edits; `AgentSafety.DESTRUCTIVE`). Official docs still say
programmatic mode “by default runs with the `auto-approve` agent.” Source
disagrees: help text and `cli.py` follow `--agent` or `default_agent`.
That docs line is stale. `--auto-approve` / `--yolo` is `store_true` and
approves all tool calls for the selected agent.

Swallowtail must pass `--agent plan` (read-only builtin). It must not
pass `--auto-approve` or `--yolo`, and it must not omit `--agent` (that
inherits host `default_agent`). This is adapter-private, like Cline
`--auto-approve false`. It is not a new kernel contract. Research 145
already named Vibe `--trust` as adapter-private.

`--trust` is required: this invocation only, not persisted to
`trusted_folders.toml`. Without it, programmatic mode may warn on stderr
and ignore project config. `--worktree` / `--add-dir` also imply trust
and stay unmapped.

`--max-turns` has no argparse default (unbounded). Swallowtail must pass
a positive bound. The fixture example `1` is not production policy; the
driver maps the host process deadline. `--max-price` and `--max-tokens`
stay unmapped as Swallowtail policy.

Programmatic `SessionOptions` always set `headless=True` and always
disable `ask_user_question` and `exit_plan_mode`. Callbacks
(`CallbackRequested`) are `session.deny_callback`: observe-and-deny, not
allow.

Auth is host-owned `MISTRAL_API_KEY` / `~/.vibe` config (`VIBE_HOME`,
`VIBE_*` overrides). `--setup` is login and stays unmapped. Swallowtail
does not authenticate or bind the API key as a credential lease.

Working resource is `--workdir` (chdir before run). Isolation is one
owned stdio child. Cleanup is join or kill. Host process deadline is
required. Limit exit is stderr
`The configured conversation limit was reached` (or last assistant text)
plus exit 1. Missing prompt is stderr
`Error: No prompt provided for programmatic mode` plus exit 1. Other
runtime errors are `Error: {message}` on stderr, exit 1. Success is exit
0. `session.close()` always runs in `finally`.

## Unmapped on this corpus

TUI (no `-p`), `vibe-acp`, `--continue` / `-c`, `--resume`, `--teleport`
(hidden `&` / Vibe Code teleport), `--max-price` as policy,
`--worktree`, `--add-dir`, `--setup`, `--check-upgrade`, stdin-as-prompt
when `--prompt` TEXT is omitted, `--output text`, `--output json` as the
streaming decoder, MCP CLI subcommands, and ACP registry `2.24.1`.

## Decision

Admit `mistral-vibe.headless` as a first-party structured-run stdio
route. Freeze identity and named fixtures under
`crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.24.2/`.
Card 275 may create the package and decoder. No production claim in this
card.

## Non-goals

- installing Vibe or extracting a platform archive
- live `--prompt`, login, or `--setup`
- `vibe-acp`, TUI, continue/resume, teleport
- version-range claims, package creation, matrix edits
