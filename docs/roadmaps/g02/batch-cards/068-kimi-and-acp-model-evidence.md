# 068 Kimi And ACP Model Evidence

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../021-model-catalogue-coverage.md`

## Objective

Add Kimi local-server catalogue coverage and expose already-authorized ACP
model options without treating session creation as catalogue discovery.

## Scope

1. Qualify Kimi local-server `GET /models` across exact `0.28.1` and `0.29.0`.
2. Add attached and owned-foreground prepared catalogue operations.
3. Preserve endpoint, bearer, state-root, topology, and version authority.
4. Expose bounded model option observations from already-open Gemini and Kimi
   ACP sessions.
5. Keep Claude Agent caller-supplied allowlists separate from discovery.
6. Add `0.29.1` and `0.29.2` Kimi currentness evidence before extending the
   qualified range.

## Acceptance Criteria

- [x] Kimi catalogue does not mutate default model or refresh providers
- [x] ACP model options require an already-authorized session
- [x] no session is opened solely for model discovery
- [x] Claude caller configuration is not relabeled provider evidence
- [x] later stable versions remain visible unverified-newer until qualified

## Evidence

- exact Kimi `0.28.1` and `0.29.0` authenticated catalogue corpus
- attached local-server prepared catalogue and production-driver coverage
- bounded immutable Gemini and Kimi negotiated-session model options
- missing and malformed option fixtures without hidden session creation
- `0.29.1` and `0.29.2` retained as visible unverified-newer points

## Auto-Continuation

No. Review the model-option API shape before hosted-provider breadth.
