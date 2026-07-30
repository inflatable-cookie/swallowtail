# Python Kimi CLI Headless Route

Status: declined
Owner: Tom

## Question

Should Swallowtail add Moonshot's Python `kimi-cli` as a separate installed
harness route?

Decision: no. Research 068 proves that maintained native Kimi Code `0.31.0`
already supplies authenticated ACP and stream-JSON headless execution. The
operator does not need another Kimi distribution while that route works.

## Evidence

- Python `kimi-cli` added `--thinking` in `0.51`.
- Its current command surface documents `--thinking` and `--no-thinking`.
- TypeScript `@moonshot-ai/kimi-code` exact `0.29.2` and current main expose no
  equivalent headless input.
- The repositories, distributions, implementations, version axes, and
  reasoning semantics differ.

## Reopen Gate

Reopen only when a concrete consumer need cannot be met by Kimi Code ACP,
headless, or local-server routes and the operator approves another production
route. Promotion must then qualify:

- executable identity, discovery, and version binding
- supported authentication and local state
- print-mode framing and usage evidence
- boolean reasoning mapping by selected model
- retention, working-resource, cancellation, and cleanup posture
- prepared facade, package proof, route matrix, and consumer selection impact

Do not widen `kimi-code.executable` or reuse `kimi-code.headless`.
