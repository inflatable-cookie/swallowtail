# 273: Runtime and shared HTTP/ACP engine defect fixes

- `swallowtail-runtime` (`sign_in.rs`): Await cleanup futures directly when no
  `TimeService` is registered instead of failing on the first `Poll::Pending`,
  avoid re-polling a completed teardown budget timer when it resolves before
  the deadline instant, and surface `RuntimeFailure` errors returned by
  `request_stop`, `force_stop`, and `wait` during sync and async sign-in
  teardown.
- `swallowtail-runtime` (`installed_discovery.rs`): Do not misclassify bounded
  child teardown as `DiscoveryStatus::CleanupFailed` when `request_stop` and
  `wait` succeed but `force_stop` errors on an already-exited process.
- `swallowtail-transport-http` (`stream.rs`, `request.rs`): Release the channel
  state mutex before calling `Waker::wake` in `ChannelSender::send` and
  `ChannelSender::drop`, use poison-safe recovery in channel `Drop` impls,
  align `SecretHeader` name validation with RFC 9110 `is_header_name_token`,
  and reject `#` fragments in embedded query strings as well as routes.
- `swallowtail-acp-client` (`connection.rs`): Check
  `pending.requests.contains_key(&id)` before inserting into `pending.requests`
  so a duplicate correlation ID never clobbers a live waiter's oneshot sender.
