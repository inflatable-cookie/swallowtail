Set `slow-timeout = { period = "60s", terminate-after = 3 }` on
`[profile.default]` in `.config/nextest.toml` so a deadlocked local unit test
inside `effigy validate:focused` or `oxide nextest run` fails fast with the
exact test name instead of hanging indefinitely and holding the Effigy lock.
`[profile.ci]` and `[profile.ci-process]` keep their existing two-period
termination bounds.
