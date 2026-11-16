"""Private five-ancestor administrative owner; live host use remains gated.

Only owned temporary directories/processes exercise mutations in validation.
No ordinary Inputs/Lease rule is changed and no installed helper is created.
"""

import json
import hashlib
import os
from pathlib import Path
import stat
import selectors
import subprocess
import time

CANDIDATE = "/opt/hostedtoolcache/node/22.23.4/x64/bin/node"
DIRECTORIES = tuple(
    str(p)
    for p in reversed(Path(CANDIDATE).parent.parents)
    if str(p).startswith("/opt/hostedtoolcache")
) + (str(Path(CANDIDATE).parent),)
MAX_JOURNAL = 65536


class Refused(OSError):
    pass


def identity(info):
    return [
        info.st_dev,
        info.st_ino,
        info.st_uid,
        info.st_gid,
        info.st_mode,
        info.st_ctime_ns,
    ]


class AdministrativeOwner:
    def __init__(
        self, journal, binding, *, inspect_inputs, root=Path("/"), uid=1001, fault=None
    ):
        if not callable(inspect_inputs):
            raise Refused("independent_administrative_input_checker_required")
        inspect_inputs()
        self.pid = os.getpid()
        self.root, self.uid = Path(root).absolute(), uid
        self.journal, self.binding = Path(journal), json.loads(json.dumps(binding))
        self.fault = fault or (lambda phase, name: None)
        self.rows, self.child = [], None
        self.tool_child = None
        self._scope, self._tree_settled = None, False
        self.retained_inputs = None
        self._waited_child = None
        self._effect_possible = False
        self._effect_observed = None
        self.check_inputs = inspect_inputs
        self.deadline = time.monotonic() + 15
        self.state = {
            "schema": "sdk240.admin.v1",
            "binding": self.binding,
            "rows": [],
            "operation": None,
            "entryAttempted": False,
            "wait": None,
            "toolIntent": None,
            "toolWaits": [],
            "phase": "opening",
        }
        self.journal_fd = None
        self.parent_fd = None
        parent = self.journal.parent
        st = parent.lstat()
        if (
            parent.resolve(strict=True) != parent.absolute()
            or not stat.S_ISDIR(st.st_mode)
            or st.st_mode & 0o022
            or st.st_uid != os.geteuid()
        ):
            raise Refused("admin_private_journal_parent")
        try:
            self.parent_fd = os.open(
                parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
            )
            self.journal_fd = os.open(
                self.journal.name,
                os.O_RDWR | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
                0o600,
                dir_fd=self.parent_fd,
            )
            self.store()
        except BaseException:
            if self.journal_fd is not None:
                os.close(self.journal_fd)
            if self.parent_fd is not None:
                os.close(self.parent_fd)
            raise

    def point(self, phase, name):
        self.fault(phase, name)

    def store(self):
        if os.getpid() != self.pid:
            raise Refused("administrative_owner_changed")
        raw = (json.dumps(self.state, sort_keys=True) + "\n").encode()
        if len(raw) > MAX_JOURNAL:
            raise Refused("admin_journal_bound")
        fd = self.journal_fd
        st = os.fstat(fd)
        if (
            not stat.S_ISREG(st.st_mode)
            or stat.S_IMODE(st.st_mode) != 0o600
            or st.st_uid != os.geteuid()
            or st.st_nlink != 1
        ):
            raise Refused("admin_journal_type_owner_mode_or_link")
        if (st.st_dev, st.st_ino) != (
            self.journal.lstat().st_dev,
            self.journal.lstat().st_ino,
        ):
            raise Refused("admin_journal_replaced")
        os.lseek(fd, 0, os.SEEK_SET)
        view = memoryview(raw)
        while view:
            count = os.write(fd, view)
            if count <= 0:
                raise Refused("admin_journal_short_write")
            view = view[count:]
        os.ftruncate(fd, len(raw))
        os.fsync(fd)
        os.fsync(self.parent_fd)

    def check(self):
        self.check_inputs()
        self.check_ancestors()

    def check_ancestors(self):
        if os.getpid() != self.pid:
            raise Refused("administrative_owner_changed")
        for handle in (self.journal_fd, self.parent_fd):
            if handle is not None and os.get_inheritable(handle):
                raise Refused("administrative_handle_inheritable")
        for row in self.rows:
            expected = row["current"]
            if (
                identity(os.fstat(row["fd"])) != expected
                or identity(row["actual"].lstat()) != expected
            ):
                raise Refused("admin_ancestor_identity_drift:" + row["name"])
            if os.get_inheritable(row["fd"]):
                raise Refused("administrative_handle_inheritable")

    def prepare(self):
        try:
            current = self.root
            names = ["/"] + [
                "/" + "/".join(Path(CANDIDATE).parent.parts[1:i])
                for i in range(2, len(Path(CANDIDATE).parent.parts) + 1)
            ]
            for name in names:
                if time.monotonic() > self.deadline:
                    raise Refused("admin_prepare_deadline")
                actual = self.root if name == "/" else self.root / name.lstrip("/")
                flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
                fd = os.open(
                    actual if not self.rows else actual.name,
                    flags,
                    **({} if not self.rows else {"dir_fd": self.rows[-1]["fd"]}),
                )
                try:
                    info = os.fstat(fd)
                except BaseException:
                    os.close(fd)
                    raise
                row = {
                    "name": name,
                    "actual": actual,
                    "fd": fd,
                    "original": identity(info),
                    "current": identity(info),
                    "changed": False,
                }
                self.rows.append(row)
                self.state["rows"].append(
                    {k: row[k] for k in ("name", "original", "current", "changed")}
                )
                mode = stat.S_IMODE(info.st_mode)
                if not stat.S_ISDIR(info.st_mode) or info.st_uid not in (0, self.uid):
                    raise Refused("admin_ancestor_type_or_owner")
                if name not in DIRECTORIES:
                    if info.st_mode & 0o022:
                        raise Refused("unrepairable_outer_ancestor:" + name)
                elif mode not in (0o755, 0o777):
                    raise Refused("admin_ancestor_mode:" + name)
                self.check()
                self.store()
                if name in DIRECTORIES and mode == 0o777:
                    self.mutate(row, 0o755, "prepare")
            if (
                tuple(row["name"] for row in self.rows if row["name"] in DIRECTORIES)
                != DIRECTORIES
            ):
                raise Refused("exact_five_ancestors_required")
            self.state["phase"] = "prepared"
            self.store()
            self.check()
            return self
        except BaseException as error:
            error.owner = self
            raise

    def prepared(self):
        self.check()
        if (
            self.state["phase"] != "prepared"
            or tuple(row["name"] for row in self.rows if row["name"] in DIRECTORIES)
            != DIRECTORIES
            or any(
                stat.S_IMODE(row["current"][4]) != 0o755
                for row in self.rows
                if row["name"] in DIRECTORIES
            )
        ):
            raise Refused("complete_prepared_chain_required")

    def mutate(self, row, mode, operation):
        name = row["name"]
        self.point(operation + "_before_intent", name)
        self.check()
        self.state["operation"] = {"name": name, "kind": operation, "targetMode": mode}
        self._effect_possible = False
        self._effect_observed = None
        self.store()
        self.point(operation + "_after_intent", name)
        # Once the effect may have happened, a missing receipt is never inferred.
        self._effect_possible = True
        os.fchmod(row["fd"], mode)
        os.fsync(row["fd"])
        observed = identity(os.fstat(row["fd"]))
        if observed[:4] != row["current"][:4] or stat.S_IMODE(observed[4]) != mode:
            raise Refused("admin_mutation_identity_or_mode")
        self._effect_observed = (row["fd"], list(observed))
        row["current"] = observed
        row["changed"] = operation == "prepare"
        index = self.rows.index(row)
        self.state["rows"][index].update(current=observed, changed=row["changed"])
        self.point(operation + "_after_effect", name)
        pending = self.state["operation"]
        self.state["operation"] = None
        try:
            self.store()
        except BaseException:
            self.state["operation"] = pending
            raise
        self.point(operation + "_after_receipt", name)

    def tool(self, command, *, environment, bound=16384):
        self.check()
        if self.state["toolIntent"] is not None:
            raise Refused("unknown_administrative_tool_retained")
        self.state["toolIntent"] = {"command": list(command)}
        self.store()
        output = bytearray()
        try:
            self.tool_child = subprocess.Popen(
                command,
                env=environment,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                close_fds=True,
            )
            self.state["toolIntent"]["pid"] = self.tool_child.pid
            self.store()
            deadline = time.monotonic() + 5
            with selectors.DefaultSelector() as poll:
                poll.register(self.tool_child.stdout, selectors.EVENT_READ)
                while poll.get_map():
                    remaining = deadline - time.monotonic()
                    if remaining <= 0:
                        raise Refused("administrative_tool_deadline")
                    ready = poll.select(remaining)
                    if not ready:
                        raise Refused("administrative_tool_deadline")
                    data = os.read(
                        self.tool_child.stdout.fileno(),
                        min(4096, bound + 1 - len(output)),
                    )
                    if not data:
                        poll.unregister(self.tool_child.stdout)
                    output.extend(data)
                    if len(output) > bound:
                        raise Refused("administrative_tool_output_bound")
            code = self.tool_child.wait(timeout=max(0.001, deadline - time.monotonic()))
            return code, bytes(output)
        finally:
            if self.tool_child is not None:
                self.settle_tool()

    def settle_tool(self):
        if self.tool_child is None:
            raise Refused("administrative_tool_wait_unknown")
        if self.tool_child.poll() is None:
            self.tool_child.kill()
        code = self.tool_child.wait(timeout=2)
        self.state["toolWaits"].append({"pid": self.tool_child.pid, "code": code})
        self.state["toolIntent"] = None
        self.store()
        self.tool_child.stdout.close()
        self.tool_child = None

    def unsettled(self):
        return (
            self.state["toolIntent"] is not None
            or self.state["operation"] is not None
            or self.state["entryAttempted"]
            and (
                self.child is None
                or self._waited_child is not self.child
                or not self._tree_settled
            )
        )

    def entry(self, factory, scope):
        self.check()
        if self.state["phase"] != "prepared" or self.state["entryAttempted"]:
            raise Refused("admin_one_entry_only")
        if scope is None:
            raise Refused("live_kernel_scope_required_before_entry")
        scope.before_entry(self)
        self._scope = scope
        self.state["entryAttempted"] = True
        self.state["phase"] = "entry_intent"
        self.store()
        try:
            self.child = factory()
            if not isinstance(self.child, subprocess.Popen):
                raise Refused("actual_owned_child_required")
            self.state["childPid"] = self.child.pid
            self.store()
            scope.attach(self.child)
            return self.child
        except BaseException as error:
            error.owner = self
            raise

    def wait(self, timeout=2):
        if self.child is None:
            raise Refused("admin_child_wait_unknown")
        try:
            code = self._scope.wait(self.child, timeout)
            if not isinstance(code, int) or self.child.returncode != code:
                raise Refused("actual_terminal_wait_required")
            self._scope.settled(self.child)
            self._tree_settled = True
            self._waited_child = self.child
            self.state["wait"] = {"pid": self.child.pid, "code": code}
            self.store()
            return code
        except BaseException as error:
            error.owner = self
            raise

    def finish_known_effect(self):
        operation = self.state["operation"]
        observed = self._effect_observed
        if operation is None:
            return
        if observed is None:
            raise Refused("administrative_effect_unobserved")
        row = next(row for row in self.rows if row["name"] == operation["name"])
        if (
            observed != (row["fd"], identity(os.fstat(row["fd"])))
            or identity(row["actual"].lstat()) != observed[1]
        ):
            raise Refused("administrative_effect_identity_changed")
        self.check()
        self.state["operation"] = None
        try:
            self.store()
        except BaseException:
            self.state["operation"] = operation
            raise

    def cancel_known_no_effect(self):
        if self.state["operation"] is None:
            return
        if self._effect_possible:
            raise Refused("ambiguous_admin_mutation_retained")
        self.check()
        self.state["operation"] = None
        self.store()

    def restore(self):
        if self.state["toolIntent"] is not None:
            raise Refused("unknown_administrative_tool_retained")
        if self.state["operation"] is not None:
            raise Refused("ambiguous_admin_mutation_retained")
        if self.state["entryAttempted"] and (
            self.child is None
            or self._waited_child is not self.child
            or self.state["wait"] is None
            or not self._tree_settled
        ):
            raise Refused("unknown_child_wait_retained")
        self.check()
        for row in reversed(self.rows):
            if row["changed"]:
                self.mutate(row, stat.S_IMODE(row["original"][4]), "restore")
        self.state["phase"] = "restored"
        self.store()
        self.close()

    @classmethod
    def recover(cls, journal, binding, *, inspect_inputs, root=Path("/"), uid=1001):
        # A new process never inherits the original owner's live wait authority.
        inspect_inputs()
        path = Path(journal)
        fd = os.open(path, os.O_RDWR | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            st = os.fstat(fd)
            if (
                not stat.S_ISREG(st.st_mode)
                or stat.S_IMODE(st.st_mode) != 0o600
                or st.st_uid != os.geteuid()
                or st.st_nlink != 1
                or path.parent.resolve(strict=True) != path.parent.absolute()
                or st.st_size > MAX_JOURNAL
            ):
                raise Refused("admin_recovery_journal_identity")
            state = json.loads(os.read(fd, MAX_JOURNAL + 1))
            if (
                state.get("binding") != binding
                or state.get("schema") != "sdk240.admin.v1"
            ):
                raise Refused("admin_recovery_binding")
            if (
                state.get("entryAttempted")
                or state.get("toolIntent") is not None
                or state.get("operation") is not None
            ):
                raise Refused("admin_recovery_unknown_intent_retained")
            owner = cls.__new__(cls)
            owner.pid = os.getpid()
            owner.root, owner.uid, owner.journal, owner.binding = (
                Path(root).absolute(),
                uid,
                path,
                dict(binding),
            )
            owner.fault = lambda phase, name: None
            owner.rows, owner.child, owner.tool_child = [], None, None
            owner._scope, owner._tree_settled, owner.retained_inputs = None, False, None
            owner._waited_child, owner._effect_possible = None, False
            owner.check_inputs = inspect_inputs
            owner._effect_observed = None
            owner.state, owner.journal_fd = state, fd
            owner.parent_fd = os.open(
                path.parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
            )
            parent = os.fstat(owner.parent_fd)
            if parent.st_uid != os.geteuid() or parent.st_mode & 0o022:
                raise Refused("admin_recovery_private_parent")
            expected_names = ["/"] + [
                "/" + "/".join(Path(CANDIDATE).parent.parts[1:i])
                for i in range(2, len(Path(CANDIDATE).parent.parts) + 1)
            ]
            rows = state.get("rows", [])
            if [row.get("name") for row in rows] != expected_names[: len(rows)] or len(
                rows
            ) > 7:
                raise Refused("admin_recovery_exact_chain")
            for record in rows:
                # Recovery accepts only the exact finite state the live owner can
                # produce, never caller-selected modes or a serialized wait claim.
                original, current = record.get("original"), record.get("current")
                if (
                    not isinstance(original, list)
                    or not isinstance(current, list)
                    or len(original) != 6
                    or len(current) != 6
                    or any(type(value) is not int for value in original + current)
                    or original[:4] != current[:4]
                    or not stat.S_ISDIR(original[4])
                    or not stat.S_ISDIR(current[4])
                    or original[2] not in (0, uid)
                    or type(record.get("changed")) is not bool
                ):
                    raise Refused("admin_recovery_row_shape")
                if record["name"] in DIRECTORIES:
                    if stat.S_IMODE(original[4]) not in (0o755, 0o777) or stat.S_IMODE(
                        current[4]
                    ) != (0o755 if record["changed"] else stat.S_IMODE(original[4])):
                        raise Refused("admin_recovery_exact_modes")
                    if record["changed"] and stat.S_IMODE(original[4]) != 0o777:
                        raise Refused("admin_recovery_exact_transition")
                elif original != current or original[4] & 0o022 or record["changed"]:
                    raise Refused("admin_recovery_outer_unchanged")
                actual = (
                    owner.root
                    if record["name"] == "/"
                    else owner.root / record["name"].lstrip("/")
                )
                handle = os.open(
                    actual if not owner.rows else actual.name,
                    os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                    **({} if not owner.rows else {"dir_fd": owner.rows[-1]["fd"]}),
                )
                owner.rows.append({**record, "actual": actual, "fd": handle})
            owner.check()
            return owner
        except BaseException:
            if "owner" in locals():
                for row in owner.rows:
                    os.close(row["fd"])
                os.close(owner.parent_fd)
            os.close(fd)
            raise

    def close(self):
        if self.state["phase"] != "restored":
            raise Refused("live_administrative_custody_retained")
        while self.rows:
            os.close(self.rows.pop()["fd"])
        if self.journal_fd is not None:
            os.close(self.journal_fd)
            self.journal_fd = None
        if self.parent_fd is not None:
            os.close(self.parent_fd)
            self.parent_fd = None


class ManifestTransfer:
    """One private root-owned file; only its regular read-only fd is passed."""

    def __init__(self, owner, name):
        self.owner, self.path = owner, Path(name)
        if (
            self.path.parent != owner.journal.parent
            or self.path.parent.resolve(strict=True) != self.path.parent
        ):
            raise Refused("manifest_private_administrative_parent")
        owner.state["manifestOutput"] = {"path": str(self.path), "phase": "creating"}
        owner.store()
        self.write_fd = os.open(
            self.path, os.O_RDWR | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600
        )
        self.read_fd = None
        self.expected, self.binding = None, identity(os.fstat(self.write_fd))
        owner.manifest_transfer = self

    def publish(self, manifest):
        if self.read_fd is not None or self.expected is not None:
            raise Refused("manifest_one_publication_only")
        self.owner.prepared()
        raw = (json.dumps(manifest, sort_keys=True) + "\n").encode()
        if len(raw) > 1048576:
            raise Refused("package_manifest_bound")
        before = os.fstat(self.write_fd)
        if (
            identity(before) != identity(self.path.lstat())
            or identity(before) != self.binding
            or before.st_uid != os.geteuid()
            or stat.S_IMODE(before.st_mode) != 0o600
            or before.st_nlink != 1
            or before.st_size
        ):
            raise Refused("manifest_output_identity")
        view = memoryview(raw)
        while view:
            count = os.write(self.write_fd, view)
            if count <= 0:
                raise Refused("manifest_short_write")
            view = view[count:]
        os.fsync(self.write_fd)
        os.fsync(self.owner.parent_fd)
        self.read_fd = os.open(self.path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        after = os.fstat(self.read_fd)
        if identity(after) != identity(os.fstat(self.write_fd)) or identity(
            after
        ) != identity(self.path.lstat()):
            raise Refused("manifest_output_replaced")
        self.binding = identity(after)
        self.expected = hashlib.sha256(raw).hexdigest()
        os.close(self.write_fd)
        self.write_fd = None
        self.check()
        self.owner.state["manifestOutput"].update(
            phase="published", sha256=self.expected, identity=self.binding
        )
        self.owner.store()
        return self.read_fd, self.expected

    def check(self):
        if self.read_fd is None or self.expected is None:
            raise Refused("manifest_publication_incomplete")
        info = os.fstat(self.read_fd)
        if (
            identity(info) != self.binding
            or identity(self.path.lstat()) != self.binding
            or info.st_nlink != 1
            or info.st_size > 1048576
            or os.get_inheritable(self.read_fd)
        ):
            raise Refused("manifest_transfer_drift")
        if (
            hashlib.sha256(os.pread(self.read_fd, 1048577, 0)).hexdigest()
            != self.expected
        ):
            raise Refused("manifest_transfer_bytes_changed")

    def close(self):
        if self.owner.unsettled():
            raise Refused("unknown_child_keeps_manifest_transfer")
        for name in ("write_fd", "read_fd"):
            fd = getattr(self, name)
            if fd is not None:
                os.close(fd)
                setattr(self, name, None)
        # The retained regular file is evidence, never an automatic teardown.
