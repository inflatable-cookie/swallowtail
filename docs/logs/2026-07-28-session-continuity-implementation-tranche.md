# 2026-07-28 Session Continuity Implementation Tranche

## Decision

Implement the five card 094 cells through existing route-specific prepared
profiles. Do not add a generic lifecycle facade or widen provider retention
claims.

## Realized Paths

- Codex app-server load projects bounded ordered `thread.turns`. Resume uses
  `excludeTurns: true` from `0.129.0`; older qualified segments bound and
  ignore returned turns.
- Claude Agent ACP load buffers ordered replay until attachment succeeds.
  Resume accepts no historical replay. Both bind exact session, working
  resource, model, access, and MCP configuration.
- OpenCode HTTP load verifies the exact attached session, follows bounded
  message cursors, reverses page order without reversing page items, and
  returns replay before readiness. Resume verifies the session without
  fetching history.

All paths return exact resume bindings. Prepared management authority records
whether the handle was loaded or resumed. Provider-state policy stays
`Prohibited`; load and resume do not imply archive, deletion, native close, or
server ownership.

## Evidence

- six Codex app-server continuity segments execute
- ten qualified Claude Agent ACP releases execute
- twelve OpenCode published segments execute across seven wire surfaces
- all five public prepared operations execute in deterministic fixtures
- full Codex, Claude Agent, and OpenCode adapter suites pass
- the full workspace suite passes with four live probes ignored
- matrix enforcement passes at 432 total `No` cells and 53 continuity `No`
  cells

No live credential or provider effect was used.

## Remaining Risks

- Gemini CLI ACP load replay ordering remains upstream-blocked and is not
  silently qualified.
- Alibaba Conversations and Anthropic Managed Agents still need a retained
  hosted-session contract branch before load or resume implementation.
- Unverified-newer releases remain executable under explicit compatibility
  posture, without guaranteed continuity support.

## Next

Execute card 095. Re-audit the 58 starting cells, run dirty-snapshot package
proof, and select the next exact tranche. Pi RPC load and resume remain the
leading contract-ready continuation.
