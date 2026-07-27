# Claude Agent Session Management

Date: 2026-07-27

## Change

Card 053 maps qualified Claude Agent ACP lifecycle behavior into the production
stdio driver and prepared facade.

Initialization now checks independent close and delete capability fields.
Missing lifecycle capability stops before session creation. Qualified handle
cleanup sends `session/close` before closing stdin, stopping the owned process,
joining the pump and active turn, and releasing resource and credential
leases. Connection close and provider-native close remain separate outcomes.

Prepared sessions return an opaque `ProviderSessionManagementBinding` with no
load or resume claim. `ClaudeAgentPreparedDelete` accepts that inactive binding
and creates one typed immutable delete plan. Execution starts a new scoped ACP
process through the same API-key, executable, ambient-authority, and read-only
resource bindings.

## Effect Truth

An exact empty delete response yields `ProviderDataDeleted` with
`ProviderDefinedDescendants`. Rejection, malformed response, disconnect,
cancellation, or deadline after dispatch cannot claim confirmed deletion.
Generic ACP remains `HistoryRemoved`.

Qualified `0.53.0..=0.61.0` sessions use native close. Published `0.62.0`
remains unverified-newer: prepared deletion requires explicit acceptance and
native close is not promoted into the guaranteed cleanup claim.

The route reads no Claude state path, extracts no credential, and claims no
hard erasure, Anthropic API service-data deletion, subscription access, private
OAuth access, history listing, load, or resume.

## Validation

- Claude Agent, ACP protocol, runtime, testkit, and management suites:
  213 tests pass
- targeted Claude Agent clippy with warnings denied: pass
- `effigy check:rust`: pass
- `effigy format:check`: pass
- docs, Northstar, and diff checks: pass
- `effigy doctor`: unchanged baseline of 25 findings
  (17 warnings, 8 errors)

## Next

Card 054 is ready. Run the shared management pack across lifecycle versions and
prove identical qualified semantics through stdio and explicit remote ACP with
no transport fallback.
