# 046 Kimi Code 0.29.2 Currentness And Corpus

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

Can Swallowtail guarantee Kimi Code `0.29.1` and `0.29.2` across its ACP and
local-server routes, and freeze the headless surface needed for a separate
structured driver?

## Method

The check compared the exact annotated `0.29.0`, `0.29.1`, and `0.29.2`
release tags. It traced:

- executable version and package identity
- ACP entry point, adapter tree, SDK pin, reasoning options, session setup,
  callbacks, cancellation, and authentication
- `kimi -p --output-format stream-json` options, default print runner, JSONL
  renderer, exits, cleanup, and experimental-engine selection
- local bearer authentication, metadata, model catalogue, prompt options,
  WebSocket control and events, archive, restore, and absence of delete

No Kimi binary, credential, account, provider request, model inference, or
live local server was used.

## Exact Release Identity

| Release | Tag object | Source commit |
| --- | --- | --- |
| `0.29.0` | `03c34eefa49513e6216390a9773326077a37f414` | `8bf5bacba9e524c38fb808c0122070037ead25a8` |
| `0.29.1` | `785c319619ad4cbf87d8598afaea36c989f6cb66` | `f4c3967a417a539372eadab6c809d27b8a14c005` |
| `0.29.2` | `57503c7c4d854f2c66ea32e10cba28b2c5715e9c` | `8a45f10eddbb35c317047e82e567cdb59a220b4f` |

The package reports the exact release through `kimi --version`.

## Surface Disposition

| Surface | Delta from `0.29.0` | Qualification |
| --- | --- | --- |
| ACP | adapter tree remains blob `458380a0eb0a2248b79735c3ed48b3f632ad5de6`; package `0.3.5`, ACP SDK `0.23.0`, and wire v1 remain fixed | extend declared-effort behavior through `0.29.2` |
| Headless command | options and default print runner are byte-identical; renderer code behavior is unchanged | freeze `0.29.0..=0.29.2` default-engine corpus for card 078 |
| Experimental v2 print | background and retry handling changes in `0.29.1` | exclude; the production route must not enable `KIMI_CODE_EXPERIMENTAL_FLAG` |
| Local auth and metadata | bearer middleware, auth routes, token service, and metadata schema are byte-identical | unchanged |
| Local prompt and events | prompt, approval, question, and event schemas are byte-identical | unchanged |
| WebSocket control | protocol remains v2; `subscribe_v2` is additive and legacy `subscribe` remains accepted | selected legacy subscription remains valid |
| WebSocket delivery | `0.29.1` broadcasts session/workspace/config events to every connection | new behavior milestone; ignore foreign-session global traffic |
| Model catalogue | `0.29.1` filters the synthetic secondary-model entry; `0.29.2` adds unselected provider-write schemas | new behavior milestone; selected read envelope stays stable |
| Archive and restore | handlers and response effects are unchanged | extend through `0.29.2` |
| Session delete | no delete route or delete effect appears | remains unsupported |
| Undo and export | `0.29.2` changes undo internals and adds an optional desktop-log export field | unselected |

## Compatibility Claims

ACP uses two segments:

- exact `0.28.1`: legacy reasoning selector
- `0.29.0..=0.29.2`: declared effort levels with legacy aliases

The ACP source is byte-identical at all three later points, so this range is
evidence-backed rather than inferred from SemVer.

Local server uses three segments:

- exact `0.28.1`: REST/WebSocket v2 baseline
- exact `0.29.0`: profile and disabled-tool prompt options
- `0.29.1..=0.29.2`: profile/tools plus global-event fan-out and filtered
  configured catalogue

Later stable releases remain visible `UnverifiedNewer` observations. They are
not denied, but do not enter guaranteed support without another exact audit.

## Headless Boundary For Card 078

The maintained non-interactive command is:

```text
kimi --model <alias> --prompt <prompt> --output-format stream-json
```

It creates or resumes provider-owned session state and emits JSONL assistant,
tool, retry, and resume-hint records. Prompt mode uses Kimi's automatic
permission posture and cannot be combined with `--auto`, `--yolo`, or
`--plan`. Swallowtail must therefore describe ambient harness authority
honestly; this is not a sandbox claim.

The selected production route is the default v1 print runner. An ambient
`KIMI_CODE_EXPERIMENTAL_FLAG` would select a different engine and behavior, so
card 078 must bind an environment that leaves that flag disabled.

## Runtime Repair

Kimi `0.29.1` global fan-out can deliver another session's lifecycle event on
a connection subscribed to one Swallowtail session. The prior driver rejected
that as a target mismatch.

The qualified behavior now:

- recognizes the exact global event families present at `0.29.1` and `0.29.2`
- ignores frames whose session identity is not the bound session
- retains strict parsing and cursor checks for the bound session
- still rejects unknown semantic event types without exposing payloads

## Corpus

`tests/fixtures/kimi-code-0.29.1-0.29.2/` freezes:

- exact tag, commit, tree, and selected blob provenance
- exact `0.29.1` and `0.29.2` metadata observations
- filtered configured-model response shape
- unsolicited foreign-session global event
- bounded assistant, tool, retry, and resume-hint JSONL

Older `0.28.1` and `0.29.0` fixtures remain intact.

Deterministic operation tests execute:

- ACP prompt, resource write callback, terminal outcome, and joined cleanup
  at every qualified milestone on local and remote-authoritative hosts
- reasoning negotiation at `0.29.1` and `0.29.2`
- local model listing, archive, and restore at every qualified milestone
- foreign-session global-event tolerance at `0.29.1` and `0.29.2`

## Promotion

- ACP and local-server compatibility claims now end at guaranteed `0.29.2`.
- The local-server global-event/catalogue change has its own behavior revision.
- Card 077 owns the currentness implementation and corpus.
- Card 078 realized separate headless and retained local-server structured
  roles without changing ACP identity.

## Realized Structured Routes

The installed solution facade now requires explicit `Acp` or `Headless`
selection. Headless retains its own descriptor, compatibility claim, process
contract, prepared operation, and deterministic fixture suite. It selects the
audited v1 runner, never enables the experimental engine, and joins the child
on completion, failure, cancellation, or deadline.

The local-server driver now registers `StructuredRun` independently from its
catalogue, interactive, and management roles. The projected operation creates
one private session and one prompt, preserves callback and provider
cancellation truth, then closes local resources without archive or delete.
Its public plan remains structured; no interactive-session authority escapes.
Durable retention is explicit.

## Evidence

- [Kimi Code 0.29.1 release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.1)
- [Kimi Code 0.29.2 release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.2)
- [0.29.2 command reference](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/docs/en/reference/kimi-command.md)
- [0.29.2 ACP adapter](https://github.com/MoonshotAI/kimi-code/tree/%40moonshot-ai%2Fkimi-code%400.29.2/packages/acp-adapter)
- [0.29.2 default headless runner](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/apps/kimi-code/src/cli/run-prompt.ts)
- [0.29.2 JSONL renderer](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/apps/kimi-code/src/cli/prompt-render.ts)
- [0.29.2 local model catalogue](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/routes/modelCatalog.ts)
- [0.29.2 WebSocket control](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/protocol/ws-control.ts)
- [0.29.2 WebSocket broadcaster](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/transport/ws/v1/sessionEventBroadcaster.ts)
- [0.29.2 session routes](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/routes/sessions.ts)
