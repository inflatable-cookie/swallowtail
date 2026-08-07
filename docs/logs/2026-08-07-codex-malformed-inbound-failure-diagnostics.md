# Codex Malformed-Inbound Failure Diagnostics

Date: 2026-08-07
Roadmap: g03.047
Card: 143

## Outcome

Nucleus hit Codex app-server `0.147.0` emitting a notification the adapter
rejected as `swallowtail.codex.app_server.malformed_notification`; the next
turn failed with "connection is closed". The consumer receives only the
sanitized safe diagnostic, and the adapter discarded the offending payload,
so provider drift was undebuggable from consumer evidence.

Malformed-inbound failures (`malformed_notification`, `malformed_message`)
are now re-issued at the RPC pump boundary with the same code and
classification, appending the notification method and a bounded sanitized
excerpt of the raw line. The pump also retains a 2048-byte app-server stderr
tail and appends a sanitized excerpt to any protocol terminal diagnostic
when stderr captured anything. Both excerpts reuse the discovery sanitizer,
now shared as one crate-internal module: 240 ASCII-normalized characters,
sensitive tokens redacted, truncation marked.

Observed shape:

```text
Codex app-server returned a malformed notification (method `item/plan/delta`, excerpt `<path>`); stderr: codex app-server warning: unrecognized plan delta field detail detail ... [stderr truncated]
```

A compact JSON line is one token, so the sanitizer redacts it to `<path>`;
the method field carries the drift datum and raw payload values stay out of
the safe message.

The policy statement lives in `docs/guides/portable-failure-handling.md`
plus the Codex route guide, not a contract amendment: the change realizes
the existing safe-diagnostic boundary rather than creating a new durable
rule.

## Local Validation

- scripted `MalformedNotification` fixture: terminal diagnostic keeps
  `swallowtail.codex.app_server.malformed_notification`, names the method,
  carries bounded line and stderr excerpts, excludes the raw padding payload,
  and the next request fails with
  `swallowtail.codex.app_server.connection_closed`
- `effigy validate:focused swallowtail-adapter-codex`: 162 passed
- `effigy package:verify-affected swallowtail-adapter-codex`: extracted
  package proof passed
- `effigy package:api`: public API matches the `0.2.0` candidate baseline

## Boundaries

No public API change, diagnostic-code change, version-range change, consumer
mutation, live provider work, tag, or release. The `v0.2.1` source tag
remains a separate operator-authorized step.
