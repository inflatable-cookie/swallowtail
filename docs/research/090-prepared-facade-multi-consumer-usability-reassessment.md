# 090 Prepared Facade Multi-Consumer Usability Reassessment

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

After sustained Nucleus and Soundcheck use, what prepared-facade friction still
belongs in Swallowtail rather than either consumer?

## Method

The reassessment compared Contract 037, the Codex prepared integration guide
and example, current Swallowtail bound operations, and both consumer
integration modules. It read the dirty consumer worktrees without modifying
them and ran no provider prompt, authentication, catalogue, session, workspace
mutation, installation, or publication.

Current official Codex authentication documentation was checked because access
posture is temporally unstable. It still separates ChatGPT subscription login
from usage-based API-key login and enterprise access tokens.

## Consumer Evidence

### Nucleus

Nucleus uses `prepare_codex`, named read-only and bounded-workspace profiles,
typed question exchange, plan mode, activity, callbacks, cancellation, and
joined cleanup. The original manual configured-instance and plan-echo failures
are gone.

Its current adapter still calls `into_parts`, reconstructs
`CodexAppServerDriver`, and invokes low-level catalogue and session roles in
three paths. `CodexPreparedCatalogue::list_models` and
`CodexPreparedSession::open_session` already provide those exact bound
operations. This is consumer adoption debt, not a missing Swallowtail
mechanism.

### Soundcheck

Soundcheck uses separate app-server catalogue and structured-exec preparation,
then calls the bound `list_models` and `start_run` operations. It proves that
the current layered facade works for a fixed function without an umbrella
provider API.

Soundcheck retains its prompt, model, reasoning, search, schema, attachment,
timeout, cancellation, progress, attempt evidence, validation, and product
application. Those remain correct consumer responsibilities.

### Repeated Library-Neutral Glue

Both consumers independently:

1. convert an explicit `Duration` into a local host-monotonic `Deadline` with
   nanosecond conversion and saturating arithmetic
2. construct the same Codex ChatGPT-subscription profile: interactive OAuth,
   subscription allowance, the Codex audience, and provider support

The first is concrete local-host clock behavior. The second is adapter-owned
route identity. Neither chooses a consumer timeout or asserts live access.

## Decisions

1. Add `LocalHostServices::deadline_after(Duration)`. The caller still chooses
   whether a deadline exists and its duration. The local host owns conversion
   to its monotonic nanosecond axis and saturates overflow.
2. Add `codex_chatgpt_subscription_access_profile(AccessProfileId)`. The
   consumer still owns the profile identity and supplies observed or explicitly
   asserted `AccessStatus` separately.
3. Keep API-key and enterprise-token Codex access separate. The ChatGPT helper
   grants no public OpenAI API credential, billing, or endpoint authority.
4. Keep `CodexPreparationInput::new` and all low-level roles public. Do not add
   a generic setup object, provider router, default model, default target, or
   default access status.
5. Record a Nucleus handoff to replace existing `into_parts` paths with bound
   operations. No Nucleus or Soundcheck edit belongs in this Swallowtail batch.

## Retained Consumer Responsibilities

- executable selection, script or wrapper policy, and fallback candidates
- approved environment variables, working resources, attachments, and network
  service policy
- configured-instance and access-profile ids
- access observation or caller assertion
- request, scope, route, model, reasoning, prompt, tool, schema, and timeout
  choices
- event projection, cancellation UX, persistence, workflows, and product state

Centralizing these would import consumer intent or create silent authority.

## Contract Result

Contracts 010 and 037 now state the two convenience boundaries. No new
contract, architecture component, operation shape, or execution authority is
required.

## Risks

- a convenience constructor can look like access discovery; its name and tests
  must preserve caller-supplied status and provenance
- Codex supports other official login methods; the ChatGPT helper must not
  become an exhaustive authentication enum or fallback
- consumer line count will remain substantial because translation, policy,
  persistence, and lifecycle handling correctly remain downstream

## Sources

- [Codex authentication](https://learn.chatgpt.com/docs/auth)
- Nucleus `crates/nucleus-agent-adapters/src/swallowtail_codex*`
- Soundcheck `src-tauri/src/swallowtail_codex*`
- Swallowtail Contract 037 and Codex prepared integration guide

