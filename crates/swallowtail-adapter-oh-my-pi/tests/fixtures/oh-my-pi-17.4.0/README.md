# Oh My Pi 17.4.0 currentness corpus

Secret-free identity freeze for npm `@oh-my-pi/pi-coding-agent@17.4.0`
before Swallowtail widens `oh-my-pi.package`.

Official artifacts stayed in `/tmp`. Extracted `--version` was not run:
the unpacked tarball needs `@oh-my-pi/pi-natives`. Host `omp` was not
on `PATH`. No provider prompt.

This is a published minor-line step from `17.3.8`. Mapped RPC sources
(`docs/rpc.md`, `modes/rpc/*`, `jsonrpc/message-framing.ts`) are
byte-identical to `v17.3.8`. Adapter-private mapping is unchanged, so
the shape is compatible-extension, not a private-milestone.

Selected `--mode rpc` flags and RPC v2 commands remain. Decoder specimen
stays `oh-my-pi-rpc-17.2.9`. Frozen `17.3.7` and `17.3.8` corpora stay.
Do not flatten onto `pi.package`.
