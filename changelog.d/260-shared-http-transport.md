The twelve `curl` HTTP/SSE adapters now share one bounded `swallowtail-transport-http`
crate. Its single exchange owns SSE framing, backpressure that blocks instead of
abandoning a live stream, connect and finite-transfer timeouts, `Ok(0)` response-bound
aborts, cancellation on close and drop, and zeroized secret headers; the adapters keep
their provider protocols and public surfaces unchanged.
