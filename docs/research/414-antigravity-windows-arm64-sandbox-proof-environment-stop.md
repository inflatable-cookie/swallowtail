# 414 Antigravity Windows ARM64 Sandbox Proof Environment Stop

Status: provider-free proof stopped before cloning or vendor execution. No
qualification or route claim changed.

Owner: swallowtail#148
Date: 2026-10-08
Axis: `antigravity-cli.release`
Route: `antigravity.headless`
Target: exact `1.2.17` Windows ARM64 PE, selected `--print --output-format
stream-json --sandbox` path

## Outcome

The approved Windows VM exists, but its current state cannot be cloned under
the documented shutdown requirement without changing the original. The task
authority requires the original VM to remain untouched and directs this run to
stop when cloning the suspended state requires touching it. No clone was
created. No Windows guest was booted and no Antigravity binary was run.

This is a finite setup report. It does not prove Windows sandbox enforcement,
and it does not qualify or narrow a route.

## Read-only VM observations

On 2026-10-08, `prlctl list -a` and `prlctl list -i
67f62782-c64a-4c02-ba9a-7d9b24d107ce` reported:

- Host: Darwin `arm64`; Parallels CLI `27.0.1`.
- Original VM: UUID `67f62782-c64a-4c02-ba9a-7d9b24d107ce`, Windows 11,
  state `suspended`.
- VM configuration: Apple hypervisor, ARM CPU type, and `efi-arm64` firmware.
  This identifies an ARM64 guest configuration. Native guest execution was
  not observed.
- The original had a shared network adapter enabled, two host-folder shares,
  shared profile and guest automount enabled. Host-defined sharing reported
  `Unknown`. Host/guest application sharing, clipboard sharing, camera and
  gamepad auto-sharing, USB support, host printer sync, and host location
  sharing were enabled.

The original configuration was read only. No `prlctl clone`, `resume`,
`stop`, source-mutating `set`, or boot command was issued. Only CLI help was
queried. No clone UUID or ownership exists.
The Windows build, guest principal, local account, and guest execution mode
remain unobserved. No credential or authentication store was read.

The [Parallels Desktop cloning guide](https://docs.parallels.com/landing/pdfm-ug/v20-en-us/parallels-desktop-for-mac-20-users-guide/advanced-topics/working-with-virtual-machines/cloning-a-virtual-machine)
specifies that the source VM must be shut down before cloning. The approved VM
is suspended. This run has no authority to shut down or resume the original,
and the installed CLI help did not establish a safe suspended-source clone
procedure. The source was left untouched.

## Exact target identity

Parent task 111 froze the official `1.2.17` Windows ARM64 asset at commit
`a56a3f24054243dd2ff2ec4401a7c5e618ea58f8`, in
`crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/headless-windows-1.2.17-evidence.json`:

- Archive: `agy_cli_windows_arm64.zip`, 53,034,452 bytes,
  SHA-256 `204e469b9be795dd8ab61070db3730a6c1fb29ed66995a17344401786069c32f`.
- Member: `antigravity.exe`, PE32+ AArch64 Windows console executable,
  178,700,440 bytes, SHA-256
  `5f7ce5bccdccb72da9c5882262a3bc433b8836ae6b88f74112d5ca14b7f5e755`.

This run read the retained identity; it did not download or re-hash the
artifact. Parent 111 established artifact identity only. It contains no
Windows runtime sandbox proof. Research 401 separately owns the newer
headless static control-flow analysis; this report does not repeat it.

## Exact missing proof

The following evidence is still required before the selected shipped PE can be
attempted:

1. A separate operator ruling that either authorizes shutting down the
   original before cloning, or identifies a vendor-confirmed way to make an
   independent full clone from this suspended state without changing the
   original.
2. A retained clone UUID and setup record; verified disabled networking,
   host folders, host profile and app sharing, clipboard, device and printer
   integration; and an approved offline transfer mechanism for task-owned
   artifacts.
3. A guest record of the Windows build, ARM64 native execution, and a fresh
   unprivileged local test principal. These require access to the isolated
   clone and were not observed here.
4. Independent proof that guest networking and host filesystem access are
   blocked before any PE attempt, plus fake-only validation of the runner's
   identity, architecture, VM, isolation, target-path, durable-record,
   timeout, and process-tree cleanup refusals.
5. With those gates recorded, an exact selected-route run using only the
   shipped ARM64 PE and fake provider/tool fixtures. Capture the selected
   request and output, permitted scratch writes, denied outside-scratch and
   command/elevation attempts, actual side effects, exit or timeout, and child
   process cleanup. If the path requires authentication, provider traffic, or
   unavailable sandbox support, record that exact missing edge and stop.

No guest artifacts were transferred, no harness/refusal fixtures were run,
and no sandbox outcome was observed. Native x64 remains pending and receives
no evidence from this ARM64 task. Keep the existing `1.2.11` headless claim and
all platform claims unchanged.

## Sources

- [Parallels Desktop cloning guide](https://docs.parallels.com/landing/pdfm-ug/v20-en-us/parallels-desktop-for-mac-20-users-guide/advanced-topics/working-with-virtual-machines/cloning-a-virtual-machine)
- Parent task 111 retained artifact record at commit
  `a56a3f24054243dd2ff2ec4401a7c5e618ea58f8`,
  `headless-windows-1.2.17-evidence.json`
- [Contract 023: Antigravity Windows offline sandbox proof](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#antigravity-windows-offline-sandbox-proof)
- [Research 401: Antigravity headless static control-flow evidence](./401-antigravity-headless-static-control-flow-evidence.md)
