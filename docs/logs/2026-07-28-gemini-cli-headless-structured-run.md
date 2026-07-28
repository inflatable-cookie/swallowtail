# Gemini CLI Headless Structured Run

Date: 2026-07-28

## Changed

- qualified Gemini CLI headless `stream-json` across `0.51.0..=0.52.0`
- added separate discovery, structured-run, process, parser, cancellation,
  deadline, terminal, and prepared-operation paths
- added one public Gemini CLI facade with explicit ACP or headless selection
- froze success, unknown, provider-failure, and malformed corpora
- mapped exact native exits without exposing stderr or provider payloads
- kept sandboxing optional and ambient
- required durable retention because Gemini persists the local transcript
- grouped ACP and headless only after the public typed facade existed

## Evidence

Both local and remote-authoritative fixtures prove exact argv, prompt over
stdin, model and provider-session binding, usage, unknown-event tolerance,
malformed and incomplete failure, cancellation, timeout, force-stop, wait,
task join, redaction, and unsupported-input rejection before process start.

The new prepared path probes the exact executable, keeps the version
observation, derives ambient plan authority, and starts only the selected
headless driver. ACP and Gemini Live regressions remain unchanged.

## Validation

- `cargo test -p swallowtail-adapter-gemini`: 43 passed, one gated live probe
  ignored
- strict Gemini Clippy and rustdoc pass
- all workspace examples compile
- route, lifecycle, 21-solution CSV, docs, next-action, format, and
  `git diff --check` gates pass
- the CSV has 45 columns, 21 provider-sorted rows, 24 unique route identities,
  16 structured `Yes`, and five `No`
- `effigy doctor` returns only the pre-existing 59 oversized-file findings:
  44 warnings and 15 errors; this batch adds none

## Next

Card 077 qualifies current Kimi Code `0.29.2` before any Kimi structured route
extends the guaranteed range.
