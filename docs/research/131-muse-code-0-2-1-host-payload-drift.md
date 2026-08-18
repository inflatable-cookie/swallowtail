# 131 Muse Code 0.2.1-R1215.1 Host Payload Drift

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 235

## Question

After Research 127, which exact-pin host-drift family should move first, and
does installed Muse Code `0.2.1-R1215.1` justify moving the opaque
`muse-code.signed-payload` pin?

## Exact-pin ranking

Installed exact-pin host drift:

| Family | Host | Pin | Posture | Rank |
| --- | --- | --- | --- | --- |
| Muse Code headless | `muse-bin-0.2.1-R1215.1` | `0.1.0-R708.1` | QualifiedOnly opaque | first |
| Command Code | local `1.15.1` matches pin | exact `1.15.1` | QualifiedOnly | later; external npm drift only |
| DeepSeek Harness | not on PATH | exact rc pins | QualifiedOnly | later |
| llama.cpp | not re-probed | exact build ids | QualifiedOnly | later |
| ZCode | unchanged on runtime axis | exact `0.16.3` | QualifiedOnly | later |

Not exact-pin (do not mix into this milestone):

- Claude Code headless is AllowUnverified on exact `2.1.220`; local `2.1.233`
  is already permitted UnverifiedNewer. Rank it with the AllowUnverified
  cluster later.
- Gemini remains deferred.

## Muse method

Compared Research 112 / frozen `0.1.0-R708.1` corpus to local
`muse-bin-0.2.1-R1215.1`:

- `--version`, root help, `exec --help`
- codesign identity
- one deterministic `exec --json --provider echo` run (no Meta provider)

No install, update, login, or Meta prompt.

## Identity

| Fact | Value |
| --- | --- |
| reported version | `Muse Code 0.2.1 (0.2.1-R1215.1)` |
| payload basename | `muse-bin-0.2.1-R1215.1` |
| payload SHA-256 | `b67f181fb7a519007146104c56fad372f47428da9608ade59835899160f2d6e9` |
| payload size | 166797968 |
| launcher SHA-256 | unchanged `21c66e550a71cac2e4af081cc33d10bec81993d0043ec492761fc449e6c440f6` |
| team | `V9WTTPBFK9` |
| identifier | `muse-arm64` |

Current discovery parser only accepts `Muse Code 0.1.0 (...)`, so the host
payload cannot classify until the parser moves with the pin.

## Protocol comparison

- JSONL `schema_version` remains `1`
- deterministic echo payload-type sequence matches the frozen 23-record
  `0.1.0-R708.1` echo corpus
- terminal completes with `echo:` text
- help deltas: root `config` subcommand; `exec --expand-file-mentions`;
  `--enable-shell-tool` wording. Selected Swallowtail argv does not use those
  new flags.

## Segment decision for card 236

Opaque pin move. Keep QualifiedOnly. Reuse behavior
`muse-code.events-v1`. Do not keep both opaque segments (core forbids more
than one opaque segment).

Card 236 moved the opaque pin to exact `0.2.1-R1215.1`, reused
`muse-code.events-v1`, and left `0.1.0-R708.1` incompatible. No new public
operation. No Meta qualification in this card.
