# OpenCode 1.14.48 Protocol Fixture

Captured: 2026-07-20

## Sources

- maintained server documentation: <https://opencode.ai/docs/server/>
- maintained permission documentation: <https://opencode.ai/docs/permissions/>
- installed binary: `opencode 1.14.48`
- installed OpenAPI 3.1 document: `GET /doc`
- installed OpenAPI SHA-256:
  `cf27aa005d0bb366671b134c4cdfdb040a939b513ae0db1af35a172af8daecf2`

The installed probe used `--pure`, an explicit loopback host, and an explicit
port. It made unauthenticated `GET /global/health` and `GET /doc` requests. It
did not read or mutate provider authentication or OpenCode configuration.

## Frozen Subset

| Operation id | Method and path | Purpose |
| --- | --- | --- |
| `global.health` | `GET /global/health` | observe server version |
| `provider.list` | `GET /provider` | discover separate provider and model ids |
| `session.create` | `POST /session` | create one deny-first session |
| `session.prompt_async` | `POST /session/{sessionID}/prompt_async` | start one correlated turn |
| `event.subscribe` | `GET /event` | receive ordered SSE events |
| `session.abort` | `POST /session/{sessionID}/abort` | interrupt provider work |

Attached close sends no dispose, auth, config, share, or delete request. The
driver owns its connection work, not the external server lifecycle.

## Drift And Stop Rules

- maintained docs report port `4096`; installed help reports port `0`
- maintained docs show the older session permission-reply route; installed
  OpenAPI exposes `/permission/{requestID}/reply`
- neither default port nor permission-reply route belongs to the frozen subset
- session creation sends an explicit deny-first permission ruleset
- unexpected permission or question events require abort and terminal failure;
  the driver never approves, answers, or mutates configuration
- unknown semantic events fail the turn until an explicit namespaced extension
  policy accepts them
