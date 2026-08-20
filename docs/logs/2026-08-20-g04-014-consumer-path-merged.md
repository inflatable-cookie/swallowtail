# g04.014 Consumer Path Merged

Date: 2026-08-20
Roadmap: `../roadmaps/g04/014-connection-lifecycle-consumer-path.md`
PR: https://github.com/inflatable-cookie/swallowtail/pull/12

## Result

PR 12 fast-forwarded onto `main` at
`7810453f0171a7ef76e0046f6224b022811332f0`. Review comment
https://github.com/inflatable-cookie/swallowtail/pull/12#issuecomment-5359124892
is the canonical verdict. `v0.3.3` still peels to `51d18620`.

The Contract 052 consumer path is on `main` for Anthropic Messages, Codex
app-server, and Ollama attach. Remaining production routes have no addable
descriptors. The g04.010 first-proof-plus-consumer-path goal is complete.
Hosted OAuth stays a remaining gate.

## Next

Reassess hosted interactive OAuth. Do not compile it without a
no-secret-extraction proof. Do not mark the remaining production routes as
addable.
