# 2026-08-06 Installed Harness Rustdoc Closure

## Result

`swallowtail-adapter-antigravity`, `swallowtail-adapter-cursor`,
`swallowtail-adapter-grok`, and `swallowtail-adapter-qwen` now enforce
crate-root denied missing docs.

The review keeps each harness route honest. Antigravity separates catalogue,
one-shot headless work, and durable continuation with context-losing
fresh-session restoration. Cursor separates catalogue, one-shot stream-JSON,
and interactive ACP with exact session-attachment recovery. Grok exposes
interactive and structured ACP over delegated OAuth with exact attachment
recovery. Qwen shares one installed CLI across catalogue, structured run, and
turn-scoped session roles; its session recovery remains a context-losing fresh
replacement.

No facade gains routing, credential discovery, retry, fallback, or stronger
recovery authority. The batch removes 262 warnings, reducing the workspace
from 1,417 to 1,155 without suppression. Eighteen of 27 packages now enforce
denied missing docs. The remaining warnings belong to nine installed-harness
and local-runtime adapters.

## Validation

- focused validation passed 132 tests across the four adapter packages
- warnings-denied clippy passed for all four packages
- extracted package proof passed for all four archives
- crate-root denied-missing-doc Rustdoc passed for all four packages
- the 27-package semantic API baseline remained unchanged
- workspace all-feature Rustdoc completed with 1,155 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the remaining stateful installed-harness adapters,
starting with Codex, Claude Agent, and Kimi.
