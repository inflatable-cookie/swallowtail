"""Inactive one-job SDK240 host composition. No original/vendor execution mode.

Only a later exact setup/registration/one-attempt grant may invoke main on the
selected GitHub image. Ordinary selectors use owned files and modeled monitors.
"""

import argparse
import errno
import hashlib
import json
import os
from pathlib import Path
import platform
import stat
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
RECORD = ROOT / "docs/knowledge/operations/claude-agent-sdk-bootstrap-session.sha256"
CASES = (("probe", "absent"), ("sidecar", "absent"), ("sidecar", "managed-deny"))


def sha(data):
    return hashlib.sha256(data).hexdigest()


def bounded_json(path, uid, bound=65536):
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        info = os.fstat(fd)
        if (
            not stat.S_ISREG(info.st_mode)
            or info.st_uid != uid
            or stat.S_IMODE(info.st_mode) != 0o600
            or info.st_nlink != 1
        ):
            raise OSError("host_receipt_identity_or_mode")
        raw = os.read(fd, bound + 1)
        stable = lambda row: (
            row.st_dev,
            row.st_ino,
            row.st_uid,
            row.st_gid,
            row.st_mode,
            row.st_nlink,
            row.st_size,
            row.st_mtime_ns,
            row.st_ctime_ns,
        )
        if (
            len(raw) > bound
            or stable(os.fstat(fd)) != stable(info)
            or stable(os.stat(path, follow_symlinks=False)) != stable(info)
        ):
            raise OSError("host_receipt_bound_or_drift")
        return json.loads(raw)
    finally:
        os.close(fd)


class PackageLease:
    """Root foundation is never released by a per-case staged-file ack."""

    def __init__(self, package, producer, scratch):
        self.package, self.producer, self.scratch = package, producer, Path(scratch)
        self.mode = 0o777
        self.stage = None
        self.roots = []
        self.closed = False

    def begin(self, root):
        root = Path(root)
        if (
            root.resolve(strict=True) != root
            or self.scratch not in root.parents
            or root.stat().st_uid != self.package.files.uid
            or stat.S_IMODE(root.stat().st_mode) != 0o700
        ):
            raise OSError("private_owned_stage_required")
        if self.stage is None:
            self.stage = self.producer.ImageFiles(uid=self.package.files.uid)
        if root not in self.roots:
            if len(self.roots) >= 2:
                raise OSError("one_observer_and_one_fake_stage_only")
            self.roots.append(root)

    def bind(self, name, expected=None):
        path = Path(name).absolute()
        foundation = self.package.files
        if str(path) in foundation.files:
            if expected is not None and foundation.files[str(path)][2] != expected:
                raise OSError("foundation_expected_source_changed")
            return
        if (
            self.stage is None
            or not any(root in path.parents for root in self.roots)
            or path.resolve(strict=True) != path
        ):
            raise OSError("outside_owned_stage_or_package")
        canonical, _ = self.stage.read(str(path))
        if (
            canonical != str(path)
            or expected is not None
            and self.stage.files[canonical][2] != expected
        ):
            raise OSError("staged_alias_or_bytes_changed")
        if (
            len(foundation.files) + len(self.stage.files) > 512
            or len(foundation.directories) + len(self.stage.directories) > 256
            or foundation.bytes + self.stage.bytes > 536870912
        ):
            raise OSError("combined_package_stage_bound")

    def check(self):
        if self.closed:
            raise OSError("root_package_lease_closed")
        self.package.files.check(candidate_mode=self.mode)
        if self.stage is not None:
            self.stage.check()

    def observation(self):
        self.check()
        return {
            "manifestSha256": sha(
                json.dumps(self.package.manifest, sort_keys=True).encode()
            ),
            "candidateMode": self.mode,
            "foundationFiles": len(self.package.files.files),
            "stagedFiles": 0 if self.stage is None else len(self.stage.files.files),
            "trustedHostOnly": True,
            "originalActivation": False,
        }

    def release_stage(self):
        self.check()
        if self.stage is not None:
            self.stage.close()
        self.stage, self.roots = None, []

    def leaf(self, receipt, mode):
        expected_phase = "prepared" if mode == 0o755 else "restored"
        value = bounded_json(receipt, self.package.files.uid)
        manifest = self.package.manifest
        fd, before, _, _ = self.package.files.files[manifest["path"]]
        stable = list(before[:4] + before[5:7])
        if (
            mode not in (0o755, 0o777)
            or value["phase"] != expected_phase
            or value["path"] != manifest["path"]
            or value["sha256"] != manifest["sha256"]
            or value["identity"] != stable
            or value["originalMode"] != 0o777
            or value["targetMode"] != 0o755
            or value.get("children")
        ):
            raise OSError("exact_ordinary_leaf_receipt_required")
        # This is the root's source-bound session step, not a public mode flag or
        # candidate attestation. All package bytes/aliases/search stay checked.
        self.package.files.check(candidate_mode=mode)
        self.mode = mode
        self.check()
        return value


def remaining(deadline, maximum, reserve=3):
    value = min(maximum, int(deadline - time.monotonic() - reserve))
    if value <= 0:
        raise OSError(errno.ETIMEDOUT, "host_transaction_budget_exhausted")
    return value


def compose(combined, boundary):
    """Reuse the existing finite monitor; no alternate kernel implementation."""

    class Guest(combined.GuestContext):
        def __init__(self, monitor, read, write):
            super().__init__(
                monitor.runtime, monitor.args, read, write, {}, monitor.files
            )
            self.deadline = monitor.deadline - 3
            self.loader = monitor.loader
            self.observer_command = monitor.custody.observer_command
            self.session_module = monitor.session_module
            self.observer = None
            self.capture_runtime = None

        def leaf_prepared(self, receipt):
            self.request("leaf_prepared", receipt=str(receipt))

        def leaf_restored(self, receipt):
            self.request("leaf_restored", receipt=str(receipt))

        def case_begin(self, mode, state):
            self.request("case_begin", mode=mode, state=state)
            self.layout = None
            self.observer = boundary.GuestObserver(self, self.files)

        def observer_prepare(self, command, kwargs, bootstrap):
            return self.observer.prepare(command, kwargs, bootstrap)

        def observer_started(self, child):
            return self.observer.started(child)

        def observer_joined(self, child):
            return self.observer.joined(child)

        def before_fixture_cleanup(self, root, proof):
            self.request("fixture_waits", root=str(root), proof=proof)

        def case_end(self, result):
            # Core already closed its capture and identity-checked its own stage
            # after the root acknowledgement. Observer files are separately owned.
            self.request(
                "case_end",
                failure=result.get("failure"),
                driverWait=result.get("driverWait"),
            )
            self.observer.release()

    class BatchMonitor(combined.Monitor):
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
            session_module,
            custody,
            elf,
            transfer,
            fixture,
        ):
            super().__init__(args, core, files, modules, {}, lease, group, journal)
            self.session_module, self.custody, self.elf = session_module, custody, elf
            self.transfer, self.fixture = transfer, fixture
            self.package = package
            self.loader = str(Path("/lib64/ld-linux-x86-64.so.2").resolve(strict=True))
            self.deadline = args.deadline
            self.phases, self.case_rows = [], []
            self.observer = None
            self.case = None
            self.case_ack = None
            self.leaf_restored = False
            self.fixture_fd = None
            self.group_removed = False
            self.refusal = None
            self.primary = remaining(self.deadline, 90)

        def check_custody(self):
            super().check_custody()
            if self.journal.get("fixtureStage") != "ready":
                return
            receipt = self.journal["fixture"]
            fd = self.fixture_fd
            info = os.fstat(fd)
            path = Path("/etc/claude-code").lstat()
            if (info.st_dev, info.st_ino, info.st_uid, stat.S_IMODE(info.st_mode)) != (
                receipt["device"],
                receipt["inode"],
                0,
                0o755,
            ) or (path.st_dev, path.st_ino) != (info.st_dev, info.st_ino):
                raise OSError("canonical_fixture_drift")
            expected = {
                self.fixture.LEASE: ((json.dumps(receipt) + "\n").encode(), 0o600)
            }
            data = self.fixture.payload(receipt["state"])
            if data is not None:
                expected[self.fixture.POLICY] = (data, 0o444)
            if set(os.listdir(fd)) != set(expected):
                raise OSError("canonical_fixture_members_drift")
            for name, (data, mode) in expected.items():
                if self.fixture.read_file(fd, name, 0, mode) != data:
                    raise OSError("canonical_fixture_bytes_drift")

        def remove_group(self):
            if not self.settlement_observed():
                raise OSError("actual_all_phase_and_init_waits_required")
            self.group.remove(self.journal["cgroup"])
            self.group_removed = True  # Only after this identity-bound actual effect.
            self.journal["cgroupStage"] = "removed"
            self.save()

        def guest_fds(self):
            self.transfer.check()
            return {self.transfer.read_fd}

        def primary_seconds(self):
            return self.primary

        def pin(self, pid):
            info = combined.identity(pid)
            if pid in self.seen and self.seen[pid]["start"] != info["start"]:
                raise OSError("host_pid_reuse_refused")
            return super().pin(pid)

        def guest(self, answer_read, write):
            # Base child closed every administrative handle before UID/cap drop.
            session = self.session_module
            raw = session.manifest_bytes(
                self.transfer.expected, descriptor=self.transfer.read_fd
            )
            manifest = json.loads(raw)
            context = Guest(self, answer_read, write)
            context.metadata = context.request("ready")
            owner = session.Session(
                self.custody,
                self.elf,
                manifest,
                Path(os.environ["TMPDIR"]) / "session.json",
                source_inputs={
                    str(ROOT / name): sha(data) for name, data in self.files.items()
                },
            )
            owner.preparation.deadline = min(
                owner.preparation.deadline, self.deadline - 3
            )
            cases = owner.run(self.core, self.args.record_sha256, context=context)
            context.finish({"failure": None, "cases": cases, "leafRestored": True})
            os._exit(0)

        def trace_refused(self, row, error):
            self.refusal = str(error)[:1000]
            # An unapproved committed image cannot run to perform a callback.
            # Stop only the actor already pinned by this live gate, then leave
            # its source parent/supervisor an opportunity to observe the wait.
            pid = row["pid"]
            if pid in self.handles:
                import signal

                try:
                    signal.pidfd_send_signal(self.handles[pid], signal.SIGKILL)
                    self.trace.resume(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            self.journal["entryRefusal"] = self.refusal

        def handle(self, request):
            if self.refusal is not None and request["kind"] not in {
                "identity",
                "stop_actor",
                "observer_joined",
                "fixture_waits",
                "leaf_restored",
            }:
                return {"error": self.refusal, "errno": errno.EPERM}
            try:
                return self.handle_owned(request)
            except (OSError, ValueError, KeyError) as error:
                self.refusal = str(error)[:1000]
                self.journal["entryRefusal"] = self.refusal
                # Reply on the existing private pipe. The ordinary parent gets
                # its normal stop/wait/finally path before outer group stop.
                return {"error": self.refusal, "errno": errno.EPERM}

        def handle_owned(self, request):
            kind = request["kind"]
            if kind in {"leaf_prepared", "leaf_restored"}:
                receipt = Path(os.environ["TMPDIR"]) / "session.json"
                if request["receipt"] != str(receipt):
                    raise OSError("ordinary_receipt_path_changed")
                if kind == "leaf_prepared":
                    if self.case_rows or self.lease.mode != 0o777:
                        raise OSError("duplicate_leaf_preparation")
                    self.lease.leaf(receipt, 0o755)
                else:
                    if len(self.case_rows) != 3 or self.case is not None:
                        raise OSError("leaf_restore_before_all_case_waits")
                    self.lease.leaf(receipt, 0o777)
                    self.leaf_restored = True
                return {}
            if kind == "case_begin":
                pair = (request["mode"], request["state"])
                if (
                    self.case is not None
                    or len(self.case_rows) >= 3
                    or pair != CASES[len(self.case_rows)]
                    or self.lease.mode != 0o755
                ):
                    raise OSError("three_exact_cases_in_order_required")
                self.case = pair
                self.case_ack = None
                self.observer = boundary.RootObserver(
                    self, self.custody.observer_command, combined.identity
                )
                self.phases.append(self.observer)
                self.journal.update(fixtureStage="not-started", fixture=None)

                def created(stage, receipt):
                    self.journal.update(fixtureStage=stage, fixture=receipt)
                    self.save()

                combined.prepare_fixture(
                    Path("/etc"), 0, pair[1], self.args.run, self.fixture, created
                )
                self.fixture_fd = os.open(
                    "/etc/claude-code", os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
                )
                self.journal["fixtureStage"] = "ready"
                return {}
            if kind == "observer_intent":
                if self.case is None or self.observer is None:
                    raise OSError("observer_without_case")
                self.journal["observerIntent"] = True
                self.save()  # Before any possible owned observer Popen.
                self.lease.begin(request["packet"]["root"])
                return self.observer.intent(request["packet"])
            if kind == "observer_started":
                return self.observer.started(request["namespacePid"])
            if kind == "observer_joined":
                return self.observer.joined(request["namespacePid"], request["code"])
            if kind == "layout":
                if self.observer is None or not self.observer.settled:
                    raise OSError("fake_layout_before_observer_wait")
                self.lease.begin(request["packet"]["root"])
            if kind == "fixture_waits":
                if self.case_ack is not None:
                    raise OSError("duplicate_fixture_cleanup_ack")
                ack = boundary.fixture_settlement(
                    self, request["root"], request["proof"]
                )
                self.journal["fixtureWaitAcknowledgement"] = ack
                self.save()  # A failed receipt retains stage bindings and owner.
                self.case_ack = {"ack": ack, "gate": self.gate}
                self.lease.release_stage()
                if self.control_fd is not None:
                    os.close(self.control_fd)
                    self.control_fd = None
                    self.control_identity = None
                return ack
            if kind == "case_end":
                if (
                    self.case_ack is None
                    or request["failure"] is not None
                    or request["driverWait"] != 0
                ):
                    raise OSError("case_result_or_actual_wait_missing")
                combined.cleanup_partial_fixture(
                    Path("/etc"), 0, self.journal["fixture"], self.fixture
                )
                os.close(self.fixture_fd)
                self.fixture_fd = None
                row = {
                    "case": self.case,
                    "ack": self.case_ack["ack"],
                    "gate": self.case_ack["gate"],
                }
                self.case_rows.append(row)
                self.case = None
                self.packet = self.gate = None
                self.layout_header = None
                self.layout_data.clear()
                self.journal.update(fixtureStage="removed", fixture=None)
                return {}
            return super().handle(request)

        def save(self):
            self.journal["hostCases"] = [row["ack"] for row in self.case_rows]
            self.journal["observerPhases"] = [
                {"attempted": row.attempted, "settled": row.settled}
                for row in self.phases
            ]
            if self.refusal is not None:
                self.journal["custodyFailure"] = self.refusal
                self.journal["traceWaits"] = self.trace.waits
                combined.persist_journal(
                    self.journal_fd, self.journal, self.journal_parent
                )
            else:
                super().save()

        def settlement_observed(self):
            if (
                self.refusal is not None
                or not self.leaf_restored
                or self.case is not None
                or len(self.case_rows) != 3
                or self.result is None
                or self.result.get("failure") is not None
                or not self.journal.get("treeSettled")
                or self.journal.get("failure")
                or not (self.group_removed or self.group.empty())
                or self.journal.get("fixtureStage") != "removed"
            ):
                return False
            observed = {
                (row["actor"], row["pid"], row["status"])
                for row in self.trace.waits
                if os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"])
            }
            if not any(
                row["pid"] == self.init
                and row["status"] == 0
                and row["actor"] == os.getpid()
                and (row["actor"], row["pid"], row["status"]) in observed
                for row in self.waits
            ):
                return False
            gates = [row.gate for row in self.phases] + [
                row["gate"] for row in self.case_rows
            ]
            if not all(row.settled for row in self.phases) or len(gates) != 6:
                return False
            return all(
                gate is not None
                and gate.complete()
                and all(
                    (
                        task["terminal"]["actor"],
                        task["terminal"]["pid"],
                        task["terminal"]["status"],
                    )
                    in observed
                    for task in gate.tasks.values()
                )
                for gate in gates
            )

    return Guest, BatchMonitor


def trusted_entry(args):
    """Initial GitHub administration is the finite trust root, not Node."""
    import re

    if (
        platform.system() != "Linux"
        or platform.machine() != "x86_64"
        or os.getuid() != 0
        or os.geteuid() != 0
        or not sys.flags.isolated
        or not sys.flags.no_site
        or not sys.dont_write_bytecode
    ):
        raise OSError("isolated_root_Linux_x64_administration_required")
    if (
        args.image_os != "ubuntu26"
        or args.image_version != "20261111.196.1"
        or args.uid != 1001
        or args.gid != 1001
        or args.cgroup_parent != "/sys/fs/cgroup"
        or not 1 <= args.seconds <= 90
        or not re.fullmatch(r"[1-9][0-9]{0,19}", args.run)
        or not re.fullmatch(r"[1-9][0-9]{0,3}", args.attempt)
        or args.repository != "inflatable-cookie/swallowtail"
        or not re.fullmatch(r"[a-f0-9]{40}", args.source_revision)
    ):
        raise OSError("exact_selected_exclusive_GitHub_job_required")
    allowed = {"PATH", "HOME", "TMPDIR", "TMP", "TEMP", "LANG", "LC_ALL"}
    if (
        set(os.environ) - allowed
        or os.environ.get("PATH") != "/usr/bin:/bin"
        or os.environ.get("LANG") != "C"
        or os.environ.get("LC_ALL") != "C"
    ):
        raise OSError("explicit_credential_free_administrative_environment_required")
    scratch = Path(os.environ["TMPDIR"])
    receipt = Path(args.receipt_root)
    for path, uid in ((scratch, args.uid), (receipt, 0)):
        info = path.lstat()
        if (
            path.resolve(strict=True) != path
            or not stat.S_ISDIR(info.st_mode)
            or info.st_uid != uid
            or stat.S_IMODE(info.st_mode) != 0o700
        ):
            raise OSError("exclusive_private_allocations_required")
    if scratch == receipt or scratch in receipt.parents or receipt in scratch.parents:
        raise OSError("separate_administrative_and_ordinary_allocations_required")
    if (ROOT / ".git/HEAD").read_text().strip() != args.source_revision:
        raise OSError("exact_detached_source_revision_required")
    raw_os = Path("/etc/os-release").read_bytes()
    if len(raw_os) > 65536:
        raise OSError("os_release_bound")
    os_fields = dict(
        row.split("=", 1) for row in raw_os.decode().splitlines() if "=" in row
    )
    if (
        os_fields.get("ID", "").strip('"') != "ubuntu"
        or os_fields.get("VERSION_ID", "").strip('"') != "26.04"
        or not os_fields.get("VERSION", "").strip('"').startswith("26.04.1 ")
    ):
        raise OSError("selected_image_os_release_required")
    source = ROOT / "scripts/claude-agent-sdk-bootstrap-session.py"
    raw = source.read_bytes()
    if sha(raw) != args.bootstrap_sha256:
        raise OSError("independent_bootstrap_source_binding_required")
    namespace = {"__name__": "sdk240_initial_reader", "__file__": str(source)}
    exec(compile(raw, str(source), "exec"), namespace)
    session = namespace["load"]("scripts/claude-agent-sdk-bootstrap-session.py", raw)
    files = session.source_packet(RECORD, args.record_sha256)
    files[str(RECORD.relative_to(ROOT))] = RECORD.read_bytes()
    if (
        files.get("scripts/claude-agent-sdk-host-session.py")
        != Path(__file__).read_bytes()
    ):
        raise OSError("host_entry_source_not_bound")
    # The selected service image/job is the independently supplied authority.
    # Root reads fresh boot/tool/source identities before effects; no old boot or
    # candidate-produced report supplies these values.
    binding = {
        "host": session.host_identity(),
        "sourceRevision": args.source_revision,
        "recordSha256": args.record_sha256,
        "imageOS": args.image_os,
        "imageVersion": args.image_version,
        "run": args.run,
        "attempt": args.attempt,
        "repository": args.repository,
        "uid": args.uid,
        "gid": args.gid,
        "seconds": args.seconds,
        "osReleaseSha256": sha(raw_os),
        "pythonSha256": sha(Path("/proc/self/exe").read_bytes()),
        "timeoutSha256": sha(Path(f"/proc/{os.getppid()}/exe").read_bytes()),
    }
    if Path(f"/proc/{os.getppid()}/exe").resolve(strict=True) != Path(
        "/usr/bin/timeout"
    ).resolve(strict=True):
        raise OSError("independent_finite_timeout_parent_required")

    def inspect_inputs():
        if (
            sha(Path("/etc/os-release").read_bytes()) != binding["osReleaseSha256"]
            or session.host_identity() != binding["host"]
            or sha(Path("/proc/self/exe").read_bytes()) != binding["pythonSha256"]
            or sha(Path(f"/proc/{os.getppid()}/exe").read_bytes())
            != binding["timeoutSha256"]
        ):
            raise OSError("fresh_host_or_administrative_tool_drift")
        session.source_packet(RECORD, binding["recordSha256"])

    return session, files, binding, inspect_inputs


def transaction(args, session, files, binding, inspect_inputs):
    """One actual root owner through package, three cases, waits and restore."""
    loaded = {}

    def module(short):
        if short not in loaded:
            name = "scripts/claude-agent-sdk-" + short + ".py"
            loaded[short] = session.load(name, files[name])
        return loaded[short]

    admin, producer, elf = (
        module("host-transaction"),
        module("host-package"),
        module("bootstrap-elf"),
    )
    core, combined, boundary = (
        module("spawn-proposal"),
        module("spawn-combined"),
        module("host-observer"),
    )
    custody, fixture = module("probe-custody"), module("admin-fixture")
    modules = {"trace": module("spawn-trace"), "cgroup": module("spawn-cgroup")}
    deadline = (
        args.started + args.seconds
    )  # Includes initial source checks/setup/package.
    args.deadline = deadline
    root = Path(args.receipt_root)
    owner = admin.AdministrativeOwner(
        root / "admin.json", binding, inspect_inputs=inspect_inputs, uid=args.uid
    )
    transfer = package = lease = group = monitor = None
    descriptor = None
    journal = {
        "sourceRecord": args.record_sha256,
        "directory": [root.stat().st_dev, root.stat().st_ino],
        "run": args.run,
        "fixture": None,
        "fixtureStage": "not-started",
        "cgroup": None,
        "cgroupStage": "not-started",
        "treeSettled": False,
        "originalActivation": False,
        "monitor": combined.identity(os.getpid()),
    }
    effect_pending = False

    def save():
        combined.persist_journal(descriptor, journal, root)

    try:
        group = modules["cgroup"].LinuxCgroup(
            args.cgroup_parent, "sdk234-" + os.urandom(12).hex()
        )
        transfer = admin.ManifestTransfer(owner, root / "host-manifest.json")
        owner.prepare()
        package = producer.produce(owner, session, elf, files, uid=args.uid)
        transfer.publish(package.manifest)
        lease = PackageLease(package, producer, Path(os.environ["TMPDIR"]))
        descriptor = os.open(
            root / "monitor.json",
            os.O_CREAT | os.O_EXCL | os.O_RDWR | os.O_NOFOLLOW,
            0o600,
        )
        args.journal_fd = descriptor
        args.profile, args.state = "sdk", "absent"
        args.node = package.manifest["path"]
        args.record_sha256 = sha(files[str(core.RECORD)])
        # A separately prepared exclusive parent is required. This constructor
        # never enables controllers or repairs parent ownership/permissions.
        journal["cgroupStage"] = "creating"
        effect_pending = True
        save()
        journal["cgroup"] = group.create()
        journal["cgroupStage"] = "created"
        save()
        effect_pending = False
        outside = root / "outside-readable-owned-sentinel"
        fd = os.open(
            outside, os.O_CREAT | os.O_EXCL | os.O_WRONLY | os.O_NOFOLLOW, 0o444
        )
        try:
            os.write(fd, b"owned boundary sentinel")
            os.fsync(fd)
        finally:
            os.close(fd)
        journal["outsidePath"] = str(outside)
        save()
        _, batch = compose(combined, boundary)
        monitor = batch(
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
        )
        result = owner.monitor_entry(monitor)
        # Actual software/tracer/init waits have already matched. Remove only
        # the identity-held empty group, then preserve those live observations
        # through the administrative reverse restore.
        effect_pending = True
        monitor.remove_group()
        effect_pending = False
        lease.check()
        owner.restore()
        package.close()
        transfer.close()
        group.close()
        os.close(descriptor)
        return {
            "failure": None,
            "sourceRecord": binding["recordSha256"],
            "sourceRevision": binding["sourceRevision"],
            "treeSettled": True,
            "cases": len(result["softwareResult"]["cases"]),
            "ancestorsRestored": True,
            "candidateRole": "fake-only Node22.23.4",
            "originalActivation": False,
            "qualified": False,
        }
    except BaseException as error:
        # Known no-spawn refusal may restore only before any ambiguous resource
        # effect, with no child/observer/kernel entry and exact held identities.
        try:
            safe = (
                not effect_pending
                and (monitor is None or not monitor.fork_attempted)
                and journal["fixtureStage"] == "not-started"
                and not owner.unsettled()
            )
        except BaseException as ownership_error:
            ownership_error.resources = (
                owner,
                transfer,
                package,
                lease,
                group,
                monitor,
                descriptor,
            )
            raise ownership_error from error
        if safe:
            try:
                if group is not None and group.created:
                    group.remove(journal["cgroup"])
                owner.restore()
                if package is not None:
                    package.close()
                if transfer is not None:
                    transfer.close()
                if group is not None:
                    group.close()
                if descriptor is not None:
                    os.close(descriptor)
            except BaseException as recovery_error:
                recovery_error.resources = (
                    owner,
                    transfer,
                    package,
                    lease,
                    group,
                    monitor,
                    descriptor,
                )
                raise recovery_error from error
        else:
            error.resources = (
                owner,
                transfer,
                package,
                lease,
                group,
                monitor,
                descriptor,
            )
        raise


def main():
    started = time.monotonic()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--record-sha256", required=True)
    parser.add_argument("--bootstrap-sha256", required=True)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--image-os", required=True)
    parser.add_argument("--image-version", required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--run", required=True)
    parser.add_argument("--attempt", required=True)
    parser.add_argument("--receipt-root", required=True)
    parser.add_argument("--cgroup-parent", required=True)
    parser.add_argument("--seconds", type=int, default=90)
    parser.add_argument("--uid", type=int, default=1001)
    parser.add_argument("--gid", type=int, default=1001)
    parser.add_argument("--recover-only", action="store_true")
    args = parser.parse_args()
    args.started = started
    import resource

    resource.setrlimit(resource.RLIMIT_NOFILE, (1024, 1024))
    session, files, binding, inspect_inputs = trusted_entry(args)
    if args.recover_only:
        name = "scripts/claude-agent-sdk-host-transaction.py"
        admin = session.load(name, files[name])
        # A fresh owner can recover only a proven no-entry prefix; it cannot
        # inherit old monitor/tool waits from a serialized receipt.
        monitor_receipt = Path(args.receipt_root) / "monitor.json"
        if monitor_receipt.exists():
            value = bounded_json(monitor_receipt, 0, 262144)
            if (
                value.get("cgroupStage") != "not-started"
                or value.get("fixtureStage") != "not-started"
            ):
                raise OSError("same_live_kernel_resource_owner_required_for_recovery")
        owner = admin.AdministrativeOwner.recover(
            Path(args.receipt_root) / "admin.json",
            binding,
            inspect_inputs=inspect_inputs,
            uid=args.uid,
        )
        owner.restore()
        result = {"knownNoEntryRecovery": True, "qualified": False}
    else:
        result = transaction(args, session, files, binding, inspect_inputs)
    print(json.dumps(result, sort_keys=True))  # Fixed small result; never a file dump.


if __name__ == "__main__":
    try:
        main()
    except BaseException as error:
        print(
            json.dumps({"hostSessionRefused": str(error)[:2000], "qualified": False}),
            flush=True,
        )
        if getattr(error, "resources", None):
            # Retain the actual owner/handles; never voluntarily drop an unknown
            # child to manufacture cleanup. The independently bound timeout owns
            # the finite90s+2s ceiling. Its forced disposal is NOT settlement.
            import signal

            signal.signal(signal.SIGTERM, lambda *unused: None)
            while True:
                time.sleep(0.05)
        raise SystemExit(78)
