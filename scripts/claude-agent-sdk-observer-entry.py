"""Inactive, source-bound observer entry; no candidate runs before root admission."""

import hashlib
import json
import os
from pathlib import Path
import stat
import sys

LIMIT = 65536


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read_gate(fd):
    if type(fd) is not int or fd < 3 or not stat.S_ISFIFO(os.fstat(fd).st_mode):
        raise OSError("observer_private_pipe_required")
    value = bytearray()
    try:
        while True:
            data = os.read(fd, min(4096, 4097 - len(value)))
            if not data:
                break
            value.extend(data)
            if len(value) > 4096:
                raise OSError("observer_gate_bound")
    finally:
        os.close(fd)
    if not value:
        raise OSError("observer_not_admitted")
    result = json.loads(value)
    if set(result) != {"kernel"} or not isinstance(result["kernel"], dict):
        raise OSError("observer_gate_shape")
    return result["kernel"]


def run(config_file, expected, gate):
    file = Path(config_file)
    with file.open("rb") as stream:
        source = stream.read(LIMIT + 1)
    if len(source) > LIMIT or digest(source) != expected:
        raise OSError("observer_config_binding")
    config = json.loads(source)
    if config["originalActivation"] is not False:
        raise OSError("observer_original_mode_closed")
    # The actual owned Python child is already behind this barrier when its
    # parent reports its PID through the source-bound supervisor's private pipe.
    kernel = read_gate(gate)
    if file.read_bytes() != source:
        raise OSError("observer_config_changed_before_entry")
    for name, sha256 in config["inputs"].items():
        if digest(Path(name).read_bytes()) != sha256:
            raise OSError("observer_source_changed_before_entry")
    binding = config["policy"]
    policy = Path(binding["file"]).read_bytes()
    if digest(policy) != binding["sha256"]:
        raise OSError("observer_policy_changed")
    namespace = {"__name__": "private_observer_policy", "__file__": binding["file"]}
    exec(compile(policy, binding["file"], "exec"), namespace)
    namespace["install"]({**binding["inputs"], **kernel}, "observer")
    # The root's committed-exec tracer independently checks this exact command,
    # environment and cwd before the first candidate instruction is released.
    command = config["command"]
    os.execve(command[0], command, config["environment"])


if __name__ == "__main__":
    try:
        run(sys.argv[1], sys.argv[2], int(sys.argv[3]))
    except (OSError, ValueError, KeyError) as error:
        print(str(error)[:1000], file=sys.stderr, flush=True)
        raise SystemExit(78)
