"""Owned fake/self-test for the narrow trusted-original runner; no vendor runs."""

import copy
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
              const stream=(async function*() { let count=0; for await(const input of prompt) { if(count++ === 0) { yield {type:"system",subtype:"init",cwd:options.cwd,model:options.model,session_id:"owned",tools:["Write"],permissionMode:"default"}; yield {type:"assistant",message:{content:[{type:"tool_use",id:"owned",name:"Write"}]}};
              await options.canUseTool("Write",{file_path:"owned"});
              yield {type:"user",message:{content:[{type:"tool_result",tool_use_id:"owned",is_error:true}]}}; }
              yield {type:"assistant",message:{content:[{type:"text",text:"continued"}]}};
              yield {type:"result",subtype:"success",result:"continued",is_error:false,usage:{input_tokens:1,output_tokens:1}}; }})();
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
            prepared = T.case(
                cases,
                sdk,
                Path(node),
                Path(node),
                "deny",
                prepared=True,
                owned_fake=True,
                bound=15,
            )
            T.assert_case(prepared, "0.3.317", "prepared")
            lifecycle = T.case(
                cases,
                sdk,
                Path(node),
                Path(node),
                "next-turn",
                prepared=True,
                owned_fake=True,
                lifecycle=True,
                bound=15,
            )
            T.assert_case(lifecycle, "0.3.317", "prepared")
            self.assertEqual(
                len([e for e in observed["events"] if e["event"] == "callback"]), 1
            )

    def test_missing_terminal_refuses_reuse_and_waits_owned_native_and_transport(self):
        node = shutil.which("node")
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            (root / "package.json").write_text(
                '{"name":"@anthropic-ai/claude-agent-sdk","version":"0.3.317"}'
            )
            (root / "manifest.json").write_text('{"version":"2.1.317"}')
            sdk = root / "sdk.mjs"
            sdk.write_text("""export function query({prompt,options}) {
              const child=options.spawnClaudeCodeProcess({command:options.pathToClaudeCodeExecutable,args:["-e","process.stdin.resume()"],cwd:options.cwd,env:options.env});
              let release; const held=new Promise(r=>release=r);
              const stream=(async function*(){ for await(const input of prompt) {
                yield {type:"system",subtype:"init",cwd:options.cwd,model:options.model,session_id:"owned",tools:["Write"],permissionMode:"default"};
                await held; return;
              }})();
              return Object.assign(stream,{initializationResult:async()=>({}),supportedModels:async()=>[{value:options.model}],
                accountInfo:async()=>({apiProvider:"firstParty",subscriptionType:"max",tokenSource:"oauth"}),
                interrupt:async()=>{},close:()=>{release();child.stdin.end();}});
            }""")
            cases = root / "cases"
            cases.mkdir()
            observed = T.case(
                cases,
                sdk,
                Path(node),
                Path(node),
                "deadline",
                prepared=True,
                owned_fake=True,
                lifecycle=True,
                expect_reuse_refusal=True,
                bound=15,
            )
            self.assertEqual(observed["returncode"], 0, observed)
            self.assertEqual(observed["terminalStatus"], "TimedOut")
            self.assertTrue(observed["reuseBlocked"])
            self.assertEqual(
                observed["reuseRefusals"], ["swallowtail.claude-agent.sdk.turn_active"]
            )
            self.assertNotIn("nextTurn", observed)
            self.assertTrue(observed["allWaited"])
            self.assertTrue(observed["driverWaitObserved"])
            self.assertTrue(observed["closeResponse"]["data"]["nativeExitObserved"])
            self.assertEqual(
                len(
                    [e for e in observed["events"] if e.get("event") == "turn_started"]
                ),
                1,
            )
            with self.assertRaisesRegex(ValueError, "genuine next turn"):
                T.assert_case(observed, "0.3.317", "prepared")


class AssertionTests(unittest.TestCase):
    def fixture(self, driver="shipped-sidecar", name="deny"):
        silent = name == "managed-deny"
        callbacks = (
            []
            if silent or name in ("error", "cancel", "deadline")
            else [
                {
                    "event": "callback",
                    "toolName": "Write",
                    "effectBeforePermission": False,
                }
            ]
        )
        events = [
            {"event": "turn_started"},
            {"event": "tool_started", "toolCallId": "owned", "toolName": "Write"},
            {"event": "tool_ended", "toolCallId": "owned", "isError": name != "allow"},
            {"event": "output_delta", "delta": "continued"},
            {
                "event": "turn_ended",
                "isError": name in ("error", "cancel"),
                "stopReason": "error_during_execution"
                if name in ("error", "cancel")
                else "success",
                "terminalReason": "api_error"
                if name == "error"
                else "aborted_streaming"
                if name == "cancel"
                else None,
                "apiErrorStatus": 400 if name == "error" else None,
            },
            {"event": "native_close", "code": 0, "signal": None},
        ]
        if driver == "prepared":
            events[-1]["event"] = "native_exit"
        else:
            events += callbacks
        result = {
            "case": name,
            "returncode": 0,
            "driverWaitCode": 0,
            "allClosed": True,
            "allWaited": True,
            "children": 1,
            "effect": name == "allow",
            "events": events,
            "closeResponse": {"success": True, "data": {"nativeExitObserved": True}},
            "terminalStatus": {
                "deadline": "TimedOut",
                "cancel": "Cancelled",
                "error": "ProviderFailed",
            }.get(name, "Completed"),
            "turnCleanup": "Clean",
            "sessionCleanup": "Clean",
            "driverWaitObserved": True,
            "callbacks": callbacks,
            "cancellation": "Requested" if name == "cancel" else None,
            "runtimeEvents": [
                {"kind": "Started", "sequence": 0},
                {"kind": "Activity:Tool:Started:Running", "sequence": 1},
                {"kind": "OutputDelta", "sequence": 2, "content": "continued"},
            ],
        }
        if driver == "shipped-sidecar" and name == "cancel":
            events.append({"type": "response", "id": "cancel", "success": True})
        return result

    def test_actual_predicate_accepts_its_owned_controls(self):
        for driver in ["shipped-sidecar", "prepared"]:
            for name in [
                "allow",
                "deny",
                "user-deny",
                "managed-deny",
                "cancel",
                "error",
            ] + (["deadline"] if driver == "prepared" else []):
                T.assert_case(self.fixture(driver, name), "0.3.322", driver)

    def test_root_only_posture_requires_its_exact_positive_receipt(self):
        value = self.fixture("prepared")
        value.update(
            sessionCleanup="RootCompleted",
            sessionCleanupDiagnostic="swallowtail.claude-agent.sdk.close_root_only_degraded",
        )
        T.assert_case(value, "0.3.322", "prepared")
        value["driverWaitObserved"] = False
        with self.assertRaises(ValueError):
            T.assert_case(value, "0.3.322", "prepared")

    def test_actual_predicate_refuses_semantic_and_wait_mutations(self):
        for driver in ["shipped-sidecar", "prepared"]:
            base = self.fixture(driver)
            variants = []
            for event in [
                "output_delta",
                "tool_started",
                "tool_ended",
                "turn_started",
                "turn_ended",
                "native_exit" if driver == "prepared" else "native_close",
            ]:
                value = copy.deepcopy(base)
                value["events"] = [
                    e for e in value["events"] if e.get("event") != event
                ]
                variants.append(value)
            value = copy.deepcopy(base)
            value["events"][1], value["events"][2] = (
                value["events"][2],
                value["events"][1],
            )
            variants.append(value)
            for key, bad in [
                ("effect", True),
                ("allWaited" if driver == "prepared" else "allClosed", False),
                ("children", 0),
                ("driverWaitCode", None),
                ("failure", "startup error"),
            ]:
                value = copy.deepcopy(base)
                value[key] = bad
                variants.append(value)
            value = copy.deepcopy(base)
            value["closeResponse"]["data"]["nativeExitObserved"] = False
            variants.append(value)
            value = copy.deepcopy(base)
            if driver == "prepared":
                value["callbacks"] *= 2
            else:
                value["events"].append(
                    {
                        "event": "callback",
                        "toolName": "Write",
                        "effectBeforePermission": False,
                    }
                )
            variants.append(value)
            if driver == "prepared":
                for missing in (
                    "Started",
                    "Activity:Tool:Started:Running",
                    "OutputDelta",
                ):
                    value = copy.deepcopy(base)
                    value["runtimeEvents"] = [
                        e for e in value["runtimeEvents"] if e["kind"] != missing
                    ]
                    variants.append(value)
                value = copy.deepcopy(base)
                value["runtimeEvents"][2]["sequence"] = 0
                variants.append(value)
                value = copy.deepcopy(base)
                value.update(
                    sessionCleanup="RootCompleted",
                    sessionCleanupDiagnostic="unrelated_degradation",
                )
                variants.append(value)
            for value in variants:
                with self.subTest(driver=driver, value=value):
                    with self.assertRaises(ValueError):
                        T.assert_case(value, "0.3.322", driver)
            value = self.fixture(driver, "error")
            value["events"][4].update(isError=False, stopReason="success")
            with self.assertRaises(ValueError):
                T.assert_case(value, "0.3.322", driver)
        for key, bad in [
            ("terminalStatus", "Cancelled"),
            ("cancellation", "Requested"),
            ("sessionCleanup", "Degraded"),
            ("driverWaitObserved", False),
        ]:
            value = self.fixture("prepared", "deadline")
            value[key] = bad
            with self.assertRaises(ValueError):
                T.assert_case(value, "0.3.322", "prepared")

    def test_actual_lifecycle_assertion_refuses_old_terminal_misattribution(self):
        base = self.fixture("prepared", "cancel")
        base["lifecycle"] = True
        base["reuseRefusals"] = ["swallowtail.claude-agent.sdk.turn_active"]
        base["nextTurn"] = {
            "terminalStatus": "Completed",
            "output": "continued",
            "callbacks": [],
            "cancellation": None,
            "runtimeEvents": [
                {"kind": "Started", "sequence": 0},
                {"kind": "OutputDelta", "sequence": 1, "content": "continued"},
            ],
        }
        base["events"][5:5] = [
            {"event": "turn_started"},
            {"event": "output_delta", "delta": "continued"},
            {"event": "turn_ended", "isError": False, "stopReason": "success"},
        ]
        T.assert_case(base, "0.3.322", "prepared")
        variants = []
        for key, value in [
            ("terminalStatus", "TimedOut"),
            ("output", None),
            ("callbacks", [{"event": "callback"}]),
        ]:
            v = copy.deepcopy(base)
            v["nextTurn"][key] = value
            variants.append(v)
        v = copy.deepcopy(base)
        v["events"][4], v["events"][5] = v["events"][5], v["events"][4]
        variants.append(v)
        v = copy.deepcopy(base)
        v.pop("nextTurn")
        variants.append(v)
        v = copy.deepcopy(base)
        v["reuseRefusals"] = ["unrelated_error"]
        variants.append(v)
        v = copy.deepcopy(base)
        v["allWaited"] = False
        variants.append(v)
        v = copy.deepcopy(base)
        v["events"][7]["isError"] = True
        variants.append(v)
        for v in variants:
            with self.subTest(value=v), self.assertRaises(ValueError):
                T.assert_case(v, "0.3.322", "prepared")

    def test_original_error_cannot_be_a_caught_startup_failure(self):
        value = {
            "case": "error",
            "returncode": 0,
            "driverWaitCode": 0,
            "allClosed": True,
            "allWaited": True,
            "children": 1,
            "effect": False,
            "failure": "SDK import failed",
            "events": [{"event": "native_close", "code": 0}],
        }
        with self.assertRaises(ValueError):
            T.assert_case(value, "0.3.322", "sdk-options")


if __name__ == "__main__":
    unittest.main()
