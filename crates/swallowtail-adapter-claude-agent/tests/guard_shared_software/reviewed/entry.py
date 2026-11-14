"""Trusted private exec entry. Runs owned payloads only in current rehearsals.

Source/config/runtime custody belongs to the invoking trusted host. This is
not a kernel verified-exec primitive and does not authorize original mode.
"""

import hashlib
import http.client
import json
import os
from pathlib import Path
import sys


def sha(file):
    value = hashlib.sha256()
    with open(file, "rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def run(config_file, role="native"):
    config = json.loads(Path(config_file).read_bytes())
    if role not in {"driver", "native"}:
        raise RuntimeError("entry_role")
    entry = config["entry" if role == "native" else "driverEntry"]
    command = entry["command"]
    if (
        config["originalActivation"] is not False
        or entry["pythonSha256"] != sha(Path(sys.executable).resolve(strict=True))
        or entry["sourceSha256"] != sha(Path(__file__))
        or entry["nodeSha256"] != sha(command[0])
        or entry["nativeSha256"] != sha(entry["nativePath"])
        or command[1:3] != ["--no-addons", "--openssl-config=/dev/null"]
    ):
        raise RuntimeError("trusted_entry_binding")
    # A host-validated gate precedes exec of the fake payload. The payload need
    # not implement a release protocol; the same mechanism can gate opaque exec.
    request = {
        "pid": os.getpid(),
        "parent": os.getppid(),
        "token": config["token"],
        "command": command[0],
        "args": command[1:],
        "entrySourceSha256": entry["sourceSha256"],
    }
    if config.get("fault") == "forged-approval":
        request["token"] = "forged"
    connection = http.client.HTTPConnection("127.0.0.1", config["port"], timeout=1)
    try:
        connection.request(
            "POST",
            "/" + role + "-entry",
            json.dumps(request),
            {"Content-Type": "application/json"},
        )
        response = connection.getresponse()
        data = response.read(4097)
        if response.status != 200 or len(data) > 4096:
            raise RuntimeError("trusted_entry_refused")
    finally:
        connection.close()
    reply = json.loads(data)
    if config.get("kernelPolicy") is not None:
        binding = config["kernelPolicy"]
        source = Path(binding["file"]).read_bytes()
        if hashlib.sha256(source).hexdigest() != binding["sha256"]:
            raise RuntimeError("kernel_policy_source_changed")
        namespace = {"__name__": "private_child_policy", "__file__": binding["file"]}
        exec(compile(source, binding["file"], "exec"), namespace)
        # This response arrives through the trusted guest's private root pipe;
        # root authenticates this live TCP socket and the actual traced task.
        namespace["install"]({**binding["inputs"], **reply["kernel"]}, role)
    elif reply:
        raise RuntimeError("unexpected_kernel_policy")
    if entry["nativeSha256"] != sha(entry["nativePath"]) or entry["nodeSha256"] != sha(
        command[0]
    ):
        raise RuntimeError("trusted_entry_changed_before_exec")
    os.execve(command[0], command, entry["env"])


if __name__ == "__main__":
    run(sys.argv[1], sys.argv[2])
