# Kimi Code 0.31.1 currentness corpus

This secret-free delta corpus freezes exact stable Kimi Code `0.31.1` before
Swallowtail widens any production route claim.

Exact source and artifact identity live in `release.json`. The ACP event mapper
and default headless stream renderer are byte-identical to `0.31.0`. The ACP
server delta is comment-only. Experimental v2 headless execution remains
unselected.

The local server retains its bearer, metadata, prompt, WebSocket control, and
model-catalogue schemas. It adds optional `turn.ended.interruptReason`, moves
session lookup through workspace-scoped handlers, and prevents provider-model
refresh from transiently clearing the catalogue. The JSONL fixture freezes the
new optional field; Swallowtail retains the required terminal `reason` and does
not expose the optional provider detail.

The official macOS arm64 artifact reported `0.31.1`, matched its published
SHA-256, passed Apple code-signature inspection, and completed one prompt-free
ACP initialization with no stderr. The installed `0.31.0` executable was not
modified.

No fixture contains a credential, bearer token, host path, account identity,
provider payload, real session id, or model observation.
