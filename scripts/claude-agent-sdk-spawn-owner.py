"""Owned finite group-owner fixture: actual Popen/wait, no native wire proxy."""

import json
import os
from pathlib import Path
import signal
import subprocess
import sys


def run(config_file):
    config = json.loads(Path(config_file).read_bytes())
    command = config["driverEntryCommand"]
    env = config["driverEnv"]
    child = subprocess.Popen(command, cwd=config["workspace"], env=env)

    def stop(signum, frame):
        if child.poll() is None:
            child.send_signal(signal.SIGTERM)

    signal.signal(signal.SIGTERM, stop)
    code = child.wait()
    print(
        json.dumps(
            {
                "probeEvent": "actor_wait",
                "role": "driver",
                "actor": os.getpid(),
                "pid": child.pid,
                "code": code,
                "source": "Popen.wait",
            }
        ),
        file=sys.stderr,
        flush=True,
    )
    return 0 if code == 0 else 1


if __name__ == "__main__":
    raise SystemExit(run(sys.argv[1]))
