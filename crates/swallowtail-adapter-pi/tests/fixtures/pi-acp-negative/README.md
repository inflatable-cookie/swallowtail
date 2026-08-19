# Pi ACP negative identity corpus

Secret-free identity for the named candidate `pi.acp`. Official Pi has no
native ACP stdio mode. Registry `pi-acp@0.0.33` is community
`svkozak/pi-acp`; it speaks ACP by spawning `pi --mode rpc`.

That collapses onto already-qualified `pi.rpc`. Swallowtail does not wrap
the community adapter and does not add a second package.

No live RPC or ACP session. No `pi-acp` install. No `pi auth`. Host `pi`
`0.83.0` was inspected for `--help` / `--version` only; the install was
not replaced.

No fixture contains a credential, host path, host account identity, provider
payload, or real session id. Host identity is SHA-256 only.
