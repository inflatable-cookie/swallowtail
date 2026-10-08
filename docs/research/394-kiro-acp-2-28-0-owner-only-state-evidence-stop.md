# Research 394: Kiro ACP 2.28.0 Owner-Only State Evidence Stop

Status: promoted evidence stop; no compatibility claim changed.
Date: 2026-10-08.
Route: `kiro.acp` on `kiro-cli.release`.
Task: swallowtail#123.
Authority: Contract 029; Contract 023 Kiro ACP Explicit Environment and Kiro
ACP Owner-Only State; decisions `8079018e-2ec0-45fd-aaf3-0351cc57716f` and
`73ff6b53-f8be-4d7f-9031-c8b0a81e3ff1`; Research 320 and 369.

## Decision

Keep `kiro.acp.release-window-1` at its existing exact `2.21.4` point with
behavior revision `kiro.acp.stdio-v1`, `QualifiedOnly`, the same exclusions,
and no new segment. The official Kiro CLI stable is `2.28.0`, and the twelve
published stable points after `2.21.4` remain unqualified. The HTTP MCP live
honouring gate remains exact `2.21.4`.

Tom approved the `2.24.0+` removal of automatic project `.env` loading as a
documented version-specific restriction: only explicitly host-approved
process environment is inherited. Tom also approved `2.27.0+` owner-only
Kiro home and prior-session state as a provider boundary. Those rulings do not
supply qualification evidence. Static inspection does not prove the owner
sweep's reachability from the selected ACP V2 command or its precise effect on
ACP session state, so the `2.27.0`–`2.28.0` points cannot be qualified here.

## Official channel and artifact identity

The official stable manifest was re-probed on 2026-10-08 at
<https://prod.download.cli.kiro.dev/stable/latest/manifest.json>. It reports
stable `2.28.0`. The matching Linux aarch64 headless `tar.xz` asset is
`kirocli-aarch64-linux.tar.xz`, SHA-256
`2b9b26915737d3f8086bc28b9830b9ebe3d4452a26c7c092d9a190d6355937be`, size
`131008436` bytes. This matches the retained archive digest. The vendor
changelog dates `2.28.0` to October 5 and lists all twelve published stable
hops after `2.21.4`, including patch releases `2.22.1`, `2.23.1`, `2.24.1`,
`2.26.1`, and `2.27.1`:
<https://kiro.dev/changelog/cli/>.

The selected runtime artifact is the official Kiro CLI Linux
`aarch64-unknown-linux-gnu` headless distribution. Public archives for the
baseline and all twelve successor points were downloaded and extracted for
read-only inspection; no artifact was executed. Each archive has eight files.
The complete archive digests, sizes, build metadata, per-file SHA-256 digests
and sizes, and exact per-hop added/removed/changed/identical path sets are
frozen in the inventory appendix below. The SHA-256 of the exact UTF-8 JSON payload between the code-fence markers
(excluding both marker lines and with no trailing newline) is
`de014207fa57e7a00a872b667a41a4b4aa2b27f587aa68532ba8a05400617b1b`
(26,606 bytes).

The artifacts expose `BUILD_VERSION`, `BUILD_TARGET_TRIPLE`, `BUILD_DATE`,
and `BUILD_HASH`; those values are frozen per point below. They are build
identities, not source revisions. Kiro states that its public GitHub
repository is an issue and feedback tracker and that product source is not
hosted there: <https://github.com/kirodotdev/Kiro/blob/main/README.md>. No
public source tag or exact source commit was available to identify the shipped
implementation.

## Selected ACP entrypoint and owner-only finding

Kiro's ACP documentation names the exact command `kiro-cli acp` and describes
its V2 methods and local session files at `~/.kiro/sessions/cli/`:
<https://kiro.dev/docs/cli/acp/>. The V3 migration page distinguishes that
V2 command from the separately selected V3 command, which requires
`--agent-engine=v3` and `--auth-method=cli`:
<https://kiro.dev/docs/cli/v3/acp-migration/>. Swallowtail continues to select
the two-argument V2 command and does not add V3 flags or operations. Kiro's
session guide says home state and prior-session data are restricted to the
owner:
<https://kiro.dev/docs/cli/chat/session-management/>.

On `2.27.0`, the official `kiro-cli-chat` artifact is a stripped AArch64 ELF
(Build ID `2915ca2b2930a2d71bbb81e22fcdbd80877127f7`). It has no symbol table
or debug line information. Read-only disassembly and embedded source-path
metadata establish these bounded facts:

- The artifact contains ACP V2 implementation metadata under
  `crates/chat-cli-v2/src/agent/acp/` and owner sweep metadata under
  `crates/chat-cli/src/owner_only_sweep.rs`. The owner sweep first appears in
  the retained hop set at `2.27.0`; it is absent through `2.26.1`.
- The owner worker range is `0x015acf18..0x015aef6c`; a thread wrapper at
  `0x01657950..0x01657ca8` calls it at `0x01657aa4`. The worker contains
  `realpath` calls at `0x015ad210` and `0x015ade14`, a `chmod` call at
  `0x015ad664`, and a `bcmp` call at `0x015adef8`.
- The owner-sweep initializer range is `0x01b896ec..0x01b8a834`. Direct calls
  to it occur at `0x01ba662c`, `0x01bbc2e8`, and `0x01bbc580`. The enclosing
  machine-code ranges also contain metadata for CLI startup and V2 ACP client
  and session-manager modules, but the stripped artifact has no source line
  map that associates those callsites with a Rust call path.
- Diagnostics in the artifact name refusal when a tree does not resolve
  inside the Kiro home, an unowned or non-directory home, inability to tighten
  permissions, and incomplete sweep results. These strings identify failure
  cases but do not determine the exact predicates or resulting ACP response.

The prior retained sweep also classified the permission-classifier additions
visible in `2.27.1` and `2.28.0` as TUI-approval-only; the other ACP client path
is not armed. The `2.24.0` `.env` boundary and this permission classification
are not the remaining blocker. The official `2.28.0` changelog describes V3
features, but changelog descriptions are discovery evidence, not proof that a
selected ACP path is unchanged.

## Exact unresolved edges

The evidence does not establish the following selected-route facts:

1. Whether `kiro-cli acp` dispatch reaches the owner-sweep initializer and
   worker, and whether every selected V2 ACP startup takes the same path.
2. Which exact paths are enumerated and changed, whether the tree walk follows
   or rejects internal and external symlinks, how canonical aliases are
   treated, which ownership predicate is applied to the home and descendants,
   and whether modes are changed on files, directories, or both.
3. Whether `.kiro/sessions/cli/` metadata and event files are read, created,
   hidden, refused, or changed by the sweep; how `session/new.cwd` affects
   discovery of the prior-session state; and what the selected process exposes
   to an ACP client owned by the approved principal.
4. The precise failure result when path resolution, ownership checks, or
   permission changes fail: continue versus abort, JSON-RPC error or process
   exit, cleanup behavior, and the outcome observed by the adapter.

The exact files changed on every hop are frozen below. The retained
all-hop selected-surface review classified the `2.24.0+` environment boundary
and the later TUI-only permission classifier; the remaining selected-route
stop is the owner-only state path. `BUILD-INFO` changes are identity metadata.
`kiro-cli` and `kiro-cli-chat` are the changed executable inputs inspected by
that review; `kiro-cli-term` changes but is not named in the selected argv.
`README`, `q`, `qchat`, and `install.sh` are byte-identical throughout. No
release point is omitted from the retained sequence. Static string counts are
discovery aids only and are not used to infer owner-sweep effects.

## Concrete separate-approval harness

A separate authorization is needed before executing any downloaded artifact.
The next proof can use the already frozen Linux aarch64 artifacts in a
throwaway Linux aarch64 GNU VM with no external network interface or route.
Use a fresh scratch `HOME`, `KIRO_HOME`, working directory, and `TMPDIR`; do
not mount or read host home, credentials, provider configuration, or the
repository. Run the exact official `2.26.1` control artifact and
`2.27.0`, `2.27.1`, and `2.28.0` under two disposable unprivileged UIDs. Keep
external egress denied at the VM boundary. If a service stub is needed for
startup, bind it to loopback, use deterministic fake responses only, and deny
forwarding; never connect to Kiro or an AWS endpoint.

Seed only synthetic, non-secret session metadata and event files under the
documented V2 session directory. Exercise the exact `kiro-cli acp` stdio
entrypoint with a local JSON-RPC client: `initialize`, `session/new` with a
scratch `cwd`, and the existing-session method only as an observational probe
of upstream owner access (it remains unmapped in Swallowtail). Capture syscall
path and permission metadata for `openat`, `newfstatat`, `readlinkat`, and
`chmod`/`fchmodat`, plus process exit and JSON-RPC errors; do not capture file
contents. Compare before/after owners and modes for an owned home, a
misowned home/entry, a non-directory home, a missing path, a symlinked home,
an internal symlink, and a link to a sentinel outside the home. Verify no
access or mutation escapes the scratch tree and no principal or authentication
state changes. Do not send `session/prompt`, start a tool call, or approve a
permission. If the process cannot reach ACP without real authentication or a
provider turn, stop at that boundary and report the remaining edge; do not add
credentials or substitute a live endpoint. This harness is a proposal, not
permission to run it.

## Scope and non-claims

The existing adapter claim, route guide, feature and lifecycle matrices, and
release compatibility baseline remain unchanged. In particular, this record
does not qualify `2.22.0` through `2.28.0`, add a Swallowtail operation,
expand filesystem authority, transfer HTTP MCP live honouring, or change the
approved execution principal, delegated environment, local-account
authentication, `session/new.cwd`, read-only working resource, or
reject-and-cancel policy. No artifact execution, authentication, provider
prompt, credential access, installation, host/home/configuration mutation,
tag, release, or publication occurred.

## Frozen complete tree inventory

The JSON below is the complete deterministic inventory for all thirteen
published artifact points, from retained `2.21.4` through official stable
`2.28.0`. It has no host paths or credentials.

```json
{
  "versions": [
    "2.21.4",
    "2.22.0",
    "2.22.1",
    "2.23.0",
    "2.23.1",
    "2.24.0",
    "2.24.1",
    "2.25.0",
    "2.26.0",
    "2.26.1",
    "2.27.0",
    "2.27.1",
    "2.28.0"
  ],
  "records": {
    "2.21.4": {
      "archive_sha256": "f582eac0e002b4d11bbd061d41fcb49d1d37626a2c1fe230373fcbf97755df6f",
      "archive_size": 511212756,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "314c317eb2730dfd27fde5be7e7e4070ed10ad918424a221f3b510238f1c4898",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "3a1318d58edbd7e6b6b8cafcd04ac659f77fd8896c8b8faf6e1c87a9cb226e66",
          "size": 102609656
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "13a8a907f86f9ff482990cb7aab22848d591de923427224fff3c851e7ab157f6",
          "size": 828860264
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "3592244b27cec80f9a4e02b55c0316fa656098d36e4826d09afa161b6d427e63",
          "size": 78447216
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-11T12:48:07.133043+00:00",
        "BUILD_HASH": "57c33e903958a4731d1eaaa8353ad807badbe7d7",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.21.4"
      }
    },
    "2.22.0": {
      "archive_sha256": "45c9a118e95fb31fadc9178325d3100a52efdc33689f166cd6cd37211805ea98",
      "archive_size": 265696292,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "7a9a0c3be6e273912c685b39e2303096e13e20760e794d49c3f95a9c41099d2e",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "da6417b2db47de891431a2147b0cf357d4382007ec642067e7a366176f1d1623",
          "size": 102607408
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "c4f3e4f2530894bd58e0981b300dfedc5ed296503749a12d8267240880d74bd2",
          "size": 452383520
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "4a31357354b835c989040838ff68cda84972a851d69f24a0b2471248cac47084",
          "size": 78500240
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-15T20:35:56.024567+00:00",
        "BUILD_HASH": "b5aa487688f66342d13b1563df274f83ded0cfb2",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.22.0"
      }
    },
    "2.22.1": {
      "archive_sha256": "52b3868df87a0a189e46368f07ae2bd854a2cecaf571e5712be7f85441b30c4e",
      "archive_size": 260228236,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "7f6c001f67b70e5a458a39dd4cd93d768d45ef0c966387b74fa32f786ae73799",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "14f6fc74fc6c2ba33a96c5fd80e40b4df804b166d31ebf8bec6acc0716820457",
          "size": 102626608
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "627a93da52e9d1936a150686bf319aece7eadb2a5520eb5aa4b0bb75661ab7c4",
          "size": 422537544
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "d2b377d20a08f1eeacdafdd261631bbec21670ac64d6caf3a858801629cbf58c",
          "size": 78498432
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-18T01:55:30.909775+00:00",
        "BUILD_HASH": "4ede1ca42f41b2c8557a6f2501d7f3063c74ead9",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.22.1"
      }
    },
    "2.23.0": {
      "archive_sha256": "f544126af90a8910cefb5361ea7c2671a9889f070e2319309300a50a1e001565",
      "archive_size": 130080976,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "fac4435dc28939bcc5df38c9802f59bb0c6bb3612d4191630082b3643e1276dc",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "661f7f125f848db7354da448a5626a8937f3797a16d852df344336c986f622b6",
          "size": 40705056
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "bddfd8413c211e2bb802d2b4851e91d22acd9f2aeb282aacc8cbff0a3024eac8",
          "size": 200934208
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "408a5d370bbbf95df94fa772f52299cc3763e19073018a4d80add730096b0477",
          "size": 31533488
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-21T01:59:09.891814+00:00",
        "BUILD_HASH": "82fac3b02ae45398d8b03b8fb9bfb9e2c97a0c69",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.23.0"
      }
    },
    "2.23.1": {
      "archive_sha256": "a96220eecb6b296d711247c980cfda9aaa38cb251a863f20fbc7c562da164e25",
      "archive_size": 130098680,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "df4ebbb01b3ddea5a5853089e78c9f30cbc32074c3f465661ac2d660f18e1f4e",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "3e7b63438c7fdb01a3c481da63a5d86721385bdca95975579defe0bd7e1f3aa6",
          "size": 40778784
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "938ecc3dbb73b75dd0b8bea5afb5c05b75dcd7e0e21940383a5697a37fd28582",
          "size": 201008000
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "61bbfbac22671772c99746ff4329723a32bb75efdf1d3862694dad5b98a00c9c",
          "size": 31517104
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-22T23:43:54.956327+00:00",
        "BUILD_HASH": "c1c98028a070a5cab7db1645d9b4f6c0cdc0e830",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.23.1"
      }
    },
    "2.24.0": {
      "archive_sha256": "c70841d8606701fc82ce251ab86ee240ac8e12dfa31a6dd840d72f267fcf403d",
      "archive_size": 130404008,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "bc4149b8135e79aa07f15fa07977e5a3aee38dffbe77e224443dcb2ca0317e8a",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "0fa7d410af725072f10c2c5d8ed064d12e2b67dfd395864d91b1304886cb05ce",
          "size": 40729632
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "d3ee4ab7c80ee6c571bc6aec6d2976e58a23ee5f55cf728be411368366a22774",
          "size": 202147264
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "e46dcbca8b88e6b9e1f13ca1b1c8f1393bcc480bb6e5a8297a15ce4577d1f6fa",
          "size": 31549872
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-23T19:26:18.590477+00:00",
        "BUILD_HASH": "240642df93e76b55141520bb767a757f4434d006",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.24.0"
      }
    },
    "2.24.1": {
      "archive_sha256": "20a4de63aa6d7c8fdf09c77d3906db13b40c8f2e086eeef86a911e15cd074b1e",
      "archive_size": 130402336,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "afbf5986b5391a069353bab0db489c0c7bcc32b2c40ccad2d4551097372be3d7",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "8c674b1aa3f5721c3fad6eea5df0a8dbb8b4db365de3712c791cd7ce7dfa3dcb",
          "size": 40709152
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "3efd0ce2c1e43d28d7c18a1c536fd61da0acccc79828841815461393e0fe1220",
          "size": 202110400
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "0bd5f7ae6069dcf9136fb10f9fb99bee7baffdda54371d93120ce94474dc6cfb",
          "size": 31500720
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-25T02:20:32.696389+00:00",
        "BUILD_HASH": "0e0f0e19e4c2ea4c94e552f834baf85de11476fa",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.24.1"
      }
    },
    "2.25.0": {
      "archive_sha256": "222087b9cc8a231d9edba49bed9f2e2de4f09298dcaabaf2c489918eec0a9b46",
      "archive_size": 130601536,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "3e4746bc5bff7f1602da275d6bad53dc72d9f1cf6424aca6baac62f79eb6da59",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "fa9a9e4bccb789929452d0e14aa844e388d34a2c280c952a32b574ffb2b5fa4b",
          "size": 40799840
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "6359cda3edad6460115b0921d4e3ab6798e4f8d8363256e5a870bb851a4050db",
          "size": 202533056
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "7a1bd2027100f7a6df29eaf62282d549e1957cdcc40d8c26aa0534a786c7b48d",
          "size": 31558064
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-28T20:16:22.147076+00:00",
        "BUILD_HASH": "118eed59c3069e6a954451f8e6a509d86ba92279",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.25.0"
      }
    },
    "2.26.0": {
      "archive_sha256": "acdd959bd2edd6b379553a15f74ccc46b5ba4eeebf69f6e2f26316a7581acc8b",
      "archive_size": 130618972,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "34d8324e06c1e561584b50cb522531d4da731710a63016c2b0e73961ee0c3e7e",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "a77ad46a8d57cd1385b6328e049c9e613b84212cfb10b330d499958e23722155",
          "size": 40812128
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "4fca5bcd20de1318b8b1f060acc2f0f0cd0a9be354f73926cf09bd21bc33a2dd",
          "size": 202512576
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "13446a4acea00b0e40db25c47145a7102289c1a3b960967e8e7987640b88980a",
          "size": 31533488
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-29T03:48:09.189361+00:00",
        "BUILD_HASH": "15d7f349b2760a900ebf8bec43e05d1bfbef6165",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.26.0"
      }
    },
    "2.26.1": {
      "archive_sha256": "91da83c971bab9f8376bbaf73fb37993bca48a5ce68d34ed30ed971dac52de49",
      "archive_size": 130634712,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "21d1e48ebba9b1d1532d43dcd1f06269dcc0a368938aad4839598aa9a7d5a42a",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "3b3554e58c34d18ae25fb135ed02d64e1e8bdd3524c8ae9fbdadb1c3156d66d7",
          "size": 40861280
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "c8db2b09552b23c5db9c409507cebe608250e6c99809eb8080b006ad6820c544",
          "size": 202565824
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "25a619c511220277658e3b1e75d464b37281058bc9fff307d7212d35afd0f82b",
          "size": 31500720
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-09-30T18:26:05.476589+00:00",
        "BUILD_HASH": "15d7f349b2760a900ebf8bec43e05d1bfbef6165",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.26.1"
      }
    },
    "2.27.0": {
      "archive_sha256": "e983d1ba524dcfb0265b405324e5e203e731bc0cf15c355781bfebd66191311c",
      "archive_size": 130827468,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "7bc7f79419a29924f77f89c3587777435b87abdcf83577e19baeb9c371727862",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "8979bbe3aafd65ce68c9e70e2a2f01d45c34758558df0897d4316adb14c1a2f6",
          "size": 40795744
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "d894d084bfddbc8f04ac802971712726bb9b97228b8ef723d031c996fb704e6a",
          "size": 202955456
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "ba50e73003e3ef09f478d8ec80179c14a93e1480e1329e706871193026d5386b",
          "size": 31562160
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-10-01T16:21:10.210016+00:00",
        "BUILD_HASH": "7c1a246f80bfdf3266858f4391fcdf75dd9faf62",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.27.0"
      }
    },
    "2.27.1": {
      "archive_sha256": "f7fddc8c6f3d19f3a24c1077ae74f253f9898d323d2d6e768e12e779d039f459",
      "archive_size": 130936664,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "acaf3770f5c95e8ab7e3451ebb4f8b077d79c7afba520ed3df61298d7e6db39d",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "f6ad6ecc28f588ffef133d052bc700faa95a171474797dc81fe0ba463939b849",
          "size": 40808032
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "1b88554e17d6138f74633f8861260eb16c20c3a3b1dd3fd3ae3795e0eadf4f46",
          "size": 203267264
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "9b511e0c0fc246cb83d0372b4286579d0bff5e6b029e2077647fec3d0506b02b",
          "size": 31553968
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-10-02T20:10:29.172703+00:00",
        "BUILD_HASH": "7c1a246f80bfdf3266858f4391fcdf75dd9faf62",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.27.1"
      }
    },
    "2.28.0": {
      "archive_sha256": "2b9b26915737d3f8086bc28b9830b9ebe3d4452a26c7c092d9a190d6355937be",
      "archive_size": 131008436,
      "files": {
        "kirocli/BUILD-INFO": {
          "sha256": "11559a0b59ae3fc0768519e1dc347eed00bbaffb311c373c51cf1fc07ed253b7",
          "size": 162
        },
        "kirocli/README": {
          "sha256": "fc6f24a67be61fe36cc413ff60a86f3a39e1915e4c457feb5b6cde2a1e2c749a",
          "size": 266
        },
        "kirocli/bin/kiro-cli": {
          "sha256": "49568ec000cd0ef6299b73f6b5af0e003d060288b2f7604794b7ab19badc1315",
          "size": 40799840
        },
        "kirocli/bin/kiro-cli-chat": {
          "sha256": "373e8bda239154811b850cc85db50e83e3e5040c24783e4603adc415fef89b16",
          "size": 203410880
        },
        "kirocli/bin/kiro-cli-term": {
          "sha256": "63f60802d631519f8a3f2f9a62b7f9d4b6ad9f9692e4ef83f2d78026efb2fadc",
          "size": 31635888
        },
        "kirocli/bin/q": {
          "sha256": "56897a672bb49ac53309edd88a726e3bf2d93533903fe548f960acf25114a671",
          "size": 65
        },
        "kirocli/bin/qchat": {
          "sha256": "9ac2f85a80fafc1d350bd8d33c7d9b25315c193cf9ca14d285e51361c8243830",
          "size": 70
        },
        "kirocli/install.sh": {
          "sha256": "2118af61165eb413c08768008ef586b134805f46855e3764db3f1e25d955957c",
          "size": 5123
        }
      },
      "build": {
        "BUILD_DATE": "2026-10-05T23:49:09.224715+00:00",
        "BUILD_HASH": "877b466eda188c6dc44136df230f7e193a4edbfa",
        "BUILD_TARGET_TRIPLE": "aarch64-unknown-linux-gnu",
        "BUILD_VERSION": "2.28.0"
      }
    }
  },
  "hops": {
    "2.21.4_to_2.22.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.22.0_to_2.22.1": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.22.1_to_2.23.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.23.0_to_2.23.1": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.23.1_to_2.24.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.24.0_to_2.24.1": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.24.1_to_2.25.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.25.0_to_2.26.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.26.0_to_2.26.1": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.26.1_to_2.27.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.27.0_to_2.27.1": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    },
    "2.27.1_to_2.28.0": {
      "added": [],
      "removed": [],
      "changed": [
        "kirocli/BUILD-INFO",
        "kirocli/bin/kiro-cli",
        "kirocli/bin/kiro-cli-chat",
        "kirocli/bin/kiro-cli-term"
      ],
      "identical": [
        "kirocli/README",
        "kirocli/bin/q",
        "kirocli/bin/qchat",
        "kirocli/install.sh"
      ]
    }
  },
  "identical_across_all_hops": [
    "kirocli/README",
    "kirocli/bin/q",
    "kirocli/bin/qchat",
    "kirocli/install.sh"
  ],
  "changed_at_every_hop": [
    "kirocli/BUILD-INFO",
    "kirocli/bin/kiro-cli",
    "kirocli/bin/kiro-cli-chat",
    "kirocli/bin/kiro-cli-term"
  ]
}
```
