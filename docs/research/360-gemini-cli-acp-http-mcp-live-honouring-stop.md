# 360 Gemini CLI ACP HTTP MCP Live Honouring Stop

Status: typed stop; honouring not accepted; exact host `0.61.0` point only
Owner: Swallowtail worker
Created: 2026-09-26
Task: swallowtail#072; planning `4970ad02b8ea281a8ce600589756e25db42eefd9`
Contracts: 063
Ruling: Tom 2026-09-26 — one Gemini live attempt, harness proof first

## Question

Does `gemini-cli.acp` on the host's qualified Gemini CLI `0.61.0` honour a
consumer-supplied streamable-HTTP MCP entry: connect, list one tool, complete
one tool call, and end the turn `Completed` with `Clean` cleanup?

## Answer

No. The honouring tuple was not accepted. The one authorized live attempt
stopped with typed `tool_not_called`. Authenticated MCP `initialize` and
`tools/list` reached the disposable server. The one turn did not call the
deterministic tool. No terminal or cleanup diagnostic was present.

The committed harness proof against a fake ACP agent and the disposable
loopback HTTP MCP server passed first. It proves the read-only profile
(`--approval-mode plan`), initialize advertising `gemini-api-key`,
`session/new` returning `plan`, and the full honouring tuple against the
fake. Host `gemini --version` was exact `0.61.0`. No host update, login,
auth change, or second live attempt.

Model recorded: `auto`. Honouring is the provider's client behaviour on the
declared HTTP entry; the model string is provenance. It is the session-negotiated
current value, not a caller-selected paid model.

## Research 356 `mode_rejected`

Frozen `0.61.0` `acpSessionManager.ts` `newSession` (byte-identical through
`0.59.0..=0.61.0`) does not throw `authRequired` when `selectedType` is
absent: it defaults to `USE_GEMINI` and returns no modes if the key is
missing. With successful auth, `currentModeId` is `config.getApprovalMode()`.
`packages/cli/src/config/config.ts:758-763` then clamps any non-`default`
mode to `default` when the folder is untrusted. Research 356's isolated
`0.59.0` `bounded_write` open expected `autoEdit` and hit that clamp.

`loadSession` still throws `authRequired` with no selected auth
(`acpSessionManager.ts:244`, `:262`); that is not the Swallowtail
`session/new` path.

The MCP tool call needs no workspace write, so the live gate used the
read-only Plan profile. The child isolated environment set
`GEMINI_CLI_TRUST_WORKSPACE=true` so Plan was not clamped. That is a
consumer environment binding, not a host settings or auth mutation. The
API-key credential stayed an opaque `CredentialRef` approved as delegated;
Swallowtail did not read the keychain or copy `GEMINI_API_KEY` from the
parent.

## Observed gate

Live record stderr (no raw stream retained):

- accepted: false
- typed stop: `tool_not_called`
- model: `auto`
- terminal diagnostic: none
- cleanup diagnostic: none

The disposable listener is test-only, behind `live-probes` for the live
binary and default-feature for the harness proof. It is not a production
path.

No raw provider stream, bearer, account identifier, session id, or private
path is retained.

## Limits

The record covers host exact `0.61.0` only. Other window points stay
unqualified for honouring. The attempt is spent; do not rerun it.

## Disposition

`gemini-cli.acp` `client_mcp_servers` stays No as a producer gap. Research
351 emission and Research 358 identity stand. Research 349 OpenCode
honouring and Research 352 Claude stop are unchanged.

## Next move

A separately authorized live gate would be required to re-test honouring,
including one completed tool call. This task performs no second attempt.
