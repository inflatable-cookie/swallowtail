# Claude Agent ACP Form Elicitation

Status: promoted
Owner: Tom
Created: 2026-07-30
Evidence checked: 2026-07-30

## Question

Can Claude Code `AskUserQuestion` cross the existing Claude Agent ACP route as
portable typed harness user input? Can the same route carry one opaque
consumer context string?

## Current Evidence

ACP wire protocol remains version `1`. Current elicitation is unstable and
capability-gated:

- clients advertise `clientCapabilities.elicitation.form = {}`
- agents issue `elicitation/create`
- form requests carry a session, optional tool call, message, and restricted
  JSON Schema
- clients answer `accept`, `decline`, or `cancel`

Primary sources:

- [ACP elicitation RFD](https://agentclientprotocol.com/rfds/elicitation)
- [ACP repository versioning](https://github.com/agentclientprotocol/agent-client-protocol)
- [claude-agent-acp releases](https://github.com/agentclientprotocol/claude-agent-acp/releases)
- [claude-agent-acp 0.64 elicitation source](https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.64.0/src/elicitation.ts)

The installed evidence package is claude-agent-acp `0.63.0`, ACP SDK `1.3.0`,
and Claude Agent SDK `0.3.220`. The latest public wrapper is `0.64.0`, released
2026-07-30. It adds a custom-answer companion marker but does not change the
answer field keys.

Tagged source comparison covers `0.53.0`, `0.54.0`, `0.57.0`, `0.58.1`,
`0.59.0`, `0.60.0`, `0.61.0`, `0.62.0`, `0.63.0`, and `0.64.0`.
`AskUserQuestion` form bridging exists throughout Swallowtail's maintained
`0.53.0..=0.61.0` window. The relevant source has three revisions:

- `0.53.0..=0.56.0`: option description and preview share
  `_meta._claude/askUserQuestionOption`
- `0.57.0`: intermediate bridge revision
- `0.58.1..=0.63.0`: description is first-class; preview remains in `_meta`
- `0.64.0`: custom `Other` fields add
  `_meta._askUserQuestionCustomAnswer`

The bridge maps questions to indexed `question_<n>` choice fields and
`question_<n>_custom` free-text companions. Accepted content becomes the
Claude tool's `updatedInput.answers`. Decline becomes an empty answer map.

## Context Finding

ACP permits `_meta` on the request, schema, properties, and enum options.
That does not create an end-to-end context signal.

claude-agent-acp builds the form from selected question fields. It does not
copy `AskUserQuestionInput.metadata`, annotations, unknown question fields, or
an arbitrary request context. It copies only option preview content into one
Claude-private enum-option metadata key. The 0.64 custom-answer marker carries
only its paired field id.

An opaque question or request `context` field in Swallowtail would therefore
be empty or invented on this route. No shared runtime field is promoted.
Option preview is a separate presentation feature; forms containing it remain
richer than the current common question record.

## Selected Boundary

Swallowtail advertises form elicitation and accepts only the exact subset that
maps losslessly to `HarnessUserInputRequest`:

- one to four ordered questions
- stable indexed question and option ids
- single or multiple choice
- two to four options
- option descriptions in either qualified bridge encoding
- one optional `Other` text answer per question
- skipped answers

The adapter returns accepted typed answers through the original JSON-RPC id.
Consumer callback failure maps to decline. Unknown, malformed, numeric,
boolean, constrained-text, preview-bearing, URL, or otherwise richer forms
are not flattened. Unsupported forms are declined.

Advertising form support also lets the bridge forward MCP forms and refusal
fallback dialogs. Only a request matching the qualified portable subset enters
the common callback exchange. No MCP, fallback, model-switch, provider tool,
or URL authority follows from the capability.

## Promotion

- Contract 015 owns ACP capability, method, response, and unsupported-form
  behavior.
- Contract 041 owns lossless typed-question admission and the no-context rule.
- Roadmap g02.048 owns corpus, implementation, conformance, and closeout.

