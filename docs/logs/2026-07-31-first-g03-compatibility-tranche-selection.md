# First g03 Compatibility Tranche Selection

Date: 2026-07-31

## Outcome

g03 card 003 selected one shared-ACP installed-harness maintenance tranche:

- Claude Agent ACP `0.62.0..=0.64.0`
- Gemini CLI ACP and headless at `0.53.0`
- one validation-only OpenCode live-selector repair at closeout

Roadmap g03.002 and cards 004-007 now own corpus selection, Claude extension,
Gemini dual-route extension, and focused package acceptance.

## Why This Tranche

Claude has the highest near-term consumer exposure and meaningful behavior
milestones. Gemini supplies a small unchanged-source comparison across two
separate routes. Together they test shared ACP maintenance without pretending
wire stability qualifies either harness.

Qwen and Pi remain ranked later candidates. Their provider-specific config,
event, and usage changes need separate corpus work. Pi continuity remains in
backlog behind the unchanged cwd-binding gate.

## Boundaries

- no production claim changed
- no provider executable ran
- no authentication, model call, consumer edit, candidate replacement, or
  publication ran
- existing contracts are sufficient

## Next

Execute g03 card 004. Freeze exact Claude and Gemini artifacts and behavior
groups before changing compatibility claims.
