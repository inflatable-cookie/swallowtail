# Grok 1.0.25 model catalogue fixtures

Fake-process stdout documents for the exact `1.0.25` `models` text grammar.
The grammar was recovered statically from the installed binary's shipped
`xai-grok-pager/src/models.rs` format pieces after the failed first live
observation:

- an optional auth-status preamble line, for example
  `You are logged in with <account>.`;
- `Default model: ` followed by the source default id and a newline;
- `Available models:` and a newline;
- one row per model: `  * <id> (default)` for the source default and
  `  - <id>` for every other model. The bullet prefix is required; a bare
  two-space row is not the exact-`1.0.25` format.

The `grok models` command never ran during planning. These fixtures encode the
statically proven skeleton only.

- `models.txt` — accepted document: `grok-4.6` default first, `grok-4.5`
  second. `grok-4.5` has no `description` in the frozen default-model
  document, so it also proves sparse optional metadata.
- `unknown-ids.txt` — a valid unknown id (`grok-4.7`) passes through with
  empty metadata instead of failing or being dropped.
