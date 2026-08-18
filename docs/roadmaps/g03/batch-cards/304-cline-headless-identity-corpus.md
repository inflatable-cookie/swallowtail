# 304 Cline Headless Identity Corpus

Status: planned
Owner: Tom
Created: 2026-08-18
Milestone: `../087-cline-headless-route.md`
Depends on: g03.086 Cline ACP Route; Card 261; Research 143

## Goal

Freeze the official Cline headless/JSON CLI identity and deterministic process corpus for `cline.headless`.

## Scope

Record executable identity, non-interactive invocation, prompt/input boundary, JSON/stream framing, stdout/stderr and exit behavior, working-resource authority, process supervision, cancellation/deadline, and cleanup. Keep ACP protocol evidence out of this corpus.

## Out Of Scope

ACP execution, driver code, prepared API, production claims, installation, login, live provider work, and version-range claims

## Acceptance Criteria

- [ ] exact headless executable and version axis are recorded
- [ ] JSON/stream/process fixtures and drift rejection are named
- [ ] input, workspace, cancellation, and cleanup limits are explicit
- [ ] no claim changes before card 305

## Validation

`effigy qa:northstar`; source and fixture review only.

## Stop Conditions

Stop if the surface is prompt-only, UI/TUI-only, undocumented, or cannot establish bounded machine output without hidden credential state.

## Auto-Continuation

Continue to card 305 after headless route identity and corpus shape are frozen.

## Evidence

Research 143; https://github.com/cline/cline; https://docs.cline.bot/cli/cli-reference
