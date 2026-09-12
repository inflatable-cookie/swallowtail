# Grok 1.0.25 model catalogue fixtures

Fake-process stdout documents for the exact `1.0.25` `models` text grammar
frozen by Research 305. The `grok models` command never ran: these encode the
statically proven skeleton (`Default model:` header, `Available models:`
header, id rows, one ` (default)` marker agreeing with the header).

- `models.txt` — accepted document: `grok-4.6` default first, `grok-4.5`
  second. `grok-4.5` has no `description` in the frozen default-model
  document, so it also proves sparse optional metadata.
- `unknown-ids.txt` — a valid unknown id (`grok-4.7`) passes through with
  empty metadata instead of failing or being dropped.
