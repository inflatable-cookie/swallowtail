# 145 Primary Wave Contract Fit And Batch Admission

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 261

## Question

Do the Research 144 dispositions fit existing Swallowtail contracts well
enough to start identity corpora, or does any admitted first route need a
new provider-neutral contract first?

## Method

Mapped each admitted first operation onto Contracts 005-006, 009-011,
015, 023, 029, 032-033, 036-037, 039-045, and 051-052. Compared ACP
candidates to existing Claude Agent / Cursor / Grok / Kimi ACP routes, and
headless candidates to existing Qwen / Command Code / Antigravity
structured-run routes. No adapter, package, matrix, or claim was edited.

## Admission

Start identity corpora. No new provider-neutral contract is required
before card 262.

Every admitted first route is an owned stdio child with host-approved
executable discovery (032), host inputs (010), async lifecycle and
joined cleanup (009, 044, 051), isolation and native-field privacy
(023), version identity (029), and prepared-facade/package acceptance
later (036-037, 045, 052). Credential lease stays provider-owned; Swallowtail
does not log in.

## Route map

| Route | First op | Shape | Existing contracts | Distinct from |
| --- | --- | --- | --- | --- |
| `cline.acp` | ACP initialize + one bounded `session/prompt` | ACP stdio child | 015 + 005-006, 009-010, 023, 029, 032-033, 039-041, 044, 051 | `cline.headless`; hub/TUI/`--id` |
| `cline.headless` | one bounded `cline --json` print run | installed structured-run | 009-010, 023, 029, 032-033, 039-041, 044, 051 | `cline.acp`; `--auto-approve true` default |
| `goose.acp` | ACP initialize + one bounded prompt | ACP stdio child | 015 + same host/lifecycle set | `goose serve`; `--with-builtin` |
| `copilot-cli.acp` | ACP initialize + one bounded prompt over `--stdio` | ACP stdio child; public preview | 015 + same host/lifecycle set | TCP `--port`; interactive slash commands |
| `mistral-vibe.headless` | one bounded `vibe --prompt` with `--output json` or `streaming` | installed structured-run | 009-010, 023, 029, 032-033, 039-041, 044, 051 | `vibe-acp`; TUI; `--continue`/`--resume` |
| `qoder.headless` | one bounded `qoder --print` with json or stream-json | installed structured-run | 009-010, 023, 029, 032-033, 039-041, 044, 051 | Qoder ACP; `--yolo` / `bypass_permissions` |
| `pi.acp` | none | not admitted | no new contract | community `pi-acp`; existing `pi.rpc` |

ACP candidates reuse Contract 015 initialize / capability / permission /
cancellation / stdout-only rules. They do not inherit Claude Agent form
elicitation, catalogue, or session-load merely because they speak ACP.

Headless candidates reuse the installed structured-run pattern: bounded
decode, terminal exit mapping, process cancellation, credential-last
cleanup. They do not inherit ACP session lifecycle.

## Adapter-private, not new contracts

Keep these as route evidence, not promoted kernel rules:

- Cline `--auto-approve` defaults true in CLI and false in ACP; Swallowtail
  does not default to auto-approve
- Copilot public-preview maturity and server-start tool/effort binding
- Goose `goose-provider` / `goose configure` access
- Vibe `--trust` folder trust
- Qoder permission modes

## Blocked before driver

`pi.acp` stays at identity card 282. Cards 283-285 do not start unless
official `@earendil-works/pi-coding-agent` exposes native ACP distinct
from `pi.rpc`. Wrapping community `pi-acp` is not a Swallowtail route.

## Primary order after admission

1. card 262 `cline.acp` identity
2. card 304 `cline.headless` identity after g03.086 closeout, including
   negative
3. card 266 `goose.acp`
4. card 270 `copilot-cli.acp`
5. card 274 `mistral-vibe.headless`
6. card 278 `qoder.headless`
7. card 282 `pi.acp` identity-only stop

Secondary wave still waits on card 286 after primary closeout.

## Non-goals

- new ACP, structured-run, or attached-server contracts
- package creation, production claims, installation, login, live work
- Copilot TCP, Goose HTTP, Vibe ACP, Qoder ACP, Cline hub
