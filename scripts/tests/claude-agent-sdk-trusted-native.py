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
            # Exercise the actual portable limits, endpoint lifetime and parent
            # wait seam with owned source, never an original artifact.
            case_root = root / "cases"
            case_root.mkdir()
            case_result = T.case(
                case_root, sdk, Path("owned-unused"), Path(node), "deny"
            )
            self.assertEqual(case_result["returncode"], 0, case_result)
            self.assertTrue(case_result["allClosed"])
            self.assertTrue(observed["allClosed"])
            self.assertEqual(observed["children"], 1)
            self.assertEqual(
                len([v for v in observed["events"] if v["event"] == "callback"]), 1
            )
            self.assertEqual(
                len([v for v in observed["events"] if v["event"] == "native_close"]), 1
            )

    def test_shipped_sidecar_driver_uses_real_wire_and_owned_child(self):
        node = shutil.which("node")
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            sdk = root / "sdk.mjs"
            (root / "package.json").write_text(
                json.dumps(
                    {"name": "@anthropic-ai/claude-agent-sdk", "version": "0.3.317"}
                )
            )
            (root / "manifest.json").write_text('{"version":"2.1.317"}')
            sdk.write_text("""export function query({prompt,options}) {
              const child=options.spawnClaudeCodeProcess({command:options.pathToClaudeCodeExecutable,args:["-e","process.stdin.resume()"],cwd:options.cwd,env:options.env});
              const stream=(async function*() { for await(const input of prompt) { yield {type:"system",subtype:"init",cwd:options.cwd,model:options.model,session_id:"owned",tools:["Write"],permissionMode:"default"}; await options.canUseTool("Write",{file_path:"owned"}); yield {type:"result",subtype:"success",result:"continued",is_error:false,usage:{input_tokens:1,output_tokens:1}}; return; }})();
              return Object.assign(stream,{initializationResult:async()=>({}), supportedModels:async()=>[{value:options.model}], accountInfo:async()=>({apiProvider:"firstParty",subscriptionType:"max",tokenSource:"oauth"}), close:()=>child.stdin.end()});
            }""")
            cases = root / "cases"
            cases.mkdir()
            observed = T.case(
                cases, sdk, Path(node), Path(node), "deny", sidecar=True, bound=5
            )
            self.assertEqual(observed["returncode"], 0, observed)
            self.assertTrue(observed["allClosed"])
            self.assertTrue(observed["closeResponse"]["data"]["nativeExitObserved"])
            self.assertEqual(
                len([e for e in observed["events"] if e["event"] == "callback"]), 1
            )


if __name__ == "__main__":
    unittest.main()
