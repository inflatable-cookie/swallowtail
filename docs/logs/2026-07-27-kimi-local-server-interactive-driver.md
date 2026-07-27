# Kimi Local Server Interactive Driver

Date: 2026-07-27
Roadmap: g02.020
Card: 064

## Changed

- added a prepared interactive-session route over Kimi's documented local
  REST and WebSocket v2 server
- kept Kimi ACP and local-server preparation, access, transport, lifecycle,
  and management authority separate
- mapped prompt submission, ordered output, volatile events, cursors, epochs,
  resynchronization, disconnect, cancellation, deadline, and terminal status
- mapped manual approvals and structured questions through explicit consumer
  callback exchange
- returned exact resume and archive/restore bindings while preserving provider
  state on interactive handle close
- joined WebSocket, pump-task, credential, and owned foreground-child cleanup
- moved task admission before provider effects

## Boundary

The route controls Kimi's foreground local server. It is neither ACP nor a
Kimi account API. The bearer remains an opaque host credential for one
host-approved loopback endpoint.

Permission mode, optional isolation, provider tools, profile, disabled tools,
model, version evidence, state root, and topology remain explicit. The adapter
does not add a generic prompt method, implicit replay, transport fallback,
delete claim, or sandbox claim.

Task admission is now a before-effect gate. A host that rejects the scoped
pump task cannot open the WebSocket or submit provider work.

## Evidence

- full Kimi adapter: 56 deterministic tests passed; one live installed probe
  remained gated and ignored
- focused interactive corpus: 7 tests passed
- strict Kimi Clippy and `git diff --check` passed
- doctor retained the known 9 oversized-file errors; card 064 added no
  oversized interactive production file
- no live Kimi authentication or provider call occurred

## Next

Card 065 adds the route and lifecycle matrices, public examples, extracted
package proof, redaction evidence, and Nucleus adoption inputs. It does not
publish Swallowtail or edit a consumer.
