# Post-Grok Claude Agent ACP Selection

Date: 2026-07-24

## Outcome

Card 142 is complete. Research 032 selects Claude Agent ACP as the next
high-information route:

- family `claude-agent`
- ACP v1 over bounded NDJSON stdio
- candidate adapter range `0.52.0..=0.61.0`
- Anthropic public-API-key access
- integration-maintainer-supported ACP adapter over the provider-supported
  Agent SDK and public API
- `Ambient` harness configuration
- `AmbientHost` process isolation
- provider-native read-tool policy without a sandbox claim

The route adds a maintained harness family without another bespoke JSONL
transport. Exact tagged source, package manifests, changelog, and mock-backed
tests permit deterministic development without a live account or container.

## Access Decision

The first route does not expose Claude subscription login.

The adapter advertises a maintained Claude subscription terminal-auth method,
but Anthropic's Agent SDK documentation says third-party products need prior
approval to offer claude.ai login or subscription rate limits. ACP registry
authorship does not establish that approval for Swallowtail.

The first proof therefore binds one host-approved API-key lease to the
Anthropic public API. Terminal auth, login, logout, provider switching, and
credential mutation remain excluded.

## Range Decision

The adapter has 53 published semantic versions. Direct `--version` observation
begins at `0.52.0`; current latest is `0.61.0`.

Card 143 will inspect every point in candidate range `0.52.0..=0.61.0` and
freeze exact milestones at `0.52.0`, `0.53.0`, `0.54.0`, `0.60.0`, and
`0.61.0`. It must qualify or split the range rather than infer continuity.

Wrapper, ACP SDK, Agent SDK, nested native binary, ACP wire, provider API, and
model versions remain separate evidence.

## Contract Decision

No new shared contract is required before exact corpus work.

Contracts 015, 017, 023, 029, 032, and 033 cover ACP, access independence,
ambient harness execution, version qualification, installed discovery, and
ambient configuration. Card 143 must stop and promote a narrow contract if
exact artifacts expose a composite-version, access, or lifecycle gap.

## Planning State

- Research 032 is promoted.
- Card 142 is complete.
- Roadmap 048 remains active.
- Card 143 is rebaselined and ready.
- Cards 144-145 are route-specific and planned.
- Roadmap 047 and cards 138-141 remain on hold.
- g01 remains active at 48 roadmaps.

## Validation

- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed
- `effigy doctor` — unchanged inherited 19 oversized-file findings:
  12 warnings and 7 errors

## Next Task

Execute card 143. Qualify or split the Claude Agent ACP candidate range and
freeze the deterministic public-API-key ACP corpus before production code.
