"""Small frozen-original qualification runner; OS/runtime base is trusted."""

import argparse
import hashlib
import http.server
import io
import json
import os
from pathlib import Path
import platform
import resource
import shutil
import subprocess
import tarfile
import tempfile
import threading
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "crates/swallowtail-adapter-claude-agent/tests/fixtures"
PLAN = FIXTURES / "claude-agent-sdk-native-proof/probe-plan.json"
PLAN_SHA = "e177a8267f37f8269533ce82fa20ddd89c1784bc0e37cbcf0efdaa15f1e29502"
CASES = ("allow", "deny", "user-deny", "managed-deny", "cancel", "error")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def fetch(url, maximum):
    if not url.startswith(
        ("https://nodejs.org/dist/", "https://registry.npmjs.org/@anthropic-ai/")
    ):
        raise ValueError("unapproved artifact URL")
    with urllib.request.urlopen(url, timeout=60) as response:
        data = response.read(maximum + 1)
    if len(data) > maximum:
        raise ValueError("artifact size bound")
    return data


def checked(data, expected):
    if digest(data) != expected:
        raise ValueError("frozen artifact digest mismatch")
    return data


def extract_sdk(data, target):
    target.mkdir()
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        members = archive.getmembers()
        if len(members) > 64:
            raise ValueError("SDK member bound")
        for member in members:
            name = Path(member.name)
            if name.is_absolute() or ".." in name.parts or name.parts[0] != "package":
                raise ValueError("unsafe archive path")
            if member.isdir():
                continue
            if not member.isfile() or member.size > 16777216:
                raise ValueError("unsafe SDK member")
            destination = target.joinpath(*name.parts[1:])
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(archive.extractfile(member).read())


def provision(base, target_platform, tuple_row):
    version = tuple_row["sdk_version"]
    sdk_data = checked(
        fetch(
            "https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk/-/claude-agent-sdk-"
            + version
            + ".tgz",
            33554432,
        ),
        tuple_row["wrapper_tarball_sha256"],
    )
    sdk = base / "sdk"
    extract_sdk(sdk_data, sdk)
    binary_row = tuple_row["platforms"][target_platform]
    native_data = fetch(
        "https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk-"
        + target_platform
        + "/-/claude-agent-sdk-"
        + target_platform
        + "-"
        + version
        + ".tgz",
        536870912,
    )
    with tarfile.open(fileobj=io.BytesIO(native_data)) as archive:
        member = archive.getmember("package/claude")
        if not member.isfile() or member.size != binary_row["binary_bytes"]:
            raise ValueError("native type/size mismatch")
        native = checked(
            archive.extractfile(member).read(), binary_row["binary_sha256"]
        )
    native_path = base / "claude"
    native_path.write_bytes(native)
    native_path.chmod(0o700)
    return sdk / "sdk.mjs", native_path


def provision_node(base, target_platform):
    ledger = json.loads(
        (FIXTURES / "claude-agent-sdk-node-22.23.3/identity.json").read_text()
    )["artifact_identities"][1]
    manifest = checked(
        fetch(ledger["checksums_manifest"], 65536), ledger["checksums_manifest_sha256"]
    )
    name = "node-v22.23.3-" + target_platform + ".tar.xz"
    expected = dict(line.split()[::-1] for line in manifest.decode().splitlines())[name]
    data = checked(
        fetch("https://nodejs.org/dist/v22.23.3/" + name, 67108864), expected
    )
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        member = archive.getmember(name[:-7] + "/bin/node")
        if not member.isfile() or member.size > 200000000:
            raise ValueError("Node member bound")
        node = archive.extractfile(member).read()
    destination = base / "node"
    destination.write_bytes(node)
    destination.chmod(0o700)
    observed = (
        subprocess.run(
            [str(destination), "--version"],
            env={"PATH": "/usr/bin:/bin", "HOME": str(base), "TMPDIR": str(base)},
            capture_output=True,
            timeout=10,
            check=True,
        )
        .stdout.decode()
        .strip()
    )
    if observed != "v22.23.3":
        raise ValueError("Node observed version mismatch")
    return destination, {
        "archiveSha256": expected,
        "binarySha256": digest(node),
        "version": "22.23.3",
    }


class Peer(http.server.BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def do_POST(self):
        length = int(self.headers.get("Content-Length", "0"))
        if not 0 < length <= 1048576:
            self.send_error(413)
            return
        body = json.loads(self.rfile.read(length))
        self.server.requests.append(
            {
                "path": self.path,
                "model": body.get("model"),
                "messageRoles": [m.get("role") for m in body.get("messages", [])],
            }
        )
        if "count_tokens" in self.path:
            data = b'{"input_tokens":16}'
            self.send_response(200)
            self.end_headers()
            self.wfile.write(data)
            return
        if self.server.case == "error":
            self.send_response(400)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(
                b'{"type":"error","error":{"type":"invalid_request_error","message":"owned fixture error"}}'
            )
            return
        if self.server.case in ("cancel", "deadline"):
            self.server.stopping.wait(4)
        continued = any(
            isinstance(message.get("content"), list)
            and any(part.get("type") == "tool_result" for part in message["content"])
            for message in body.get("messages", [])
        )
        content = (
            {"type": "text", "text": "continued"}
            if continued
            else {
                "type": "tool_use",
                "id": "owned-tool",
                "name": "Write",
                "input": {
                    "file_path": str(self.server.effect),
                    "content": "owned effect",
                },
            }
        )
        message = {
            "id": "msg-owned",
            "type": "message",
            "role": "assistant",
            "model": body.get("model", "fixture"),
            "content": [content],
            "stop_reason": "end_turn" if continued else "tool_use",
            "stop_sequence": None,
            "usage": {"input_tokens": 16, "output_tokens": 4},
        }
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.end_headers()
        start = {**message, "content": [], "stop_reason": None}
        block = (
            {**content, "input": {}} if not continued else {"type": "text", "text": ""}
        )
        delta = (
            {"type": "text_delta", "text": "continued"}
            if continued
            else {
                "type": "input_json_delta",
                "partial_json": json.dumps(content["input"]),
            }
        )
        records = [
            ("message_start", {"type": "message_start", "message": start}),
            (
                "content_block_start",
                {"type": "content_block_start", "index": 0, "content_block": block},
            ),
            (
                "content_block_delta",
                {"type": "content_block_delta", "index": 0, "delta": delta},
            ),
            ("content_block_stop", {"type": "content_block_stop", "index": 0}),
            (
                "message_delta",
                {
                    "type": "message_delta",
                    "delta": {
                        "stop_reason": message["stop_reason"],
                        "stop_sequence": None,
                    },
                    "usage": {"output_tokens": 4},
                },
            ),
            ("message_stop", {"type": "message_stop"}),
        ]
        try:
            for event, value in records:
                self.wfile.write(
                    (
                        "event: " + event + "\ndata: " + json.dumps(value) + "\n\n"
                    ).encode()
                )
            self.wfile.flush()
        except (BrokenPipeError, ConnectionResetError):
            pass


def case(
    base,
    sdk,
    native,
    node,
    name,
    *,
    sidecar=False,
    prepared=False,
    owned_fake=False,
    version="0.3.317",
    bound=45,
):
    root = base / (("prepared-" if prepared else "sidecar-" if sidecar else "") + name)
    root.mkdir()
    home, workspace = root / "home", root / "workspace"
    home.mkdir()
    workspace.mkdir()
    effect = home / "owned-effect.txt"
    settings = {"permissions": {"denyInsteadOfPrompting": True}}
    if name in ("user-deny", "selected-user-deny"):
        (home / ".claude").mkdir()
        (home / ".claude/settings.json").write_text(json.dumps(settings))
    managed = (
        Path("/etc/claude-code/managed-settings.json")
        if platform.system() == "Linux"
        else Path("/Library/Application Support/ClaudeCode/managed-settings.json")
    )
    if name == "managed-deny":
        if managed.exists():
            raise ValueError("disposable fixture settings already present")
        managed.parent.mkdir(parents=True, exist_ok=True)
        managed.write_text(json.dumps(settings))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Peer)
    server.daemon_threads = False
    server.requests, server.effect, server.case = [], effect, name
    server.stopping = threading.Event()
    thread = threading.Thread(target=server.serve_forever)
    thread.start()
    environment = {
        "HOME": str(home),
        "TMPDIR": str(root),
        "PATH": str(node.parent) + ":/usr/bin:/bin",
        "LANG": "C.UTF-8",
        "ANTHROPIC_API_KEY": "owned-fixture-key",
        "ANTHROPIC_BASE_URL": "http://127.0.0.1:" + str(server.server_port),
        "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
        "DISABLE_AUTOUPDATER": "1",
        "CLAUDE_CODE_ENTRYPOINT": "sdk-ts",
    }
    config = {
        "sdk": str(sdk),
        "native": str(native),
        "workspace": str(workspace),
        "case": name,
        "effect": str(effect),
        "sidecar": str(
            ROOT
            / "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
        ),
        "sdkVersion": version,
        "nativeVersion": "2.1." + version.rsplit(".", 1)[1],
        "manifest": str(sdk.parent / "manifest.json"),
        "environment": environment,
        "node": str(node),
        "observer": str(ROOT / "scripts/claude-agent-sdk-trusted-sidecar.mjs"),
        "observation": str(root / "observation.json"),
        "ownedFake": owned_fake,
    }
    config_path = root / "case.json"
    config_path.write_text(json.dumps(config))
    command = [
        str(node),
        str(
            ROOT
            / (
                "scripts/claude-agent-sdk-trusted-sidecar.mjs"
                if sidecar
                else "scripts/claude-agent-sdk-trusted-case.mjs"
            )
        ),
        str(config_path),
    ]
    if prepared:
        command = [
            os.environ.get(
                "SDK240_PREPARED_DRIVER",
                str(ROOT / "target/debug/examples/sdk240-trusted-prepared"),
            ),
            str(config_path),
        ]
        if os.environ.get("GITHUB_ACTIONS") == "true":
            environment["GITHUB_ACTIONS"] = "true"
    if platform.system() == "Darwin":
        command = [
            "/usr/bin/sandbox-exec",
            "-p",
            '(version 1)(allow default)(deny network*)(allow network-outbound (remote ip "localhost:*"))(allow network-inbound (local ip "localhost:*"))',
            *command,
        ]

    def limits():
        for kind, value in (
            (resource.RLIMIT_CPU, 45),
            (resource.RLIMIT_NOFILE, 256),
            (resource.RLIMIT_FSIZE, 1048576),
            (resource.RLIMIT_CORE, 0),
            (resource.RLIMIT_AS, 32 * 1024**4),
        ):
            hard = resource.getrlimit(kind)[1]
            target = value if hard == resource.RLIM_INFINITY else min(value, hard)
            resource.setrlimit(kind, (target, target))

    try:
        with (
            (root / "stdout.json").open("wb") as output,
            (root / "stderr.txt").open("wb") as error,
        ):
            process = subprocess.Popen(
                command,
                env=environment,
                cwd=workspace,
                stdout=output,
                stderr=error,
                start_new_session=True,
                preexec_fn=limits,
            )
            try:
                code = process.wait(timeout=bound)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, 9)
                code = process.wait(timeout=5)
                raise RuntimeError(
                    "case deadline; actual driver wait="
                    + str(code)
                    + " stderr="
                    + (root / "stderr.txt").read_text(errors="replace")[:32768]
                )
        output = (root / "stdout.json").read_bytes()
        if len(output) > 1048576 or (root / "stderr.txt").stat().st_size > 1048576:
            raise ValueError("case output bound")
        try:
            result = json.loads(output) if output else {}
        except ValueError:
            result = {"failure": "invalid driver output"}
        if prepared and (root / "observation.json").is_file():
            result.update(json.loads((root / "observation.json").read_bytes()))
            result["closeResponse"] = next(
                (
                    e
                    for e in result["events"]
                    if e.get("type") == "response"
                    and "nativeExitObserved" in e.get("data", {})
                ),
                {},
            )
        result["driverWaitCode"] = code
        result["driverStderr"] = (root / "stderr.txt").read_text(errors="replace")[
            :32768
        ]
        result.update(
            returncode=code, effect=effect.exists(), requests=server.requests, case=name
        )
        return result
    finally:
        server.stopping.set()
        server.shutdown()
        server.server_close()
        thread.join(timeout=5)
        if thread.is_alive():
            raise RuntimeError("endpoint wait missing")
        if name == "managed-deny":
            managed.unlink()


def assert_case(result, version, driver):
    """The actual original-proof predicate, also exercised by owned mutations."""

    def need(condition, label):
        if not condition:
            raise ValueError(label)

    name = result["case"]
    events = result.get("events", [])
    callbacks = (
        result.get("callbacks", [])
        if driver == "prepared"
        else [e for e in events if e.get("event") == "callback"]
    )
    need(
        result.get("returncode") == 0 and result.get("driverWaitCode") == 0,
        "driver actual wait",
    )
    need(
        result.get("allWaited" if driver == "prepared" else "allClosed") is True
        and result.get("children") == 1,
        "native actual wait",
    )
    closes = [
        e
        for e in events
        if e.get("event") == ("native_exit" if driver == "prepared" else "native_close")
    ]
    need(
        len(closes) == 1
        and (closes[0].get("code") is not None or closes[0].get("signal") is not None),
        "exact native close",
    )
    silent = (
        name in ("managed-deny", "selected-user-deny")
        and int(version.rsplit(".", 1)[1]) >= 319
    )
    count = 0 if silent or name in ("cancel", "deadline", "error") else 1
    need(len(callbacks) == count, "exact permission callback count")
    need(
        all(
            e.get("toolName") == "Write" and e.get("effectBeforePermission") is False
            for e in callbacks
        ),
        "permission before effect",
    )
    need(result.get("effect") is (name == "allow"), "exact controlled effect")
    if driver == "prepared":
        expected = {
            "cancel": "Cancelled",
            "deadline": "TimedOut",
            "error": "ProviderFailed",
        }.get(name, "Completed")
        need(result.get("terminalStatus") == expected, "distinct typed terminal")
        need(
            result.get("sessionCleanup") in ("Clean", "RootCompleted")
            and (
                result.get("sessionCleanup") != "RootCompleted"
                or result.get("sessionCleanupDiagnostic")
                == "swallowtail.claude-agent.sdk.close_root_only_degraded"
            )
            and result.get("turnCleanup") in ("Clean", "NotApplicable")
            and result.get("driverWaitObserved") is True,
            "prepared actual cleanup: "
            + repr(
                {
                    key: result.get(key)
                    for key in ("turnCleanup", "sessionCleanup", "driverWaitObserved")
                }
            ),
        )
        need(
            (result.get("cancellation") == "Requested") is (name == "cancel"),
            "deadline is not cancellation",
        )
        runtime = result.get("runtimeEvents", [])
        need(runtime and runtime[0]["kind"] == "Started", "prepared ordered start")
        need(
            all(a["sequence"] < b["sequence"] for a, b in zip(runtime, runtime[1:])),
            "prepared event order",
        )
        if name not in ("cancel", "deadline", "error"):
            need(
                any(
                    e["kind"].startswith("Activity:") and "Tool" in e["kind"]
                    for e in runtime
                ),
                "prepared tool activity",
            )
            need(
                any(
                    e["kind"] == "OutputDelta"
                    and "continued" in (e.get("content") or "")
                    for e in runtime
                ),
                "prepared continued output",
            )
    if driver in ("shipped-sidecar", "prepared"):
        need(not result.get("failure"), "no unexpected driver failure")
        close = result.get("closeResponse", {})
        need(
            close.get("success") is True
            and close.get("data", {}).get("nativeExitObserved") is True,
            "sidecar close and wait receipt",
        )
        starts = [i for i, e in enumerate(events) if e.get("event") == "turn_started"]
        ends = [i for i, e in enumerate(events) if e.get("event") == "turn_ended"]
        need(len(starts) == 1, "exact turn start")
        if name == "deadline":
            need(
                driver == "prepared"
                and not callbacks
                and not result.get("cancellation"),
                "real prepared deadline only",
            )
            return
        need(len(ends) == 1 and starts[0] < ends[0], "ordered terminal result")
        end = events[ends[0]]
        if name == "error":
            need(
                end.get("isError") is True
                and end.get("apiErrorStatus") == 400
                and end.get("terminalReason") == "api_error",
                "real provider error",
            )
        elif name == "cancel":
            need(
                end.get("isError") is True
                and end.get("terminalReason") == "aborted_streaming",
                "actual interrupted native result",
            )
            if driver == "shipped-sidecar":
                need(
                    any(
                        e.get("type") == "response"
                        and e.get("id") == "cancel"
                        and e.get("success") is True
                        for e in events
                    ),
                    "actual interrupt acknowledgement",
                )
        else:
            need(
                end.get("isError") is False and end.get("stopReason") == "success",
                "successful continued terminal",
            )
            tools = [
                i for i, e in enumerate(events) if e.get("event") == "tool_started"
            ]
            done = [i for i, e in enumerate(events) if e.get("event") == "tool_ended"]
            output = [
                i
                for i, e in enumerate(events)
                if e.get("event") == "output_delta"
                and "continued" in e.get("delta", "")
            ]
            need(
                len(tools) == 1 and len(done) == 1 and output,
                "tool and continued output required",
            )
            need(
                starts[0] < tools[0] < done[0] < output[-1] < ends[0],
                "start/tool/result ordering",
            )
            need(
                events[tools[0]].get("toolName") == "Write"
                and events[tools[0]].get("toolCallId")
                == events[done[0]].get("toolCallId"),
                "correlated tool activity",
            )
            need(
                events[done[0]].get("isError") is (name != "allow"),
                "tool decision outcome",
            )
    else:
        need(any(e.get("event") == "initialized" for e in events), "SDK initialized")
        results = [e for e in events if e.get("type") == "result"]
        if name == "cancel":
            need(
                result.get("aborted") is True
                and any(e.get("event") == "abort_requested" for e in events),
                "SDK actual cancellation",
            )
        else:
            need(
                not result.get("failure") and len(results) == 1,
                "SDK actual result, not startup error",
            )
            if name == "error":
                need(results[0].get("is_error") is True, "SDK real error result")
            else:
                need(
                    results[0].get("is_error") is False
                    and results[0].get("result") == "continued",
                    "SDK successful continuation",
                )
                need(
                    any(
                        any(
                            b.get("type") == "tool_use" and b.get("name") == "Write"
                            for b in e.get("blocks") or []
                        )
                        for e in events
                    ),
                    "SDK tool activity",
                )


def originals(work, inside=False):
    if os.environ.get("GITHUB_ACTIONS") != "true":
        raise ValueError("originals run only on disposable GitHub runners")
    target = (
        "darwin-arm64"
        if platform.system() == "Darwin" and platform.machine() == "arm64"
        else "linux-x64"
    )
    if target == "linux-x64" and not inside:
        command = [
            "docker",
            "run",
            "--rm",
            "--network=none",
            "--memory=2g",
            "--cpus=2",
            "--pids-limit=128",
            "--ulimit",
            "nofile=256:256",
            "--ulimit",
            "fsize=1048576:1048576",
            "-e",
            "GITHUB_ACTIONS=true",
            "-e",
            "SDK240_PREPARED_DRIVER="
            + os.environ.get(
                "SDK240_PREPARED_DRIVER",
                str(ROOT / "target/debug/examples/sdk240-trusted-prepared"),
            ),
            "-e",
            "GITHUB_SHA=" + os.environ["GITHUB_SHA"],
            "-v",
            str(ROOT) + ":" + str(ROOT) + ":ro",
            "-v",
            str(work) + ":" + str(work),
            "python:3.12-trixie",
            "python3",
            "-B",
            str(Path(__file__).resolve()),
            "--inside",
            "--work",
            str(work),
        ]
        # Frozen provisioning occurs before the network-none container starts.
        prepare(work, target)
        return subprocess.run(command, timeout=1100, check=True).returncode
    if not inside:
        prepare(work, target)
    manifest = json.loads((work / "inputs.json").read_text())
    results = {
        "schema": "sdk240.trusted-original.v1",
        "platform": target,
        "os": platform.platform(),
        "node": manifest["node"],
        "source": os.environ.get("GITHUB_SHA"),
        "cases": [],
    }
    failures = []
    for row in manifest["tuples"]:
        cases = [("sdk-options", n) for n in CASES + ("selected-user-deny",)]
        cases += [("shipped-sidecar", n) for n in CASES]
        cases += [("prepared", n) for n in CASES + ("deadline",)]
        for driver, name in cases:
            result = case(
                Path(row["root"]),
                Path(row["sdk"]),
                Path(row["native"]),
                Path(manifest["nodePath"]),
                name,
                sidecar=driver == "shipped-sidecar",
                prepared=driver == "prepared",
                version=row["sdk_version"],
            )
            result.update(
                driver=driver,
                sdkVersion=row["sdk_version"],
                nativeSha256=row["binary_sha256"],
            )
            results["cases"].append(result)
            try:
                assert_case(result, row["sdk_version"], driver)
            except ValueError as error:
                failures.append((row["sdk_version"], driver, name, str(error)))
            (work / "result.json").write_text(json.dumps(results, indent=2) + "\n")
    results["failures"] = failures
    (work / "result.json").write_text(json.dumps(results, indent=2) + "\n")
    if failures:
        raise RuntimeError(str(failures))
    return 0


def prepare(work, target):
    (work / "metadata.json").write_text(
        json.dumps(
            {
                "system": platform.system(),
                "machine": platform.machine(),
                "platform": platform.platform(),
                "imageOS": os.environ.get("ImageOS"),
                "imageVersion": os.environ.get("ImageVersion"),
                "source": os.environ.get("GITHUB_SHA"),
            },
            indent=2,
        )
    )
    plan = json.loads(checked(PLAN.read_bytes(), PLAN_SHA))
    node, node_row = provision_node(work, target)
    rows = []
    for row in plan["artifacts"]:
        base = work / row["sdk_version"]
        base.mkdir()
        sdk, native = provision(base, target, row)
        rows.append(
            {
                **row,
                "root": str(base),
                "sdk": str(sdk),
                "native": str(native),
                "binary_sha256": row["platforms"][target]["binary_sha256"],
            }
        )
    (work / "inputs.json").write_text(
        json.dumps({"nodePath": str(node), "node": node_row, "tuples": rows}, indent=2)
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--inside", action="store_true")
    parser.add_argument("--work", type=Path)
    args = parser.parse_args()
    if args.inside or os.environ.get("SDK240_TRUSTED_ORIGINAL") == "1":
        work = args.work or Path(os.environ["SDK240_TRUSTED_WORK"])
        return originals(work.resolve(), args.inside)
    return subprocess.run(
        [
            "python3",
            "-B",
            str(ROOT / "scripts/tests/claude-agent-sdk-trusted-native.py"),
        ],
        check=True,
    ).returncode


if __name__ == "__main__":
    raise SystemExit(main())
