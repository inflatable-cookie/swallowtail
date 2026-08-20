# g04.007 Sign-In Loop And Host Ports

Date: 2026-08-20
Roadmap: `../roadmaps/g04/007-sign-in-loop-and-host-ports.md`
Cards: `../roadmaps/g04/batch-cards/019-interactive-sign-in-host-ports.md`,
`../roadmaps/g04/batch-cards/020-sign-in-loop.md`,
`../roadmaps/g04/batch-cards/021-sign-in-fail-closed-and-api-key-collection.md`

## Result

Swallowtail owns interactive sign-in through host ports.

`HostServiceKind` gains optional `UrlOpen`, `LoopbackCallback`, and
`DeviceCodeDisplay`. They do not collapse into Credential, Process, or
Network. Registering a port does not start sign-in. Ports never return
secret bytes. Spawning an approved login helper stays process authority.

`swallowtail-runtime` owns start, poll, complete, cancel, and timeout for
interactive OAuth, device OAuth, delegated CLI login, and API-key
collection. `SignInAction` remains an advertisement. ACP `authenticate` is
not this loop. A mechanism or account change fails closed. Missing required
ports fail the matching loop. Complete materializes `CredentialRef` values;
the 057 store holds those references, not secrets. Contract 014 still owns
leases.

`swallowtail-host-local` ships `LocalSignInPorts` test doubles. Testkit
recording hosts can opt into the same ports. Local process composition does
not imply a browser. `public-api-0.3.3` is unchanged. Additive API is in
`public-api-unreleased` for core, runtime, host-local, and testkit. No
production adapter crate changed. 047 still has no emails or tokens.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-45e4be90`
Worker branch: `t3code/sign-in-loop-host-ports`

PR: https://github.com/inflatable-cookie/swallowtail/pull/6

## Next

PR 6 fast-forwarded onto `main` at `91e14e3d`. g04.008 cards 022-024 are
ready. Overlay stays planned.
