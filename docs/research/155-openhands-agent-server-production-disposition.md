# 155 OpenHands Agent Server Production Disposition

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 290

## Question

Should `openhands.agent-server` enter the production route matrix after
cards 287-289, given live HTTP/WebSocket conversation stays unwired?

## Method

Reconciled Research 154, the 288 driver, and the 289 prepared facade.
`OpenHandsAgentServerPreparedRun::start_run` uses
`OpenHandsAgentServerDriver::new`, which fail-closes with
`swallowtail.openhands.agent_server.live_http_unwired` before spawn.
Fixture JSON events prove decode, terminals, cancel, deadline, and
joined cleanup. They are not a live Agent Server conversation.

No install. Host Python is `3.9.6`; the wheel requires `>=3.12`. No
login, prompt, or live server.

## Disposition

Deferred. Not negative: identity, spawn argv, AlwaysConfirm,
loopback host, and process cleanup remain admitted. Not production:
consumers cannot run a conversation against a real Agent Server.

Keep `swallowtail-adapter-openhands` as an unreleased package without a
production route. Do not add guide, example, or matrix rows. Revisit
when live loopback HTTP plus `/sockets/events/{id}` is wired against
the frozen `1.42.1` corpus.

Current source stays 38 packages and 45 production routes.
