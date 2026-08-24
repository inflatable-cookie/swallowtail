# 151 Qoder Headless 1.1.25 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Updated: 2026-08-24
Card: g03 batch 278; max-turns Authority reconciled under g04.053 / Research 200

## Question

Is official Qoder CLI `@qoder-ai/qodercli@1.1.25` programmatic `--print`
a distinct bounded headless wire that can freeze one print run without
flattening onto `--acp`, SDK stdio, the TUI, `--yolo` /
`bypass_permissions`, or `--input-format stream-json`?

## Method

Reconciled Research 144/145 with official CLI docs (overview, run in
scripts, CLI reference, permissions, authentication, ACP), npm
`@qoder-ai/qodercli@1.1.25`, the ACP registry entry `qoder` `0.2.14`,
and selected strings from the npm tarball (source review only).

Downloaded the 33-file npm tarball. Did not install Qoder. Did not
extract optional `@qoder-ai/qodercli-ripgrep-*` packages. Did not log
in, run `qoder login`, or send a live `--print`. Host PATH has no
`qoder` or `qodercli`.

Observed versions are not qualified claims. No headless compatibility
claim in this record.

## Identity

| Surface | Value |
| --- | --- |
| Route | `qoder.headless` |
| Axis (provisional) | `qoder.package` |
| Package (provisional) | `swallowtail-adapter-qoder` |
| npm | `@qoder-ai/qodercli@1.1.25`, published 2026-08-18T10:57:38.729Z, `latest` on 2026-08-19 |
| Tarball SHA-256 | `627749221c609bfb5514f4486fb42f464597cf49472ed52c087c36a1d2fbb4ab` |
| Integrity | `sha512-Z1U7W+RBtnHVxiqt8eCySMwxkGXaGzkFQNFbI5QPpkGSAY4Mz2WTLrI+1l2x5tnK4ftru/PkS1jEDnogJV8Tpw==` |
| Shasum | `16374dc8b576e263a74f934c10a00a07d03fcd63` |
| Unpacked | 33 files, 62086670 bytes |
| npm `bin.qodercli` | `bundle/qodercli.js` (CLI) |
| npm `bin.qoder` | `bundle/qoder-npm-dispatcher.cjs` (CLI or IDE) |
| gitHead / repository | absent |
| ACP registry | `qoder` `0.2.14`, npx `@qoder-ai/qodercli@0.2.14` args `--acp` (lagged; discovery only) |
| Host | absent |

`qodercli` is the CLI bundle. `qoder` is an npm dispatcher: flag-first
argv (`--print`, `--help`) and default no-args run the CLI; `ide` /
`chat` / `serve-web` / `tunnel` and an existing-path first operand run
Qoder IDE. Swallowtail binds a host-approved `qodercli` executable. It
may accept `qoder` only when discovery proves the dispatcher CLI path.
It does not wrap Node, the IDE launcher, or ripgrep platform packages.

Optional host archive, not extracted:
`@qoder-ai/qodercli-ripgrep-darwin-arm64@1.1.25` integrity
`sha512-L7i4wufeBtsMqDEo41zbOTLIFNzBF69P0kNHqBVHVFfCjMgTLwZl4t2UZeWgt2v/Y0YCbJpbEo5/kcUU67ONlQ==`.

Registry lag at `0.2.14` is not a claim. Axis stays `qoder.package`.

## Selected wire

Entrypoint:

```
qodercli --print --output-format stream-json --permission-mode dont_ask --max-turns <positive bound> --no-session-persistence --cwd <opaque working resource> <one prompt operand>
```

Mode dispatch `Lao` in the tagged bundle:

- `--acp` → `{kind:"acp"}` (checked before headless; `--print --acp` is ACP)
- SDK stdio → `{kind:"sdk"}` only when an SDK entrypoint sets print plus
  `--input-format stream-json` and `--output-format stream-json`
- otherwise `--print` / hidden `--prompt` → `{kind:"headless"}` with
  input `text` unless `--input-format stream-json`

This route is headless print. It is not ACP and not SDK.

`--output-format stream-json` is the selected decode wire: one JSON
object per stdout line. Tagged builders emit `system` (including
`subtype:"init"`, `protocol_version` `1.2.0`), `assistant`, and
`result`. `stream_event` exists for partial assistant chunks and stays
unselected unless a later card proves `--include-partial-messages`.

`--output-format json` is a dump-at-end sibling: one JSON object with
the result and metadata. Do not mix that object with the streaming
decoder. `--output-format text` is the CLI default and is unselected.
Invalid formats exit 1.

`-p` / `--print` is the public programmatic flag. Hidden `--prompt
<text>` is an unselected alias. Prompt text is one argv operand.
Piped stdin stays unselected. `--print` plus both `--prompt` and a
positional query is rejected.

First useful op:

1. spawn the selected argv
2. drain stdout stream-json (`system` / `assistant` / `result`)
3. map `result.subtype` `success` with `is_error` false → `end_turn`;
   host kill / `error_during_execution` with abort → `cancelled`;
   `error_max_turns` (`Maximum turns exceeded`) → bounded limit, not
   `end_turn`
4. join or kill the child

## Authority

Omitting `--permission-mode` inherits host
`general.defaultPermissionMode` (may be `accept_edits` or
`bypass_permissions`). Tagged help values are `default`,
`accept_edits`, `bypass_permissions`, `dont_ask`, `auto`. Validation
also accepts `plan` (legacy: default plus Plan work state).

`--yolo` and `--dangerously-skip-permissions` are hidden aliases of
`bypass_permissions`. Headless `ask` auto-denies, but `accept_edits` /
`auto` / `bypass_permissions` auto-approve work without asking.

Swallowtail must pass `--permission-mode dont_ask`. It must not pass
`--yolo`, `--dangerously-skip-permissions`, `bypass_permissions`,
`accept_edits`, `auto`, or `plan`, and it must not omit
`--permission-mode`. This is adapter-private, like Cline
`--auto-approve false` and Vibe `--agent plan`. It is not a new kernel
contract. Research 145 already named Qoder permission modes as
adapter-private.

`--max-turns` is hideHelp. CLI values are raw strings copied onto Config
`maxSessionTurns` (text-error formatter). The selected CLI headless
QueryEngine factory hardcodes AgentLoop `maxTurns` to `1000`
(`kN`); argv does not set that ceiling. Swallowtail retains historical
inert argv `--max-turns 8` for exact `1.1.25` compatibility and does not
omit the flag from route argv. Fixture example `1` is corpus-only, not
production policy. Synthetic `error_max_turns` fixtures prove decoder
mapping only (Research 200). The host process deadline remains the
Swallowtail timeout.

`--no-session-persistence` is documented as print-only. Swallowtail
passes it so this run does not write a restoreable session.

Auth is host-owned `QODER_PERSONAL_ACCESS_TOKEN` or persisted
`qoder login` / `/login` state under `~/.qoder` (`QODER_CONFIG_DIR`).
Swallowtail does not authenticate, does not run `qoder login`, and does
not bind the PAT as a credential lease. Headless auth is blocking.

Working resource is `-w` / `--cwd`. Isolation is one owned stdio child.
Cleanup is join or kill. Host process deadline is required. Stderr is
diagnostics, not the selected decoder.

## Unmapped on this corpus

TUI (no `--print`), `--acp`, SDK stdio (`--input-format stream-json`
plus SDK entrypoint), hidden `--prompt`, `--output-format text`,
`--output-format json` as the streaming decoder, `--yolo` /
`--dangerously-skip-permissions` / `bypass_permissions` / `accept_edits`
/ `auto` / `plan`, `--continue` / `--resume` / `--session-id` /
`--fork-session`, `--worktree`, `--add-dir`, `--teleport` / `--remote` /
`--remote-control`, `--sandbox`, `--include-partial-messages`,
`qoder login`, `qoder ide`, and ACP registry `0.2.14`.

## Decision

Admit `qoder.headless` as a first-party structured-run stdio route.
Freeze identity and named fixtures under
`crates/swallowtail-adapter-qoder/tests/fixtures/qoder-headless-1.1.25/`.
Card 279 may create the package and decoder. No production claim in this
card.

## Non-goals

- installing Qoder or extracting a ripgrep platform package
- live `--print`, login, or PAT use
- `--acp`, SDK stdio, TUI, continue/resume, worktree mutation policy
- version-range claims, package creation, matrix edits
