#!/usr/bin/env python3
"""Private fake-only Darwin/Linux probe supervisor; original activation is absent.

The trusted caller binds the detached source record. This host still lacks a
settled same-user interpreter-custody contract. Never use this for vendor code.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import selectors
import signal
import socket
import stat
import subprocess
import tempfile
import time
import sys

ROOT = Path(__file__).resolve().parents[1]
SUPPORT = Path("crates/swallowtail-adapter-claude-agent/tests/native_proof_support")
FIXTURE = Path(
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof"
)
RECORD = FIXTURE / "confined-execution.sha256"
SOURCES = (
    Path("scripts/claude-agent-sdk-confined-probe.py"),
    Path("scripts/claude-agent-sdk-confined-linux.py"),
    Path("scripts/tests/claude-agent-sdk-confined-probe.py"),
    Path(".config/nextest.toml"),
    Path(
        "crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_native_proof_harness.rs"
    ),
    SUPPORT / "confined-spawn.mjs",
    SUPPORT / "confined-runner.mjs",
    SUPPORT / "confined-hostile.mjs",
    SUPPORT / "confined-sidecar-driver.mjs",
    SUPPORT / "confined-native-startup.mjs",
    SUPPORT / "probe-runner.mjs",
    SUPPORT / "fake-sdk-native-wire.mjs",
    SUPPORT / "fake-native.mjs",
    FIXTURE / "probe-plan.json",
    FIXTURE / "probe-admission.json",
    FIXTURE / "probe-options.json",
    FIXTURE / "probe-execution.sha256",
    FIXTURE / "confined-plan.json",
    Path(
        "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
    ),
)
MAX_OUTPUT = 1_000_000
MAX_FILE = 1_000_000
WALL_SECONDS = 10


class Refused(Exception):
    """An unavailable capability or a changed input; no qualification follows."""


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read_regular(file):
    """No symlink component; retain the bytes read from this descriptor."""
    file = Path(file).absolute()
    if file.resolve() != file:
        raise Refused(f"symlink_refused: {file.name}")
    descriptor = os.open(file, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        info = os.fstat(descriptor)
        if not stat.S_ISREG(info.st_mode):
            raise Refused(f"non_regular_input: {file.name}")
        with os.fdopen(descriptor, "rb", closefd=False) as stream:
            return stream.read()
    finally:
        os.close(descriptor)


def verified_sources(root, expected_record):
    record = read_regular(root / RECORD)
    if digest(record) != expected_record:
        raise Refused("execution_record_substitution")
    snapshots = {name: read_regular(root / name) for name in SOURCES}
    expected = "".join(
        f"{digest(data)}  {name}\n" for name, data in snapshots.items()
    ).encode()
    if record != expected:
        raise Refused("source_substitution_before_import")
    admission = json.loads(snapshots[FIXTURE / "probe-admission.json"])
    runner = snapshots[SUPPORT / "probe-runner.mjs"]
    if digest(snapshots[FIXTURE / "probe-admission.json"]).encode() not in runner:
        raise Refused("admission_anchor_mismatch")
    if (
        admission["production_admission"] is not False
        or admission["authority"]["original_run_authorised"] is not False
        or admission["fake_peer"]["sdk_module_sha256"]
        != digest(snapshots[SUPPORT / "fake-sdk-native-wire.mjs"])
        or admission["options_template_sha256"]
        != digest(snapshots[FIXTURE / "probe-options.json"])
    ):
        raise Refused("probe_admission_mismatch")
    return snapshots


def host_capabilities():
    return {
        "platform": f"{platform.system()}-{platform.machine()}",
        "kernel": platform.release(),
        "sandbox_exec": Path("/usr/bin/sandbox-exec").is_file(),
        "original_mode": False,
        "gates": [
            "same-user interpreter/source custody is unresolved",
            "original administrator-tier settings injection is not admitted",
            "exact original-artifact authority and declared Nucleus proof are absent",
        ],
    }


def runtime_files(node):
    """Enumerate installed Mach-O dependencies without executing Node."""
    pending = [node]
    seen = set()
    metadata = set()
    while pending:
        binary = pending.pop()
        if binary in seen or str(binary).startswith(("/System/", "/usr/lib/")):
            continue
        seen.add(binary)
        result = subprocess.run(
            ["/usr/bin/otool", "-L", str(binary)],
            capture_output=True,
            text=True,
            timeout=5,
            check=True,
            env={"PATH": "/usr/bin:/bin"},
        )
        for line in result.stdout.splitlines()[1:]:
            dependency = line.strip().split(" (", 1)[0]
            if dependency.startswith("@rpath/"):
                dependency = str(binary.parent.parent / "lib" / dependency[7:])
            elif dependency.startswith("@loader_path/"):
                dependency = str(binary.parent / dependency[13:])
            elif dependency.startswith("@"):
                raise Refused("unresolved_runtime_dependency")
            metadata.add(Path(dependency).absolute())
            # dyld may canonicalize the parent directory before following the
            # leaf dylib symlink. Bind that intermediate spelling as well.
            metadata.add(Path(dependency).parent.resolve() / Path(dependency).name)
            dependency = Path(dependency).resolve()
            if dependency != binary:
                pending.append(dependency)
    return sorted(seen), sorted(metadata)


def policy(root, runtime, endpoint, metadata=(), settings_file=None):
    literals = " ".join(f"(literal {json.dumps(str(file))})" for file in runtime)
    aliases = " ".join(f"(literal {json.dumps(str(file))})" for file in metadata)
    ancestors = " ".join(
        f"(path-ancestors {json.dumps(str(file))}) (literal {json.dumps(str(file))})"
        for file in (*runtime, *metadata)
    )
    network = ""
    settings = f"(literal {json.dumps(str(settings_file))})" if settings_file else ""
    if endpoint is not None:
        host, port = endpoint.split(":")
        if host != "127.0.0.1" or not port.isdigit() or not 0 < int(port) < 65536:
            raise Refused("endpoint_refused")
        network = f'(allow network-outbound (remote tcp "localhost:{int(port)}"))'
    return f"""(version 1)
(deny default)
(import "/System/Library/Sandbox/Profiles/dyld-support.sb")
(allow syscall*)
(allow file-read-metadata (path-ancestors {json.dumps(str(root))}) {ancestors})
(allow file-read-data (path-ancestors {json.dumps(str(root))}) {ancestors})
(allow file-read* (subpath "/System/Library") (subpath "/usr/lib")
  (subpath "/System/Volumes/Preboot/Cryptexes/OS") (subpath "/System/Cryptexes/OS")
  (literal "/dev/urandom") (literal "/dev/random") (literal "/dev/null")
  (subpath {json.dumps(str(root))}) {literals} {aliases} {settings})
(allow file-write-data (literal "/dev/null")
  (literal {json.dumps(str(root / "workspace/storage.bin"))}))
(allow process-exec {literals})
(allow file-map-executable (subpath "/System/Library") (subpath "/usr/lib")
  (subpath "/System/Volumes/Preboot/Cryptexes/OS") (subpath "/System/Cryptexes/OS") {literals} {aliases})
(allow sysctl-read (sysctl-name-regex #"^hw\\.")
  (sysctl-name "kern.osrelease" "kern.ostype" "kern.osversion" "kern.version"
    "kern.argmax" "kern.hostname" "kern.osproductversion" "kern.osproductversioncompat"))
{network}
"""


def limits():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (MAX_FILE, MAX_FILE))
    resource.setrlimit(resource.RLIMIT_NOFILE, (64, 64))
    resource.setrlimit(resource.RLIMIT_CPU, (10, 10))


def supervise(
    commands,
    cwd,
    environment,
    native_fds,
    driver_fds,
    bound,
    output_limit,
    cancel_on_barrier=False,
    driver_environment=None,
    cancelled=None,
    extra_native_fds=(),
):
    """Observe actual children; fork denial makes these the entire child tree."""
    children = []
    captured = {"native": bytearray(), "driver": bytearray(), "stderr": bytearray()}
    reason = None
    observed = []
    selection = selectors.DefaultSelector()
    exit_read, exit_write = os.pipe()
    owned_fds = set(
        (*native_fds, *driver_fds, *extra_native_fds, exit_read, exit_write)
    )

    def close_owned(descriptor):
        if descriptor in owned_fds:
            os.close(descriptor)
            owned_fds.remove(descriptor)

    total = 0
    try:
        native = subprocess.Popen(
            commands[0],
            cwd=cwd,
            env=environment,
            stdin=native_fds[0],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            pass_fds=(native_fds[0], *extra_native_fds),
            preexec_fn=limits,
            close_fds=True,
        )
        children.append(native)
        config = json.loads((cwd.parent / "confined-config.json").read_text())
        config.update({"nativePid": native.pid, "exitFd": exit_read})
        (cwd.parent / "confined-config.json").write_text(json.dumps(config))
        driver = subprocess.Popen(
            commands[1],
            cwd=cwd,
            env=driver_environment or environment,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            pass_fds=(*driver_fds, exit_read),
            preexec_fn=limits,
            close_fds=True,
        )
        children.append(driver)
        # Close duplicate ends: only the driver writes native input. Native
        # output crosses the supervisor's bounded relay before the driver.
        for descriptor in set(
            (native_fds[0], *driver_fds, *extra_native_fds, exit_read)
        ):
            close_owned(descriptor)
        relays = {}
        for stream, destination, label in (
            (native.stdout, native_fds[1], "native"),
            (native.stderr, native_fds[2], "stderr"),
        ):
            os.set_blocking(stream.fileno(), False)
            os.set_blocking(destination, False)
            relays[destination] = {
                "pending": bytearray(),
                "eof": False,
                "closed": False,
            }
            selection.register(stream, selectors.EVENT_READ, (label, destination))
        for stream, label in ((driver.stdout, "driver"), (driver.stderr, "stderr")):
            os.set_blocking(stream.fileno(), False)
            selection.register(stream, selectors.EVENT_READ, (label, None))
        deadline = time.monotonic() + bound
        exit_sent = False
        while selection.get_map() or any(child.poll() is None for child in children):
            if cancelled is not None and cancelled():
                reason = "supervisor_cancelled"
                break
            if time.monotonic() >= deadline:
                reason = "deadline_or_hanging_native"
                break
            if native.poll() is not None and not exit_sent:
                result = native.returncode
                message = {
                    "code": result if result >= 0 else None,
                    "signal": signal.Signals(-result).name if result < 0 else None,
                }
                try:
                    os.write(exit_write, (json.dumps(message) + "\n").encode())
                except BrokenPipeError:
                    pass
                close_owned(exit_write)
                exit_write = None
                exit_sent = True
            for key, _ in selection.select(
                min(0.02, max(0, deadline - time.monotonic()))
            ):
                if key.events == selectors.EVENT_WRITE:
                    relay = relays[key.fd]
                    try:
                        sent = os.write(key.fd, relay["pending"])
                        del relay["pending"][:sent]
                    except BrokenPipeError:
                        relay["pending"].clear()
                        relay["closed"] = True
                    if not relay["pending"]:
                        selection.unregister(key.fd)
                        if relay["eof"] or relay["closed"]:
                            close_owned(key.fd)
                    continue
                data = os.read(key.fd, 65536)
                label, destination = key.data
                if not data:
                    selection.unregister(key.fileobj)
                    key.fileobj.close()
                    if destination is not None:
                        relay = relays[destination]
                        relay["eof"] = True
                        if not relay["pending"]:
                            close_owned(destination)
                    continue
                total += len(data)
                if total > output_limit:
                    reason = "live_output_limit"
                    break
                captured[label].extend(data)
                if cancel_on_barrier and b'"barrier_ready"' in captured["native"]:
                    reason = "cancelled_at_owned_barrier"
                    break
                if destination is not None:
                    relay = relays[destination]
                    if relay["closed"]:
                        continue
                    was_empty = not relay["pending"]
                    relay["pending"].extend(data)
                    if was_empty:
                        selection.register(destination, selectors.EVENT_WRITE)
            if reason is not None:
                break
    finally:
        # No SDK callback or pid-name lookup participates in cleanup.
        for child in children:
            if child.poll() is None:
                child.kill()
        for child in children:
            try:
                observed.append({"pid": child.pid, "returncode": child.wait(timeout=2)})
            except subprocess.TimeoutExpired as error:
                raise Refused("full_tree_join_unobserved") from error
        for child in children:
            for stream in (child.stdout, child.stderr):
                if stream is not None:
                    stream.close()
        for descriptor in list(owned_fds):
            close_owned(descriptor)
        selection.close()
    if reason is None and any(row["returncode"] != 0 for row in observed):
        reason = "child_failed"
    return {
        "failure": reason,
        "exits": observed,
        "outputBytes": total,
        "driver": captured["driver"].decode(errors="replace"),
        "native": captured["native"].decode(errors="replace"),
        "stderr": captured["stderr"].decode(errors="replace"),
    }


def run_fake(
    node,
    expected_node,
    expected_record,
    scenario="ok",
    endpoint=None,
    bound=WALL_SECONDS,
    output_limit=MAX_OUTPUT,
    source_root=ROOT,
    attack=None,
    forbidden_port=None,
    cancel_on_barrier=False,
    driver_mode="probe",
    sdk_config=None,
    sdk_version="0.3.322",
    cancelled=None,
    provider_protocol=False,
    managed_deny=False,
):
    is_linux = platform.system() == "Linux" and platform.machine() == "x86_64"
    is_darwin = platform.system() == "Darwin" and platform.machine() == "arm64"
    if not (is_linux or is_darwin):
        raise Refused("selected_platform_boundary_unavailable")
    if is_darwin and not Path("/usr/bin/sandbox-exec").is_file():
        raise Refused("sandbox_capability_missing_or_revoked")
    if not 0 < bound <= WALL_SECONDS or not 0 < output_limit <= MAX_OUTPUT:
        raise Refused("bound_may_only_tighten")
    snapshots = verified_sources(source_root, expected_record)
    admission = json.loads(snapshots[FIXTURE / "probe-admission.json"])
    tuples = {row["sdk_version"]: row for row in admission["tuples"]}
    if sdk_version not in tuples:
        raise Refused("unadmitted_tuple")
    if driver_mode == "sidecar":
        sdk_version = "0.3.317"
    selected_tuple = tuples[sdk_version]
    if selected_tuple["fake_native_sha256"] != digest(
        snapshots[SUPPORT / "fake-native.mjs"]
    ):
        raise Refused("fake_native_substitution")
    node = Path(node).resolve(strict=True)
    node_bytes = read_regular(node)
    if digest(node_bytes) != expected_node:
        raise Refused("node_substitution")
    if is_darwin:
        runtime, metadata = runtime_files(node)
    else:
        runtime, metadata = [node], []
    identities = {str(file): digest(read_regular(file)) for file in runtime}
    if is_darwin:
        identities["/usr/bin/sandbox-exec"] = digest(
            read_regular("/usr/bin/sandbox-exec")
        )
        identities["/System/Library/Sandbox/Profiles/dyld-support.sb"] = digest(
            read_regular("/System/Library/Sandbox/Profiles/dyld-support.sb")
        )
    elif Path("/etc/ld.so.cache").is_file():
        identities["/etc/ld.so.cache"] = digest(read_regular("/etc/ld.so.cache"))
    if cancelled is not None and cancelled():
        raise Refused("cancelled_before_launch")
    scratch = Path(os.environ["TMPDIR"]).resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="sdk-confined-", dir=scratch) as temporary:
        fixture = Path(temporary)
        root = fixture / "jail"
        root.mkdir(mode=0o700)
        for name in ("workspace", "home", "tmp"):
            (root / name).mkdir(mode=0o700)
        (root / "workspace/storage.bin").touch()
        for source, destination in (
            (SUPPORT / "confined-runner.mjs", "confined-runner.mjs"),
            (SUPPORT / "confined-spawn.mjs", "confined-spawn.mjs"),
            (SUPPORT / "fake-sdk-native-wire.mjs", "probe-sdk.mjs"),
            (SUPPORT / "fake-native.mjs", "fake-native.mjs"),
            (SUPPORT / "confined-hostile.mjs", "confined-hostile.mjs"),
            (SUPPORT / "confined-sidecar-driver.mjs", "confined-sidecar-driver.mjs"),
            (SUPPORT / "confined-native-startup.mjs", "confined-native-startup.mjs"),
            (Path("scripts/claude-agent-sdk-confined-linux.py"), "linux-entry.py"),
            (
                Path(
                    "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
                ),
                "claude-agent-sdk-sidecar.mjs",
            ),
            (FIXTURE / "probe-options.json", "probe-options.json"),
        ):
            (root / destination).write_bytes(snapshots[source])
        (root / "package.json").write_text(
            json.dumps(
                {"name": "@anthropic-ai/claude-agent-sdk", "version": sdk_version}
            )
        )
        (root / "manifest.json").write_text(
            json.dumps({"version": selected_tuple["native_version"]})
        )
        (root / "sdk-config.json").write_text(json.dumps(sdk_config or {}))
        # The denial target is an owned sibling fixture outside the allowed
        # tree. Never try a user file or a real credential as a negative case.
        outside = fixture / "outside"
        outside.write_text("owned sentinel")
        administrator = fixture / "administrator"
        administrator.mkdir(mode=0o700)
        managed_settings = administrator / "managed-settings.json"
        managed_settings.write_text(
            json.dumps({"permissions": {"denyInsteadOfPrompting": managed_deny}})
        )
        for tier in (root / "home", root / "workspace"):
            (tier / ".claude").mkdir()
            (tier / ".claude/settings.json").write_text(
                json.dumps(
                    {"permissions": {"denyInsteadOfPrompting": not managed_deny}}
                )
            )
        os.symlink(outside, root / "escape")
        native_config = {
            "scenario": scenario,
            "attack": attack,
            "outside": str(outside),
            "escape": str(root / "escape"),
            "forbiddenPort": forbidden_port,
            "administratorSettings": str(managed_settings),
            "providerProtocol": provider_protocol,
            "expectedManagedDeny": managed_deny,
            "inheritedEndpointFd": None,
            "supervisorPid": os.getpid(),
        }
        if endpoint is not None:
            # Validate before opening any socket or launching a peer.
            host, separator, port = endpoint.rpartition(":")
            if (
                host != "127.0.0.1"
                or not separator
                or not port.isdigit()
                or len(port) > 5
                or not 0 < int(port) < 65536
            ):
                raise Refused("endpoint_refused")
            native_config["providerEndpoint"] = endpoint
            if not provider_protocol:
                native_config["endpoint"] = endpoint
        if provider_protocol:
            native_config["scenario"] = "managed-deny" if managed_deny else "permission"
        endpoint_fds = ()
        if is_linux and endpoint is not None:
            connection = socket.create_connection(("127.0.0.1", int(port)), timeout=2)
            connection.settimeout(None)
            descriptor = connection.detach()
            endpoint_fds = (descriptor,)
            native_config["inheritedEndpointFd"] = descriptor
        (root / "native-config.json").write_text(json.dumps(native_config))
        template = json.loads(snapshots[FIXTURE / "probe-options.json"])["options"]
        native_name = "fake-native.mjs" if attack is None else "confined-hostile.mjs"
        native_args = [
            str(root / native_name),
            "--output-format",
            "stream-json",
            "--verbose",
            "--input-format",
            "stream-json",
            "--model",
            template["model"],
            "--permission-prompt-tool=stdio",
            "--disallowedTools",
            ",".join(template["disallowedTools"]),
            "--tools",
            ",".join(template["tools"]),
            "--setting-sources",
            "",
            "--strict-mcp-config",
            "--permission-mode",
            "default",
        ]
        # Validate policy before allocating descriptors, so an invalid endpoint
        # or unavailable mechanism cannot leak a pipe into a later attempt.
        if driver_mode not in ("probe", "sidecar"):
            raise Refused("unknown_driver_mode")
        input_read, input_write = os.pipe()
        output_read, output_write = os.pipe()
        error_read, error_write = os.pipe()
        config = {
            "nativeArgs": native_args,
            "workspace": str(root / "workspace"),
            "inputFd": input_write,
            "outputFd": output_read,
            "errorFd": error_read,
            "scenario": scenario,
            "withoutSdkJoin": bool((sdk_config or {}).get("withoutSdkJoin")),
        }
        (root / "confined-config.json").write_text(json.dumps(config))
        if is_darwin:
            profile = policy(root, runtime, endpoint, metadata, managed_settings)
            prefix = ["/usr/bin/sandbox-exec", "-p", profile, str(node)]
        else:
            runtime_roots = sorted(
                {
                    str(Path(name).resolve())
                    for name in (
                        "/usr/lib",
                        "/lib",
                        "/lib64",
                        str(node.parent.parent / "lib"),
                    )
                    if Path(name).is_dir()
                }
            )
            policy_data = {
                "root": str(root),
                "node": str(node),
                "runtimeRoots": runtime_roots,
                "writable": str(root / "workspace/storage.bin"),
                "readFiles": [str(managed_settings), "/dev/urandom", "/dev/random"]
                + (["/etc/ld.so.cache"] if Path("/etc/ld.so.cache").is_file() else []),
            }
            profile = json.dumps(policy_data, sort_keys=True)
            (root / "linux-policy.json").write_text(profile)
            prefix = [
                sys.executable,
                "-B",
                str(root / "linux-entry.py"),
                str(root / "linux-policy.json"),
                str(node),
            ]
        env = {
            "PATH": "/usr/bin:/bin",
            "HOME": str(root / "home"),
            "TMPDIR": str(root / "tmp"),
            "TMP": str(root / "tmp"),
            "TEMP": str(root / "tmp"),
            "LANG": "en_US.UTF-8",
            "LC_ALL": "en_US.UTF-8",
        }
        driver_env = dict(env)
        if driver_mode == "sidecar":
            driver_env.update(
                {
                    "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE": str(root / "probe-sdk.mjs"),
                    "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY": str(
                        root / "fake-native.mjs"
                    ),
                    "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST": str(root / "manifest.json"),
                }
            )
        elif driver_mode != "probe":
            raise Refused("unknown_driver_mode")
        prefix += ["--no-addons", "--openssl-config=/dev/null"]
        commands = [
            prefix
            + ["--import", str(root / "confined-native-startup.mjs")]
            + native_args,
            prefix
            + [
                "--import",
                str(root / "confined-spawn.mjs"),
                str(
                    root
                    / (
                        "confined-sidecar-driver.mjs"
                        if driver_mode == "sidecar"
                        else "confined-runner.mjs"
                    )
                ),
            ],
        ]
        try:
            result = supervise(
                commands,
                root / "workspace",
                env,
                (input_read, output_write, error_write),
                (input_write, output_read, error_read),
                bound,
                output_limit,
                cancel_on_barrier,
                driver_env,
                cancelled,
                endpoint_fds,
            )
            result["outsideUnchanged"] = outside.read_text() == "owned sentinel"
            result["storageBytes"] = (root / "workspace/storage.bin").stat().st_size
        finally:
            outside.unlink()
        result.update(
            {
                "probeOnly": True,
                "productionAdmission": False,
                "nodeSha256": expected_node,
                "runtimeFiles": identities,
                "executionSha256": expected_record,
                "tuple": selected_tuple,
                "policySha256": digest(profile.encode()),
                "environment": sorted(env),
                "processLimit": 2,
                "writableFileLimit": MAX_FILE,
                "capabilities": host_capabilities(),
                "settingsInjection": "owned-fake-administrator-file; original canonical path not injected",
            }
        )
        return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inspect", action="store_true")
    parser.add_argument("--mode", choices=("fake", "original"), default="fake")
    parser.add_argument("--node")
    parser.add_argument("--node-sha256")
    parser.add_argument("--execution-sha256")
    parser.add_argument("--scenario", default="ok")
    parser.add_argument("--sdk-version", default="0.3.322")
    parser.add_argument("--endpoint")
    args = parser.parse_args()
    if args.inspect:
        print(json.dumps(host_capabilities(), indent=2))
        return 0
    if args.mode == "original":
        raise Refused(
            "original_mode_unavailable: custody, administrator-tier injection, authority and Nucleus proof gate"
        )
    if not all((args.node, args.node_sha256, args.execution_sha256)):
        raise Refused("exact_node_and_execution_identity_required")
    cancellation = {"requested": False}

    def cancel(_signal, _frame):
        cancellation["requested"] = True

    signal.signal(signal.SIGTERM, cancel)
    signal.signal(signal.SIGINT, cancel)
    result = run_fake(
        args.node,
        args.node_sha256,
        args.execution_sha256,
        args.scenario,
        endpoint=args.endpoint,
        sdk_version=args.sdk_version,
        cancelled=lambda: cancellation["requested"],
    )
    print(json.dumps(result, indent=2))
    return (
        0
        if result["failure"] is None
        and all(row["returncode"] == 0 for row in result["exits"])
        else 1
    )


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (Refused, OSError, subprocess.SubprocessError) as error:
        print(json.dumps({"probeOnly": True, "refused": str(error)}))
        raise SystemExit(2)
