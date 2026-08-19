# OpenHands Agent Server Production Wiring

Status: deferred
Owner: Tom
Source: g03.093 cards 287-290; Research 154-155

## Deferred Work

`swallowtail-adapter-openhands` exists. Card 290 deferred the production
route: live HTTP/WebSocket stays unwired and `start_run` fail-closes.

The package remains. It is not a production route.

## Promotion Gate

Promote only when:

- remote host, workspace, persistence, attachment, cancellation, and cleanup
  can be represented honestly
- live HTTP/WebSocket wiring has exact evidence
- the operator accepts a production row for `openhands.agent-server`

There is no implied revisit date and no ambient fallback from the fail-closed
driver.
