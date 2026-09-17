# Qoder headless 1.1.54 reopen corpus

Secret-free official npm identity for the reopened `qoder.headless` route.
The previous exact ceiling was `1.1.52`; the official stable is now `1.1.54`.
The `1.1.53` and `1.1.54` package trees are frozen with their registry
metadata and selected-source inventory.

The selected route remains `qodercli --print --output-format stream-json
--permission-mode dont_ask --max-turns 8 --no-session-persistence --cwd DIR
PROMPT`. The `8` is now a deliberate adapter-owned AgentLoop ceiling. A run
that reaches it terminates with the typed `error_max_turns` result and is
projected as a bounded provider failure.

The `qodercli.js` bundle changes at both hops, so the selected literals and
the effective forwarded `maxTurns` path are rechecked rather than inferred
from package-version continuity. The two `vendor/sites` files added in
`1.1.54` are unselected.

No prompt, login, credential, installation, host update, or downloaded
artifact execution occurred. No fixture contains a credential, host path,
account identity, provider payload, or real session id.
