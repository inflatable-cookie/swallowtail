# Kimi Code 0.28.1 ACP Fixtures

Deterministic ACP v1 transcripts for the ambient Kimi Code portability subset.

Pins:

- maintained TypeScript Kimi Code `0.28.1`, not the wound-down Python line
- annotated release tag object and peeled source commit independently
- `@moonshot-ai/acp-adapter` `0.3.4`
- `@agentclientprotocol/sdk` `0.23.0`
- ACP wire version `1`
- stable schema artifact `schema-v1.19.1`
- official `darwin-arm64` release archive and extracted executable digests

The host resolves the pinned embedded executable; ambient `PATH` is not an
authority source. `KIMI_CODE_HOME` points at isolated provider state and
`KIMI_CODE_NO_AUTO_UPDATE=1` prevents checks, background installs, and prompts.
The upstream Developer ID signature is retained. This route is explicit
`AmbientHost`: the working directory is a location and callback scope, not a
filesystem boundary. No re-signing, App Sandbox, container, or other process
containment is required or claimed.

The corpus covers initialization, one bound new session, load with replay,
resume without replay, text prompt/update, active-turn cancellation, bounded
text replacement callbacks, authority rejection, capability/version drift,
authentication failure, disconnect, and process-owned close evidence.

It does not launch Kimi, authenticate, approve tools, mutate provider state,
run local shell or background work, enable plugins, MCP, subagents, image or
embedded-context prompts, claim native session close, or treat filesystem
callbacks as process containment. Enforced isolation remains a separate,
optional route if a future provider or host mechanism qualifies.
