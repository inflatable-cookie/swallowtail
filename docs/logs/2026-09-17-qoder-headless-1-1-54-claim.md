# 2026-09-17 Qoder Headless 1.1.54 Claim

g05.080 advanced `qoder.headless` from the g05.070 identity stop to one exact
`1.1.54` `QualifiedOnly` point. The claim id is
`qoder.headless.package-window-2`; the new adapter-private behavior revision is
`qoder.headless.stdio-stream-json-v2`. The historical `1.1.25` point is not
retained as a second segment.

The route now owns a deliberate fixed `--max-turns 8` AgentLoop ceiling. Eight
is the upper edge of the established short one-prompt bounded posture: useful
tool-assisted work remains possible while an AgentLoop cannot run effectively
unbounded. A stream-json `result` with `subtype: "error_max_turns"`,
`is_error: true`, and `num_turns: 8` maps to the provider-failed diagnostic
`swallowtail.qoder.headless.max_turns`. The selected stream, permission,
retention, cancellation, deadline, and cleanup semantics remain unchanged.

Research 328 froze official npm `@qoder-ai/qodercli@1.1.54`, and the current
identity and reopen fixtures assert the exact point, the forwarded bound, the
terminal shape, and the unselected `vendor/sites` additions. Focused Qoder
fixtures and package validation passed. No provider prompt, login, credential,
installation, host update, downloaded-artifact execution, release, tag,
publication, or consumer mutation occurred.
