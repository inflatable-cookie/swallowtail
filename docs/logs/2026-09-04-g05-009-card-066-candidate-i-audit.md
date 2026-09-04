# g05.009 Card 066 Candidate I Audit

Status: complete
Date: 2026-09-04

Card 066 audited the 47-row DeepSeek and DeepSeek Harness candidate I surface.
The exact-head review accepted PR 207 at `21ba9396`; it merged as `85221307`.
The audit reconciles 39 named rows, 6 construction-time withholds, and 2
blocked rows. It stopped because provider-session catalogue and history rows
require shared post-open observation vocabulary, so candidate I is not
promotable on current `main`.

The card changed only its result and one triage note, with zero Rust changes.
No implementation card was compiled. Chatterbox owns reconciliation of the
stop and the deferred shared observation decision.
