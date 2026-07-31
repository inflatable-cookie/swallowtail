# 2026-07-31 Prepared Facade Multi-Consumer Usability

## Result

Nucleus and Soundcheck integration evidence produced two portable library
conveniences. Swallowtail now derives local monotonic deadlines from an
explicit caller duration and constructs the fixed Codex ChatGPT-subscription
access profile from a caller-owned identity.

No broader facade was justified. Soundcheck already uses bound prepared
operations. Nucleus can remove its remaining same-role low-level extraction by
adopting existing bound methods.

## Implemented

- `LocalHostServices::deadline_after` converts explicit durations to local
  monotonic deadlines with saturation at both conversion and addition
- `codex_chatgpt_subscription_access_profile` encodes interactive OAuth,
  subscription allowance, the `codex` audience, provider support, and no
  credential reference
- access status and observed versus caller-asserted provenance remain separate
  preparation evidence
- the compile-tested Codex example composes canonical profile and status, then
  uses bound catalogue, session, run, and lifecycle operations
- guidance marks bound operations as the normal path and `into_parts()` as an
  advanced escape hatch
- the public API baseline now includes both additions and reconciles accepted
  public surfaces from the already-completed g03 Antigravity, Cursor, Claude,
  Grok, Kimi, Pi, and Qwen work

Current official Codex documentation keeps ChatGPT sign-in, API-key sign-in,
and enterprise access tokens distinct. The canonical helper therefore covers
only cached ChatGPT subscription access and does not imply public OpenAI API
authority: <https://learn.chatgpt.com/docs/auth>.

## Nucleus Adoption Delta

Nucleus may independently:

1. replace duplicated local nanosecond arithmetic with
   `LocalHostServices::deadline_after`
2. replace duplicated ChatGPT-subscription profile construction with
   `codex_chatgpt_subscription_access_profile`
3. call `CodexPreparedCatalogue::list_models` and
   `CodexPreparedSession::open_session` directly in its catalogue, session,
   task-execution, and smoke paths instead of extracting parts and rebuilding
   the same app-server role

Nucleus keeps executable selection, environment approval, instance identity,
models, prompts, tools, authorization, UI, cancellation ownership, and durable
state. Soundcheck needs no bound-operation migration.

## Validation

- focused host-local and Codex validation: 174 passed
- affected extracted-package proof: two packages passed
- public API declaration baseline: 26 crates passed
- no authenticated Codex run, provider prompt, catalogue, session, or consumer
  repository edit

## Next

Return to the g03 compatibility-maintenance checkpoint. Promote only a new
consumer defect or material non-deferred upstream drift.
