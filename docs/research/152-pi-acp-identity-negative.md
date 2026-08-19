# 152 Pi ACP Identity Negative

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 282

## Question

Does official `@earendil-works/pi-coding-agent` expose a native ACP stdio
wire distinct from already-qualified `pi.rpc`, or is registry `pi-acp`
still a community wrapper that collapses onto RPC?

## Method

Reconciled Research 144/145 with the 2026-08-19 ACP registry snapshot,
npm `pi-acp@0.0.33`, npm `@earendil-works/pi-coding-agent@0.84.2`, host
`pi --help` / `--version` (no session), official RPC docs, and the
community adapter tarball.

Downloaded the 5-file `pi-acp@0.0.33` tarball. Did not install `pi-acp`.
Did not run `pi --mode rpc`, `pi --mode acp`, `pi-acp`, or `pi auth`.
Did not send `initialize` or a prompt. Did not extract credentials.

Observed versions are not new `pi.rpc` claims. No `pi.acp` compatibility
claim.

## Identity

| Surface | Value |
| --- | --- |
| Route | `pi.acp` (named only; not admitted) |
| Official native ACP | absent |
| Official package | `@earendil-works/pi-coding-agent@0.84.2` (already qualified for `pi.rpc`) |
| Official integrity | `sha512-l4E+B7hgXKWddRo8bC/eSue2aWZjEgJ9xIpf5p0Og+lq8a2TArCwJ0HCoCPCgaBP/tN4zbYH/wOwvx9pJpeLCA==` |
| Official gitHead | `914cf1472e715297caa30db4b9535d534a9eb718` |
| Official modes | `text`, `json`, `rpc`; help does not list `acp` |
| Host | `/opt/homebrew/bin/pi` `0.83.0`; SHA-256 `af302f231437eaf6f37691bce4b34234fcb626bcb5eb3910d4fc3f6519bf78ca` (unchanged from the `pi.rpc` 0.84.2 identity corpus) |
| ACP registry | `pi-acp` `0.0.33`; repo `svkozak/pi-acp`; npx `pi-acp@0.0.33`; no first-party args |
| Community npm | `pi-acp@0.0.33`, published 2026-07-30T17:06:15.441Z, still `latest` on 2026-08-19 |
| Community tarball SHA-256 | `9fdeb8a6780c056b32c07242f359084472007308e1ab57757f3339dd9630de4b` |
| Community integrity | `sha512-vX9kY1tK14E72G4dBAx+RGCk/k7XPjTHls6dLUxA8WSkBav6B6JHuSBv3eusp50LCR/GTRsR2kIKsG0Z5jANzw==` |
| Community gitHead | `1bfcb394088ed879db8fd936b570bb626017f878` |

`@earendil-works/pi-coding-agent` still points at
`github.com/earendil-works/pi`. Card-cited `badlogic/pi-mono` is not a
separate official ACP channel. Fork PRs that add `pi --mode acp` are
not the maintained official package.

## Collapse

Community `pi-acp` is an ACP JSON-RPC stdio adapter that spawns
`pi --mode rpc --no-themes` (optional `--session`). Default executable
is `pi` / `pi.cmd`. Optional override is `PI_ACP_PI_COMMAND`. README:
it “spawns `pi --mode rpc`, bridging requests/events between the two.”

That is a foreign wrapper over the already-qualified `pi.rpc` wire, not
an official `@earendil-works/pi-coding-agent` ACP surface.

## Disposition

`pi.acp` closes as negative evidence. Keep `swallowtail-adapter-pi`.
Do not add a wrapper package. Do not wrap `pi-acp`. Do not start cards
283-285 unless official native ACP appears later as a distinct wire.

This does not reopen or change `pi.rpc`.

## Authority And Cleanup (non-route)

Community adapter auth is Pi's existing provider config; `--terminal-login`
spawns Pi for login. Swallowtail does not log in or extract `pi auth`
credentials. Isolation would be ambient-host if this were a route; it
is not. Cleanup would join/kill a wrapper plus RPC child; Swallowtail
does not own that process.

## Non-goals

- qualifying a new Pi version range
- mapping `pi --print`, JSON mode, TUI, continue/resume, or `pi auth`
- wrapping `npx pi-acp`
- installing or sending a live prompt

## Sources

- Research 144/145
- ACP registry snapshot 2026-08-19
- npm `pi-acp@0.0.33` tarball and README
- npm `@earendil-works/pi-coding-agent@0.84.2`
- host `pi --help` / `--version` (no session)
- [earendil-works/pi RPC docs](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/rpc.md)
- [earendil-works/pi discussion 4444](https://github.com/earendil-works/pi/discussions/4444)
