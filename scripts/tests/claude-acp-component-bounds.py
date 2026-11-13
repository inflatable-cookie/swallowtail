#!/usr/bin/env python3
"""Owned process/output/storage counterexamples; no original artifacts."""
import importlib.util
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True

source = Path(__file__).resolve().parents[1] / "claude-acp-component-probe.py"
spec = importlib.util.spec_from_file_location("component_owner", source)
owner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(owner)

def child(code):
    return subprocess.Popen([sys.executable, "-c", code], stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={"PATH":"/usr/bin:/bin"})

peer = child("import sys;sys.stdout.write('ok');sys.stderr.write('error')")
assert owner.bounded_output(peer, 5, 128) == (b"ok", b"error")
assert peer.returncode == 0
for code, seconds, limit, label in [
    ("import sys;sys.stdout.write('x'*1024)", 5, 128, "output limit"),
    ("import time;time.sleep(10)", 0.1, 128, "timeout"),
]:
    peer = child(code)
    try:
        owner.bounded_output(peer, seconds, limit)
        raise AssertionError("missing limit refusal")
    except ValueError as error:
        assert label in str(error), str(error)
    assert peer.poll() is not None, "owned fake must exit after refusal"
root = Path(tempfile.mkdtemp(prefix="acp-component-bounds-", dir=os.environ["TMPDIR"]))
try:
    (root/"input").write_bytes(b"abc")
    owner.bounded_copy(root/"input", root/"copy", 3)
    assert (root/"copy").read_bytes() == b"abc"
    try:
        owner.bounded_copy(root/"input", root/"too-small", 2)
        raise AssertionError("missing storage refusal")
    except ValueError as error:
        assert "scratch input limit" in str(error)
finally:
    shutil.rmtree(root)
print("component owner success/output/timeout/storage counterexamples passed; owned fake exits joined")
