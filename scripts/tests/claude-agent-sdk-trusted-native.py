"""Owned fake/self-test for the narrow trusted-original runner; no vendor runs."""

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import threading
import unittest
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "trusted", ROOT / "scripts/claude-agent-sdk-trusted-native.py"
)
T = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(T)


class TrustedTests(unittest.TestCase):
    def test_frozen_plan_and_platforms(self):
        plan = json.loads(T.checked(T.PLAN.read_bytes(), T.PLAN_SHA))
        self.assertEqual(
            [row["sdk_version"] for row in plan["artifacts"]],
            ["0.3.317", "0.3.318", "0.3.319", "0.3.321", "0.3.322"],
        )
        with self.assertRaisesRegex(ValueError, "digest"):
            T.checked(b"substitution", T.PLAN_SHA)

    def test_unsafe_sdk_archive_never_extracts(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            raw = io.BytesIO()
            with tarfile.open(fileobj=raw, mode="w") as archive:
                row = tarfile.TarInfo("package/../../escape")
                row.size = 1
                archive.addfile(row, io.BytesIO(b"x"))
            with self.assertRaisesRegex(ValueError, "unsafe archive"):
                T.extract_sdk(raw.getvalue(), Path(temporary) / "sdk")

    def test_model_peer_emits_tool_then_continued_output(self):
        server = T.http.server.ThreadingHTTPServer(("127.0.0.1", 0), T.Peer)
        server.requests, server.case = [], "allow"
        server.effect, server.stopping = Path("/owned-effect"), threading.Event()
        thread = threading.Thread(target=server.serve_forever)
        thread.start()
        try:
            for messages, expected in [
                ([], "tool_use"),
                ([{"role": "user", "content": [{"type": "tool_result"}]}], "continued"),
            ]:
                req = urllib.request.Request(
                    "http://127.0.0.1:" + str(server.server_port) + "/v1/messages",
                    data=json.dumps({"messages": messages}).encode(),
                )
                with urllib.request.urlopen(req, timeout=2) as response:
                    raw = response.read(16384).decode()
                self.assertIn(expected, raw)
                self.assertIn("message_stop", raw)
        finally:
            server.shutdown()
            server.server_close()
            thread.join(timeout=5)
        self.assertFalse(thread.is_alive())

    def test_driver_observes_owned_child_close_and_callback(self):
        node = shutil.which("node")
        self.assertIsNotNone(node)
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            sdk = root / "fake.mjs"
            sdk.write_text("""import process from "node:process";
export function query({options}) {
 const child=options.spawnClaudeCodeProcess({command:process.execPath,args:["-e","process.stdin.resume()"],cwd:options.cwd,env:options.env});
 return { initializationResult:async()=>({}), close:()=>child.stdin.end(),
 async *[Symbol.asyncIterator]() { await options.canUseTool("Write",{file_path:"owned"}); yield {type:"result",result:"continued"}; } };
}""")
            config = root / "case.json"
            config.write_text(
                json.dumps(
                    {
                        "sdk": str(sdk),
                        "native": "unused-owned-fake",
                        "workspace": str(root),
                        "case": "deny",
                        "environment": {
                            "PATH": str(Path(node).parent),
                            "HOME": str(root),
                        },
                    }
                )
            )
            result = subprocess.run(
                [
                    node,
                    str(ROOT / "scripts/claude-agent-sdk-trusted-case.mjs"),
                    str(config),
                ],
                capture_output=True,
                timeout=10,
                check=True,
            )
            observed = json.loads(result.stdout)
            self.assertTrue(observed["allClosed"])
            self.assertEqual(observed["children"], 1)
            self.assertEqual(
                len([v for v in observed["events"] if v["event"] == "callback"]), 1
            )
            self.assertEqual(
                len([v for v in observed["events"] if v["event"] == "native_close"]), 1
            )


if __name__ == "__main__":
    unittest.main()
