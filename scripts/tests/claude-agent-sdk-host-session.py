"""Owned files/Python children and protocol models; no privileged host activation."""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace
import unittest
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    spec = importlib.util.spec_from_file_location(
        name, ROOT / ("scripts/claude-agent-sdk-" + name + ".py")
    )
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


H, P, A, C, B = (
    load(name)
    for name in (
        "host-session",
        "host-package",
        "host-transaction",
        "spawn-combined",
        "host-observer",
    )
)


class CompositionTests(unittest.TestCase):
    def setUp(self):
        self.root = Path(
            tempfile.mkdtemp(prefix="host-composition-", dir=os.environ["TMPDIR"])
        ).resolve()
        self.scratch = self.root / "ordinary"
        self.scratch.mkdir(mode=0o700)
        self.admin_root = self.root / "receipts"
        self.admin_root.mkdir(mode=0o700)
        for name in A.DIRECTORIES:
            path = self.root / name.lstrip("/")
            path.mkdir(mode=0o777)
            path.chmod(0o777)
        self.node = self.root / P.CANDIDATE.lstrip("/")
        self.node.write_bytes(b"owned data; never executed")
        self.node.chmod(0o777)
        self.owner = A.AdministrativeOwner(
            self.admin_root / "admin.json",
            {"owned": True},
            inspect_inputs=lambda: None,
            root=self.root,
            uid=os.getuid(),
        )
        self.owner.prepare()
        files = P.ImageFiles(root=self.root, uid=os.getuid(), owner=self.owner)
        files.read(P.CANDIDATE, candidate=True)
        self.package = P.HostPackage(
            files, {"path": P.CANDIDATE, "sha256": H.sha(self.node.read_bytes())}, {}
        )
        self.lease = H.PackageLease(self.package, P, self.scratch)
        self.previous_tmp = os.environ["TMPDIR"]
        os.environ["TMPDIR"] = str(self.scratch)

    def tearDown(self):
        os.environ["TMPDIR"] = self.previous_tmp
        self.node.chmod(0o777)
        if self.lease.stage is not None:
            self.lease.stage.close()
        self.owner.restore()
        self.package.close()
        shutil.rmtree(self.root)

    def receipt(self, mode):
        before = self.package.files.files[P.CANDIDATE][1]
        data = {
            "phase": "prepared" if mode == 0o755 else "restored",
            "path": P.CANDIDATE,
            "sha256": self.package.manifest["sha256"],
            "identity": list(before[:4] + before[5:7]),
            "originalMode": 0o777,
            "targetMode": 0o755,
            "children": [],
        }
        path = self.scratch / "session.json"
        path.write_text(json.dumps(data))
        path.chmod(0o600)
        return path

    def test_only_exact_live_leaf_transition_changes_foundation_mode(self):
        self.lease.check()
        self.node.chmod(0o755)
        with self.assertRaisesRegex(P.Refused, "candidate_drift"):
            self.lease.check()
        receipt = self.receipt(0o755)
        self.lease.leaf(receipt, 0o755)
        self.lease.check()
        with self.assertRaisesRegex(P.Refused, "candidate_drift"):
            self.package.check()  # Ordinary/default package custody stays strict.
        self.node.write_bytes(b"different owned bytes")
        with self.assertRaises(P.Refused):
            self.lease.check()
        self.node.write_bytes(b"owned data; never executed")
        # This test's write changed identity/mtime. Teardown restores only its
        # owned test directory; no live candidate or child is involved.

    def test_staged_release_keeps_foundation_and_refuses_alias_or_outside(self):
        stage = self.scratch / "stage"
        stage.mkdir(mode=0o700)
        source = stage / "source.py"
        source.write_bytes(b"owned = True\n")
        self.lease.begin(stage)
        self.lease.bind(source, H.sha(source.read_bytes()))
        self.lease.check()
        fd = self.package.files.files[P.CANDIDATE][0]
        outside = self.scratch / "outside.py"
        outside.write_bytes(b"owned")
        with self.assertRaisesRegex(OSError, "outside_owned_stage"):
            self.lease.bind(outside)
        alias = stage / "alias.py"
        alias.symlink_to(source)
        with self.assertRaisesRegex(OSError, "outside_owned_stage"):
            self.lease.bind(alias)
        self.lease.release_stage()
        self.assertEqual(os.fstat(fd).st_ino, self.node.stat().st_ino)
        self.assertEqual(self.lease.mode, 0o777)
        self.assertIsNone(self.lease.stage)

    def test_leaf_receipt_cannot_claim_waits_or_other_identity(self):
        self.node.chmod(0o755)
        path = self.receipt(0o755)
        value = json.loads(path.read_text())
        value["children"] = [-1]
        path.write_text(json.dumps(value))
        with self.assertRaisesRegex(OSError, "exact_ordinary_leaf"):
            self.lease.leaf(path, 0o755)
        self.assertEqual(self.lease.mode, 0o777)

    def monitor_model(self):
        _, cls = H.compose(C, B)
        monitor = cls.__new__(cls)
        monitor.refusal = None
        monitor.leaf_restored = True
        monitor.case = None
        monitor.group_removed = False
        monitor.group = SimpleNamespace(empty=lambda: True)
        monitor.result = {"failure": None}
        monitor.journal = {
            "treeSettled": True,
            "fixtureStage": "removed",
            "failure": None,
        }
        monitor.init = 900
        monitor.seen = {pid: {} for pid in range(900, 907)}
        row = {"actor": os.getpid(), "pid": 900, "status": 0}
        monitor.waits = [row]
        monitor.trace = SimpleNamespace(waits=[row])
        gates = []
        for index in range(6):
            terminal = {"actor": os.getpid(), "pid": 901 + index, "status": 0}
            monitor.trace.waits.append(terminal)
            gates.append(
                SimpleNamespace(
                    complete=lambda: True,
                    roles={"driver": 901 + index},
                    tasks={901 + index: {"terminal": terminal}},
                )
            )
        monitor.phases = [
            SimpleNamespace(settled=True, gate=gate) for gate in gates[:3]
        ]
        monitor.case_rows = [{"gate": gate} for gate in gates[3:]]
        return monitor

    def test_all_six_live_phase_waits_and_actual_init_wait_are_required(self):
        monitor = self.monitor_model()
        self.assertTrue(monitor.settlement_observed())
        saved = monitor.trace.waits.pop()
        self.assertFalse(monitor.settlement_observed())
        monitor.trace.waits.append(saved)
        monitor.waits = [{"actor": os.getpid(), "pid": 900, "status": 0}]
        monitor.trace.waits = monitor.trace.waits[1:]
        self.assertFalse(monitor.settlement_observed())
        monitor.trace.waits.insert(0, monitor.waits[0])
        monitor.phases[1].settled = False
        self.assertFalse(monitor.settlement_observed())
        monitor.phases[1].settled = True
        monitor.leaf_restored = False
        self.assertFalse(monitor.settlement_observed())

    def test_group_removal_happens_after_waits_and_does_not_replace_them(self):
        monitor = self.monitor_model()
        monitor.group.remove = mock.Mock()
        monitor.journal["cgroup"] = {"owned": True}
        monitor.save = mock.Mock()
        monitor.remove_group()
        self.assertTrue(monitor.group_removed)
        monitor.group.empty = mock.Mock(side_effect=OSError("removed"))
        self.assertTrue(monitor.settlement_observed())
        monitor.trace.waits.pop()
        self.assertFalse(monitor.settlement_observed())

    def test_deadline_caps_downward_instead_of_summing_phase_budgets(self):
        with mock.patch.object(H.time, "monotonic", return_value=100):
            self.assertEqual(H.remaining(120, 90), 17)
            self.assertEqual(H.remaining(200, 80), 80)
            with self.assertRaisesRegex(OSError, "budget_exhausted"):
                H.remaining(103, 90)

    def test_three_case_order_and_restore_cannot_be_skipped(self):
        _, cls = H.compose(C, B)
        monitor = cls.__new__(cls)
        monitor.case = None
        monitor.case_rows = []
        monitor.case_stop = False
        monitor.lease = SimpleNamespace(mode=0o755)
        with self.assertRaisesRegex(OSError, "three_exact_cases"):
            monitor.handle_owned(
                {"kind": "case_begin", "mode": "sidecar", "state": "managed-deny"}
            )
        with self.assertRaisesRegex(OSError, "leaf_restore_before_all"):
            monitor.handle_owned(
                {"kind": "leaf_restored", "receipt": str(self.scratch / "session.json")}
            )

    def test_restored_receipt_is_checked_against_the_actual_inode_and_bytes(self):
        self.node.chmod(0o755)
        self.lease.leaf(self.receipt(0o755), 0o755)
        self.node.chmod(0o777)
        self.lease.leaf(self.receipt(0o777), 0o777)
        self.package.check()
        self.assertEqual(self.lease.mode, 0o777)

    def test_refusal_replies_on_private_pipe_and_prevents_new_entry(self):
        _, cls = H.compose(C, B)
        monitor = cls.__new__(cls)
        monitor.refusal = None
        monitor.journal = {}
        monitor.handle_owned = mock.Mock(side_effect=OSError("owned admission refused"))
        reply = monitor.handle({"kind": "observer_started", "namespacePid": 3})
        self.assertIn("owned admission refused", reply["error"])
        self.assertEqual(reply["errno"], 1)
        monitor.handle_owned.reset_mock()
        blocked = monitor.handle(
            {"kind": "case_begin", "mode": "probe", "state": "absent"}
        )
        self.assertEqual(blocked["error"], reply["error"])
        monitor.handle_owned.assert_not_called()

    def test_refused_case_uses_the_same_actual_wait_predicate(self):
        monitor = self.monitor_model()
        monitor.phases = monitor.phases[:1]
        monitor.case_rows = monitor.case_rows[:1]
        allowed = {monitor.init} | {
            pid
            for gate in (monitor.phases[0].gate, monitor.case_rows[0]["gate"])
            for pid in gate.roles.values()
        }
        monitor.seen = {pid: {} for pid in allowed}
        monitor.trace.waits = [
            row for row in monitor.trace.waits if row["pid"] in allowed
        ]
        monitor.waits[0]["status"] = 78 << 8
        monitor.result = None
        monitor.journal["failure"] = "known protocol refusal"
        self.assertTrue(monitor.settlement_observed())
        monitor.trace.waits.pop()
        self.assertFalse(monitor.settlement_observed())


class PipelineTests(unittest.TestCase):
    """Software-owned scope replaces kernel activation; real handles/waits remain."""

    def setUp(self):
        self.root = Path(
            tempfile.mkdtemp(prefix="host-pipeline-", dir=os.environ["TMPDIR"])
        ).resolve()
        for name in A.DIRECTORIES:
            path = self.root / name.lstrip("/")
            path.mkdir(mode=0o777)
            path.chmod(0o777)
        node = self.root / P.CANDIDATE.lstrip("/")
        node.write_bytes(b"owned bytes only")
        node.chmod(0o777)
        self.receipts = self.root / "receipts"
        self.receipts.mkdir(mode=0o700)
        (self.root / "kernel-parent").mkdir(mode=0o700)
        self.ordinary = self.root / "ordinary"
        self.ordinary.mkdir(mode=0o700)
        self.previous_tmp = os.environ["TMPDIR"]
        os.environ["TMPDIR"] = str(self.ordinary)
        self.instances = {}
        self.withhold = self.refuse_parent = self.protocol_refuse = False
        outer = self

        def administrative(journal, binding, **kwargs):
            value = A.AdministrativeOwner(journal, binding, root=self.root, **kwargs)
            self.instances["owner"] = value
            return value

        self.admin = SimpleNamespace(
            AdministrativeOwner=administrative, ManifestTransfer=A.ManifestTransfer
        )

        def produce(owner, *args, **kwargs):
            files = P.ImageFiles(root=self.root, uid=os.getuid(), owner=owner)
            files.read(P.CANDIDATE, candidate=True)
            result = P.HostPackage(files, {"path": P.CANDIDATE}, {})
            self.instances["package"] = result
            return result

        self.producer = SimpleNamespace(produce=produce, ImageFiles=P.ImageFiles)

        class Group:
            def __init__(self, parent, name):
                if outer.refuse_parent:
                    raise OSError("owned delegation refused")
                self.path = outer.root / "kernel-parent/group"
                self.created = False
                outer.instances["group"] = self

            def create(self):
                self.path.mkdir(mode=0o700)
                self.created = True
                self.receipt_value = {
                    "device": self.path.stat().st_dev,
                    "inode": self.path.stat().st_ino,
                }
                return self.receipt_value

            def remove(self, expected):
                if expected != self.receipt_value or list(self.path.iterdir()):
                    raise OSError("owned group changed")
                self.path.rmdir()
                self.created = False

            def close(self):
                pass

        class Monitor:
            def __init__(
                self,
                args,
                core,
                files,
                modules,
                package,
                lease,
                group,
                journal,
                session,
                custody,
                elf,
                transfer,
                fixture,
            ):
                self.fork_attempted = False
                self.init = None
                self.case_rows = [{"passed": True} for _ in H.CASES]
                self.children = []
                self.group = group
                self.journal = journal
                self.transfer = transfer
                outer.instances.update(
                    monitor=self,
                    lease=lease,
                    transfer=transfer,
                    descriptor=args.journal_fd,
                )

            def run(self):
                owner = outer.instances["owner"]
                forbidden = [
                    (os.fstat(row["fd"]).st_dev, os.fstat(row["fd"]).st_ino)
                    for row in owner.rows
                ]
                forbidden += [
                    (
                        os.fstat(owner.journal_fd).st_dev,
                        os.fstat(owner.journal_fd).st_ino,
                    )
                ]
                code = (
                    "import os,sys; forbidden="
                    + repr(forbidden)
                    + "; found=[]\nfor fd in range(3,1024):\n try:\n  s=os.fstat(fd)\n except OSError: continue\n if (s.st_dev,s.st_ino) in forbidden: found.append(fd)\nsys.exit(bool(found))\n"
                )
                for pair in H.CASES:
                    self.fork_attempted = True
                    child = subprocess.Popen(
                        [sys.executable, "-I", "-S", "-B", "-c", code], close_fds=True
                    )
                    self.children.append(child)
                    if self.init is None:
                        self.init = child.pid
                    child.wait(timeout=2)
                if outer.withhold:
                    raise OSError("owned wait observations withheld")
                if outer.protocol_refuse:
                    raise OSError("owned protocol refusal after actual waits")
                return {"softwareResult": {"cases": list(H.CASES)}}

            def settlement_observed(self):
                return (
                    not outer.withhold
                    and len(self.children) == 3
                    and all(child.returncode == 0 for child in self.children)
                )

            def remove_group(self):
                if not self.settlement_observed():
                    raise OSError("actual software waits required")
                self.group.remove(self.journal["cgroup"])

        self.monitor_type = Monitor
        self.modules = {
            "host-transaction": self.admin,
            "host-package": self.producer,
            "bootstrap-elf": SimpleNamespace(),
            "spawn-proposal": SimpleNamespace(RECORD="owned.record"),
            "spawn-combined": C,
            "host-observer": B,
            "probe-custody": SimpleNamespace(),
            "admin-fixture": SimpleNamespace(),
            "spawn-trace": SimpleNamespace(),
            "spawn-cgroup": SimpleNamespace(LinuxCgroup=Group),
        }
        self.files = {
            "scripts/claude-agent-sdk-" + name + ".py": name.encode()
            for name in self.modules
        }
        self.files["owned.record"] = b"owned source record"
        self.session = SimpleNamespace(
            load=lambda name, data: self.modules[data.decode()]
        )
        self.args = SimpleNamespace(
            uid=os.getuid(),
            gid=os.getgid(),
            record_sha256="owned-record",
            receipt_root=str(self.receipts),
            run="1",
            cgroup_parent="owned-only",
            started=time.monotonic(),
            seconds=90,
        )

    def invoke(self):
        with (
            mock.patch.object(H, "compose", return_value=(None, self.monitor_type)),
            mock.patch.object(
                C,
                "identity",
                side_effect=lambda pid: {"pid": pid, "scope": "owned software only"},
            ),
        ):
            return H.transaction(
                self.args,
                self.session,
                self.files,
                {"recordSha256": "owned-record", "sourceRevision": "owned-source"},
                lambda: None,
            )

    def tearDown(self):
        owner = self.instances.get("owner")
        if owner is not None and owner.state["phase"] != "restored":
            self.withhold = False
            monitor = self.instances.get("monitor")
            if monitor is not None:
                for child in monitor.children:
                    if child.poll() is None:
                        child.kill()
                    child.wait(timeout=2)
                owner.finish_monitor()
                monitor.remove_group()
            owner.restore()
            self.instances["lease"].close_sentinel()
            for name in ("package", "transfer", "group"):
                value = self.instances.get(name)
                if value is not None:
                    value.close()
            descriptor = self.instances.get("descriptor")
            if descriptor is not None:
                os.close(descriptor)
        os.environ["TMPDIR"] = self.previous_tmp
        shutil.rmtree(self.root)

    def test_full_owner_sequence_keeps_root_handles_out_of_actual_owned_children(self):
        result = self.invoke()
        self.assertEqual(result["cases"], 3)
        self.assertFalse(result["qualified"])
        self.assertEqual(self.instances["owner"].state["phase"], "restored")
        self.assertTrue(
            all(child.returncode == 0 for child in self.instances["monitor"].children)
        )
        self.assertFalse(self.instances["group"].path.exists())
        for name in A.DIRECTORIES:
            self.assertEqual(stat_mode(self.root / name.lstrip("/")), 0o777)

    def test_missing_wait_observation_retains_same_owner_manifest_and_group(self):
        self.withhold = True
        with self.assertRaisesRegex(OSError, "observations withheld") as caught:
            self.invoke()
        owner, transfer, package, lease, group, monitor, descriptor = (
            caught.exception.resources
        )
        self.assertIs(owner, self.instances["owner"])
        self.assertIs(monitor, self.instances["monitor"])
        self.assertTrue(owner.unsettled())
        transfer.check()
        self.assertTrue(group.path.exists())
        for name in A.DIRECTORIES:
            self.assertEqual(stat_mode(self.root / name.lstrip("/")), 0o755)
        self.withhold = False
        owner.finish_monitor()  # Same actual owned Popen waits, never a replay.
        self.assertFalse(owner.unsettled())

    def test_known_protocol_refusal_restores_after_the_same_actual_waits(self):
        self.protocol_refuse = True
        with self.assertRaisesRegex(
            OSError, "protocol refusal after actual waits"
        ) as caught:
            self.invoke()
        self.assertFalse(hasattr(caught.exception, "resources"))
        self.assertEqual(self.instances["owner"].state["phase"], "restored")
        self.assertFalse(self.instances["group"].path.exists())

    def test_parent_prerequisite_refuses_before_any_ancestor_mutation(self):
        self.refuse_parent = True
        with self.assertRaisesRegex(OSError, "delegation refused"):
            self.invoke()
        self.assertEqual(self.instances["owner"].state["phase"], "restored")
        for name in A.DIRECTORIES:
            self.assertEqual(stat_mode(self.root / name.lstrip("/")), 0o777)


def stat_mode(path):
    import stat

    return stat.S_IMODE(path.stat().st_mode)


class MetadataTests(unittest.TestCase):
    def exercise(self, kind):
        import shlex

        patch = (
            ROOT / "docs/knowledge/operations/claude-agent-sdk-host-setup.patch"
        ).read_text()
        workflow = json.loads(
            "\n".join(
                row[1:]
                for row in patch.splitlines()
                if row.startswith("+") and not row.startswith("+++")
            )
        )
        script = workflow["jobs"]["host"]["steps"][2]["run"]
        script = script[script.index("# Metadata only:") :]
        with tempfile.TemporaryDirectory(
            prefix="metadata-owned-", dir=os.environ["TMPDIR"]
        ) as temporary:
            root = Path(temporary).resolve()
            receipts = root / "receipts"
            receipts.mkdir()
            if kind != "absent":
                (receipts / "admin.json").write_text('{"entryAttempted":false}')
            if kind == "complete":
                for name in ("monitor.json", "host-manifest.json"):
                    (receipts / name).write_text("{}")
            (root / "host-output.json").write_text(
                '{"hostSessionRefused":"owned refusal"}'
            )
            # Same published shell pipeline and actual owned tar/cat, with only
            # Linux privilege/limit/timeout launchers replaced by an owned limit shim.
            shim = root / "limit.py"
            shim.write_text(
                "import os,resource,signal,sys\nlimit=int(sys.argv[1]);resource.setrlimit(resource.RLIMIT_FSIZE,(limit,limit))\nsignal.signal(signal.SIGPIPE,signal.SIG_DFL);signal.signal(signal.SIGXFSZ,signal.SIG_DFL)\nos.execv(sys.argv[2],sys.argv[2:])\n"
            )
            command = (
                shlex.quote(sys.executable) + " -I -B -S " + shlex.quote(str(shim))
            )
            script = script.replace(
                "/usr/bin/prlimit --fsize=16384 -- /usr/bin/timeout --signal=TERM --kill-after=2s 10s",
                command + " 16384",
            )
            script = script.replace(
                "/usr/bin/prlimit --fsize=2031616 --", command + " 2031616"
            )
            script = script.replace("/usr/bin/cat", shlex.quote(shutil.which("cat")))
            if kind in ("stdout", "stderr"):
                flood = root / "flood.py"
                fd = 1 if kind == "stdout" else 2
                flood.write_text(
                    f"import os\nfor i in range(1024): os.write({fd},b'x'*4096)\n"
                )
                script = script.replace(
                    "/usr/bin/tar",
                    shlex.quote(sys.executable)
                    + " -I -B -S "
                    + shlex.quote(str(flood)),
                )
            prelude = (
                'set +e\nsudo() { shift; "$@"; }\nordinary_root='
                + shlex.quote(str(root))
                + "\nadmin_root="
                + shlex.quote(str(receipts))
                + "\nhost_exit=78\n"
            )
            result = subprocess.run(
                [shutil.which("bash"), "--noprofile", "--norc", "-c", prelude + script],
                capture_output=True,
                timeout=10,
            )
            self.assertEqual(result.returncode, 78)
            status = json.loads((root / "export-status.json").read_bytes())
            self.assertEqual(status["hostExit"], 78)
            self.assertFalse(status["qualified"])
            self.assertLessEqual((root / "export.log").stat().st_size, 16384)
            self.assertLessEqual((root / "evidence.tar").stat().st_size, 2031616)
            total = sum(
                (root / name).stat().st_size
                for name in (
                    "host-output.json",
                    "export.log",
                    "export-status.json",
                    "evidence.tar",
                )
            )
            self.assertLess(total, 2097152)
            if kind in ("stdout", "stderr"):
                self.assertNotEqual(status["metadataExportExit"], 0)
            else:
                import tarfile

                self.assertGreater(
                    (root / "evidence.tar").stat().st_size,
                    0,
                    (root / "export.log").read_text() + result.stderr.decode(),
                )
                with tarfile.open(root / "evidence.tar") as archive:
                    self.assertEqual(
                        sorted(archive.getnames()),
                        sorted(p.name for p in receipts.iterdir()),
                    )
                self.assertEqual(
                    status["missing"],
                    [
                        name
                        for name in ("admin.json", "monitor.json", "host-manifest.json")
                        if not (receipts / name).exists()
                    ],
                )
                self.assertEqual(status["metadataExportExit"] == 0, kind == "complete")

    def test_partial_absent_and_complete_exports_preserve_refusal_and_missing_facts(
        self,
    ):
        for kind in ("partial", "absent", "complete"):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_export_stdout_and_stderr_remain_inside_existing_total_bound(self):
        for kind in ("stdout", "stderr"):
            with self.subTest(kind=kind):
                self.exercise(kind)


def check_packet():
    """Exact proposal bytes and bash syntax; never apply/register/run a job."""
    packet = json.loads(
        (
            ROOT / "docs/knowledge/operations/claude-agent-sdk-host-setup.json"
        ).read_bytes()
    )
    if (
        packet.get("activationAuthorized") is not False
        or packet.get("qualified") is not False
    ):
        raise AssertionError(
            "setup packet cannot authorize activation or qualification"
        )
    source = packet["sourceCommit"]
    if len(source) != 40 or any(ch not in "0123456789abcdef" for ch in source):
        raise AssertionError("exact source commit")
    record = ROOT / packet["sourceRecord"]["path"]
    if H.sha(record.read_bytes()) != packet["sourceRecord"]["sha256"]:
        raise AssertionError("setup record drift")
    for row in record.read_text().splitlines():
        expected, name = row.split("  ", 1)
        data = (ROOT / name).read_bytes()
        committed = subprocess.check_output(
            ["git", "show", source + ":" + name], cwd=ROOT
        )
        if H.sha(data) != expected or data != committed:
            raise AssertionError("immutable source input drift: " + name)
    patch = (ROOT / packet["workflowPatch"]["path"]).read_bytes()
    if H.sha(patch) != packet["workflowPatch"]["sha256"]:
        raise AssertionError("unapplied patch drift")
    lines = patch.decode().splitlines()
    header = lines.index(next(row for row in lines if row.startswith("@@ ")))
    body = (
        "\n".join(row[1:] for row in lines[header + 1 :] if row.startswith("+")) + "\n"
    )
    if H.sha(body.encode()) != packet["workflowBlobSha256"]:
        raise AssertionError("workflow blob drift")
    workflow = json.loads(body)
    if workflow["on"] != {"workflow_dispatch": {}} or workflow["permissions"] != {
        "contents": "read"
    }:
        raise AssertionError("manual/read-only platform route")
    if set(workflow["jobs"]) != {"host"}:
        raise AssertionError("one exclusive job")
    job = workflow["jobs"]["host"]
    if job["runs-on"] != "ubuntu-26.04" or job["timeout-minutes"] != 2:
        raise AssertionError("exact route/bound")
    if (
        job["env"]["SDK240_SOURCE"] != source
        or job["env"]["SDK240_RECORD"] != packet["sourceRecord"]["sha256"]
    ):
        raise AssertionError("workflow source bindings")
    expected = packet["uploader"]
    if job["steps"][-1]["uses"] != expected:
        raise AssertionError("immutable metadata uploader")
    if job["steps"][-1]["with"]["path"] != "\n".join(
        "${{ steps.prepare.outputs.evidence }}/" + name
        for name in (
            "evidence.tar",
            "host-output.json",
            "export.log",
            "export-status.json",
        )
    ):
        raise AssertionError("exact bounded failure metadata members")
    fixture = Path(tempfile.mkdtemp(prefix="host-packet-", dir=os.environ["TMPDIR"]))
    try:
        base = packet["workflowBase"]
        prior = subprocess.check_output(
            [
                "git",
                "show",
                base["proposalHead"] + ":" + packet["workflowPatch"]["path"],
            ],
            cwd=ROOT,
        ).decode()
        original = (
            "\n".join(
                row[1:]
                for row in prior.splitlines()
                if row.startswith("+") and not row.startswith("+++")
            )
            + "\n"
        ).encode()
        if (
            H.sha(original) != base["sha256"]
            or hashlib.sha1(
                b"blob " + str(len(original)).encode() + b"\0" + original
            ).hexdigest()
            != base["gitBlob"]
        ):
            raise AssertionError("exact registered workflow base")
        destination = fixture / packet["workflowPath"]
        destination.parent.mkdir(parents=True)
        destination.write_bytes(original)
        for arguments in (["--check"], []):
            subprocess.run(
                [
                    "git",
                    "apply",
                    *arguments,
                    str((ROOT / packet["workflowPatch"]["path"]).resolve()),
                ],
                cwd=fixture,
                check=True,
                capture_output=True,
                timeout=5,
            )
        if destination.read_bytes() != body.encode():
            raise AssertionError("exact replacement workflow patch")
        for index, step in enumerate(job["steps"]):
            if "run" not in step:
                continue
            script = fixture / (str(index) + ".sh")
            script.write_text(step["run"])
            output = fixture / (str(index) + ".log")
            import resource

            def output_limit():
                resource.setrlimit(resource.RLIMIT_FSIZE, (16384, 16384))

            with output.open("xb") as stream:
                checked = subprocess.run(
                    [shutil.which("bash"), "--noprofile", "--norc", "-n", str(script)],
                    stdout=stream,
                    stderr=subprocess.STDOUT,
                    preexec_fn=output_limit,
                    timeout=10,
                )
            if checked.returncode or output.stat().st_size >= 16384:
                raise AssertionError("bounded unapplied shell syntax refused")
        print(
            "host setup JSON/YAML, source commit/record, patch/blob, two-minute job and bounded bash syntax bindings passed; unapplied"
        )
    finally:
        shutil.rmtree(fixture)


if __name__ == "__main__":
    if sys.argv[1:] == ["--check-packet"]:
        check_packet()
    else:
        unittest.main()
