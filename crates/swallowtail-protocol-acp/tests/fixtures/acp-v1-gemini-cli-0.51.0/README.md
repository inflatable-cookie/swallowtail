# ACP v1 / Gemini CLI 0.51.0 Fixture

Deterministic, credential-free transcript records for the first Swallowtail ACP
proof. Each NDJSON line wraps one exact wire message with its direction. The
`message` value, not the wrapper, is written on ACP stdio.

Authority checked 2026-07-20:

- ACP stable wire version `1`
- ACP schema release `schema-v1.19.0`
- Gemini CLI stable release `0.51.0`
- Gemini CLI ACP SDK `0.16.1`

The corpus freezes a smaller subset than either implementation. It uses new
read-only sessions, text prompts, updates, native turn cancellation, permission
cancellation, and bounded read-only filesystem callbacks. Authentication
mutation, load/resume, mode/model mutation, MCP injection, filesystem writes,
terminal callbacks, and unstable extensions remain outside the first driver.

The values are synthetic. No provider response, credential, user path, or live
Gemini installation was captured.
