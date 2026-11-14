#!/usr/bin/env python3
"""Stage a verified Node interpreter fixture; no install or code execution.

Only the frozen Darwin arm64 executable is copied. Nothing is added to PATH,
HOME or a host installation. The consumer is the first-party fake witness.
"""
import hashlib
import json
import os
from pathlib import Path
import platform
import tarfile
import tempfile
import urllib.request

if platform.system() != "Darwin" or platform.machine() != "arm64":
    raise SystemExit("fake runtime fixture platform not admitted")
root = Path(tempfile.mkdtemp(prefix="acp-fake-node-", dir=os.environ["TMPDIR"])).resolve()
archive = root / "node.tar.xz"
expected_archive = "5eff7a9011895aae3f29d06f167b84a62b028a591370c7cafb59103559fd26e1"
expected_node = "18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572"
url = "https://nodejs.org/dist/v22.23.2/node-v22.23.2-darwin-arm64.tar.xz"
try:
    hasher, count = hashlib.sha256(), 0
    with urllib.request.urlopen(url, timeout=30) as incoming, archive.open("xb") as output:
        while block := incoming.read(1024 * 1024):
            count += len(block)
            if count > 64 * 1024 * 1024:
                raise ValueError("runtime archive limit")
            hasher.update(block)
            output.write(block)
    if hasher.hexdigest() != expected_archive:
        raise ValueError("runtime archive substitution")
    with tarfile.open(archive, "r:xz") as tree:
        name = "node-v22.23.2-darwin-arm64/bin/node"
        entries = [entry for entry in tree.getmembers() if entry.name == name]
        if len(entries) != 1 or not entries[0].isreg() or entries[0].size != 112937728:
            raise ValueError("runtime archive member identity")
        hasher, count = hashlib.sha256(), 0
        with tree.extractfile(entries[0]) as incoming, (root / "node").open("xb") as output:
            while block := incoming.read(1024 * 1024):
                count += len(block)
                if count > 112937728:
                    raise ValueError("runtime source limit")
                hasher.update(block)
                output.write(block)
        if count != 112937728 or hasher.hexdigest() != expected_node:
            raise ValueError("runtime executable substitution")
    (root / "node").chmod(0o500)
    record = {"scope":"first-party-fake-runtime-fixture-only","node":str(root/"node"),"version":"22.23.2","nodeSha256":expected_node,"archiveSha256":expected_archive,"nodeBytes":count,"hostInstallationChanged":False,"executed":False}
    (root / "identity.json").write_text(json.dumps(record, indent=2)+"\n")
    print(json.dumps(record))
except Exception:
    print("retained failed owned runtime fixture: " + str(root), file=__import__("sys").stderr)
    raise
