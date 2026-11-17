#!/usr/bin/env python3
"""Frozen original ACP qualification on trusted disposable runners only."""

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import platform
import runpy
import shutil
import subprocess
import sys
import tarfile
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = (
    ROOT
    / "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0"
)
GRAPHS = (
    ROOT
    / "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-graphs"
)
HOPS = (
    "0.101.0",
    "0.101.1",
    "0.102.0",
    "0.103.0",
    "0.104.0",
    "0.104.1",
    "0.105.0",
    "0.106.0",
)


def digest(file):
    result = hashlib.sha256()
    with Path(file).open("rb") as stream:
        for data in iter(lambda: stream.read(1024 * 1024), b""):
            result.update(data)
    return result.hexdigest()


def trusted_ci():
    if (
        os.environ.get("GITHUB_ACTIONS") != "true"
        or os.environ.get("RUNNER_ENVIRONMENT") != "github-hosted"
    ):
        raise RuntimeError(
            "original qualification requires a trusted disposable GitHub runner; never the persistent Mac"
        )


def fetch(url, target, maximum):
    if not url.startswith("https://registry.npmjs.org/"):
        raise ValueError("unfrozen official package URL")
    if target.exists():
        return
    for attempt in range(3):
        try:
            with (
                urllib.request.urlopen(url, timeout=30) as response,
                target.open("xb") as output,
            ):
                count = 0
                while True:
                    data = response.read(1024 * 1024)
                    if not data:
                        break
                    count += len(data)
                    if count > maximum:
                        raise ValueError("frozen artifact size bound")
                    output.write(data)
            return
        except (OSError, TimeoutError):
            target.unlink(missing_ok=True)  # Only this invocation's partial file.
            if attempt == 2:
                raise
            time.sleep(1 + attempt)


def selected_files(archive, selected, output, maximum=64 * 1024 * 1024):
    found, total, names = set(), 0, set()
    with tarfile.open(archive, "r:gz") as source:
        for index, member in enumerate(source, 1):
            path = PurePosixPath(member.name)
            if (
                index > 16384
                or path.is_absolute()
                or ".." in path.parts
                or "\\" in member.name
                or not path.parts
                or path.parts[0] != "package"
                or not (member.isfile() or member.isdir())
                or member.sparse
                or str(path) in names
                or str(path) != member.name.rstrip("/")
            ):
                raise ValueError("unsafe frozen archive")
            names.add(str(path))
            total += member.size
            if total > maximum:
                raise ValueError("expanded frozen artifact bound")
            relative = str(path.relative_to("package"))
            if member.isfile() and relative in selected:
                target = output / relative
                target.parent.mkdir(parents=True, exist_ok=True)
                with source.extractfile(member) as stream, target.open("xb") as sink:
                    shutil.copyfileobj(stream, sink, 1024 * 1024)
                if digest(target) != selected[relative]:
                    raise ValueError("frozen extracted SHA256 mismatch: " + relative)
                found.add(relative)
    if found != set(selected):
        raise ValueError("missing frozen selected files")


def prepare(root, hops):
    trusted_ci()
    registry = json.loads((FIXTURE / "registry.json").read_text())["versions"]
    dependencies = json.loads((FIXTURE / "dependencies.json").read_text())
    extras = json.loads((FIXTURE / "import-edge-artifacts.json").read_text())
    platform_name = {
        ("Linux", "x86_64"): "linux-x64",
        ("Darwin", "arm64"): "darwin-arm64",
    }[platform.system(), platform.machine()]
    cache = root / "archives"
    cache.mkdir()
    node = root / "node"
    shutil.copyfile(Path(shutil.which("node")).resolve(), node)
    node.chmod(0o755)
    cases = []
    receipts = []
    native_cache = {}
    for hop in hops:
        row = registry[hop]
        sdk_version = row["dependencies"]["@anthropic-ai/claude-agent-sdk"]
        graph = json.loads((GRAPHS / (hop + ".json")).read_text())
        sources = root / ("sources-" + hop)
        packages = {
            "acp": row,
            "acpsdk": dependencies["artifacts"][
                "@agentclientprotocol/sdk@"
                + row["dependencies"]["@agentclientprotocol/sdk"]
            ],
            "agentsdk": dependencies["artifacts"][
                "@anthropic-ai/claude-agent-sdk@" + sdk_version
            ],
            "zod": extras["zod_4.6.7"],
            "diff": extras["diff_9.0.0"],
        }
        for prefix, package in packages.items():
            sha = package.get("tarball_sha256", package.get("sha256"))
            archive = cache / (sha + ".tgz")
            fetch(package["tarball"], archive, 8 * 1024 * 1024)
            raw = archive.read_bytes()
            if (
                digest(archive) != sha
                or "sha512-" + base64.b64encode(hashlib.sha512(raw).digest()).decode()
                != package["integrity"]
            ):
                raise ValueError("frozen source archive SHA256/integrity mismatch")
            selected = {
                p.split("/", 1)[1]: data["sha256"]
                for p, data in graph["files"].items()
                if p.startswith(prefix + "/")
            }
            selected_files(archive, selected, sources / prefix)
            receipts.append(
                {
                    "hop": hop,
                    "prefix": prefix,
                    "url": package["tarball"],
                    "sha256": sha,
                    "integrity": package["integrity"],
                    "bytes": len(raw),
                }
            )
        native_identity = dependencies["native_manifests"][sdk_version]["platforms"][
            platform_name
        ]
        if sdk_version not in native_cache:
            native_root = root / ("native-" + sdk_version)
            native_root.mkdir()
            name = "claude-agent-sdk-" + platform_name
            # Exact optional package/version is frozen in the SDK dependency row.
            assert (
                dependencies["artifacts"][
                    "@anthropic-ai/claude-agent-sdk@" + sdk_version
                ]["optionalDependencies"]["@anthropic-ai/" + name]
                == sdk_version
            )
            url = f"https://registry.npmjs.org/@anthropic-ai/{name}/-/{name}-{sdk_version}.tgz"
            archive = cache / ("native-" + sdk_version + ".tgz")
            fetch(url, archive, native_identity["size"] + 1024 * 1024)
            selected_files(
                archive,
                {"claude": native_identity["checksum"]},
                native_root,
                native_identity["size"] + 1024 * 1024,
            )
            native = native_root / "claude"
            if native.stat().st_size != native_identity["size"]:
                raise ValueError("frozen native size mismatch")
            native.chmod(0o755)
            native_cache[sdk_version] = str(native)
            receipts.append(
                {
                    "sdk": sdk_version,
                    "platform": platform_name,
                    "url": url,
                    "archiveSha256Observed": digest(archive),
                    "binarySha256Frozen": digest(native),
                    "binaryBytes": native.stat().st_size,
                }
            )
        cases.append(
            {
                "original": True,
                "hop": hop,
                "platform": platform_name,
                "sourceRoot": str(sources),
                "node": str(node),
                "native": native_cache[sdk_version],
            }
        )
    (root / "cases.json").write_text(json.dumps(cases, indent=2))
    (root / "evidence" / "artifact-receipts.json").write_text(
        json.dumps(receipts, indent=2)
    )


def inside(root):
    trusted_ci()
    import resource

    cases = json.loads((root / "cases.json").read_text())
    evidence = root / "evidence"
    limits = {}
    for name, value in (
        ("RLIMIT_NPROC", 256),
        ("RLIMIT_NOFILE", 2048),
        ("RLIMIT_RSS", 4 * 1024**3),
    ):
        kind = getattr(resource, name)
        _, hard = resource.getrlimit(kind)
        value = min(value, hard) if hard != resource.RLIM_INFINITY else value
        resource.setrlimit(kind, (value, value))
        limits[name] = value
    (evidence / "execution-os.json").write_text(
        json.dumps(
            {
                "os": platform.uname()._asdict(),
                "limits": limits,
                "rssLimit": "ordinary OS advisory; Linux container additionally enforces4GiB",
                "kernelAttestationClaim": False,
            },
            indent=2,
        )
    )
    if platform.system() == "Linux":
        # The offline container owns its copy. Do not mutate a toolcache or
        # reclassify uid1001 bind-mounted sources as root-owned source custody.
        import tempfile

        owned = Path(tempfile.mkdtemp(prefix="acp239-", dir=os.environ["TMPDIR"]))
        shutil.copytree(
            root,
            owned,
            dirs_exist_ok=True,
            ignore=shutil.ignore_patterns("archives", "evidence", "Dockerfile"),
        )
        for case in cases:
            for field in ("sourceRoot", "node", "native"):
                case[field] = str(owned / Path(case[field]).relative_to(root))
        root = owned
    ModelFixture = runpy.run_path(str(root / "model.py"))["ModelFixture"]
    system_policy = (
        Path("/etc/claude-code/managed-settings.json")
        if platform.system() == "Linux"
        else Path("/Library/Application Support/ClaudeCode/managed-settings.json")
    )
    # This path exists only in a disposable job/container. Refuse pre-existing
    # account configuration instead of saving or replacing it.
    if system_policy.exists():
        raise RuntimeError("disposable runner unexpectedly has administrator policy")
    if platform.system() == "Darwin":
        subprocess.run(
            ["sudo", "-n", "mkdir", "-p", str(system_policy.parent)], check=True
        )
    else:
        system_policy.parent.mkdir(parents=True, exist_ok=True)
    results = []
    for case in cases:
        for scenario in (
            "allow",
            "user-deny",
            "managed-deny",
            "cancel",
            "deadline",
            "native-exit",
            "error",
        ):
            work = root / (case["hop"] + "-" + scenario)
            for relative in ("workspace", "home/.claude", "outside"):
                (work / relative).mkdir(parents=True, exist_ok=True)
            (work / "outside/secret.txt").write_text("owned-secret-value\n")
            (work / "home/.claude/settings.json").write_text(
                json.dumps(
                    {
                        "permissions": {
                            "ask": ["Read"],
                            "denyInsteadOfPrompting": scenario == "user-deny",
                        }
                    }
                )
            )
            policy = json.dumps({"permissions": {"denyInsteadOfPrompting": True}})
            if scenario == "managed-deny":
                if platform.system() == "Darwin":
                    subprocess.run(
                        ["sudo", "-n", "tee", str(system_policy)],
                        input=policy.encode(),
                        stdout=subprocess.DEVNULL,
                        check=True,
                    )
                else:
                    system_policy.write_text(policy)
            else:
                if platform.system() == "Darwin":
                    subprocess.run(
                        ["sudo", "-n", "rm", "-f", str(system_policy)], check=True
                    )
                else:
                    system_policy.unlink(missing_ok=True)
            fixture = ModelFixture(work, scenario)
            env = {
                "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                "HOME": str(work / "home"),
                "TMPDIR": str(work),
                "TMP": str(work),
                "TEMP": str(work),
                "ANTHROPIC_API_KEY": "owned-fixture-key",
                "ANTHROPIC_BASE_URL": fixture.url,
                "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
                "DISABLE_TELEMETRY": "1",
                "DISABLE_ERROR_REPORTING": "1",
                "DISABLE_AUTOUPDATER": "1",
            }
            config = {
                **case,
                "caseRoot": str(work),
                "scenario": scenario,
                "environment": env,
            }
            file = work / "case.json"
            file.write_text(json.dumps(config))
            child_env = {
                "PATH": env["PATH"],
                "HOME": env["HOME"],
                "TMPDIR": str(work),
                "TMP": str(work),
                "TEMP": str(work),
                "GITHUB_ACTIONS": "true",
                "RUNNER_ENVIRONMENT": "github-hosted",
                "SWALLOWTAIL_ACP_TRUSTED_CASE": str(file),
            }
            command = [
                str(root / "test-runner"),
                "--ignored",
                "--exact",
                "trusted_native_tests::original_case",
                "--nocapture",
            ]
            if platform.system() == "Darwin":
                command = [
                    "sandbox-exec",
                    "-p",
                    '(version 1)(allow default)(deny network*)(allow network* (remote ip "localhost:*"))(allow network* (local ip "localhost:*"))',
                    *command,
                ]
            log = evidence / (case["hop"] + "-" + scenario + ".log")
            try:
                with log.open("wb") as output:
                    started = time.monotonic()
                    result = subprocess.run(
                        command,
                        env=child_env,
                        stdout=output,
                        stderr=subprocess.STDOUT,
                        timeout=150,
                    )
                    output.write(("\nexit=" + str(result.returncode) + "\n").encode())
                if log.stat().st_size > 1024 * 1024:
                    raise RuntimeError("case output exceeds1MiB")
                results.append(
                    {
                        "hop": case["hop"],
                        "scenario": scenario,
                        "exit": result.returncode,
                        "seconds": time.monotonic() - started,
                        "logSha256": digest(log),
                        "fixture": fixture.rows,
                    }
                )
                (evidence / "results.json").write_text(json.dumps(results, indent=2))
                print(json.dumps(results[-1]), flush=True)
                if result.returncode:
                    raise RuntimeError(
                        "real prepared ACP failure: " + case["hop"] + "/" + scenario
                    )
            finally:
                fixture.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inside", type=Path)
    parser.add_argument("--hop", choices=HOPS, action="append")
    args = parser.parse_args()
    if args.inside:
        inside(args.inside)
        return
    trusted_ci()
    import tempfile

    root = Path(tempfile.mkdtemp(prefix="acp239-trusted-", dir=os.environ["TMPDIR"]))
    (root / "evidence").mkdir()
    (root / "evidence/runner.json").write_text(
        json.dumps(
            {
                "os": platform.uname()._asdict(),
                "image": os.environ.get("ImageVersion"),
                "source": subprocess.check_output(
                    ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
                ).strip(),
                "nodeVersion": subprocess.check_output(
                    [shutil.which("node"), "--version"], text=True
                ).strip(),
                "trust": "disposable OS, ordinary libraries",
                "liveProvider": False,
            },
            indent=2,
        )
    )
    build = subprocess.run(
        [
            "bash",
            "scripts/oxide-rust.sh",
            "test",
            "--locked",
            "-p",
            "swallowtail-adapter-claude-agent",
            "--lib",
            "--no-run",
            "--message-format=json",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    executable = next(
        json.loads(line)["executable"]
        for line in build.stdout.splitlines()
        if line.startswith("{")
        and json.loads(line).get("reason") == "compiler-artifact"
        and json.loads(line).get("executable")
    )
    shutil.copyfile(executable, root / "test-runner")
    (root / "test-runner").chmod(0o755)
    metadata = json.loads((root / "evidence/runner.json").read_text())
    metadata.update(
        nodeSha256=digest(Path(shutil.which("node")).resolve()),
        testRunnerSha256=digest(root / "test-runner"),
        rustVersion=subprocess.check_output(["rustc", "--version"], text=True).strip(),
    )
    (root / "evidence/runner.json").write_text(json.dumps(metadata, indent=2))
    shutil.copyfile(__file__, root / "run.py")
    shutil.copyfile(ROOT / "scripts/claude-acp-trusted-model.py", root / "model.py")
    try:
        prepare(root, args.hop or HOPS)
        if platform.system() == "Linux":
            dockerfile = root / "Dockerfile"
            dockerfile.write_text(
                "FROM ubuntu:24.04\nRUN apt-get update && apt-get install -y --no-install-recommends python3 libstdc++6 procps && rm -rf /var/lib/apt/lists/* && mkdir /work\n"
            )
            subprocess.run(
                [
                    "docker",
                    "build",
                    "-t",
                    "acp239-trusted",
                    "-f",
                    str(dockerfile),
                    str(root / "evidence"),
                ],
                check=True,
                timeout=180,
            )
            image = subprocess.check_output(
                ["docker", "image", "inspect", "acp239-trusted", "--format", "{{.Id}}"],
                text=True,
            ).strip()
            (root / "evidence/container.json").write_text(
                json.dumps(
                    {
                        "image": image,
                        "base": "ubuntu:24.04",
                        "network": "none",
                        "memory": "4GiB",
                        "pids": 256,
                        "trust": "ordinary disposable test OS, not an admission pin",
                    },
                    indent=2,
                )
            )
            command = [
                "docker",
                "run",
                "--rm",
                "--network",
                "none",
                "--memory",
                "4g",
                "--pids-limit",
                "256",
                "-v",
                f"{root}:{root}",
                "-e",
                "TMPDIR=/work",
                "-e",
                "GITHUB_ACTIONS=true",
                "-e",
                "RUNNER_ENVIRONMENT=github-hosted",
                "acp239-trusted",
                "python3",
                str(root / "run.py"),
                "--inside",
                str(root),
            ]
        else:
            command = [
                sys.executable,
                "-I",
                "-B",
                str(root / "run.py"),
                "--inside",
                str(root),
            ]
        subprocess.run(command, check=True, timeout=1000)
    finally:
        output = Path(os.environ["ACP_TRUSTED_EVIDENCE"])
        output.mkdir(parents=True, exist_ok=True)
        total = sum(p.stat().st_size for p in (root / "evidence").iterdir())
        if total > 16 * 1024 * 1024:
            raise RuntimeError(f"required evidence exceeds16MiB: {total}")
        with tarfile.open(
            output / (platform.system() + "-original-evidence.tgz"), "w:gz"
        ) as archive:
            for p in (root / "evidence").iterdir():
                archive.add(p, arcname=p.name, recursive=False)


if __name__ == "__main__":
    main()
