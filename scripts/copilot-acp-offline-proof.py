#!/usr/bin/env python3
"""Run Copilot ACP proof fixtures inside a network-denied macOS sandbox.

The fake proof validates containment and record ordering. Exact artifact runs
are separately gated by a durable, secret-free record created by --prepare.
"""

from __future__ import annotations

import argparse
import hashlib
import ipaddress
import json
import os
import platform
import re
import select
import socket
import socketserver
import ssl
import selectors
import signal
import subprocess
import sys
import tempfile
import threading
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof"
)
INVENTORY_PATH = FIXTURE_DIR / "artifact-inventory.json"
AUTHENTICATED_PLAN_PATH = FIXTURE_DIR / "authenticated-proof-plan.json"
AUTHENTICATED_SCHEMA_PATH = FIXTURE_DIR / "authenticated-proof-plan.schema.json"
PERMISSION_PROOF_PLAN_PATH = FIXTURE_DIR / "permission-proof-plan.json"
VERSIONS = ("1.0.80", "1.0.81", "1.0.93")
MAX_FRAME_BYTES = 256 * 1024
READ_TIMEOUT_SECONDS = 20.0
RUN_TIMEOUT_SECONDS = 18.0
FAKE_TOKEN = "SWALLOWTAIL_OFFLINE_FAKE_TOKEN"
FAKE_DELEGATED_CREDENTIAL = b"test-only-delegated-capability"
FAKE_ACCOUNT_REF = "github-account:betterthanclay"
FAKE_MODEL_ID = "fake-github-hosted-model"
FAKE_AUDIENCE = "copilot-api.fake.test"
MAX_AUTH_FAKE_SECONDS = 5.0
DISCOVERY_REQUEST_METHODS = frozenset({"initialize", "authenticate", "session/new"})
DISCOVERY_METADATA_UPDATES = frozenset(
    {"available_commands_update", "config_option_update", "current_mode_update"}
)
OFFICIAL_COPILOT_HOSTS = frozenset(
    {
        "github.com",
        "github.githubassets.com",
        "avatars.githubusercontent.com",
        "api.github.com",
        "default.exp-tas.com",
        "copilot-proxy.githubusercontent.com",
        "origin-tracker.githubusercontent.com",
    }
)
OFFICIAL_COPILOT_SUFFIXES = ("githubcopilot.com",)
LIVE_PERMISSION_SECONDS = 60.0
LIVE_CLEANUP_SECONDS = 3.0


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace(
        "+00:00", "Z"
    )


def json_bytes(value: Any) -> bytes:
    return (json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


def json_digest(value: Any) -> str:
    canonical = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(canonical.encode()).hexdigest()


def write_json_durable(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp_path = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    with temp_path.open("wb") as output:
        output.write(json.dumps(value, indent=2, sort_keys=True).encode() + b"\n")
        output.flush()
        os.fsync(output.fileno())
    os.replace(temp_path, path)
    directory_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_fd)
    finally:
        os.close(directory_fd)


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise ValueError(f"expected an object in {path.name}")
    return value


def validate_schema_value(value: Any, schema: dict[str, Any], label: str = "$") -> None:
    """Validate the small JSON Schema subset used by the committed plan."""
    supported = {
        "$schema",
        "$id",
        "title",
        "description",
        "type",
        "const",
        "enum",
        "required",
        "properties",
        "items",
        "additionalProperties",
        "minItems",
        "maxItems",
        "minLength",
        "maxLength",
        "pattern",
    }
    unknown = set(schema) - supported
    if unknown:
        raise ValueError(f"unsupported plan schema keywords at {label}: {sorted(unknown)}")

    expected_type = schema.get("type")
    if expected_type is not None:
        matches = {
            "object": isinstance(value, dict),
            "array": isinstance(value, list),
            "string": isinstance(value, str),
            "integer": isinstance(value, int) and not isinstance(value, bool),
            "boolean": isinstance(value, bool),
            "null": value is None,
        }.get(expected_type)
        if matches is not True:
            raise ValueError(f"{label} must be {expected_type}")
    if "const" in schema and value != schema["const"]:
        raise ValueError(f"{label} does not match its required constant")
    if "enum" in schema and value not in schema["enum"]:
        raise ValueError(f"{label} is outside its allowed values")

    if isinstance(value, dict):
        required = schema.get("required", [])
        missing = set(required) - set(value)
        if missing:
            raise ValueError(f"{label} is missing required fields: {sorted(missing)}")
        properties = schema.get("properties", {})
        extra = set(value) - set(properties)
        if schema.get("additionalProperties") is False and extra:
            raise ValueError(f"{label} has unexpected fields: {sorted(extra)}")
        for key, child_schema in properties.items():
            if key in value:
                validate_schema_value(value[key], child_schema, f"{label}.{key}")
    elif isinstance(value, list):
        if len(value) < schema.get("minItems", 0):
            raise ValueError(f"{label} has fewer items than required")
        if len(value) > schema.get("maxItems", sys.maxsize):
            raise ValueError(f"{label} has more items than allowed")
        if "items" in schema:
            for index, item in enumerate(value):
                validate_schema_value(item, schema["items"], f"{label}[{index}]")
    elif isinstance(value, str):
        if len(value) < schema.get("minLength", 0):
            raise ValueError(f"{label} is shorter than required")
        if len(value) > schema.get("maxLength", sys.maxsize):
            raise ValueError(f"{label} is longer than allowed")
        pattern = schema.get("pattern")
        if pattern is not None and re.search(pattern, value) is None:
            raise ValueError(f"{label} does not match its required pattern")


def validate_authenticated_plan() -> dict[str, Any]:
    schema = load_json(AUTHENTICATED_SCHEMA_PATH)
    plan = load_json(AUTHENTICATED_PLAN_PATH)
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise ValueError("authenticated plan schema is not Draft 2020-12")
    validate_schema_value(plan, schema)
    serialized = json.dumps(plan, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"authenticated plan contains forbidden secret/path marker {forbidden!r}")
    if plan.get("execution_authorized") is not False:
        raise ValueError("unattested plan must remain non-executable")
    return plan


def inventory_packages(inventory: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    result: dict[tuple[str, str], dict[str, Any]] = {}
    for package in inventory.get("packages", []):
        name = package["name"]
        for version in package.get("versions", []):
            result[(name, version["version"])] = version
    return result


def verify_inventory(path: Path = INVENTORY_PATH) -> dict[str, Any]:
    inventory = load_json(path)
    if inventory.get("schema") != "copilot-cli-acp-offline-artifact-inventory.v1":
        raise ValueError("unexpected Copilot artifact inventory schema")
    if inventory.get("platform") != "darwin-arm64":
        raise ValueError("inventory is not for the selected darwin-arm64 host")
    packages = inventory_packages(inventory)
    for name in ("@github/copilot", "@github/copilot-darwin-arm64"):
        for version in VERSIONS:
            record = packages.get((name, version))
            if record is None:
                raise ValueError(f"inventory is missing {name}@{version}")
            files = record.get("files")
            if not isinstance(files, list) or len(files) != record.get("dist", {}).get("fileCount"):
                raise ValueError(f"inventory file count mismatch for {name}@{version}")
            paths = [item.get("path") for item in files]
            if paths != sorted(paths) or len(paths) != len(set(paths)):
                raise ValueError(f"inventory paths are not a sorted unique set for {name}@{version}")
            manifest = "".join(
                f"{item['path']}\0{item['kind']}\0{item['size']}\0{item['mode']}\0{item['sha256']}\n"
                for item in files
            ).encode()
            if hashlib.sha256(manifest).hexdigest() != record.get("inventory_sha256"):
                raise ValueError(f"inventory digest mismatch for {name}@{version}")
            if sum(item["size"] for item in files) != record.get("dist", {}).get("unpackedSize"):
                raise ValueError(f"inventory byte count mismatch for {name}@{version}")
            if any(
                not item.get("path", "").startswith("package/")
                or item.get("kind") != "file"
                or len(item.get("sha256", "")) != 64
                for item in files
            ):
                raise ValueError(f"invalid file entry for {name}@{version}")
    tags = inventory.get("stable_channel_cross_check", {})
    if not (
        tags.get("npm_latest")
        == tags.get("native_latest")
        == tags.get("github_latest_release", {}).get("tag", "").removeprefix("v")
        == "1.0.93"
    ):
        raise ValueError("the frozen official stable channels do not agree on 1.0.93")
    if tags.get("npm_prerelease") == "1.0.93":
        raise ValueError("the prerelease dist-tag was confused with stable")
    return inventory


def quote_profile_path(path: Path) -> str:
    return json.dumps(str(path.resolve()))


def sandbox_profile(
    scratch: Path, host_home: Path, repository_root: Path = ROOT
) -> str:
    scratch = scratch.resolve()
    host_home = host_home.resolve()
    repository_root = repository_root.resolve()
    isolated_home_helper = (ROOT / "scripts/run-with-isolated-home.sh").resolve()
    return " ".join(
        (
            "(version 1)",
            "(deny default)",
            "(allow process*)",
            "(allow sysctl-read)",
            "(allow mach-lookup)",
            "(deny mach-lookup (global-name \"com.apple.securityd\"))",
            "(allow file-read*)",
            f"(deny file-read* (subpath {quote_profile_path(host_home)}))",
            f"(deny file-read* (subpath {quote_profile_path(repository_root)}))",
            f"(allow file-read* (literal {quote_profile_path(isolated_home_helper)}))",
            f"(allow file-write* (subpath {quote_profile_path(scratch)}))",
            "(deny network*)",
        )
    )


def permission_sandbox_profile(
    scratch: Path,
    record_dir: Path,
    host_home: Path,
    repository_root: Path,
    executable: Path,
    proxy_port: int,
    mapped_code_roots: tuple[Path, ...] = (),
) -> str:
    scratch = scratch.resolve()
    record_dir = record_dir.resolve()
    host_home = host_home.resolve()
    repository_root = repository_root.resolve()
    executable = executable.resolve(strict=True)
    return " ".join(
        (
            "(version 1)",
            '(import "/System/Library/Sandbox/Profiles/dyld-support.sb")',
            "(deny default)",
            "(allow process-fork)",
            f"(allow process-exec* (literal {quote_profile_path(executable)}))",
            f"(allow file-map-executable (literal {quote_profile_path(executable)}))",
            '(allow file-map-executable (subpath "/System/Library"))',
            '(allow file-map-executable (subpath "/usr/lib"))',
            *(
                f"(allow file-map-executable (subpath {quote_profile_path(root)}))"
                for root in mapped_code_roots
            ),
            "(allow sysctl-read)",
            '(allow mach-lookup (global-name "com.apple.securityd"))',
            "(allow file-read*)",
            f"(deny file-read* (subpath {quote_profile_path(host_home)}))",
            f"(deny file-read* (subpath {quote_profile_path(repository_root)}))",
            f"(deny file-read* (subpath {quote_profile_path(record_dir)}))",
            f"(allow file-write* (subpath {quote_profile_path(scratch)}))",
            "(deny network-inbound)",
            "(deny network-bind)",
            f'(allow network-outbound (remote ip "localhost:{proxy_port}"))',
        )
    )


def path_is_within(path: Path, root: Path) -> bool:
    try:
        return path.resolve(strict=False).is_relative_to(root.resolve(strict=False))
    except (OSError, RuntimeError):
        return False


def discovery_request_allowed(method: str) -> bool:
    return method in DISCOVERY_REQUEST_METHODS


def discovery_callback_allowed(method: str, update_type: str | None = None) -> bool:
    return method == "session/update" and update_type in DISCOVERY_METADATA_UPDATES


def discovery_endpoint_allowed(origin: str, allowed_origins: set[str]) -> bool:
    return origin in allowed_origins


def discovery_budget_allowed(invocation_number: int, elapsed_seconds: float) -> bool:
    return invocation_number == 1 and 0 <= elapsed_seconds <= 60.0


def discovery_read_allowed(path: Path, host_home: Path, repository_root: Path) -> bool:
    return not path_is_within(path, host_home) and not path_is_within(
        path, repository_root
    )


def discovery_write_allowed(path: Path, scratch: Path) -> bool:
    return path_is_within(path, scratch)


def validate_permission_proof_plan() -> dict[str, Any]:
    plan = load_json(PERMISSION_PROOF_PLAN_PATH)
    if plan.get("schema") != "copilot-cli-acp-permission-proof-plan.v1":
        raise ValueError("unexpected original permission proof plan schema")
    if plan.get("operator_decision") != "8b098b85-fb9e-4543-b5f1-1fbfe8bc5301":
        raise ValueError("original permission proof does not bind the approved operator ruling")
    if plan.get("execution_authorized") is not True:
        raise ValueError("original permission proof lacks the approved execution boundary")
    if plan.get("scope") != {
        "route": "copilot-cli.acp",
        "platform": "darwin-arm64",
        "versions": ["1.0.80", "1.0.81", "1.0.93"],
        "inventory_sha256": "2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f",
    }:
        raise ValueError("original permission proof scope differs from the frozen artifact inventory")
    if plan.get("selection_policy", {}).get("requested") != "Auto" or plan.get(
        "selection_policy", {}
    ).get("binding") != "--model auto on the original process":
        raise ValueError("original permission proof does not bind the requested Auto selection")
    if plan.get("credential_boundary", {}).get("item_level_mediation") != (
        "unavailable; no per-item filter claimed"
    ):
        raise ValueError("original permission proof misstates the approved Keychain boundary")
    if plan.get("containment", {}).get("network") != (
        "local CONNECT proxy only; verify official host and matching TLS SNI before external dial"
    ):
        raise ValueError("original permission proof lacks its verified egress gate")
    if plan.get("start_order") != ["1.0.93", "1.0.81", "1.0.80"]:
        raise ValueError("original permission proof must start with 1.0.93")
    network = plan.get("network_policy", {})
    if set(network.get("allowed_hosts", [])) != OFFICIAL_COPILOT_HOSTS:
        raise ValueError("network allowlist differs from the reviewed official host set")
    if tuple(network.get("allowed_subdomains", [])) != OFFICIAL_COPILOT_SUFFIXES:
        raise ValueError("network wildcard differs from the reviewed official host set")
    budgets = plan.get("budgets", {})
    if budgets != {
        "max_invocations_per_version": 1,
        "max_total_invocations": 3,
        "max_prompts_per_invocation": 1,
        "max_seconds_per_invocation": 60,
        "max_harness_retries": 0,
        "max_harness_resends": 0,
        "max_harness_model_fallbacks": 0,
        "vendor_managed_retries": "allowed_within_single_prompt_and_deadline; count_unobserved",
        "max_tool_effects": 0,
        "reviewer_original_attempts": 0,
    }:
        raise ValueError("original permission proof budgets differ from the approved boundary")
    serialized = json.dumps(plan, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"original permission plan contains forbidden marker {forbidden!r}")
    return plan


def host_allowed(host: str, exact_hosts: set[str], allowed_subdomains: tuple[str, ...]) -> bool:
    normalized = host.rstrip(".").lower()
    if normalized in exact_hosts:
        return True
    return any(normalized.endswith(f".{suffix}") for suffix in allowed_subdomains)


def parse_connect_request(request: bytes) -> tuple[str, int] | None:
    try:
        first_line = request.split(b"\r\n", 1)[0].decode("ascii")
        method, authority, protocol = first_line.split(" ")
        if method != "CONNECT" or protocol not in {"HTTP/1.0", "HTTP/1.1"}:
            return None
        host, separator, port_text = authority.rpartition(":")
        if not separator or not host or not port_text.isdecimal():
            return None
        if not host.isascii() or any(character in host for character in "/@[]"):
            return None
        port = int(port_text)
        if port != 443:
            return None
        return host.rstrip(".").lower(), port
    except (UnicodeDecodeError, ValueError):
        return None


def read_http_headers(connection: socket.socket) -> tuple[bytes, bytes]:
    data = bytearray()
    while b"\r\n\r\n" not in data:
        chunk = connection.recv(4096)
        if not chunk:
            raise ConnectionError("proxy client closed before CONNECT headers")
        data.extend(chunk)
        if len(data) > 16 * 1024:
            raise ValueError("proxy CONNECT headers exceeded the proof limit")
    end = data.index(b"\r\n\r\n") + 4
    return bytes(data[:end]), bytes(data[end:])


def read_client_hello(connection: socket.socket, prefix: bytes = b"") -> tuple[bytes, str | None]:
    data = bytearray(prefix)
    while len(data) < 5:
        chunk = connection.recv(4096)
        if not chunk:
            raise ConnectionError("proxy client closed before TLS ClientHello")
        data.extend(chunk)
    if data[0] != 0x16:
        raise ValueError("proxy tunnel did not start with a TLS handshake")
    record_length = int.from_bytes(data[3:5], "big")
    if record_length < 4 or record_length > 65535:
        raise ValueError("proxy TLS record length is outside the proof limit")
    total_length = 5 + record_length
    while len(data) < total_length:
        chunk = connection.recv(min(4096, total_length - len(data)))
        if not chunk:
            raise ConnectionError("proxy client closed during TLS ClientHello")
        data.extend(chunk)
    record = bytes(data[:total_length])
    payload = record[5:]
    if payload[0] != 0x01:
        raise ValueError("proxy TLS handshake did not begin with ClientHello")
    hello_length = int.from_bytes(payload[1:4], "big")
    hello = payload[4 : 4 + hello_length]
    if len(hello) != hello_length or hello_length < 38:
        raise ValueError("proxy TLS ClientHello is incomplete")
    offset = 34
    session_length = hello[offset]
    offset += 1 + session_length
    if offset + 2 > len(hello):
        raise ValueError("proxy TLS ClientHello has no cipher list")
    cipher_length = int.from_bytes(hello[offset : offset + 2], "big")
    offset += 2 + cipher_length
    if offset >= len(hello):
        raise ValueError("proxy TLS ClientHello has no compression list")
    compression_length = hello[offset]
    offset += 1 + compression_length
    if offset + 2 > len(hello):
        return record, None
    extensions_length = int.from_bytes(hello[offset : offset + 2], "big")
    offset += 2
    extension_end = min(len(hello), offset + extensions_length)
    while offset + 4 <= extension_end:
        extension_type = int.from_bytes(hello[offset : offset + 2], "big")
        extension_length = int.from_bytes(hello[offset + 2 : offset + 4], "big")
        offset += 4
        extension = hello[offset : offset + extension_length]
        offset += extension_length
        if extension_type != 0x0000 or len(extension) < 5:
            continue
        names_length = int.from_bytes(extension[:2], "big")
        name_offset = 2
        names_end = min(len(extension), 2 + names_length)
        while name_offset + 3 <= names_end:
            name_type = extension[name_offset]
            name_length = int.from_bytes(
                extension[name_offset + 1 : name_offset + 3], "big"
            )
            name_offset += 3
            name = extension[name_offset : name_offset + name_length]
            name_offset += name_length
            if name_type == 0 and len(name) == name_length:
                try:
                    return record, name.decode("ascii").rstrip(".").lower()
                except UnicodeDecodeError:
                    return record, None
    return record, None


class CopilotEgressProxy(socketserver.ThreadingMixIn, socketserver.TCPServer):
    allow_reuse_address = False
    daemon_threads = True
    block_on_close = False

    def __init__(
        self,
        allowed_hosts: set[str],
        allowed_subdomains: tuple[str, ...],
        deadline: float,
        *,
        fake_only: bool = False,
    ) -> None:
        self.allowed_hosts = allowed_hosts
        self.allowed_subdomains = allowed_subdomains
        self.deadline = deadline
        self.fake_only = fake_only
        self.allowed_destinations: set[str] = set()
        self.unlisted_destination_count = 0
        self.sni_mismatch_count = 0
        self.connection_failure_count = 0
        self._evidence_lock = threading.Lock()
        self._active_sockets: set[socket.socket] = set()
        self._client_threads: set[threading.Thread] = set()
        self._threads_lock = threading.Lock()
        super().__init__(("127.0.0.1", 0), CopilotEgressProxyHandler)
        self._serve_thread = threading.Thread(target=self.serve_forever, daemon=True)

    def start(self) -> int:
        self._serve_thread = threading.Thread(
            target=lambda: self.serve_forever(poll_interval=0.05), daemon=True
        )
        self._serve_thread.start()
        return int(self.server_address[1])

    def attach_socket(self, connection: socket.socket) -> None:
        with self._evidence_lock:
            self._active_sockets.add(connection)

    def detach_socket(self, connection: socket.socket) -> None:
        with self._evidence_lock:
            self._active_sockets.discard(connection)

    def attach_upstream(self, connection: socket.socket) -> None:
        with self._evidence_lock:
            self._active_sockets.add(connection)

    def record_unlisted(self) -> None:
        with self._evidence_lock:
            self.unlisted_destination_count += 1

    def record_sni_mismatch(self) -> None:
        with self._evidence_lock:
            self.sni_mismatch_count += 1

    def record_connection_failure(self) -> None:
        with self._evidence_lock:
            self.connection_failure_count += 1

    def record_allowed(self, host: str) -> None:
        with self._evidence_lock:
            self.allowed_destinations.add(host)

    def snapshot(self) -> dict[str, Any]:
        with self._evidence_lock:
            return {
                "allowed_destination_hosts": sorted(self.allowed_destinations),
                "unlisted_destination_count": self.unlisted_destination_count,
                "sni_mismatch_count": self.sni_mismatch_count,
                "connection_failure_count": self.connection_failure_count,
            }

    def process_request(self, request: socket.socket, client_address: Any) -> None:
        thread = threading.Thread(
            target=self.process_request_thread, args=(request, client_address)
        )
        thread.daemon = True
        with self._threads_lock:
            self._client_threads.add(thread)
        thread.start()

    def process_request_thread(self, request: socket.socket, client_address: Any) -> None:
        self.attach_socket(request)
        try:
            super().process_request_thread(request, client_address)
        finally:
            self.detach_socket(request)
            with self._threads_lock:
                self._client_threads.discard(threading.current_thread())

    def stop_and_join(self, timeout: float) -> bool:
        self.shutdown()
        self.server_close()
        with self._evidence_lock:
            sockets = list(self._active_sockets)
        for connection in sockets:
            try:
                connection.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass
            try:
                connection.close()
            except OSError:
                pass
        deadline = time.monotonic() + timeout
        self._serve_thread.join(max(0.0, deadline - time.monotonic()))
        while time.monotonic() < deadline:
            with self._threads_lock:
                threads = list(self._client_threads)
            if not threads:
                break
            for thread in threads:
                thread.join(min(0.05, max(0.0, deadline - time.monotonic())))
        with self._threads_lock:
            return not self._serve_thread.is_alive() and not self._client_threads


class CopilotEgressProxyHandler(socketserver.BaseRequestHandler):
    def handle(self) -> None:
        server: CopilotEgressProxy = self.server  # type: ignore[assignment]
        connection: socket.socket = self.request
        upstream: socket.socket | None = None
        try:
            connection.settimeout(min(3.0, max(0.1, server.deadline - time.monotonic())))
            request, buffered = read_http_headers(connection)
            target = parse_connect_request(request)
            if target is None:
                server.record_unlisted()
                connection.sendall(b"HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n")
                return
            host, port = target
            if not host_allowed(host, server.allowed_hosts, server.allowed_subdomains):
                server.record_unlisted()
                connection.sendall(b"HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n")
                return
            connection.sendall(
                b"HTTP/1.1 200 Connection Established\r\nProxy-Agent: Swallowtail-proof\r\n\r\n"
            )
            hello, sni = read_client_hello(connection, buffered)
            if sni is None or sni != host:
                server.record_sni_mismatch()
                return
            if not host_allowed(sni, server.allowed_hosts, server.allowed_subdomains):
                server.record_unlisted()
                return
            if server.fake_only:
                server.record_allowed(host)
                return
            upstream = connect_public_host(host, port, server.deadline)
            server.attach_upstream(upstream)
            upstream.sendall(hello)
            server.record_allowed(host)
            relay_until_deadline(connection, upstream, server.deadline)
        except (OSError, ValueError, ConnectionError, TimeoutError):
            server.record_connection_failure()
        finally:
            if upstream is not None:
                server.detach_socket(upstream)
                try:
                    upstream.close()
                except OSError:
                    pass


def connect_public_host(host: str, port: int, deadline: float) -> socket.socket:
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise TimeoutError("Copilot egress deadline expired")
    addresses = socket.getaddrinfo(host, port, type=socket.SOCK_STREAM)
    last_error: OSError | None = None
    for family, socktype, proto, _, address in addresses:
        ip = ipaddress.ip_address(address[0].split("%", 1)[0])
        if not ip.is_global:
            continue
        upstream = socket.socket(family, socktype, proto)
        try:
            upstream.settimeout(min(3.0, max(0.1, deadline - time.monotonic())))
            upstream.connect(address)
            return upstream
        except OSError as error:
            last_error = error
            upstream.close()
    raise OSError("no public address was reachable for an approved Copilot host") from last_error


def relay_until_deadline(client: socket.socket, upstream: socket.socket, deadline: float) -> None:
    client.setblocking(False)
    upstream.setblocking(False)
    while time.monotonic() < deadline:
        readable, _, _ = select.select(
            [client, upstream], [], [], min(0.5, deadline - time.monotonic())
        )
        if not readable:
            continue
        for source in readable:
            data = source.recv(64 * 1024)
            if not data:
                return
            target = upstream if source is client else client
            target.setblocking(True)
            target.settimeout(min(1.0, max(0.1, deadline - time.monotonic())))
            target.sendall(data)
            target.setblocking(False)


def clean_child_environment(
    scratch: Path, *, include_fake_token: bool = True
) -> dict[str, str]:
    home = scratch / "home"
    temp = scratch / "tmp"
    config = scratch / "config"
    cache = scratch / "cache"
    for directory in (home, temp, config, cache):
        directory.mkdir(parents=True, exist_ok=True)
    environment = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(home),
        "COPILOT_HOME": str(home),
        "GH_COPILOT_HOME": str(home),
        "XDG_CONFIG_HOME": str(config),
        "XDG_CACHE_HOME": str(cache),
        "TMPDIR": str(temp),
        "TERM": "dumb",
    }
    if include_fake_token:
        environment["GITHUB_TOKEN"] = FAKE_TOKEN
        environment["GH_TOKEN"] = FAKE_TOKEN
    return environment


class StdioClient:
    def __init__(self, process: subprocess.Popen[bytes]):
        if process.stdout is None or process.stdin is None:
            raise ValueError("ACP child has no piped stdio")
        self.process = process
        self.stdout = process.stdout
        self.stdin = process.stdin
        self.buffer = bytearray()
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.stdout, selectors.EVENT_READ)

    def send(self, value: dict[str, Any]) -> None:
        self.stdin.write(json_bytes(value))
        self.stdin.flush()

    def receive(self, timeout: float) -> dict[str, Any]:
        deadline = time.monotonic() + timeout
        while True:
            newline = self.buffer.find(b"\n")
            if newline >= 0:
                if newline > MAX_FRAME_BYTES:
                    raise ValueError("ACP frame exceeded the 256 KiB proof limit")
                line = bytes(self.buffer[:newline])
                del self.buffer[: newline + 1]
                value = json.loads(line)
                if not isinstance(value, dict):
                    raise ValueError("ACP frame was not an object")
                return value
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError("ACP response timeout")
            if not self.selector.select(remaining):
                raise TimeoutError("ACP response timeout")
            chunk = os.read(self.stdout.fileno(), 65536)
            if not chunk:
                raise EOFError("ACP child closed stdout")
            self.buffer.extend(chunk)
            if len(self.buffer) > MAX_FRAME_BYTES and b"\n" not in self.buffer:
                raise ValueError("ACP frame exceeded the 256 KiB proof limit")

    def close(self) -> None:
        self.selector.close()


def wait_for_response(
    client: StdioClient,
    request_id: int,
    evidence: dict[str, Any],
    timeout: float,
    *,
    discovery_mode: bool = False,
) -> dict[str, Any] | None:
    deadline = time.monotonic() + timeout
    while True:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            evidence["timed_out"] = True
            return None
        try:
            message = client.receive(remaining)
        except TimeoutError:
            evidence["timed_out"] = True
            return None
        except (EOFError, ValueError, json.JSONDecodeError):
            evidence["stream_failed"] = True
            return None
        method = message.get("method")
        if method == "session/request_permission":
            if discovery_mode:
                evidence["unexpected_discovery_callback"] = method
                return None
            evidence["permission_request_observed"] = True
            evidence["permission_methods_observed"].append(method)
            params = message.get("params")
            options = params.get("options", []) if isinstance(params, dict) else []
            if not isinstance(options, list):
                options = []
            evidence["permission_option_ids"] = sorted(
                {
                    option["optionId"]
                    for option in options
                    if isinstance(option, dict)
                    and isinstance(option.get("optionId"), str)
                    and option["optionId"]
                    in {"allow_once", "allow_always", "reject_once", "reject_always"}
                }
            )
            permission_id = message.get("id")
            if permission_id is not None:
                pending_wait = PendingPermissionWait()
                pending_wait.abandon()
                evidence["pending_permission_wait_abandoned"] = (
                    pending_wait.state == "abandoned"
                )
                evidence["late_permission_response_accepted"] = pending_wait.resolve(
                    "allow_once"
                )
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": permission_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
                evidence["permission_reply"] = "cancelled"
            continue
        if method == "session/update":
            update = message.get("params", {}).get("update", {})
            update_type = update.get("sessionUpdate") if isinstance(update, dict) else None
            if discovery_mode and not discovery_callback_allowed(method, update_type):
                evidence["unexpected_discovery_callback"] = update_type or method
                return None
            evidence["session_updates"] += 1
            if update.get("sessionUpdate") == "tool_call":
                evidence["tool_call_updates"] += 1
            continue
        if discovery_mode and method is not None:
            evidence["unexpected_discovery_callback"] = method
            return None
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1


class PendingPermissionWait:
    """Model one unresolved host decision that cancellation abandons."""

    def __init__(self) -> None:
        self.state = "pending"

    def abandon(self) -> None:
        if self.state == "pending":
            self.state = "abandoned"

    def resolve(self, option_id: str) -> bool:
        if self.state != "pending" or option_id not in {"allow_once", "reject_once"}:
            return False
        self.state = "resolved"
        return True


def record_rpc_error(
    stage: str,
    response: dict[str, Any],
    evidence: dict[str, Any],
) -> bool:
    error = response.get("error")
    if not isinstance(error, dict):
        return False
    code = error.get("code")
    safe_code = code if isinstance(code, int) and not isinstance(code, bool) else None
    authentication_required = (
        safe_code == -32000 and error.get("message") == "Authentication required"
    )
    evidence[stage] = (
        "authentication-required" if authentication_required else "json-rpc-error"
    )
    evidence[f"{stage}_error"] = {
        "code": safe_code,
        "message": "Authentication required" if authentication_required else "redacted",
        "data_omitted": "data" in error,
    }
    return True


def auth_method_ids(result: dict[str, Any]) -> list[str]:
    methods = result.get("authMethods", [])
    if not isinstance(methods, list):
        return []
    ids = {
        method["id"]
        for method in methods
        if isinstance(method, dict)
        and isinstance(method.get("id"), str)
        and 0 < len(method["id"]) <= 80
        and all(character.isalnum() or character in "_.:-" for character in method["id"])
    }
    return sorted(ids)


def stop_process(process: subprocess.Popen[bytes], grace_seconds: float = 1.0) -> tuple[int | None, bool]:
    if process.stdin is not None:
        try:
            process.stdin.close()
        except OSError:
            pass
    forced = False
    try:
        code = process.wait(timeout=grace_seconds)
    except subprocess.TimeoutExpired:
        forced = True
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        code = process.wait(timeout=2.0)
    return code, forced


def launch_command(profile: str, command: list[str], child_environment: dict[str, str]) -> list[str]:
    helper = ROOT / "scripts/run-with-isolated-home.sh"
    return [
        "/bin/bash",
        str(helper),
        "--home-var",
        "COPILOT_HOME",
        "--home-var",
        "GH_COPILOT_HOME",
        "--",
        "/usr/bin/env",
        "-i",
        *[f"{key}={value}" for key, value in child_environment.items()],
        "/usr/bin/sandbox-exec",
        "-p",
        profile,
        *command,
    ]


FAKE_AGENT_SOURCE = r'''#!/bin/bash
set -eu
scratch=${SWALLOWTAIL_EXPECTED_HOME%/home}
[[ "$HOME" == "$SWALLOWTAIL_EXPECTED_HOME" ]]
[[ "$GITHUB_TOKEN" == "SWALLOWTAIL_OFFLINE_FAKE_TOKEN" ]]

for host in 127.0.0.1 203.0.113.1; do
  denial_file="$scratch/network-denied-${host}.txt"
  if (exec 3<>"/dev/tcp/$host/443") 2>"$denial_file"; then
    exit 31
  fi
  denial=$(<"$denial_file")
  [[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]
done

keychain_denial="$scratch/keychain-denied.txt"
if (exec 3<"$SWALLOWTAIL_BLOCKED_HOST_HOME/Library/Keychains/login.keychain-db") 2>"$keychain_denial"; then
  exit 32
fi
denial=$(<"$keychain_denial")
[[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]

for blocked_file in \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/config.json" \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/settings.json" \
  "$SWALLOWTAIL_BLOCKED_REPOSITORY/README.md"; do
  denial_file="$scratch/resource-read-denied-$(basename "$blocked_file").txt"
  if (exec 3<"$blocked_file") 2>"$denial_file"; then
    exit 33
  fi
  denial=$(<"$denial_file")
  [[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]
done

for blocked_file in \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/config.json" \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/settings.json" \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/Library/Keychains/copilot-cli.fake" \
  "$SWALLOWTAIL_BLOCKED_REPOSITORY/.swallowtail-write-probe"; do
  denial_file="$scratch/resource-write-denied-$(basename "$blocked_file").txt"
  if (printf 'must-not-write' >"$blocked_file") 2>"$denial_file"; then
    exit 34
  fi
  denial=$(<"$denial_file")
  [[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]
done

record_has_pre_execution=false
while IFS= read -r line; do
  if [[ "$line" == *'"artifact_execution_started": false'* ]]; then
    record_has_pre_execution=true
  fi
done < "$SWALLOWTAIL_RECORD_PATH"
[[ "$record_has_pre_execution" == true ]]

IFS= read -r initialize
[[ "$initialize" == *'"method":"initialize"'* ]]
printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"agentInfo":{"name":"fake-copilot","version":"fake"}}}'
IFS= read -r session_new
[[ "$session_new" == *'"method":"session/new"'* ]]
printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"sessionId":"fake-session"}}'
IFS= read -r prompt
[[ "$prompt" == *'"method":"session/prompt"'* ]]
printf '%s\n' '{"jsonrpc":"2.0","id":99,"method":"session/request_permission","params":{"sessionId":"fake-session","toolCall":{"toolCallId":"fake-tool","status":"pending"},"options":[{"optionId":"allow_always","name":"Allow always","kind":"allow_always"},{"optionId":"allow_once","name":"Allow once","kind":"allow_once"},{"optionId":"reject_once","name":"Reject once","kind":"reject_once"},{"optionId":"reject_always","name":"Reject always","kind":"reject_always"}]}}'
IFS= read -r permission
[[ "$permission" == *'"id":99'* && "$permission" == *'"outcome":"cancelled"'* ]]
[[ "$permission" != *'"optionId":"allow_always"'* ]]
[[ ! -e "$SWALLOWTAIL_EFFECT_MARKER" ]]
printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"stopReason":"cancelled"}}'
'''


FAKE_AUTH_AGENT_SOURCE = r'''import hashlib, json, os, select, sys

auth_fd = int(os.environ["SWALLOWTAIL_AUTH_FD"])
credential = os.read(auth_fd, 128)
os.close(auth_fd)
if hashlib.sha256(credential).hexdigest() != "de9e245bec7a5380291b92667b75b0b789c8ce92bd2ce370e84f41f33d12af91":
    raise SystemExit(71)
if any(name in os.environ for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")):
    raise SystemExit(72)
marker = os.environ["SWALLOWTAIL_EFFECT_MARKER"]

def send(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()

def read_line(timeout):
    ready, _, _ = select.select([sys.stdin], [], [], timeout)
    if not ready:
        return None
    line = sys.stdin.buffer.readline()
    return json.loads(line) if line else None

def report(**values):
    sys.stderr.write(json.dumps(values, sort_keys=True) + "\n")
    sys.stderr.flush()

initialize = read_line(1.0)
if not initialize or initialize.get("method") != "initialize":
    raise SystemExit(73)
send({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"authMethods":[{"id":"copilot-login","name":"Fake delegated auth"}],"agentInfo":{"name":"fake-copilot-auth","version":"fake"}}})
authenticate = read_line(1.0)
if not authenticate or authenticate.get("method") != "authenticate":
    raise SystemExit(74)
if authenticate.get("params") != {"methodId":"copilot-login"}:
    raise SystemExit(75)
send({"jsonrpc":"2.0","id":2,"result":{}})
session_new = read_line(1.0)
if not session_new or session_new.get("method") != "session/new":
    raise SystemExit(76)
send({"jsonrpc":"2.0","id":3,"result":{"sessionId":"fake-auth-session","models":{"currentModelId":"fake-github-hosted-model"}}})
prompt = read_line(1.0)
if not prompt or prompt.get("method") != "session/prompt":
    raise SystemExit(77)
if os.path.exists(marker):
    raise SystemExit(78)
send({"jsonrpc":"2.0","id":99,"method":"session/request_permission","params":{"sessionId":"fake-auth-session","toolCall":{"toolCallId":"fake-tool","status":"pending"},"options":[{"optionId":"allow_once","name":"Allow once","kind":"allow_once"},{"optionId":"reject_once","name":"Reject once","kind":"reject_once"}]}})
permission = read_line(1.0)
if not permission or permission.get("id") != 99:
    raise SystemExit(79)
outcome = permission.get("result", {}).get("outcome", {}).get("outcome")
if outcome != "cancelled":
    selected = permission.get("result", {}).get("outcome", {}).get("optionId")
    if selected in ("allow_once", "allow_always"):
        with open(marker, "w", encoding="utf-8") as output:
            output.write("effect")
    report(permission_outcome="not-cancelled", effect_marker_present=os.path.exists(marker))
    raise SystemExit(80)
report(permission_outcome="cancelled", pending_prompt=True, effect_marker_present=os.path.exists(marker))
cancel = read_line(0.8)
if not cancel or cancel.get("method") != "session/cancel":
    raise SystemExit(81)
send({"jsonrpc":"2.0","id":4,"result":{"stopReason":"cancelled"}})
report(permission_outcome="cancelled", pending_prompt=False, stop_reason="cancelled", effect_marker_present=os.path.exists(marker))
'''


FAKE_DISCOVERY_AGENT_SOURCE = r'''import hashlib, json, os, select, sys

auth_fd = int(os.environ["SWALLOWTAIL_AUTH_FD"])
credential = os.read(auth_fd, 128)
os.close(auth_fd)
if hashlib.sha256(credential).hexdigest() != "de9e245bec7a5380291b92667b75b0b789c8ce92bd2ce370e84f41f33d12af91":
    raise SystemExit(91)
if any(name in os.environ for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")):
    raise SystemExit(92)

def send(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()

def read_line(timeout):
    ready, _, _ = select.select([sys.stdin], [], [], timeout)
    if not ready:
        return None
    line = sys.stdin.buffer.readline()
    return json.loads(line) if line else None

initialize = read_line(1.0)
if not initialize or initialize.get("method") != "initialize":
    raise SystemExit(93)
send({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"authMethods":[{"id":"copilot-login","name":"Fake delegated auth"}],"agentInfo":{"name":"fake-copilot-discovery","version":"fake"}}})
authenticate = read_line(1.0)
if not authenticate or authenticate.get("method") != "authenticate":
    raise SystemExit(94)
if authenticate.get("params") != {"methodId":"copilot-login"}:
    raise SystemExit(95)
send({"jsonrpc":"2.0","id":2,"result":{}})
session_new = read_line(1.0)
if not session_new or session_new.get("method") != "session/new":
    raise SystemExit(96)
send({"jsonrpc":"2.0","id":3,"result":{"sessionId":"fake-discovery-session","models":{"currentModelId":"fake-auto","availableModels":[{"modelId":"fake-auto"},{"modelId":"fake-gpt-5.4"}]},"configOptions":[{"id":"model","currentValue":"fake-auto","options":[{"value":"fake-auto"},{"value":"fake-gpt-5.4"}]}]}})
if read_line(1.0) is not None:
    raise SystemExit(97)
'''


FAKE_PERMISSION_AGENT_SOURCE = r'''import errno, json, os, sys, time

def write_diagnostic(error):
    try:
        with open(os.environ["SWALLOWTAIL_FAKE_DIAGNOSTIC"], "w", encoding="utf-8") as output:
            json.dump({"type": type(error).__name__, "errno": getattr(error, "errno", None)}, output)
    except BaseException:
        pass

def mark(stage):
    with open(os.environ["SWALLOWTAIL_FAKE_DIAGNOSTIC"], "w", encoding="utf-8") as output:
        json.dump({"stage": stage}, output)

def report_exception(kind, error, trace):
    mark("uncaught-exception")
    write_diagnostic(error)

sys.excepthook = report_exception
mark("python-started")

import select, signal, socket, subprocess

if sys.argv[1:] != ["--model", "auto", "--acp", "--stdio"]:
    raise SystemExit(101)
if any(name in os.environ for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")):
    raise SystemExit(102)
if os.environ.get("HOME") != os.environ["SWALLOWTAIL_BLOCKED_HOST_HOME"]:
    raise SystemExit(103)
if os.environ.get("COPILOT_MODEL") is not None or os.environ.get("COPILOT_HOME") is not None:
    raise SystemExit(104)
marker = os.environ["SWALLOWTAIL_EFFECT_MARKER"]

def denied(operation):
    try:
        operation()
    except OSError as error:
        if error.errno in (errno.EPERM, errno.EACCES):
            return
        raise
    raise SystemExit(105)

def read_file(path):
    with open(path, "rb") as source:
        source.read(1)

def write_file(path):
    with open(path, "wb") as output:
        output.write(b"must-not-write")

host_home = os.environ["SWALLOWTAIL_BLOCKED_HOST_HOME"]
repository = os.environ["SWALLOWTAIL_BLOCKED_REPOSITORY"]
denied(lambda: read_file(os.path.join(host_home, ".copilot", "config.json")))
denied(lambda: read_file(os.path.join(host_home, ".copilot", "settings.json")))
denied(lambda: read_file(os.path.join(repository, "README.md")))
denied(lambda: write_file(os.path.join(host_home, ".copilot", "settings.json")))
denied(lambda: write_file(os.path.join(repository, ".swallowtail-proof-write")))
mark("host-and-repository-denials-passed")
denied(lambda: socket.create_connection(("127.0.0.1", int(os.environ["SWALLOWTAIL_PROXY_PORT"]) + 1), 0.2))
denied(lambda: socket.create_connection(("203.0.113.1", 443), 0.2))
mark("network-denials-passed")
try:
    subprocess.run(["/bin/sh", "-c", "exit 0"], check=True, timeout=1)
except OSError as error:
    if error.errno not in (errno.EPERM, errno.EACCES):
        raise
else:
    raise SystemExit(106)
mark("shell-exec-denied")

def proxy_request(target, server_name=None):
    proxy_port = int(os.environ["SWALLOWTAIL_PROXY_PORT"])
    connection = socket.create_connection(("127.0.0.1", proxy_port), 1)
    connection.sendall(("CONNECT " + target + " HTTP/1.1\r\nHost: " + target + "\r\n\r\n").encode())
    response = bytearray()
    while b"\r\n\r\n" not in response:
        response.extend(connection.recv(1024))
    status = bytes(response).split(b"\r\n", 1)[0]
    if status.startswith(b"HTTP/1.1 200"):
        name = (server_name or target.split(":", 1)[0]).encode("ascii")
        server_name_entry = b"\x00" + len(name).to_bytes(2, "big") + name
        server_names = len(server_name_entry).to_bytes(2, "big") + server_name_entry
        sni_extension = b"\x00\x00" + len(server_names).to_bytes(2, "big") + server_names
        ciphers = b"\x00\x02\x13\x01"
        compression = b"\x01\x00"
        extensions = len(sni_extension).to_bytes(2, "big") + sni_extension
        hello = b"\x03\x03" + (b"R" * 32) + b"\x00" + ciphers + compression + extensions
        handshake = b"\x01" + len(hello).to_bytes(3, "big") + hello
        connection.sendall(b"\x16\x03\x01" + len(handshake).to_bytes(2, "big") + handshake)
        connection.close()
    else:
        connection.close()
    return status

if not proxy_request("api.github.com:443").startswith(b"HTTP/1.1 200"):
    raise SystemExit(107)
mark("allowlisted-proxy-passed")
if not proxy_request("api.github.com:443", "unlisted.example").startswith(b"HTTP/1.1 200"):
    raise SystemExit(108)
if not proxy_request("unlisted.example:443").startswith(b"HTTP/1.1 403"):
    raise SystemExit(109)
mark("proxy-negative-controls-passed")

def send(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()

def read_line(timeout):
    deadline = time.monotonic() + timeout
    while True:
        newline = pending_input.find(b"\n")
        if newline >= 0:
            line = bytes(pending_input[:newline])
            del pending_input[: newline + 1]
            return json.loads(line)
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            return None
        ready, _, _ = select.select([sys.stdin.fileno()], [], [], remaining)
        if not ready:
            return None
        chunk = os.read(sys.stdin.fileno(), 4096)
        if not chunk:
            return None
        pending_input.extend(chunk)

pending_input = bytearray()
initialize = read_line(1.0)
mark("initialize-request-received")
if not initialize or initialize.get("method") != "initialize":
    raise SystemExit(110)
send({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"authMethods":[{"id":"copilot-login","name":"Existing host login"}],"agentInfo":{"name":"fake-copilot","version":"1.0.93"}}})
session_new = read_line(1.0)
if not session_new or session_new.get("method") != "session/new":
    raise SystemExit(111)
send({"jsonrpc":"2.0","id":2,"result":{"sessionId":"fake-session","models":{"currentModelId":"fake-underlying-model"}}})
prompt = read_line(1.0)
if not prompt or prompt.get("method") != "session/prompt":
    raise SystemExit(112)
if os.path.exists(marker):
    raise SystemExit(113)
send({"jsonrpc":"2.0","id":99,"method":"session/request_permission","params":{"sessionId":"fake-session","toolCall":{"toolCallId":"fake-tool","status":"pending"},"options":[{"optionId":"allow_once","name":"Allow once","kind":"allow_once"},{"optionId":"reject_once","name":"Reject once","kind":"reject_once"}]}})
permission = read_line(1.0)
if not permission or permission.get("id") != 99:
    raise SystemExit(114)
if permission.get("result", {}).get("outcome") != {"outcome":"cancelled"}:
    raise SystemExit(115)
if os.path.exists(marker):
    raise SystemExit(116)
cancel = read_line(1.0)
mark("cancel-received:" + (str(cancel.get("method")) if isinstance(cancel, dict) else type(cancel).__name__))
if not cancel or cancel.get("method") != "session/cancel":
    raise SystemExit(117)
signal.signal(signal.SIGTERM, signal.SIG_IGN)
send({"jsonrpc":"2.0","id":3,"result":{"stopReason":"cancelled"}})
if os.path.exists(marker):
    raise SystemExit(118)
while True:
    time.sleep(1)
'''


def fake_delegated_credential(context: dict[str, Any]) -> bytes | None:
    """Return a non-secret fake capability only for an exact synthetic grant."""
    if context != {
        "account_ref": FAKE_ACCOUNT_REF,
        "entitlement": "confirmed-in-fake",
        "model_id": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_request": "one-use-copilot-session",
        "audience": FAKE_AUDIENCE,
        "provider_traffic": [FAKE_AUDIENCE],
    }:
        return None
    return FAKE_DELEGATED_CREDENTIAL


def discovery_keychain_service_allowed(
    service: str, approved_service: str
) -> bool:
    return bool(service) and service == approved_service


def discovery_fake_preflight(
    scratch: Path, host_home: Path, repository_root: Path
) -> dict[str, Any]:
    valid = {
        "account_ref": FAKE_ACCOUNT_REF,
        "entitlement": "confirmed-in-fake",
        "model_id": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_request": "one-use-copilot-session",
        "audience": FAKE_AUDIENCE,
        "provider_traffic": [FAKE_AUDIENCE],
    }
    allowed_origins = {"https://github.fake.test", "https://copilot.fake.test"}
    controls = {
        "wrong_account": fake_delegated_credential(
            {**valid, "account_ref": "github-account:other"}
        ) is None,
        "unapproved_endpoint": not discovery_endpoint_allowed(
            "https://unapproved.fake.test", allowed_origins
        ),
        "prompt_request": not discovery_request_allowed("session/prompt"),
        "config_write_rpc": not discovery_request_allowed(
            "session/set_config_option"
        ),
        "permission_callback": not discovery_callback_allowed(
            "session/request_permission"
        ),
        "tool_callback": not discovery_callback_allowed(
            "session/update", "tool_call"
        ),
        "model_output_callback": not discovery_callback_allowed(
            "session/update", "agent_message_chunk"
        ),
        "excess_invocation": not discovery_budget_allowed(2, 1.0),
        "excess_time": not discovery_budget_allowed(1, 60.001),
        "wrong_keychain_service": not discovery_keychain_service_allowed(
            "other-service", "copilot-cli"
        ),
        "auth_config_write": not discovery_write_allowed(
            host_home / ".copilot" / "config.json", scratch
        )
        and not discovery_write_allowed(
            host_home / ".copilot" / "settings.json", scratch
        ),
        "keychain_write": not discovery_write_allowed(
            host_home / "Library" / "Keychains" / "copilot-cli.fake", scratch
        ),
        "host_home_read_escape": not discovery_read_allowed(
            host_home / ".copilot" / "config.json", host_home, repository_root
        ),
        "repository_read_escape": not discovery_read_allowed(
            repository_root / "README.md", host_home, repository_root
        ),
    }
    failed = [name for name, rejected in controls.items() if not rejected]
    if failed:
        raise RuntimeError(f"discovery fake admitted negative controls: {failed}")
    if not all(
        discovery_request_allowed(method)
        for method in ("initialize", "authenticate", "session/new")
    ):
        raise RuntimeError("discovery fake rejected an allowed ACP operation")
    if not discovery_endpoint_allowed(
        "https://copilot.fake.test", allowed_origins
    ):
        raise RuntimeError("discovery fake rejected its exact synthetic endpoint")
    if not discovery_budget_allowed(1, 60.0):
        raise RuntimeError("discovery fake rejected the exact one-invocation budget")
    if not discovery_write_allowed(scratch / "record.json", scratch):
        raise RuntimeError("discovery fake rejected task-scratch writes")
    if not discovery_keychain_service_allowed("copilot-cli", "copilot-cli"):
        raise RuntimeError("discovery fake rejected its exact synthetic keychain service")
    return {
        "status": "passed",
        "allowed_acp_requests": sorted(DISCOVERY_REQUEST_METHODS),
        "allowed_synthetic_origins": sorted(allowed_origins),
        "negative_controls": {
            name: "rejected-before-original-start" for name in controls
        },
    }


def authenticated_fake_preflight(
    scratch: Path, host_home: Path, repository_root: Path
) -> dict[str, Any]:
    valid = {
        "account_ref": FAKE_ACCOUNT_REF,
        "entitlement": "confirmed-in-fake",
        "model_id": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_request": "one-use-copilot-session",
        "audience": FAKE_AUDIENCE,
        "provider_traffic": [FAKE_AUDIENCE],
    }
    if fake_delegated_credential(valid) != FAKE_DELEGATED_CREDENTIAL:
        raise RuntimeError("valid fake host delegation was not admitted")

    negative_cases: dict[str, Any] = {
        "wrong_account": {**valid, "account_ref": "github-account:other"},
        "wrong_audience": {**valid, "audience": "unapproved.fake.test"},
        "wrong_model": {**valid, "model_id": "fake-local-gemma-12b"},
        "missing_entitlement": {**valid, "entitlement": "missing"},
        "unexpected_credential_request": {**valid, "credential_request": "read-host-keychain"},
        "unexpected_provider_traffic": {
            **valid,
            "provider_traffic": [FAKE_AUDIENCE, "unapproved.fake.test"],
        },
    }
    rejected: dict[str, dict[str, Any]] = {}
    for name, context in negative_cases.items():
        if fake_delegated_credential(context) is not None:
            raise RuntimeError(f"fake preflight admitted negative case {name}")
        rejected[name] = {
            "status": "rejected-before-process-start",
            "prompt_sent": False,
            "effect_marker_present": False,
        }
    return {
        "status": "passed",
        "accepted_fake_grant": "exact-account-entitlement-model-policy-and-audience",
        "credential_transport": "one-use-inherited-pipe",
        "credential_environment_variables": "absent",
        "negative_controls": rejected,
        "discovery_controls": discovery_fake_preflight(
            scratch, host_home, repository_root
        ),
    }


def discovery_session_observation(result: dict[str, Any]) -> dict[str, Any]:
    def safe_model_id(value: Any) -> str | None:
        if (
            isinstance(value, str)
            and 0 < len(value) <= 160
            and re.fullmatch(r"[A-Za-z0-9._:/+-]+", value)
        ):
            return value
        return None

    models = result.get("models")
    if not isinstance(models, dict):
        models = {}
    current_model_id = safe_model_id(models.get("currentModelId"))
    available = models.get("availableModels", [])
    model_ids = {
        model_id
        for model in available
        if isinstance(model, dict)
        for model_id in (safe_model_id(model.get("modelId")),)
        if model_id is not None
    } if isinstance(available, list) else set()
    if current_model_id is not None:
        model_ids.add(current_model_id)

    model_config: dict[str, Any] | None = None
    options = result.get("configOptions", [])
    if isinstance(options, list):
        for option in options:
            if not isinstance(option, dict) or option.get("id") != "model":
                continue
            values = option.get("options", [])
            value_ids = {
                model_id
                for item in values
                if isinstance(item, dict)
                for model_id in (safe_model_id(item.get("value")),)
                if model_id is not None
            } if isinstance(values, list) else set()
            model_config = {
                "option_id": "model",
                "current_value": safe_model_id(option.get("currentValue")),
                "available_values": sorted(value_ids),
            }
            model_ids.update(value_ids)
            break
    return {
        "current_model_id": current_model_id,
        "available_model_ids": sorted(model_ids),
        "model_config_option": model_config,
    }


def launch_acp(
    executable: Path,
    command: list[str],
    scratch: Path,
    host_home: Path,
    record_path: Path,
    exact_version: str | None,
    *,
    delegated_credential: bytes | None = None,
    authenticate_method_id: str | None = None,
    run_timeout_seconds: float = RUN_TIMEOUT_SECONDS,
    mode: str = "permission-proof",
    repository_root: Path = ROOT,
) -> dict[str, Any]:
    if not command or Path(command[0]) != executable:
        raise ValueError("the launched executable must match the selected artifact")
    if mode not in {"permission-proof", "pre-prompt-discovery"}:
        raise ValueError("unknown Copilot ACP harness mode")
    scratch.mkdir(parents=True, exist_ok=True)
    marker = scratch / "permission-effect-marker"
    if marker.exists():
        marker.unlink()
    auth_read_fd: int | None = None
    pass_fds: tuple[int, ...] = ()
    env = clean_child_environment(scratch, include_fake_token=delegated_credential is None)
    if delegated_credential is not None:
        auth_read_fd, auth_write_fd = os.pipe()
        try:
            written = os.write(auth_write_fd, delegated_credential)
            if written != len(delegated_credential):
                raise OSError("short write while delegating fake capability")
        except OSError:
            os.close(auth_read_fd)
            auth_read_fd = None
            raise
        finally:
            os.close(auth_write_fd)
        pass_fds = (auth_read_fd,)
        env["SWALLOWTAIL_AUTH_FD"] = str(auth_read_fd)
        env["SWALLOWTAIL_AUTH_TRANSPORT"] = "one-use-inherited-pipe"
    env.update(
        {
            "SWALLOWTAIL_BLOCKED_HOST_HOME": str(host_home.resolve()),
            "SWALLOWTAIL_EXPECTED_HOME": env["HOME"],
            "SWALLOWTAIL_RECORD_PATH": str(record_path.resolve()),
            "SWALLOWTAIL_EFFECT_MARKER": str(marker),
            "SWALLOWTAIL_BLOCKED_REPOSITORY": str(repository_root.resolve()),
        }
    )
    profile = sandbox_profile(scratch, host_home, repository_root)
    argv = launch_command(profile, command, env)
    started = utc_now()
    try:
        process = subprocess.Popen(
            argv,
            cwd=scratch,
            env={
                "HOME": str(host_home.resolve()),
                "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                "TMPDIR": str((scratch / "tmp").resolve()),
            },
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE if exact_version is None else subprocess.DEVNULL,
            bufsize=0,
            start_new_session=True,
            pass_fds=pass_fds,
        )
    finally:
        if auth_read_fd is not None:
            os.close(auth_read_fd)
    client = StdioClient(process)
    evidence: dict[str, Any] = {
        "started_at_utc": started,
        "selected_argv": (
            [Path(command[0]).name, *command[-2:]]
            if exact_version
            else ["python3", "fake ACP agent"]
        ),
        "initialize": "not-reached",
        "auth_method_ids": [],
        "authenticate": "not-requested",
        "authenticate_method_selected": None,
        "session_new": "not-reached",
        "session_prompt": "not-reached",
        "permission_request_observed": False,
        "permission_methods_observed": [],
        "permission_reply": "none",
        "session_cancel_sent": False,
        "session_updates": 0,
        "tool_call_updates": 0,
        "unexpected_messages": 0,
        "timed_out": False,
        "stream_failed": False,
    }
    if delegated_credential is not None:
        evidence["credential_transport"] = "one-use-inherited-pipe"
        evidence["credential_environment_variables_absent"] = not any(
            name in env for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")
        )
        evidence["effect_marker_absent_before_prompt"] = not marker.exists()

    def send_request(method: str, request: dict[str, Any]) -> None:
        if mode == "pre-prompt-discovery" and not discovery_request_allowed(method):
            raise RuntimeError(f"discovery operation is not permitted: {method}")
        client.send(request)

    try:
        send_request(
            "initialize",
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientCapabilities": {}},
            }
        )
        response = wait_for_response(
            client,
            1,
            evidence,
            READ_TIMEOUT_SECONDS,
            discovery_mode=mode == "pre-prompt-discovery",
        )
        if response is None:
            evidence["initialize"] = "no-result"
            return evidence
        if record_rpc_error("initialize", response, evidence):
            return evidence
        if "result" not in response or not isinstance(response["result"], dict):
            evidence["initialize"] = "invalid-response"
            return evidence
        evidence["initialize"] = "success"
        result = response["result"]
        evidence["auth_method_ids"] = auth_method_ids(result)
        reported = result.get("agentInfo", {}).get("version")
        evidence["reported_version_matches"] = (
            reported == exact_version if exact_version else reported == "fake"
        )

        next_request_id = 2
        if authenticate_method_id is not None:
            if authenticate_method_id not in evidence["auth_method_ids"]:
                evidence["authenticate"] = "advertised-method-missing"
                return evidence
            evidence["authenticate_method_selected"] = authenticate_method_id
            send_request(
                "authenticate",
                {
                    "jsonrpc": "2.0",
                    "id": next_request_id,
                    "method": "authenticate",
                    "params": {"methodId": authenticate_method_id},
                }
            )
            response = wait_for_response(
                client,
                next_request_id,
                evidence,
                READ_TIMEOUT_SECONDS,
                discovery_mode=mode == "pre-prompt-discovery",
            )
            if response is None:
                evidence["authenticate"] = "no-result"
                return evidence
            if record_rpc_error("authenticate", response, evidence):
                evidence["authenticate"] = "rpc-error"
                return evidence
            if "result" not in response or not isinstance(response["result"], dict):
                evidence["authenticate"] = "invalid-response"
                return evidence
            evidence["authenticate"] = "success"
            next_request_id += 1

        send_request(
            "session/new",
            {
                "jsonrpc": "2.0",
                "id": next_request_id,
                "method": "session/new",
                "params": {"cwd": str(scratch), "mcpServers": []},
            }
        )
        response = wait_for_response(
            client,
            next_request_id,
            evidence,
            READ_TIMEOUT_SECONDS,
            discovery_mode=mode == "pre-prompt-discovery",
        )
        if response is None:
            evidence["session_new"] = "no-result"
            return evidence
        if record_rpc_error("session_new", response, evidence):
            return evidence
        if "result" not in response or not isinstance(response["result"], dict):
            evidence["session_new"] = "invalid-response"
            return evidence
        evidence["session_new"] = "success"
        session_id = response["result"].get("sessionId")
        models = response["result"].get("models", {})
        if isinstance(models, dict) and isinstance(models.get("currentModelId"), str):
            evidence["observed_model_id"] = models["currentModelId"]
        if not isinstance(session_id, str):
            evidence["session_new"] = "result-without-session-id"
            return evidence
        if mode == "pre-prompt-discovery":
            evidence["session_prompt"] = "not-authorized-not-sent"
            evidence["discovery_observation"] = discovery_session_observation(
                response["result"]
            )
            return evidence
        prompt_request_id = next_request_id + 1

        marker_prompt = f"Run the tool needed to create this empty marker file, then stop: {marker}"
        send_request(
            "session/prompt",
            {
                "jsonrpc": "2.0",
                "id": prompt_request_id,
                "method": "session/prompt",
                "params": {
                    "sessionId": session_id,
                    "prompt": [{"type": "text", "text": marker_prompt}],
                },
            }
        )
        evidence["session_prompt"] = "sent"
        response = wait_for_response(
            client, prompt_request_id, evidence, run_timeout_seconds
        )
        if response is None:
            try:
                send_request(
                    "session/cancel",
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session_id},
                    }
                )
                evidence["session_cancel_sent"] = True
                response = wait_for_response(client, prompt_request_id, evidence, 2.0)
            except (BrokenPipeError, OSError):
                evidence["stream_failed"] = True
        if response is not None:
            if "result" in response:
                evidence["session_prompt"] = str(
                    response["result"].get("stopReason", "result")
                )[:40]
            elif record_rpc_error("session_prompt", response, evidence):
                pass
            else:
                evidence["session_prompt"] = "error-response"
        else:
            evidence["session_prompt"] = "no-result"
        if evidence["permission_request_observed"]:
            evidence["permission_policy_outcome"] = "cancelled"
        elif evidence["session_prompt"] == "no-result":
            evidence["permission_policy_outcome"] = "not-reached"
        else:
            evidence["permission_policy_outcome"] = "no-request-observed"
        return evidence
    finally:
        client.close()
        code, forced = stop_process(process)
        evidence["exit_code"] = code
        evidence["forced_process_group_kill"] = forced
        evidence["process_joined"] = process.returncode is not None
        evidence["effect_marker_present"] = marker.exists()
        if exact_version is None and process.stderr is not None:
            diagnostic = process.stderr.read(4096).decode("utf-8", errors="replace")
            evidence["fake_diagnostic"] = (
                diagnostic.replace(str(host_home), "<host-home>")
                .replace(str(scratch), "<task-scratch>")
                .replace(FAKE_TOKEN, "<fake-token>")
                .replace(
                    FAKE_DELEGATED_CREDENTIAL.decode(),
                    "<fake-delegated-credential>",
                )[:500]
            )


def safe_model_id(value: Any) -> str | None:
    if not isinstance(value, str) or not re.fullmatch(r"[A-Za-z0-9._:+/-]{1,100}", value):
        return None
    if value.startswith(("gho_", "ghp_", "ghs_", "github_pat_", "Bearer")):
        return None
    return value


def capture_model_observation(result: dict[str, Any]) -> dict[str, Any]:
    models = result.get("models")
    current = None
    if isinstance(models, dict):
        current = safe_model_id(models.get("currentModelId"))
    underlying = current if current and current.lower() != "auto" else None
    return {
        "session_new_current_model_id": current,
        "underlying_model_identity": underlying,
        "underlying_model_identity_status": "observed" if underlying else "unobserved",
        "observation_source": "session/new result.models.currentModelId"
        if current
        else "not-exposed-by-session/new",
    }


def capture_model_update(update: dict[str, Any], observations: list[dict[str, Any]]) -> None:
    if update.get("sessionUpdate") != "config_option_update":
        return
    option_id = update.get("configOptionId", update.get("configId"))
    value = update.get("value", update.get("currentValue"))
    config_option = update.get("configOption")
    if isinstance(config_option, dict):
        option_id = config_option.get("id", option_id)
        value = config_option.get("currentValue", value)
    if option_id != "model":
        return
    model_id = safe_model_id(value)
    if model_id is not None:
        observations.append(
            {
                "model_id": model_id,
                "source": "session/update config_option_update for model",
            }
        )


def wait_for_original_response(
    client: StdioClient,
    request_id: int,
    evidence: dict[str, Any],
    deadline: float,
) -> dict[str, Any] | None:
    while time.monotonic() < deadline:
        try:
            message = client.receive(
                min(READ_TIMEOUT_SECONDS, deadline - time.monotonic())
            )
        except TimeoutError:
            evidence["timed_out"] = True
            return None
        except (EOFError, ValueError, json.JSONDecodeError):
            evidence["stream_failed"] = True
            return None
        method = message.get("method")
        if method == "session/update":
            params = message.get("params")
            update = params.get("update") if isinstance(params, dict) else None
            if isinstance(update, dict):
                evidence["session_updates"] += 1
                if update.get("sessionUpdate") == "tool_call":
                    evidence["tool_call_updates"] += 1
                capture_model_update(update, evidence["model_observations"])
            continue
        if method == "session/request_permission":
            evidence["pre_prompt_permission_request"] = True
            permission_id = message.get("id")
            if isinstance(permission_id, int):
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": permission_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
            continue
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1
    evidence["timed_out"] = True
    return None


def safe_rpc_error(response: dict[str, Any]) -> dict[str, Any] | None:
    error = response.get("error")
    if not isinstance(error, dict):
        return None
    code = error.get("code")
    safe_code = code if isinstance(code, int) and not isinstance(code, bool) else None
    message = error.get("message")
    normalized = message.strip().lower() if isinstance(message, str) else ""
    if normalized == "authentication required":
        kind = "authentication-required"
    elif any(word in normalized for word in ("quota", "entitlement", "rate limit", "billing")):
        kind = "quota-or-entitlement-stop"
    else:
        kind = "redacted-provider-error"
    return {
        "kind": kind,
        "code": safe_code,
        "message": "Authentication required" if kind == "authentication-required" else "redacted",
        "data_omitted": "data" in error,
    }


def permission_evidence_is_complete(evidence: dict[str, Any]) -> bool:
    proxy = evidence.get("proxy", {})
    return bool(
        evidence.get("initialize") == "success"
        and evidence.get("reported_version_matches") is True
        and evidence.get("session_new") == "success"
        and evidence.get("auto_session_created") is True
        and not evidence.get("pre_prompt_permission_request")
        and evidence.get("prompt_requests_sent") == 1
        and evidence.get("credential_environment_variables_absent") is True
        and evidence.get("permission_request_count") == 1
        and evidence.get("permission_reply") == "cancelled"
        and not evidence.get("permission_callback_ambiguous")
        and not evidence.get("late_permission_response_accepted")
        and evidence.get("session_cancel_sent") is True
        and evidence.get("session_prompt") == "cancelled"
        and not evidence.get("effect_marker_present")
        and evidence.get("process_joined") is True
        and evidence.get("process_group_joined") is True
        and evidence.get("proxy_threads_joined") is True
        and evidence.get("elapsed_seconds", LIVE_PERMISSION_SECONDS + 1)
        <= LIVE_PERMISSION_SECONDS
        and proxy.get("unlisted_destination_count") == 0
        and proxy.get("sni_mismatch_count") == 0
        and proxy.get("connection_failure_count") == 0
    )


def wait_for_original_prompt(
    client: StdioClient,
    request_id: int,
    session_id: str,
    evidence: dict[str, Any],
    deadline: float,
) -> dict[str, Any] | None:
    while time.monotonic() < deadline:
        try:
            message = client.receive(
                min(READ_TIMEOUT_SECONDS, deadline - time.monotonic())
            )
        except TimeoutError:
            evidence["timed_out"] = True
            return None
        except (EOFError, ValueError, json.JSONDecodeError):
            evidence["stream_failed"] = True
            return None
        method = message.get("method")
        if method == "session/request_permission":
            evidence["permission_request_count"] += 1
            params = message.get("params")
            options = params.get("options", []) if isinstance(params, dict) else []
            if isinstance(options, list):
                evidence["permission_option_ids"].append(
                    sorted(
                        {
                            option["optionId"]
                            for option in options
                            if isinstance(option, dict)
                            and isinstance(option.get("optionId"), str)
                            and option["optionId"] in {
                                "allow_once",
                                "allow_always",
                                "reject_once",
                                "reject_always",
                            }
                        }
                    )
                )
            permission_id = message.get("id")
            if not isinstance(permission_id, int):
                evidence["permission_callback_ambiguous"] = True
                if not evidence["session_cancel_sent"]:
                    client.send(
                        {
                            "jsonrpc": "2.0",
                            "method": "session/cancel",
                            "params": {"sessionId": session_id},
                        }
                    )
                    evidence["session_cancel_sent"] = True
                continue
            pending_wait = PendingPermissionWait()
            pending_wait.abandon()
            evidence["pending_permission_wait_abandoned"] = (
                pending_wait.state == "abandoned"
            )
            evidence["late_permission_response_accepted"] = pending_wait.resolve(
                "allow_once"
            )
            client.send(
                {
                    "jsonrpc": "2.0",
                    "id": permission_id,
                    "result": {"outcome": {"outcome": "cancelled"}},
                }
            )
            evidence["permission_reply"] = "cancelled"
            if not evidence["session_cancel_sent"]:
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session_id},
                    }
                )
                evidence["session_cancel_sent"] = True
            continue
        if method == "session/update":
            params = message.get("params")
            update = params.get("update") if isinstance(params, dict) else None
            if isinstance(update, dict):
                evidence["session_updates"] += 1
                if update.get("sessionUpdate") == "tool_call":
                    evidence["tool_call_updates"] += 1
                capture_model_update(update, evidence["model_observations"])
            continue
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1
    evidence["timed_out"] = True
    return None


def run_permission_attempt(
    binary: Path,
    version: str,
    scratch: Path,
    record_dir: Path,
    record_path: Path,
    record: dict[str, Any],
    repository_root: Path,
    plan: dict[str, Any],
) -> dict[str, Any]:
    host_home = Path(os.environ["HOME"]).resolve(strict=True)
    scratch.mkdir(parents=True, exist_ok=False)
    (scratch / "tmp").mkdir()
    marker = scratch / "permission-effect-marker"
    proxy = CopilotEgressProxy(
        set(plan["network_policy"]["allowed_hosts"]),
        tuple(plan["network_policy"]["allowed_subdomains"]),
        time.monotonic() + LIVE_PERMISSION_SECONDS,
    )
    proxy_port = proxy.start()
    executable = binary.resolve(strict=True)
    command = [str(executable), "--model", "auto", "--acp", "--stdio"]
    profile = permission_sandbox_profile(
        scratch, record_dir, host_home, repository_root, executable, proxy_port
    )
    environment = original_child_environment(host_home, scratch, proxy_port)
    profile_digest = hashlib.sha256(profile.encode()).hexdigest()
    packages = inventory_packages(verify_inventory())
    native_package = packages[("@github/copilot-darwin-arm64", version)]
    binary_record = next(
        item for item in native_package["files"] if item["path"] == "package/copilot"
    )
    binary_sha256 = binary_record["sha256"]
    invocation: dict[str, Any] = {
        "version": version,
        "binary_sha256": binary_sha256,
        "invocation_consumed": True,
        "pre_execution_fsynced": False,
        "original_start_issued": False,
        "pre_execution_record": {
            "requires_fsync_before_original_start": True,
            "identity": {
                "package": "@github/copilot-darwin-arm64",
                "version": version,
                "platform": "darwin-arm64",
                "binary_sha256": binary_sha256,
            },
            "account_access_ref": "github-account:betterthanclay",
            "account_identity_observation": "operator-reported-reference; not independently queried",
            "credential_boundary": {
                "source": "existing original-CLI host-owned login",
                "normal_keychain_access": "permitted through com.apple.securityd",
                "keychain_item_level_mediation": "unavailable; no per-item filter claimed",
                "token_handling_by_harness": "never extracted, copied, logged, or persisted",
                "token_environment_variables_absent": True,
                "copilot_home_relocated": False,
                "acp_authenticate_request_sent": False,
            },
            "selection_policy": {
                "requested": "Auto",
                "binding": "original CLI --model auto process argument",
                "underlying_model_identity": "record only if structured ACP data exposes it; otherwise unobserved",
                "vendor_managed_retries": "allowed within this prompt and the process/network ceiling; internal count unobserved",
                "harness_model_fallbacks": 0,
            },
            "network_policy": {
                "authority": "official Copilot authentication and provider destinations",
                "source": plan["network_policy"]["source"],
                "allowed_exact_hosts": plan["network_policy"]["allowed_hosts"],
                "allowed_subdomains": plan["network_policy"]["allowed_subdomains"],
                "default": "deny",
                "enforcement": "sandbox permits only local CONNECT proxy; proxy verifies official CONNECT host and matching TLS SNI before external dial",
                "unexpected_destinations": "reject and fail attempt without expanding policy",
                "proxy_endpoint": {"host": "127.0.0.1", "port": proxy_port},
            },
            "containment": {
                "profile_sha256": profile_digest,
                "host_home_reads": "denied",
                "repository_reads": "denied",
                "writes": "task scratch only",
        "subprocess_escape": "forks inherit this sandbox; process exec limited to this exact artifact; shell exec denied",
                "keychain_service_lookup": "com.apple.securityd only; item-level reads trusted to hash-verified vendor CLI",
            },
            "action": {
                "kind": "create-one-empty-task-scratch-marker",
                "path": "task-scratch/permission-effect-marker",
                "permission_required": True,
                "approval": "never",
                "max_effects": 0,
            },
            "budgets": plan["budgets"],
            "selected_argv": ["copilot", "--model", "auto", "--acp", "--stdio"],
        },
        "pre_execution_record_sha256": None,
        "started_at_utc": utc_now(),
        "permission_request_count": 0,
        "permission_reply": "none",
        "session_cancel_sent": False,
        "effect_marker_present": False,
        "model_observations": [],
    }
    invocation["pre_execution_record_sha256"] = json_digest(
        invocation["pre_execution_record"]
    )
    invocation["pre_execution_fsynced"] = True
    record["invocations"].append(invocation)
    write_json_durable(record_path, record)
    saved = load_json(record_path)
    if saved.get("invocations", [])[-1] != invocation:
        proxy_joined = proxy.stop_and_join(LIVE_CLEANUP_SECONDS)
        if not proxy_joined:
            raise RuntimeError("proxy threads survived failed pre-execution record read-back")
        raise RuntimeError(f"pre-execution record did not persist before {version} start")

    started = time.monotonic()
    deadline = started + LIVE_PERMISSION_SECONDS - (2 * LIVE_CLEANUP_SECONDS)
    proxy.deadline = started + LIVE_PERMISSION_SECONDS - LIVE_CLEANUP_SECONDS
    evidence: dict[str, Any] = {
        "initialize": "not-reached",
        "reported_version_matches": False,
        "auth_method_ids": [],
        "authenticate_request_sent": False,
        "session_new": "not-reached",
        "auto_policy_requested": True,
        "auto_session_created": False,
        "model_observations": [],
        "model_identity_observation_source": None,
        "session_prompt": "not-reached",
        "prompt_requests_sent": 0,
        "permission_request_count": 0,
        "permission_option_ids": [],
        "permission_reply": "none",
        "pending_permission_wait_abandoned": False,
        "late_permission_response_accepted": False,
        "permission_callback_ambiguous": False,
        "session_cancel_sent": False,
        "session_updates": 0,
        "tool_call_updates": 0,
        "unexpected_messages": 0,
        "pre_prompt_permission_request": False,
        "timed_out": False,
        "stream_failed": False,
        "effect_marker_absent_before_prompt": not marker.exists(),
        "credential_environment_variables_absent": not any(
            name in environment
            for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")
        ),
        "proxy_threads_joined": False,
        "process_group_joined": False,
    }
    process: subprocess.Popen[bytes] | None = None
    client: StdioClient | None = None
    try:
        process = subprocess.Popen(
            ["/usr/bin/sandbox-exec", "-p", profile, *command],
            cwd=scratch,
            env=environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            bufsize=0,
            start_new_session=True,
        )
        invocation["original_start_issued"] = True
        record["artifact_execution_started"] = True
        write_json_durable(record_path, record)
        client = StdioClient(process)
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientCapabilities": {}},
            }
        )
        response = wait_for_original_response(client, 1, evidence, deadline)
        if response is not None:
            error = safe_rpc_error(response)
            if error:
                evidence["initialize_error"] = error
                evidence["initialize"] = error["kind"]
            elif isinstance(response.get("result"), dict):
                evidence["initialize"] = "success"
                evidence["auth_method_ids"] = auth_method_ids(response["result"])
                agent_info = response["result"].get("agentInfo")
                reported = agent_info.get("version") if isinstance(agent_info, dict) else None
                evidence["reported_version_matches"] = reported == version
                evidence["reported_version"] = (
                    reported if reported == version else "mismatch-or-unobserved"
                )
        if evidence["initialize"] == "success" and evidence["reported_version_matches"]:
            client.send(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "method": "session/new",
                    "params": {"cwd": str(scratch), "mcpServers": []},
                }
            )
            response = wait_for_original_response(client, 2, evidence, deadline)
            if response is not None:
                error = safe_rpc_error(response)
                if error:
                    evidence["session_new_error"] = error
                    evidence["session_new"] = error["kind"]
                elif isinstance(response.get("result"), dict):
                    result = response["result"]
                    session_id = result.get("sessionId")
                    evidence["session_new"] = "success" if isinstance(session_id, str) else "result-without-session-id"
                    evidence["auto_session_created"] = isinstance(session_id, str)
                    model_observation = capture_model_observation(result)
                    evidence["model_identity_observation_source"] = model_observation["observation_source"]
                    evidence["underlying_model_identity"] = model_observation["underlying_model_identity"]
                    if model_observation["session_new_current_model_id"]:
                        evidence["model_observations"].append(
                            {
                                "model_id": model_observation["session_new_current_model_id"],
                                "source": model_observation["observation_source"],
                            }
                        )
                    if isinstance(session_id, str):
                        marker_prompt = (
                            "Use the shell tool once to create an empty file at this exact task-scratch path: "
                            f"{marker}. Do not perform any other action. The host will cancel the permission request."
                        )
                        client.send(
                            {
                                "jsonrpc": "2.0",
                                "id": 3,
                                "method": "session/prompt",
                                "params": {
                                    "sessionId": session_id,
                                    "prompt": [{"type": "text", "text": marker_prompt}],
                                },
                            }
                        )
                        evidence["prompt_requests_sent"] = 1
                        evidence["session_prompt"] = "sent"
                        response = wait_for_original_prompt(
                            client, 3, session_id, evidence, deadline
                        )
                        if response is None:
                            evidence["session_prompt"] = "no-result"
                            if not evidence["session_cancel_sent"]:
                                try:
                                    client.send(
                                        {
                                            "jsonrpc": "2.0",
                                            "method": "session/cancel",
                                            "params": {"sessionId": session_id},
                                        }
                                    )
                                    evidence["session_cancel_sent"] = True
                                except (BrokenPipeError, OSError):
                                    evidence["stream_failed"] = True
                        elif safe_rpc_error(response):
                            evidence["prompt_error"] = safe_rpc_error(response)
                            evidence["session_prompt"] = evidence["prompt_error"]["kind"]
                        elif isinstance(response.get("result"), dict):
                            reason = response["result"].get("stopReason")
                            evidence["session_prompt"] = (
                                reason if isinstance(reason, str) and len(reason) <= 40 else "result"
                            )
        if evidence["session_new"] in {"not-reached", "authentication-required", "quota-or-entitlement-stop"}:
            evidence["stop_before_prompt_reason"] = evidence["session_new"]
    except (OSError, RuntimeError, ValueError, TimeoutError, EOFError) as error:
        evidence["launch_error_class"] = type(error).__name__
    finally:
        if client is not None:
            client.close()
        process_exit_code: int | None = None
        forced_stop = False
        process_group_joined = True
        if process is not None:
            process_exit_code, forced_stop, process_group_joined = stop_owned_process_group(
                process, LIVE_CLEANUP_SECONDS
            )
        evidence["process_exit_code"] = process_exit_code
        evidence["forced_process_group_stop"] = forced_stop
        evidence["process_joined"] = process is not None and process.returncode is not None
        evidence["process_group_joined"] = process_group_joined
        evidence["effect_marker_present"] = marker.exists()
        elapsed_before_proxy_close = time.monotonic() - started
        remaining_cleanup = max(
            0.0, LIVE_PERMISSION_SECONDS - elapsed_before_proxy_close
        )
        evidence["proxy_threads_joined"] = proxy.stop_and_join(
            min(LIVE_CLEANUP_SECONDS, remaining_cleanup)
        )
        evidence["proxy"] = proxy.snapshot()
        evidence["elapsed_seconds"] = round(time.monotonic() - started, 3)

    evidence["observed_network_audiences_only"] = evidence["proxy"][
        "allowed_destination_hosts"
    ]
    for observation in evidence["model_observations"]:
        model_id = observation.get("model_id")
        if isinstance(model_id, str) and model_id.lower() != "auto":
            evidence["underlying_model_identity"] = model_id
            evidence["model_identity_observation_source"] = observation["source"]
            break
    evidence["underlying_model_identity"] = evidence.get("underlying_model_identity") or "unobserved"
    evidence["vendor_internal_request_count"] = "unobserved"
    evidence["vendor_managed_retry_policy"] = "allowed within one ACP prompt and enforced deadline"
    evidence["harness_prompt_retry_count"] = 0
    evidence["harness_resend_count"] = 0
    evidence["harness_model_fallback_count"] = 0
    evidence["permission_evidence_complete"] = permission_evidence_is_complete(evidence)
    evidence["safe_to_continue_older_versions"] = evidence[
        "permission_evidence_complete"
    ]
    invocation["result"] = evidence
    invocation["result_recorded_at_utc"] = utc_now()
    record["permission_boundary_proven_for_all_targets"] = all(
        item.get("result", {}).get("permission_evidence_complete") is True
        for item in record["invocations"]
    )
    write_json_durable(record_path, record)
    return evidence


def require_macos_sandbox() -> str:
    if sys.platform != "darwin":
        raise RuntimeError("the offline containment proof requires macOS sandbox-exec")
    sandbox = "/usr/bin/sandbox-exec"
    if not os.path.isfile(sandbox) or not os.access(sandbox, os.X_OK):
        raise RuntimeError("/usr/bin/sandbox-exec is unavailable")
    return "/bin/bash"


def validate_fake_invocation_record(record: dict[str, Any]) -> None:
    if record.get("schema") != "copilot-cli-acp-authenticated-invocation.v1":
        raise ValueError("unexpected authenticated invocation record schema")
    if record.get("execution_kind") != "fake_control" or record.get("original_artifact_started"):
        raise ValueError("fake invocation record confuses a control with original execution")
    if record.get("pre_execution_fsynced") is not True:
        raise ValueError("fake invocation is missing its durable pre-execution record")
    pre_execution = record.get("pre_execution_record")
    if not isinstance(pre_execution, dict):
        raise ValueError("fake invocation record has no pre-execution object")
    expected_digest = json_digest(pre_execution)
    if record.get("pre_execution_sha256") != expected_digest:
        raise ValueError("fake invocation result does not bind the persisted pre-execution record")
    result = record.get("result")
    if not isinstance(result, dict):
        raise ValueError("fake invocation record has no result")
    if result.get("pre_execution_sha256") != expected_digest:
        raise ValueError("result record does not correlate to the pre-execution record")
    if result.get("pre_execution_record_sha256") != expected_digest:
        raise ValueError("result record is missing its pre-execution record digest")
    required_result_fields = {
        "recorded_provider_selection",
        "underlying_model_identity_and_observation_source",
        "pre_execution_record_sha256",
        "auth_method_observed",
        "permission_request_count_and_safe_shape",
        "permission_response_cancelled_or_rejected",
        "pending_permission_wait_abandoned",
        "late_permission_response_accepted",
        "session_cancel_sent",
        "prompt_terminal_outcome",
        "effect_marker_absent_before_prompt",
        "effect_marker_absent_after_stop",
        "observed_network_audiences_only",
        "retry_resend_fallback_and_tool_attempt_counts",
        "process_exit_status_and_joined_state",
        "forced_process_group_stop",
        "elapsed_seconds",
    }
    if not required_result_fields <= set(result):
        raise ValueError("fake result record is missing required permission or cleanup evidence")
    serialized = json.dumps(record, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "test-only-delegated-capability",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"invocation record contains forbidden secret/path marker {forbidden!r}")


def authenticated_fake_run(
    scratch: Path, host_home: Path, repository_root: Path
) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    preflight = authenticated_fake_preflight(scratch, host_home, repository_root)
    credential = fake_delegated_credential(
        {
            "account_ref": FAKE_ACCOUNT_REF,
            "entitlement": "confirmed-in-fake",
            "model_id": FAKE_MODEL_ID,
            "model_policy": "pinned-no-fallback",
            "credential_request": "one-use-copilot-session",
            "audience": FAKE_AUDIENCE,
            "provider_traffic": [FAKE_AUDIENCE],
        }
    )
    if credential is None:
        raise RuntimeError("fake host broker withheld the valid test capability")

    auth_scratch = scratch / "authenticated-fake"
    auth_scratch.mkdir()
    record_path = auth_scratch / "authenticated-invocation.json"
    marker = auth_scratch / "permission-effect-marker"
    pre_execution = {
        "target_identity": {
            "route": "copilot-cli.acp",
            "version": "1.0.80",
            "binary_sha256": "fe779da7dd2342c1d23f0744873fa27d0251eaaee4dc6637fa53093639c0f3c9",
        },
        "executed_identity": {
            "kind": "fake-acp-auth-agent",
            "source_sha256": hashlib.sha256(FAKE_AUTH_AGENT_SOURCE.encode()).hexdigest(),
        },
        "account_access_ref": FAKE_ACCOUNT_REF,
        "entitlement": "synthetic-fake-approval-only",
        "selected_model": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_mechanism": "fake-host-broker-one-use-inherited-pipe",
        "network_audiences": [FAKE_AUDIENCE],
        "containment": "deny-all-network-and-host-home; task-scratch-writes-only",
        "intended_action": "create-one-empty-task-scratch-marker-if-approved",
        "approval_policy": "cancel-permission; never-approve",
        "budgets": {
            "prompts": 1,
            "seconds": MAX_AUTH_FAKE_SECONDS,
            "retries": 0,
            "resends": 0,
            "auto_fallbacks": 0,
            "permission_requests": 1,
            "tool_attempts": 1,
            "effects": 0,
        },
    }
    pre_execution_sha256 = json_digest(pre_execution)
    record = {
        "schema": "copilot-cli-acp-authenticated-invocation.v1",
        "execution_kind": "fake_control",
        "original_artifact_started": False,
        "pre_execution_fsynced": True,
        "pre_execution_sha256": pre_execution_sha256,
        "pre_execution_record": pre_execution,
        "result": None,
    }
    write_json_durable(record_path, record)
    persisted = load_json(record_path)
    if persisted != record or persisted.get("result") is not None:
        raise RuntimeError("fake pre-execution record did not persist before process start")

    started = time.monotonic()
    fake_agent = Path(sys.executable)
    outcome = launch_acp(
        fake_agent,
        [sys.executable, "-c", FAKE_AUTH_AGENT_SOURCE],
        auth_scratch,
        host_home,
        record_path,
        None,
        delegated_credential=credential,
        authenticate_method_id="copilot-login",
        run_timeout_seconds=0.25,
        repository_root=repository_root,
    )
    elapsed = time.monotonic() - started
    diagnostics = [
        json.loads(line)
        for line in outcome.get("fake_diagnostic", "").splitlines()
        if line.strip()
    ]
    final_agent_report = diagnostics[-1] if diagnostics else {}
    if marker.exists():
        raise RuntimeError("authenticated fake left an action marker after cancellation")
    if outcome.get("initialize") != "success" or outcome.get("session_new") != "success":
        raise RuntimeError("authenticated fake did not accept the delegated in-memory capability")
    if outcome.get("auth_method_ids") != ["copilot-login"]:
        raise RuntimeError("authenticated fake did not report the expected auth method")
    if (
        outcome.get("authenticate") != "success"
        or outcome.get("authenticate_method_selected") != "copilot-login"
    ):
        raise RuntimeError("authenticated fake ACP authenticate exchange did not complete")
    if outcome.get("observed_model_id") != FAKE_MODEL_ID:
        raise RuntimeError("authenticated fake did not report its selected model")
    if not outcome.get("permission_request_observed") or outcome.get("permission_reply") != "cancelled":
        raise RuntimeError("authenticated fake permission was not observed and cancelled")
    if len(outcome.get("permission_methods_observed", [])) != 1:
        raise RuntimeError("authenticated fake exceeded its one-permission-request budget")
    if outcome.get("permission_option_ids") != ["allow_once", "reject_once"]:
        raise RuntimeError("authenticated fake permission options were not safely recorded")
    if not outcome.get("pending_permission_wait_abandoned") or outcome.get("late_permission_response_accepted"):
        raise RuntimeError("authenticated fake kept a pending permission wait or accepted a late reply")
    if outcome.get("session_cancel_sent") is not True or outcome.get("session_prompt") != "cancelled":
        raise RuntimeError("authenticated fake prompt did not stop through ACP cancellation")
    if outcome.get("effect_marker_absent_before_prompt") is not True or outcome.get("effect_marker_present"):
        raise RuntimeError("authenticated fake marker was present before prompt or after stop")
    if outcome.get("credential_environment_variables_absent") is not True:
        raise RuntimeError("authenticated fake delegation leaked into token environment variables")
    if outcome.get("process_joined") is not True or outcome.get("exit_code") != 0:
        raise RuntimeError("authenticated fake child was not cleanly joined")
    if outcome.get("forced_process_group_kill") or elapsed > MAX_AUTH_FAKE_SECONDS:
        raise RuntimeError("authenticated fake exceeded its bounded cleanup window")
    if final_agent_report.get("permission_outcome") != "cancelled" or final_agent_report.get("effect_marker_present"):
        raise RuntimeError("authenticated fake child did not observe cancellation without an effect")

    record["result"] = {
        "pre_execution_sha256": pre_execution_sha256,
        "pre_execution_record_sha256": pre_execution_sha256,
        "recorded_provider_selection": outcome["observed_model_id"],
        "underlying_model_identity_and_observation_source": (
            "fake currentModelId from session/new"
        ),
        "auth_method_observed": {
            "advertised_ids": outcome["auth_method_ids"],
            "selected_method_id": outcome["authenticate_method_selected"],
            "result": outcome["authenticate"],
        },
        "permission_request_count_and_safe_shape": {
            "count": len(outcome["permission_methods_observed"]),
            "method": outcome["permission_methods_observed"][0],
            "option_ids": outcome["permission_option_ids"],
        },
        "permission_response_cancelled_or_rejected": outcome["permission_reply"],
        "pending_permission_wait_abandoned": outcome["pending_permission_wait_abandoned"],
        "late_permission_response_accepted": outcome["late_permission_response_accepted"],
        "session_cancel_sent": outcome["session_cancel_sent"],
        "prompt_terminal_outcome": outcome["session_prompt"],
        "effect_marker_absent_before_prompt": outcome["effect_marker_absent_before_prompt"],
        "effect_marker_absent_after_stop": not outcome["effect_marker_present"],
        "observed_network_audiences_only": [],
        "retry_resend_fallback_and_tool_attempt_counts": {
            "prompts": 1,
            "retries": 0,
            "resends": 0,
            "auto_fallbacks": 0,
            "permission_requests": len(outcome["permission_methods_observed"]),
            "tool_attempts": 1,
            "effects": 0,
        },
        "process_exit_status_and_joined_state": {
            "exit_code": outcome["exit_code"],
            "joined": outcome["process_joined"],
        },
        "forced_process_group_stop": outcome["forced_process_group_kill"],
        "elapsed_seconds": round(elapsed, 3),
    }
    write_json_durable(record_path, record)
    persisted = load_json(record_path)
    validate_fake_invocation_record(persisted)
    discovery_result = discovery_fake_run(
        scratch, host_home, repository_root, credential
    )
    return preflight, persisted["result"], discovery_result


def validate_fake_discovery_record(record: dict[str, Any]) -> None:
    if (
        record.get("schema") != "copilot-cli-acp-authenticated-invocation.v1"
        or record.get("execution_kind") != "fake_control"
        or record.get("original_artifact_started") is not False
        or record.get("pre_execution_fsynced") is not True
    ):
        raise ValueError("discovery fake record is not a durable fake-only control")
    pre_execution = record.get("pre_execution_record")
    result = record.get("result")
    if not isinstance(pre_execution, dict) or not isinstance(result, dict):
        raise ValueError("discovery fake record is missing its plan or result")
    expected = json_digest(pre_execution)
    if record.get("pre_execution_sha256") != expected:
        raise ValueError("discovery fake result is not bound to its pre-execution plan")
    if result.get("pre_execution_record_sha256") != expected:
        raise ValueError("discovery fake result is not correlated to its plan")
    if result.get("outbound_methods") != ["initialize", "authenticate", "session/new"]:
        raise ValueError("discovery fake used an unapproved ACP operation")
    if result.get("prompt_requests") != 0:
        raise ValueError("discovery fake sent a prompt")
    if result.get("permission_callbacks") != 0 or result.get("tool_callbacks") != 0:
        raise ValueError("discovery fake observed a permission or tool callback")
    if result.get("effect_marker_absent_after_close") is not True:
        raise ValueError("discovery fake produced an action effect")
    serialized = json.dumps(record, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "test-only-delegated-capability",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"discovery fake record contains forbidden marker {forbidden!r}")


def original_child_environment(
    host_home: Path, scratch: Path, proxy_port: int
) -> dict[str, str]:
    proxy_url = f"http://127.0.0.1:{proxy_port}"
    environment = {
        "HOME": str(host_home.resolve()),
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "TMPDIR": str((scratch / "tmp").resolve()),
        "TERM": "dumb",
        "COPILOT_AUTO_UPDATE": "false",
        "HTTP_PROXY": proxy_url,
        "HTTPS_PROXY": proxy_url,
        "ALL_PROXY": proxy_url,
        "http_proxy": proxy_url,
        "https_proxy": proxy_url,
        "all_proxy": proxy_url,
    }
    for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN", "COPILOT_HOME"):
        if name in environment:
            raise RuntimeError(f"original environment unexpectedly contains {name}")
    return environment


def process_group_exists(process_group_id: int) -> bool:
    try:
        os.killpg(process_group_id, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


def stop_owned_process_group(
    process: subprocess.Popen[bytes], timeout: float = LIVE_CLEANUP_SECONDS
) -> tuple[int | None, bool, bool]:
    if process.stdin is not None:
        try:
            process.stdin.close()
        except OSError:
            pass
    process_group_id = process.pid
    deadline = time.monotonic() + timeout
    forced = False
    if process_group_exists(process_group_id):
        try:
            os.killpg(process_group_id, signal.SIGTERM)
        except ProcessLookupError:
            pass
        except PermissionError:
            # The exact Popen child is ours even when the host refuses group
            # signalling. Its sandbox denies process-fork, so signal that PID
            # directly and still require the process group to be empty below.
            try:
                process.send_signal(signal.SIGTERM)
            except ProcessLookupError:
                pass
    try:
        process.wait(timeout=min(0.8, max(0.0, deadline - time.monotonic())))
    except subprocess.TimeoutExpired:
        forced = True
    if process_group_exists(process_group_id):
        forced = True
        try:
            os.killpg(process_group_id, signal.SIGKILL)
        except ProcessLookupError:
            pass
        except PermissionError:
            try:
                process.kill()
            except ProcessLookupError:
                pass
    try:
        process.wait(timeout=max(0.0, deadline - time.monotonic()))
    except subprocess.TimeoutExpired:
        pass
    while process_group_exists(process_group_id) and time.monotonic() < deadline:
        time.sleep(min(0.05, deadline - time.monotonic()))
    return process.poll(), forced, not process_group_exists(process_group_id)


def run_permission_boundary_fake(
    scratch: Path, host_home: Path, repository_root: Path
) -> dict[str, Any]:
    proof_scratch = scratch / "host-login-permission-fake"
    record_dir = proof_scratch / "records"
    action_scratch = proof_scratch / "action"
    (action_scratch / "tmp").mkdir(parents=True)
    record_dir.mkdir()
    marker = action_scratch / "permission-effect-marker"
    diagnostic_path = action_scratch / "fake-child-diagnostic.json"
    proxy = CopilotEgressProxy(
        set(OFFICIAL_COPILOT_HOSTS),
        OFFICIAL_COPILOT_SUFFIXES,
        time.monotonic() + 10.0,
        fake_only=True,
    )
    try:
        proxy_port = proxy.start()
    except OSError as error:
        raise RuntimeError(
            f"host-login fake could not bind its local proxy (errno={error.errno})"
        ) from None
    python_app_executable = (
        Path(sys.prefix).resolve(strict=True)
        / "Resources/Python.app/Contents/MacOS/Python"
    )
    executable = (
        python_app_executable.resolve(strict=True)
        if python_app_executable.is_file()
        else Path(sys.executable).resolve(strict=True)
    )
    profile = permission_sandbox_profile(
        action_scratch,
        record_dir,
        host_home,
        repository_root,
        executable,
        proxy_port,
        (Path(sys.prefix).resolve(strict=True),),
    )
    if '(allow mach-lookup (global-name "com.apple.securityd"))' not in profile:
        raise RuntimeError("host-login sandbox omitted the approved securityd lookup")
    if "(allow process*)" in profile or "(allow network*)" in profile:
        raise RuntimeError("host-login sandbox grants broad process or network access")
    if "(deny network-inbound)" not in profile or "(deny network-bind)" not in profile:
        raise RuntimeError("host-login sandbox does not deny inbound or bound sockets")
    if f'(allow network-outbound (remote ip "localhost:{proxy_port}"))' not in profile:
        raise RuntimeError("host-login sandbox does not bind outbound traffic to the tested proxy")
    pre_execution = {
        "target": "fake-acp-agent; no original artifact",
        "model_policy": "--model auto",
        "credential_boundary": "no token environment or file; securityd exception only",
        "network_allowlist": sorted(OFFICIAL_COPILOT_HOSTS),
        "network_subdomains": list(OFFICIAL_COPILOT_SUFFIXES),
        "network_default": "deny",
        "profile_sha256": hashlib.sha256(profile.encode()).hexdigest(),
        "action": "cancel one permission request before marker creation",
        "budgets": {"prompts": 1, "seconds": 3, "effects": 0},
    }
    fake_record_path = record_dir / "invocation.json"
    fake_record = {
        "schema": "copilot-cli-acp-permission-proof-preflight-invocation.v1",
        "execution_kind": "fake_control",
        "original_artifact_started": False,
        "pre_execution_fsynced": True,
        "pre_execution_sha256": json_digest(pre_execution),
        "pre_execution_record": pre_execution,
        "result": None,
    }
    secret_free_serialization(fake_record)
    write_json_durable(fake_record_path, fake_record)
    if load_json(fake_record_path) != fake_record:
        raise RuntimeError("host-login fake pre-execution record did not persist before child start")
    environment = original_child_environment(host_home, action_scratch, proxy_port)
    environment.update(
        {
            "SWALLOWTAIL_BLOCKED_HOST_HOME": str(host_home.resolve()),
            "SWALLOWTAIL_BLOCKED_REPOSITORY": str(repository_root.resolve()),
            "SWALLOWTAIL_EFFECT_MARKER": str(marker),
            "SWALLOWTAIL_PROXY_PORT": str(proxy_port),
            "SWALLOWTAIL_FAKE_DIAGNOSTIC": str(diagnostic_path),
        }
    )
    command = [
        str(executable),
        "-c",
        FAKE_PERMISSION_AGENT_SOURCE,
        "--model",
        "auto",
        "--acp",
        "--stdio",
    ]
    try:
        process = subprocess.Popen(
            ["/usr/bin/sandbox-exec", "-p", profile, *command],
            cwd=action_scratch,
            env=environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
        )
    except OSError as error:
        proxy.stop_and_join(LIVE_CLEANUP_SECONDS)
        raise RuntimeError(
            f"host-login fake could not start in its sandbox (errno={error.errno})"
        ) from None
    client = StdioClient(process)
    permission_count = 0
    pending_wait_abandoned = False
    late_permission_response_accepted = False
    session_cancel_sent = False
    prompt_result: dict[str, Any] | None = None
    child_closed_stdout = False
    try:
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientCapabilities": {}},
            }
        )
        initialize = client.receive(3.0)
        if initialize.get("id") != 1 or initialize.get("result", {}).get("agentInfo", {}).get("version") != "1.0.93":
            raise RuntimeError("host-login fake did not initialize as its pinned control")
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "session/new",
                "params": {"cwd": str(action_scratch), "mcpServers": []},
            }
        )
        session = client.receive(3.0)
        if session.get("id") != 2 or not isinstance(session.get("result", {}).get("sessionId"), str):
            raise RuntimeError("host-login fake did not create its authenticated session")
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "session/prompt",
                "params": {
                    "sessionId": session["result"]["sessionId"],
                    "prompt": [{"type": "text", "text": f"Create the empty marker at {marker}"}],
                },
            }
        )
        deadline = time.monotonic() + 3.0
        while time.monotonic() < deadline:
            message = client.receive(deadline - time.monotonic())
            if message.get("method") == "session/request_permission":
                permission_count += 1
                wait = PendingPermissionWait()
                wait.abandon()
                pending_wait_abandoned = wait.state == "abandoned"
                late_permission_response_accepted = wait.resolve("allow_once")
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": message["id"],
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session["result"]["sessionId"]},
                    }
                )
                session_cancel_sent = True
                continue
            if message.get("id") == 3:
                prompt_result = message
                break
        if prompt_result is None:
            raise RuntimeError("host-login fake did not finish its cancelled prompt")
        if prompt_result.get("result", {}).get("stopReason") != "cancelled":
            raise RuntimeError("host-login fake prompt did not report cancellation")
    except EOFError:
        child_closed_stdout = True
    finally:
        client.close()
        exit_code, forced, process_group_joined = stop_owned_process_group(process)
        proxy_joined = proxy.stop_and_join(LIVE_CLEANUP_SECONDS)

    if child_closed_stdout:
        stderr = process.stderr.read(4096) if process.stderr is not None else b""
        child_diagnostic = (
            load_json(diagnostic_path) if diagnostic_path.is_file() else {}
        )
        diagnostic = ""
        match = re.search(rb"([A-Za-z][A-Za-z0-9]+): \[Errno ([0-9]+)\]", stderr)
        if match:
            diagnostic = f"; child_error={match.group(1).decode()}(errno={match.group(2).decode()})"
        elif b"operation not permitted" in stderr.lower():
            diagnostic = "; child_error=operation-not-permitted"
        elif b"sandbox" in stderr.lower():
            safe_stderr = stderr.decode("utf-8", errors="replace").strip()
            for path, label in (
                (str(action_scratch.resolve()), "<action-scratch>"),
                (str(record_dir.resolve()), "<record-dir>"),
                (str(host_home.resolve()), "<host-home>"),
                (str(repository_root.resolve()), "<repository>"),
            ):
                safe_stderr = safe_stderr.replace(path, label)
            diagnostic = f"; child_stderr={safe_stderr[-240:]!r}"
        elif stderr:
            safe_stderr = stderr.decode("utf-8", errors="replace").strip()
            for path, label in (
                (str(action_scratch.resolve()), "<action-scratch>"),
                (str(record_dir.resolve()), "<record-dir>"),
                (str(host_home.resolve()), "<host-home>"),
                (str(repository_root.resolve()), "<repository>"),
            ):
                safe_stderr = safe_stderr.replace(path, label)
            diagnostic = f"; child_stderr={safe_stderr[-240:]!r}"
        elif exit_code is not None:
            diagnostic = f"; child_exit={exit_code}"
        if child_diagnostic:
            error_type = child_diagnostic.get("type")
            error_number = child_diagnostic.get("errno")
            stage = child_diagnostic.get("stage")
            diagnostic += f"; child_stage={stage}; child_exception={error_type}(errno={error_number})"
        raise RuntimeError(f"host-login fake closed stdout before ACP initialize{diagnostic}")

    snapshot = proxy.snapshot()
    if permission_count != 1 or not pending_wait_abandoned or late_permission_response_accepted:
        raise RuntimeError("host-login fake did not enforce one cancelled permission request")
    if not session_cancel_sent or marker.exists():
        raise RuntimeError("host-login fake cancellation caused an effect or missed session/cancel")
    if exit_code != -signal.SIGKILL or not forced or not process_group_joined or not proxy_joined:
        raise RuntimeError("host-login fake did not exercise bounded forced cleanup and join")
    if snapshot != {
        "allowed_destination_hosts": ["api.github.com"],
        "unlisted_destination_count": 1,
        "sni_mismatch_count": 1,
        "connection_failure_count": 0,
    }:
        raise RuntimeError(f"host-login fake egress controls differ: {snapshot}")
    valid_evidence = {
        "initialize": "success",
        "reported_version_matches": True,
        "session_new": "success",
        "auto_session_created": True,
        "pre_prompt_permission_request": False,
        "prompt_requests_sent": 1,
        "credential_environment_variables_absent": True,
        "permission_request_count": permission_count,
        "permission_reply": "cancelled",
        "permission_callback_ambiguous": False,
        "late_permission_response_accepted": late_permission_response_accepted,
        "session_cancel_sent": session_cancel_sent,
        "session_prompt": "cancelled",
        "effect_marker_present": marker.exists(),
        "process_joined": process.returncode is not None,
        "process_group_joined": process_group_joined,
        "proxy_threads_joined": proxy_joined,
        "elapsed_seconds": 1.0,
        "proxy": {
            "allowed_destination_hosts": ["api.github.com"],
            "unlisted_destination_count": 0,
            "sni_mismatch_count": 0,
            "connection_failure_count": 0,
        },
    }
    negative_controls = {
        "missing_permission_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "permission_request_count": 0}
        ),
        "late_approval_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "late_permission_response_accepted": True}
        ),
        "effect_marker_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "effect_marker_present": True}
        ),
        "unlisted_egress_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "proxy": {**snapshot, "unlisted_destination_count": 1}}
        ),
        "sni_mismatch_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "proxy": {**valid_evidence["proxy"], "sni_mismatch_count": 1}}
        ),
    }
    if not permission_evidence_is_complete(valid_evidence) or not all(negative_controls.values()):
        raise RuntimeError("host-login fake permission acceptance controls failed")
    fake_record["result"] = {
        "prompt_requests": 1,
        "permission_requests": permission_count,
        "permission_reply": "cancelled",
        "session_cancel_sent": session_cancel_sent,
        "effect_marker_absent": not marker.exists(),
        "egress": snapshot,
        "process_group_joined": process_group_joined,
        "forced_cleanup_verified": forced,
        "proxy_threads_joined": proxy_joined,
        "negative_controls": negative_controls,
    }
    write_json_durable(fake_record_path, fake_record)
    secret_free_serialization(load_json(fake_record_path))
    return {
        "pre_execution_record_fsynced_before_child": True,
        "pre_execution_record_sha256": fake_record["pre_execution_sha256"],
        "sandbox_profile": "securityd-only Mach lookup; host home/repository reads denied; scratch-only writes; forks inherit the sandbox and shell exec is denied",
        "network": snapshot,
        "model_policy": "--model auto applied to original command shape",
        "permission_requests": permission_count,
        "permission_reply": "cancelled",
        "pending_wait_abandoned": pending_wait_abandoned,
        "late_approval_accepted": late_permission_response_accepted,
        "session_cancel_sent": session_cancel_sent,
        "effect_marker_absent": not marker.exists(),
        "child_exit_code": exit_code,
        "forced_cleanup_verified": forced,
        "process_group_joined": process_group_joined,
        "proxy_threads_joined": proxy_joined,
        "negative_controls": negative_controls,
    }


def discovery_fake_run(
    scratch: Path,
    host_home: Path,
    repository_root: Path,
    credential: bytes,
) -> dict[str, Any]:
    discovery_scratch = scratch / "authenticated-discovery-fake"
    discovery_scratch.mkdir()
    record_path = discovery_scratch / "discovery-invocation.json"
    pre_execution = {
        "target_identity": {
            "kind": "fake-acp-discovery-agent",
            "source_sha256": hashlib.sha256(
                FAKE_DISCOVERY_AGENT_SOURCE.encode()
            ).hexdigest(),
        },
        "account_access_ref": FAKE_ACCOUNT_REF,
        "credential_mechanism": "fake-host-broker-one-use-inherited-pipe",
        "allowed_acp_requests": sorted(DISCOVERY_REQUEST_METHODS),
        "endpoint_allowlist": [
            "https://github.fake.test",
            "https://copilot.fake.test",
        ],
        "containment": "deny-network-host-home-repository-and-keychain; scratch-writes-only",
        "budgets": {"invocations": 1, "seconds": 60, "prompts": 0},
    }
    digest = json_digest(pre_execution)
    record = {
        "schema": "copilot-cli-acp-authenticated-invocation.v1",
        "execution_kind": "fake_control",
        "original_artifact_started": False,
        "pre_execution_fsynced": True,
        "pre_execution_sha256": digest,
        "pre_execution_record": pre_execution,
        "result": None,
    }
    write_json_durable(record_path, record)
    if load_json(record_path) != record:
        raise RuntimeError("discovery fake plan was not durable before child start")

    started = time.monotonic()
    outcome = launch_acp(
        Path(sys.executable),
        [sys.executable, "-c", FAKE_DISCOVERY_AGENT_SOURCE],
        discovery_scratch,
        host_home,
        record_path,
        None,
        delegated_credential=credential,
        authenticate_method_id="copilot-login",
        mode="pre-prompt-discovery",
        repository_root=repository_root,
    )
    elapsed = time.monotonic() - started
    if (
        outcome.get("initialize") != "success"
        or outcome.get("authenticate") != "success"
        or outcome.get("session_new") != "success"
    ):
        raise RuntimeError("discovery fake did not complete initialize/authenticate/session-new")
    if outcome.get("session_prompt") != "not-authorized-not-sent":
        raise RuntimeError("discovery fake did not stop before session/prompt")
    observation = outcome.get("discovery_observation")
    if not isinstance(observation, dict) or observation.get("available_model_ids") != [
        "fake-auto",
        "fake-gpt-5.4",
    ]:
        raise RuntimeError("discovery fake did not retain its synthetic model identifiers")
    if observation.get("current_model_id") != "fake-auto":
        raise RuntimeError("discovery fake did not retain its synthetic current model")
    if observation.get("model_config_option") != {
        "option_id": "model",
        "current_value": "fake-auto",
        "available_values": ["fake-auto", "fake-gpt-5.4"],
    }:
        raise RuntimeError("discovery fake did not retain its synthetic model config surface")
    if outcome.get("permission_request_observed") or outcome.get("tool_call_updates"):
        raise RuntimeError("discovery fake observed a permission or tool effect")
    if outcome.get("effect_marker_present") or not outcome.get("process_joined"):
        raise RuntimeError("discovery fake did not close cleanly without an effect")
    if outcome.get("forced_process_group_kill") or elapsed > MAX_AUTH_FAKE_SECONDS:
        raise RuntimeError("discovery fake exceeded its joined cleanup budget")

    record["result"] = {
        "pre_execution_record_sha256": digest,
        "outbound_methods": ["initialize", "authenticate", "session/new"],
        "prompt_requests": 0,
        "permission_callbacks": 0,
        "tool_callbacks": 0,
        "model_observation": observation,
        "effect_marker_absent_after_close": not outcome["effect_marker_present"],
        "process_joined": outcome["process_joined"],
        "elapsed_seconds": round(elapsed, 3),
    }
    write_json_durable(record_path, record)
    validate_fake_discovery_record(load_json(record_path))
    return record["result"]


def self_test() -> dict[str, Any]:
    fake_shell = require_macos_sandbox()
    plan = validate_authenticated_plan()
    permission_plan = validate_permission_proof_plan()
    with tempfile.TemporaryDirectory(prefix="copilot-acp-offline-proof-") as temp_root:
        task_root = Path(temp_root).resolve()
        scratch = task_root / "task-scratch"
        scratch.mkdir()
        host_home = task_root / "fake-host-home"
        (host_home / ".copilot").mkdir(parents=True)
        (host_home / "Library" / "Keychains").mkdir(parents=True)
        repository_root = task_root / "fake-repository"
        repository_root.mkdir()
        for path, contents in (
            (host_home / ".copilot" / "config.json", b"fake config\n"),
            (host_home / ".copilot" / "settings.json", b"fake settings\n"),
            (host_home / "Library" / "Keychains" / "login.keychain-db", b"fake keychain\n"),
            (host_home / "Library" / "Keychains" / "copilot-cli.fake", b"fake item\n"),
            (repository_root / "README.md", b"fake repository\n"),
        ):
            path.write_bytes(contents)
        record_path = scratch / "pre-execution-record.json"
        record = {
            "schema": "copilot-cli-acp-offline-execution-record.v1",
            "artifact_execution_started": False,
            "pre_execution_record_persisted": True,
        }
        write_json_durable(record_path, record)
        if load_json(record_path) != record:
            raise RuntimeError("pre-execution record did not persist before fake launch")
        fake_path = scratch / "fake-copilot-acp.py"
        fake_path.write_text(FAKE_AGENT_SOURCE)
        fake_path.chmod(0o700)
        profile = sandbox_profile(scratch, host_home, repository_root)
        if "(deny network*)" not in profile or "(allow network" in profile:
            raise RuntimeError("sandbox profile does not deny every network operation")
        if f"(allow file-write* (subpath {quote_profile_path(scratch)}))" not in profile:
            raise RuntimeError("sandbox profile does not bind writes to task scratch")
        if f"(deny file-read* (subpath {quote_profile_path(host_home)}))" not in profile:
            raise RuntimeError("sandbox profile does not deny the host home")
        if f"(deny file-read* (subpath {quote_profile_path(repository_root)}))" not in profile:
            raise RuntimeError("sandbox profile does not deny the task repository")
        outcome = launch_acp(
            Path(fake_shell),
            [fake_shell, str(fake_path)],
            scratch,
            host_home,
            record_path,
            None,
            repository_root=repository_root,
        )
        if outcome.get("initialize") != "success":
            raise RuntimeError("fake ACP initialize did not complete")
        if outcome.get("session_new") != "success":
            raise RuntimeError(f"fake ACP session/new did not complete: {json.dumps(outcome, sort_keys=True)}")
        if outcome.get("session_prompt") != "cancelled":
            raise RuntimeError("fake ACP permission denial did not cancel the prompt")
        if not outcome.get("permission_request_observed"):
            raise RuntimeError("fake ACP permission request was not observed")
        if outcome.get("permission_reply") != "cancelled":
            raise RuntimeError("fake ACP permission request was not cancelled")
        if not outcome.get("pending_permission_wait_abandoned"):
            raise RuntimeError("fake ACP pending permission wait was not abandoned")
        if outcome.get("late_permission_response_accepted"):
            raise RuntimeError("fake ACP accepted a late permission response")
        if outcome.get("effect_marker_present"):
            raise RuntimeError("fake ACP rejection allowed the tool effect")
        if (
            outcome.get("exit_code") != 0
            or outcome.get("forced_process_group_kill")
            or not outcome.get("process_joined")
        ):
            raise RuntimeError("fake ACP agent did not exit cleanly")
        auth_preflight, auth_result, discovery_result = authenticated_fake_run(
            scratch, host_home, repository_root
        )
        permission_boundary_result = run_permission_boundary_fake(
            scratch, host_home, repository_root
        )
        return {
            "status": "passed",
            "network_denial": "loopback and reserved external connect both returned EPERM/EACCES",
            "filesystem_boundary": "writes allowed only inside task scratch; fake host home and repository reads denied",
            "keychain_boundary": "securityd Mach lookup denied by profile",
            "record_before_execution": True,
            "authenticated_plan": {
                "schema": plan["schema"],
                "execution_authorized": plan["execution_authorized"],
                "invocations": len(plan["invocations"]),
                "missing_owner_attestations": len(plan["missing_owner_attestations"]),
                "record_schema_validated": True,
            },
            "fake_acp": {
                "initialize": outcome["initialize"],
                "session_new": outcome["session_new"],
                "permission_request": "observed",
                "permission_reply": "cancelled",
                "pending_permission_wait_abandoned": outcome["pending_permission_wait_abandoned"],
                "prompt_stop_reason": outcome["session_prompt"],
                "tool_effect": "absent",
                "exit_code": outcome["exit_code"],
                "process_joined": outcome["process_joined"],
            },
            "authenticated_fake_preflight": auth_preflight,
            "authenticated_fake_result": auth_result,
            "authenticated_fake_discovery_result": discovery_result,
            "permission_boundary_plan": {
                "execution_authorized": permission_plan["execution_authorized"],
                "decision_id": permission_plan["operator_decision"],
                "allowed_hosts": len(permission_plan["network_policy"]["allowed_hosts"]),
                "attempts": permission_plan["budgets"]["max_total_invocations"],
            },
            "host_login_permission_fake": permission_boundary_result,
        }


def prepare_record(record_path: Path) -> None:
    previous_preflights: list[dict[str, Any]] = []
    if record_path.exists():
        previous = load_json(record_path)
        if previous.get("artifact_execution_started") or previous.get("executions"):
            raise RuntimeError("artifact execution already started; refusing to rewrite the record")
        previous_preflights.extend(previous.get("preflight_attempts", []))
        if previous.get("preflight"):
            previous_preflights.append(previous["preflight"])
    inventory = verify_inventory()
    fake_proof = self_test()
    inventory_digest = hashlib.sha256(INVENTORY_PATH.read_bytes()).hexdigest()
    harness_digest = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    record = {
        "schema": "copilot-cli-acp-offline-execution-record.v1",
        "recorded_at_utc": utc_now(),
        "scope": {
            "route": "copilot-cli.acp",
            "selected_argv": ["copilot", "--acp", "--stdio"],
            "versions": list(VERSIONS),
            "network": "denied, including loopback",
            "authentication": "synthetic placeholder only",
        },
        "artifact_inventory_observed_at_utc": inventory["observed_at_utc"],
        "artifact_inventory_sha256_at_execution": inventory_digest,
        "artifact_inventory_sha256_committed": inventory_digest,
        "harness_source": "scripts/copilot-acp-offline-proof.py",
        "harness_sha256": harness_digest,
        "harness_sha256_recorded_at_utc": utc_now(),
        "execution_host": {
            "architecture": platform.machine().lower(),
            "host_name_recorded": False,
            "os": f"{platform.system()} {platform.release()}",
        },
        "pre_execution_record_persisted": True,
        "artifact_execution_started": False,
        "permission_boundary_proven_for_all_targets": False,
        "exact_permission_path_reached": False,
        "preflight": fake_proof,
        "preflight_attempts": [*previous_preflights, fake_proof],
        "executions": [],
    }
    write_json_durable(record_path, record)
    saved = load_json(record_path)
    if not saved.get("pre_execution_record_persisted") or saved.get("artifact_execution_started"):
        raise RuntimeError("pre-execution record failed its read-back gate")
    print(json.dumps({"record": str(record_path), "preflight": fake_proof}, indent=2))


def validate_preflight_record(path: Path, plan: dict[str, Any]) -> dict[str, Any]:
    preflight = load_json(path)
    if preflight.get("schema") != "copilot-cli-acp-permission-proof-preflight.v1":
        raise ValueError("permission proof requires the validated fake preflight record")
    if preflight.get("harness_sha256") != hashlib.sha256(Path(__file__).read_bytes()).hexdigest():
        raise ValueError("fake preflight was produced by a different harness revision")
    if preflight.get("plan_sha256") != hashlib.sha256(
        PERMISSION_PROOF_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("fake preflight was produced for a different permission plan")
    result = preflight.get("result", {})
    fake = result.get("host_login_permission_fake", {})
    if result.get("status") != "passed" or fake.get("permission_reply") != "cancelled":
        raise ValueError("permission proof requires a passing host-login cancellation fake")
    if fake.get("effect_marker_absent") is not True:
        raise ValueError("host-login cancellation fake did not prove absence of the effect")
    if fake.get("process_group_joined") is not True or fake.get("proxy_threads_joined") is not True:
        raise ValueError("host-login cancellation fake did not prove bounded joined cleanup")
    if fake.get("network", {}).get("allowed_destination_hosts") != ["api.github.com"]:
        raise ValueError("host-login fake did not exercise the selected proxy path")
    if not plan["network_policy"]["allowed_hosts"] or fake.get("network", {}).get("unlisted_destination_count") != 1:
        raise ValueError("host-login fake did not prove default-deny destinations")
    return preflight


def secret_free_serialization(value: Any) -> None:
    serialized = json.dumps(value, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
        "test-only-delegated-capability",
    ):
        if forbidden in serialized:
            raise ValueError(f"permission proof record contains forbidden marker {forbidden!r}")


def run_permission_proof(
    record_path: Path,
    artifact_root: Path,
    preflight_path: Path,
) -> None:
    require_macos_sandbox()
    plan = validate_permission_proof_plan()
    inventory = verify_inventory()
    requested_record_path = record_path
    requested_preflight_path = preflight_path
    requested_artifact_root = artifact_root
    if requested_record_path.is_symlink() or requested_preflight_path.is_symlink() or requested_artifact_root.is_symlink():
        raise RuntimeError("original proof paths must not be symlinks")
    record_path = record_path.resolve(strict=False)
    preflight_path = preflight_path.resolve(strict=True)
    artifact_root = artifact_root.resolve(strict=True)
    if record_path.exists() or record_path.is_symlink():
        raise RuntimeError("refusing to repeat or overwrite an existing original proof record")
    if record_path.parent.name != "records":
        raise RuntimeError("proof records must use the records directory in fresh task scratch")
    task_root = record_path.parent.parent.resolve(strict=True)
    system_temp = Path(tempfile.gettempdir()).resolve()
    if (
        not task_root.is_relative_to(system_temp)
        or not task_root.name.startswith("copilot-acp-permission-proof.")
    ):
        raise RuntimeError("original proof requires a fresh mktemp task directory")
    if not artifact_root.is_relative_to(task_root) or artifact_root.is_symlink():
        raise RuntimeError("exact originals must be staged inside the fresh task directory")
    if not preflight_path.is_relative_to(task_root) or preflight_path == record_path:
        raise RuntimeError("fake preflight record must be in the same task scratch")
    preflight = validate_preflight_record(preflight_path, plan)
    packages = inventory_packages(inventory)
    staged: dict[str, tuple[Path, str]] = {}
    for version in plan["start_order"]:
        binary_path = artifact_root / version / "copilot"
        if binary_path.is_symlink() or not binary_path.is_file():
            raise RuntimeError(f"frozen original binary is absent for {version}")
        binary = binary_path.resolve(strict=True)
        if not binary.is_relative_to(artifact_root):
            raise RuntimeError(f"frozen original binary escapes its staged root for {version}")
        package = packages[("@github/copilot-darwin-arm64", version)]
        selected = next(
            item for item in package["files"] if item["path"] == "package/copilot"
        )
        digest = hashlib.sha256(binary.read_bytes()).hexdigest()
        if digest != selected["sha256"]:
            raise RuntimeError(f"frozen original binary digest mismatch for {version}")
        staged[version] = (binary, digest)

    record: dict[str, Any] = {
        "schema": "copilot-cli-acp-permission-proof-execution.v1",
        "plan_sha256": hashlib.sha256(PERMISSION_PROOF_PLAN_PATH.read_bytes()).hexdigest(),
        "artifact_inventory_sha256": hashlib.sha256(INVENTORY_PATH.read_bytes()).hexdigest(),
        "harness_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "account_access_ref": "github-account:betterthanclay",
        "account_identity_observation": "operator-reported reference; not independently queried",
        "keychain_boundary": {
            "normal_host_login_access": "permitted through original vendor CLI and com.apple.securityd",
            "queried_item_or_service": "unobserved; no item-level mediation claimed",
            "token_handling_by_harness": "none",
        },
        "network_authority_policy": {
            "source": plan["network_policy"]["source"],
            "allowed_exact_hosts": sorted(plan["network_policy"]["allowed_hosts"]),
            "allowed_subdomains": plan["network_policy"]["allowed_subdomains"],
            "default": "deny",
            "unexpected_destinations": "rejected; policy never expanded",
        },
        "budgets": plan["budgets"],
        "preflight_sha256": hashlib.sha256(preflight_path.read_bytes()).hexdigest(),
        "preflight_status": "passed",
        "artifact_execution_started": False,
        "permission_boundary_proven_for_all_targets": False,
        "exact_permission_path_reached": False,
        "invocations": [],
    }
    secret_free_serialization(record)
    write_json_durable(record_path, record)
    if load_json(record_path) != record:
        raise RuntimeError("top-level execution record did not persist before original starts")

    for index, version in enumerate(plan["start_order"]):
        binary, digest = staged[version]
        if digest != next(
            item["sha256"]
            for item in packages[("@github/copilot-darwin-arm64", version)]["files"]
            if item["path"] == "package/copilot"
        ):
            raise RuntimeError(f"frozen original identity changed before {version} start")
        result = run_permission_attempt(
            binary,
            version,
            task_root / "action" / version,
            record_path.parent,
            record_path,
            record,
            ROOT,
            plan,
        )
        secret_free_serialization(record)
        print(json.dumps({"version": version, "result": result}, sort_keys=True))
        if not result["safe_to_continue_older_versions"]:
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break
        if (
            result.get("prompt_error", {}).get("kind") == "quota-or-entitlement-stop"
            or result.get("session_new_error", {}).get("kind") == "quota-or-entitlement-stop"
            or result.get("initialize_error", {}).get("kind") == "quota-or-entitlement-stop"
        ):
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break
        if result["effect_marker_present"] or not result["process_joined"] or not result["process_group_joined"] or not result["proxy_threads_joined"]:
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break
        if result["proxy"]["unlisted_destination_count"] or result["proxy"]["sni_mismatch_count"] or result["proxy"]["connection_failure_count"]:
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break

    record["permission_boundary_proven_for_all_targets"] = bool(
        [item for item in record["invocations"] if item.get("result")]
        and len(record["invocations"]) == len(plan["start_order"])
        and all(
            item["result"].get("permission_evidence_complete") is True
            for item in record["invocations"]
        )
    )
    record["exact_permission_path_reached"] = any(
        item.get("result", {}).get("permission_request_count", 0) > 0
        for item in record["invocations"]
    )
    record["prompts_used"] = sum(
        item.get("result", {}).get("prompt_requests_sent", 0)
        for item in record["invocations"]
    )
    record["remaining_prompt_allowance"] = max(0, 3 - record["prompts_used"])
    record["qualification_changed"] = False
    record["completed_at_utc"] = utc_now()
    secret_free_serialization(record)
    write_json_durable(record_path, record)
    print(
        json.dumps(
            {
                "permission_boundary_proven_for_all_targets": record[
                    "permission_boundary_proven_for_all_targets"
                ],
                "exact_permission_path_reached": record["exact_permission_path_reached"],
                "invocations": len(record["invocations"]),
                "prompts_used": record["prompts_used"],
                "remaining_prompt_allowance": record["remaining_prompt_allowance"],
                "versions_not_started": record.get("versions_not_started", []),
                "record_schema": record["schema"],
            },
            sort_keys=True,
        )
    )


def validate_permission_execution_record(record_path: Path) -> dict[str, Any]:
    plan = validate_permission_proof_plan()
    record = load_json(record_path)
    if record.get("schema") != "copilot-cli-acp-permission-proof-execution.v1":
        raise ValueError("unexpected original permission execution record schema")
    if record.get("plan_sha256") != hashlib.sha256(
        PERMISSION_PROOF_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("original execution record does not match the approved plan")
    if record.get("artifact_inventory_sha256") != hashlib.sha256(
        INVENTORY_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("original execution record does not match the frozen inventory")
    if record.get("harness_sha256") != hashlib.sha256(Path(__file__).read_bytes()).hexdigest():
        raise ValueError("original execution record does not match this harness revision")
    invocations = record.get("invocations")
    if not isinstance(invocations, list) or not 0 < len(invocations) <= 3:
        raise ValueError("original execution record has an invalid invocation count")
    order = plan["start_order"]
    if [item.get("version") for item in invocations] != order[: len(invocations)]:
        raise ValueError("original execution record is out of approved start order")
    for index, item in enumerate(invocations):
        safe_to_continue = item.get("result", {}).get(
            "safe_to_continue_older_versions"
        )
        if index < len(invocations) - 1 and safe_to_continue is not True:
            raise ValueError("an older version started after an incomplete permission proof")
        if index == len(invocations) - 1 and len(invocations) < len(order) and safe_to_continue is not False:
            raise ValueError("an incomplete version sequence lacks its required stop")
    if record.get("versions_not_started", []) != order[len(invocations) :]:
        raise ValueError("original execution record misstates the unstarted versions")
    packages = inventory_packages(verify_inventory())
    for item in invocations:
        version = item.get("version")
        expected_hash = next(
            file["sha256"]
            for file in packages[("@github/copilot-darwin-arm64", version)]["files"]
            if file["path"] == "package/copilot"
        )
        pre_execution = item.get("pre_execution_record")
        if (
            item.get("invocation_consumed") is not True
            or item.get("pre_execution_fsynced") is not True
            or not isinstance(pre_execution, dict)
            or item.get("pre_execution_record_sha256") != json_digest(pre_execution)
            or pre_execution.get("identity", {}).get("version") != version
            or pre_execution.get("identity", {}).get("binary_sha256") != expected_hash
            or pre_execution.get("account_access_ref") != "github-account:betterthanclay"
            or pre_execution.get("selection_policy", {}).get("requested") != "Auto"
            or pre_execution.get("credential_boundary", {}).get("token_handling_by_harness")
            != "never extracted, copied, logged, or persisted"
        ):
            raise ValueError(f"invalid or unbound pre-execution record for {version}")
        result = item.get("result")
        if not isinstance(result, dict):
            raise ValueError(f"original invocation {version} has no result")
        if result.get("prompt_requests_sent", 0) not in (0, 1):
            raise ValueError(f"original invocation {version} exceeded its prompt budget")
        if result.get("permission_reply") not in {"none", "cancelled"}:
            raise ValueError(f"original invocation {version} was not safely cancelled")
        network = result.get("proxy", {})
        for host in network.get("allowed_destination_hosts", []):
            if not host_allowed(
                host,
                set(plan["network_policy"]["allowed_hosts"]),
                tuple(plan["network_policy"]["allowed_subdomains"]),
            ):
                raise ValueError(f"execution record includes a non-allowlisted host for {version}")
    calculated_prompts = sum(
        item.get("result", {}).get("prompt_requests_sent", 0) for item in invocations
    )
    if record.get("prompts_used") != calculated_prompts or calculated_prompts > 3:
        raise ValueError("original execution record does not reconcile prompt accounting")
    if record.get("qualification_changed") is not False:
        raise ValueError("permission evidence must not silently change route qualification")
    if record.get("permission_boundary_proven_for_all_targets") is not all(
        len(invocations) == 3
        and item.get("result", {}).get("permission_evidence_complete") is True
        for item in invocations
    ):
        raise ValueError("original execution record misstates its permission proof result")
    secret_free_serialization(record)
    return {
        "status": "valid",
        "invocations": len(invocations),
        "prompts_used": calculated_prompts,
        "permission_boundary_proven_for_all_targets": record[
            "permission_boundary_proven_for_all_targets"
        ],
        "exact_permission_path_reached": record.get("exact_permission_path_reached") is True,
        "qualification_changed": False,
    }


def execute_artifacts(record_path: Path, artifact_root: Path) -> None:
    require_macos_sandbox()
    inventory = verify_inventory()
    record = load_json(record_path)
    if record.get("pre_execution_record_persisted") is not True:
        raise RuntimeError("exact artifacts require a persisted pre-execution record")
    if record.get("preflight", {}).get("status") != "passed":
        raise RuntimeError("exact artifacts require a passing fake containment proof")
    if record.get("artifact_execution_started") or record.get("executions"):
        raise RuntimeError("exact artifact executions are already recorded; refusing a repeat")
    packages = inventory_packages(inventory)
    host_home = Path(os.environ["HOME"]).resolve()
    root = artifact_root.resolve(strict=True)
    temp_root = Path(tempfile.gettempdir()).resolve()
    if artifact_root.is_symlink() or not root.is_relative_to(temp_root):
        raise RuntimeError("exact artifacts must be staged inside a fresh task-owned temp directory")
    staged: list[tuple[str, Path, str, Path]] = []

    for version in VERSIONS:
        binary_path = root / version / "copilot"
        if binary_path.is_symlink() or not binary_path.is_file():
            raise RuntimeError(f"selected native binary is absent for {version}")
        binary = binary_path.resolve(strict=True)
        if not binary.is_relative_to(root):
            raise RuntimeError(f"selected native binary escapes the artifact root for {version}")
        package = packages[("@github/copilot-darwin-arm64", version)]
        binary_record = next((item for item in package["files"] if item["path"] == "package/copilot"), None)
        if binary_record is None:
            raise RuntimeError(f"inventory has no selected binary for {version}")
        digest_builder = hashlib.sha256()
        with binary.open("rb") as input_file:
            for block in iter(lambda: input_file.read(1024 * 1024), b""):
                digest_builder.update(block)
        digest = digest_builder.hexdigest()
        if digest != binary_record["sha256"]:
            raise RuntimeError(f"native binary digest mismatch for {version}")
        run_scratch = root.parent / f"execution-{version}"
        if run_scratch.exists():
            raise RuntimeError(f"task scratch already has an execution directory for {version}")
        staged.append((version, binary, digest, run_scratch))

    record["executions"] = []
    for version, binary, digest, run_scratch in staged:
        run_scratch.mkdir(parents=True, exist_ok=False)
        attempt: dict[str, Any] = {
            "version": version,
            "binary_sha256": digest,
            "selected_argv": ["copilot", "--acp", "--stdio"],
            "recorded_before_process_start": True,
            "network": "sandbox-denied",
            "credentials": "synthetic-placeholder-only",
            "started_at_utc": utc_now(),
            "permission_request_observed": False,
            "permission_reply": "none",
            "effect_marker_present": False,
        }
        record["executions"].append(attempt)
        record["artifact_execution_started"] = True
        record.setdefault("execution_started_at_utc", utc_now())
        write_json_durable(record_path, record)
        readback = load_json(record_path)
        if readback["executions"][-1] != attempt:
            raise RuntimeError(f"record-before-execution gate failed for {version}")
        try:
            outcome = launch_acp(
                binary,
                [str(binary), "--acp", "--stdio"],
                run_scratch,
                host_home,
                record_path,
                version,
            )
        except Exception as error:
            outcome = {
                "launch_error_class": type(error).__name__,
                "permission_request_observed": False,
                "permission_reply": "none",
                "effect_marker_present": False,
            }
        attempt.update(outcome)
        attempt["artifact_execution_result_recorded_at_utc"] = utc_now()
        attempt["permission_evidence_complete"] = bool(
            outcome.get("permission_request_observed")
            and outcome.get("permission_reply") == "cancelled"
            and outcome.get("session_prompt") == "cancelled"
            and not outcome.get("effect_marker_present")
        )
        write_json_durable(record_path, record)
        print(json.dumps({"version": version, "result": attempt}, sort_keys=True))
    record["permission_boundary_proven_for_all_targets"] = all(
        attempt.get("permission_evidence_complete") is True for attempt in record["executions"]
    )
    record["exact_permission_path_reached"] = any(
        attempt.get("permission_request_observed")
        or attempt.get("session_prompt") not in ("not-reached", None)
        for attempt in record["executions"]
    )
    if not record["permission_boundary_proven_for_all_targets"]:
        record["limitation"] = (
            "All exact stable artifacts advertised copilot-login and rejected session/new "
            "with Authentication required under the synthetic placeholder. No real "
            "authentication or network access was used, so no permission request, "
            "cancellation, or tool-effect boundary is claimed."
        )
    record["completion_recorded_at_utc"] = utc_now()
    write_json_durable(record_path, record)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--validate-plan", action="store_true")
    parser.add_argument("--validate-permission-plan", action="store_true")
    parser.add_argument("--validate-permission-record", action="store_true")
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--record", type=Path)
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--permission-proof", action="store_true")
    parser.add_argument("--preflight-record", type=Path)
    args = parser.parse_args()
    try:
        if args.validate_plan:
            plan = validate_authenticated_plan()
            print(
                json.dumps(
                    {
                        "status": "passed",
                        "schema": plan["schema"],
                        "execution_authorized": plan["execution_authorized"],
                        "invocations": len(plan["invocations"]),
                        "missing_owner_attestations": len(plan["missing_owner_attestations"]),
                    },
                    sort_keys=True,
                )
            )
        elif args.validate_permission_plan:
            plan = validate_permission_proof_plan()
            print(
                json.dumps(
                    {
                        "status": "passed",
                        "operator_decision": plan["operator_decision"],
                        "execution_authorized": plan["execution_authorized"],
                        "start_order": plan["start_order"],
                        "allowed_hosts": len(plan["network_policy"]["allowed_hosts"]),
                        "maximum_prompts": plan["budgets"]["max_total_invocations"],
                    },
                    sort_keys=True,
                )
            )
        elif args.validate_permission_record and args.record:
            print(json.dumps(validate_permission_execution_record(args.record), sort_keys=True))
        elif args.self_test:
            verify_inventory()
            proof = self_test()
            preflight_path = os.environ.get(
                "SWALLOWTAIL_COPILOT_PROOF_PREFLIGHT_RECORD"
            )
            if preflight_path:
                target = Path(preflight_path).resolve(strict=False)
                temp_root = Path(tempfile.gettempdir()).resolve()
                if not target.parent.resolve(strict=True).is_relative_to(temp_root):
                    raise RuntimeError("fake preflight records must stay in task temp scratch")
                preflight = {
                    "schema": "copilot-cli-acp-permission-proof-preflight.v1",
                    "recorded_at_utc": utc_now(),
                    "plan_sha256": hashlib.sha256(
                        PERMISSION_PROOF_PLAN_PATH.read_bytes()
                    ).hexdigest(),
                    "harness_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                    "result": proof,
                }
                write_json_durable(target, preflight)
                if load_json(target) != preflight:
                    raise RuntimeError("fake preflight record did not persist")
            print(json.dumps(proof, indent=2))
        elif args.prepare and args.record:
            prepare_record(args.record)
        elif args.execute and args.record and args.artifact_root:
            execute_artifacts(args.record, args.artifact_root)
        elif (
            args.permission_proof
            and args.record
            and args.artifact_root
            and args.preflight_record
        ):
            run_permission_proof(args.record, args.artifact_root, args.preflight_record)
        else:
            parser.error(
                "choose --validate-plan, --self-test, --prepare --record, "
                "--execute --record --artifact-root, or --permission-proof "
                "--record --artifact-root --preflight-record"
            )
    except (
        OSError,
        RuntimeError,
        ValueError,
        KeyError,
        EOFError,
        json.JSONDecodeError,
    ) as error:
        if isinstance(error, OSError):
            frames: list[str] = []
            trace = error.__traceback__
            while trace is not None:
                frames.append(
                    f"{trace.tb_frame.f_code.co_name}:{trace.tb_lineno}"
                )
                trace = trace.tb_next
            detail = f"{type(error).__name__}(errno={error.errno})"
            if frames:
                detail += " at " + " -> ".join(frames[-6:])
        else:
            detail = str(error)
        print(f"offline proof failed closed: {detail}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
