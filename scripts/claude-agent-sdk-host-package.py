"""Private Ubuntu26.04/x86_64 producer. Candidate bytes are never executed.

Initial image/system loader/dpkg/Python are trusted administrative tooling.
This derives an exact bounded package; observed candidate reports grant nothing.
"""

import ast
import hashlib
import importlib.machinery
import importlib.util
import json
import marshal
import types
import os
from pathlib import Path
import re
import stat
import struct
import sys
import time

CANDIDATE = "/opt/hostedtoolcache/node/22.23.4/x64/bin/node"
CANDIDATE_BYTES = 124902584
CANDIDATE_SHA = "94efd062ed066c1a0b332e18249859c905e920eeada5df55d24c99b919629a90"
ENV = {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"}


class Refused(OSError):
    pass


def identity(st):
    return (
        st.st_dev,
        st.st_ino,
        st.st_uid,
        st.st_gid,
        st.st_mode,
        st.st_size,
        st.st_mtime_ns,
        st.st_ctime_ns,
    )


class ImageFiles:
    def __init__(self, *, root=Path("/"), uid=1001, owner=None):
        self.root, self.uid = Path(root).resolve(strict=True), uid
        self.owner = owner
        self.files, self.links, self.directories = {}, {}, {}
        self.aliases = {}
        self.lookup, self.absent = {}, set()
        self.deadline = time.monotonic() + 30
        self.bytes = 0

    def actual(self, name):
        if (
            not isinstance(name, str)
            or not name.startswith("/")
            or ".." in Path(name).parts
            or name.startswith("//")
            or str(Path(name)) != name
            or "\0" in name
            or len(name) > 4096
        ):
            raise Refused("package_absolute_path")
        return self.root / name.lstrip("/")

    def literal(self, actual):
        return "/" if actual == self.root else "/" + str(actual.relative_to(self.root))

    def canonical(self, name):
        path = self.actual(name)
        target = path.resolve(strict=True)
        if target != self.root and self.root not in target.parents:
            raise Refused("package_alias_escape")
        for part in (*reversed(path.parents), path):
            if part != self.root and self.root not in part.parents:
                continue
            st = part.lstat()
            if st.st_uid not in (0, self.uid):
                raise Refused("package_input_owner")
            if stat.S_ISLNK(st.st_mode):
                self.links[str(part)] = (identity(st), os.readlink(part))
            elif stat.S_ISDIR(st.st_mode):
                if st.st_mode & 0o022:
                    raise Refused("package_unsafe_ancestor")
                self.directories[str(part)] = identity(st)[:5]
        if len(self.directories) > 256:
            raise Refused("package_ancestor_bound")
        canonical = self.literal(target)
        if name != canonical:
            self.aliases[name] = canonical
        return canonical

    def read(self, name, *, candidate=False):
        canonical = self.canonical(name)
        if canonical not in self.files:
            if len(self.files) >= 512:
                raise Refused("package_input_bound")
            fd = os.open(
                self.actual(canonical), os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK
            )
            try:
                st = os.fstat(fd)
                mode = stat.S_IMODE(st.st_mode)
                if not stat.S_ISREG(st.st_mode) or st.st_size > 268435456:
                    raise Refused("package_input_type_or_size")
                if candidate:
                    if canonical != CANDIDATE or st.st_uid != self.uid or mode != 0o777:
                        raise Refused("package_candidate_identity_or_mode")
                elif st.st_mode & 0o6022:
                    raise Refused("package_writable_input")
                self.bytes += st.st_size
                if self.bytes > 536870912:
                    raise Refused("package_byte_bound")
                digest = hashlib.sha256()
                while True:
                    if time.monotonic() > self.deadline:
                        raise Refused("package_deadline")
                    data = os.read(fd, 65536)
                    if not data:
                        break
                    digest.update(data)
                if identity(os.fstat(fd)) != identity(st) or identity(
                    self.actual(canonical).lstat()
                ) != identity(st):
                    raise Refused("package_input_drift")
                self.files[canonical] = (
                    fd,
                    identity(st),
                    digest.hexdigest(),
                    candidate,
                )
            except BaseException:
                os.close(fd)
                raise
        fd, before, digest, is_candidate = self.files[canonical]
        return canonical, os.pread(fd, before[5] + 1, 0)

    def inventory(self, name):
        actual = self.actual(name)
        if not actual.exists():
            # Bind the existing parent and its immediate inventory as absence evidence.
            parent = actual.parent
            while not parent.exists():
                parent = parent.parent
            canonical_parent = self.canonical(self.literal(parent))
            first_missing = actual.relative_to(parent).parts[0]
            self.absent.add(canonical_parent.rstrip("/") + "/" + first_missing)
            if len(self.absent) > 128:
                raise Refused("package_absence_bound")
            return
        canonical = self.canonical(name)
        names = sorted(os.listdir(self.actual(canonical)))
        if len(names) > 8192 or len(self.lookup) >= 32 and canonical not in self.lookup:
            raise Refused("package_search_bound")
        self.lookup[canonical] = names

    def check(self):
        if self.owner is not None:
            self.owner.check_ancestors()
        for name, (fd, expected, digest, candidate) in self.files.items():
            current = identity(os.fstat(fd))
            if current != identity(self.actual(name).lstat()):
                raise Refused("package_path_replaced")
            if candidate:
                # Only the later leaf transaction may alter mode/ctime; require restored0777 and exact bytes.
                if (
                    current[:4] + current[5:7] != expected[:4] + expected[5:7]
                    or stat.S_IMODE(current[4]) != 0o777
                ):
                    raise Refused("package_candidate_drift")
                value = hashlib.sha256()
                offset = 0
                while offset < current[5]:
                    data = os.pread(fd, min(65536, current[5] - offset), offset)
                    if not data:
                        raise Refused("package_candidate_short_read")
                    value.update(data)
                    offset += len(data)
                if value.hexdigest() != digest:
                    raise Refused("package_candidate_bytes_drift")
            elif current != expected:
                raise Refused("package_input_drift")
        for name, (expected, target) in self.links.items():
            if identity(Path(name).lstat()) != expected or os.readlink(name) != target:
                raise Refused("package_alias_drift")
        for name, expected in self.directories.items():
            if self.owner is not None and self.literal(Path(name)) in {
                row["name"]
                for row in self.owner.rows
                if row["name"].startswith("/opt/hostedtoolcache")
            }:
                continue
            if identity(Path(name).lstat())[:5] != expected:
                raise Refused("package_directory_drift")
        for name, entries in self.lookup.items():
            if sorted(os.listdir(self.actual(name))) != entries:
                raise Refused("package_search_drift")
        for name in self.absent:
            if os.path.lexists(self.actual(name)):
                raise Refused("package_absence_drift")

    def hashes(self, names):
        return {name: self.files[name][2] for name in sorted(names)}

    def close(self):
        for fd, *_ in self.files.values():
            os.close(fd)
        self.files.clear()


def cache_entries(data):
    # glibc cache_file_new/file_entry_new, little-endian x86_64; no old compat layout.
    if len(data) < 48 or data[:20] != b"glibc-ld.so.cache1.1":
        raise Refused("package_cache_format")
    count, strings = struct.unpack_from("<II", data, 20)
    start = 48 + 24 * count
    if count > 8192 or start + strings > len(data) or data[28] not in (0, 2):
        raise Refused("package_cache_shape")

    def text(offset):
        if not start <= offset < start + strings:
            raise Refused("package_cache_string")
        end = data.find(b"\0", offset, min(start + strings, offset + 4097))
        if end < 0:
            raise Refused("package_cache_string")
        return data[offset:end].decode("ascii")

    result = {}
    for i in range(count):
        flags, key, value, unused, hwcap = struct.unpack_from(
            "<iIIIQ", data, 48 + i * 24
        )
        if flags == 0x303:
            result.setdefault(text(key), []).append((text(value), hwcap))
    return result


def loader_directories(help_text):
    try:
        section = help_text.split("Shared library search path:\n", 1)[1]
    except IndexError as error:
        raise Refused("package_loader_help_shape") from error
    section = section.split("\n\n", 1)[0]
    rows = section.splitlines()
    if "  (libraries located via /etc/ld.so.cache)" not in rows:
        raise Refused("package_cache_search_missing")
    result = []
    for row in rows:
        if row == "  (libraries located via /etc/ld.so.cache)":
            continue
        match = re.fullmatch(r"  (/[^ ]+) \(system search path\)", row)
        if not match:
            raise Refused("package_non_system_loader_search")
        name = match[1]
        if name not in (
            "/lib/x86_64-linux-gnu",
            "/usr/lib/x86_64-linux-gnu",
            "/lib",
            "/usr/lib",
        ):
            raise Refused("package_unreviewed_default_search")
        result.append(name)
    if not result or len(set(result)) != len(result):
        raise Refused("package_default_search_shape")
    header = "Subdirectories of glibc-hwcaps directories, in priority order:\n"
    if help_text.count(header) != 1:
        raise Refused("package_hwcaps_policy")
    caps = []
    for row in help_text.split(header, 1)[1].splitlines():
        if not row:
            continue
        match = re.fullmatch(r"  (x86-64-v[234])(?: \(supported, searched\))?", row)
        if not match:
            raise Refused("package_hwcaps_policy")
        caps.append(match[1])
    if caps != ["x86-64-v4", "x86-64-v3", "x86-64-v2"]:
        raise Refused("package_hwcaps_policy")
    if "Legacy HWCAP" in help_text:
        raise Refused("package_legacy_hwcaps_refused")
    return result


class LoaderGraph:
    def __init__(self, files, elf, cache, directories):
        self.files, self.elf, self.cache, self.directories = (
            files,
            elf,
            cache,
            directories,
        )
        self.aliases = {}
        for name in directories:
            files.inventory(name)
            self.no_hwcaps(name)
        if os.path.lexists(files.actual("/etc/ld.so.preload")):
            raise Refused("package_preload_refused")
        files.inventory("/etc/ld.so.preload")

    def no_hwcaps(self, directory):
        name = directory + "/glibc-hwcaps"
        if os.path.lexists(self.files.actual(name)):
            raise Refused("package_hwcaps_directory_refused")
        self.files.inventory(name)

    def resolve(self, needed):
        paths = []
        for name, hwcap in self.cache.get(needed, []):
            if hwcap:
                raise Refused("package_cached_hwcaps_refused")
            paths.append(name)
        paths += [
            name + "/" + needed
            for name in self.directories
            if os.path.lexists(self.files.actual(name + "/" + needed))
        ]
        choices = set()
        for name in paths:
            parent = str(Path(name).parent)
            self.files.inventory(parent)
            self.no_hwcaps(parent)
            canonical, data = self.files.read(name)
            choices.add(canonical)
            self.aliases[name] = canonical
        if len(choices) != 1:
            raise Refused("package_missing_or_ambiguous_resolution:" + needed)
        return choices.pop()

    def graph(self, root, *, candidate=False):
        root, data = self.files.read(root, candidate=candidate)
        objects, edges, interpreters = {}, {}, {}
        pending = [(root, data)]
        while pending:
            name, data = pending.pop()
            if name in objects:
                continue
            if len(objects) >= 128:
                raise Refused("package_elf_object_bound")
            metadata = self.elf.parse_elf(data)
            objects[name], edges[name] = data, {}
            for needed in metadata["needed"]:
                target = self.resolve(needed)
                edges[name][needed] = target
                pending.append(self.files.read(target))
            if metadata["interpreter"]:
                literal = metadata["interpreter"]
                target, content = self.files.read(literal)
                interpreters[literal] = target
                self.aliases[literal] = target
                pending.append((target, content))
        if candidate:
            self.elf.verify_graph(root, objects, edges, interpreter_paths=interpreters)
        return set(objects), edges, interpreters


def source_imports(data, package="", *, supervisor=False):
    tree = ast.parse(data)
    names = set()

    class Visitor(ast.NodeVisitor):
        def visit_If(self, node):
            expression = ast.unparse(node.test)
            false = expression in (
                "TYPE_CHECKING",
                "typing.TYPE_CHECKING",
                "__name__ == '__main__'",
                "os.name == 'nt'",
                "sys.platform == 'win32'",
            )
            for child in node.orelse if false else node.body + node.orelse:
                self.visit(child)

        def visit_Call(self, node):
            dynamic = (
                isinstance(node.func, ast.Name)
                and node.func.id == "__import__"
                or isinstance(node.func, ast.Attribute)
                and node.func.attr == "import_module"
            )
            if dynamic:
                if (
                    node.args
                    and isinstance(node.args[0], ast.Constant)
                    and isinstance(node.args[0].value, str)
                ):
                    names.add(node.args[0].value)
                elif supervisor:
                    raise Refused("package_dynamic_supervisor_import")
            self.generic_visit(node)

        def visit_Import(self, node):
            names.update(a.name for a in node.names)

        def visit_ImportFrom(self, node):
            if node.level:
                base = importlib.util.resolve_name(
                    "." * node.level + (node.module or ""), package
                )
            else:
                base = node.module or ""
            if base:
                names.add(base)
            names.update(
                base + "." + a.name for a in node.names if base and a.name != "*"
            )

    Visitor().visit(tree)
    return names


def checked_bytecode(source, data):
    if (
        len(data) < 16
        or len(data) > 16777216
        or data[:4] != importlib.util.MAGIC_NUMBER
    ):
        raise Refused("package_python_cache_format")
    code = marshal.loads(data[16:])
    if not isinstance(code, types.CodeType) or code != compile(
        source, code.co_filename, "exec", dont_inherit=True, optimize=0
    ):
        raise Refused("package_python_cache_source_mismatch")


def python_context():
    loaded = set()
    for module in tuple(sys.modules.values()):
        for name in (
            getattr(module, "__file__", None),
            getattr(module, "__cached__", None),
        ):
            if name and Path(name).is_file():
                loaded.add(str(Path(name).absolute()))
    return {
        "version": sys.version_info[:2],
        "cacheTag": sys.implementation.cache_tag,
        "cachePrefix": sys.pycache_prefix,
        "optimize": sys.flags.optimize,
        "search": list(sys.path),
        "executable": str(Path(sys.executable).resolve(strict=True)),
        "loaded": loaded,
    }


def python_closure(files, source_files, context):
    # Fixed system Python3.14 profile, no site/user/zip/third-party search.
    search = [
        "/usr/lib/python314.zip",
        "/usr/lib/python3.14",
        "/usr/lib/python3.14/lib-dynload",
    ]
    if tuple(context["version"]) != (3, 14) or set(context["search"]) != set(search):
        raise Refused("package_system_python_profile")
    if (
        context.get("cacheTag") != "cpython-314"
        or context.get("cachePrefix") is not None
        or context.get("optimize") != 0
    ):
        raise Refused("package_system_python_cache_profile")
    for name in search:
        files.inventory(name)
    pending = {
        "encodings",
        "encodings.utf_8",
        "encodings.ascii",
        "encodings.latin_1",
        "_strptime",
    }
    selected = set()
    for name, data in source_files.items():
        if name.endswith(".py") and "/tests/" not in name:
            pending.update(source_imports(data, supervisor=True))
    patch = source_files.get(
        "docs/knowledge/operations/claude-agent-sdk-runtime-permissions.patch"
    )
    if patch is not None:
        helper = "\n".join(
            line[1:]
            for line in patch.decode().splitlines()
            if line.startswith("+") and not line.startswith("+++")
        )
        pending.update(source_imports(helper, supervisor=True))
    for name in context["loaded"]:
        canonical, _ = files.read(name)
        selected.add(canonical)
    seen = set()
    while pending:
        name = pending.pop()
        if name in seen:
            continue
        seen.add(name)
        if len(seen) > 512:
            raise Refused("package_python_module_bound")
        if importlib.machinery.BuiltinImporter.find_spec(
            name
        ) or importlib.machinery.FrozenImporter.find_spec(name):
            continue
        locations = [str(files.actual(path)) for path in search[1:]]
        spec = None
        for index, part in enumerate(name.split(".")):
            full = ".".join(name.split(".")[: index + 1])
            spec = importlib.machinery.PathFinder.find_spec(full, locations)
            if spec is None:
                break  # Attribute or unavailable platform import; held search inventories prove absence.
            if spec.submodule_search_locations:
                locations = list(spec.submodule_search_locations)
                for directory in locations:
                    files.inventory(files.literal(Path(directory)))
            elif index + 1 < len(name.split(".")):
                spec = None
                break
        if spec is None:
            if name in {
                "encodings.utf_8",
                "encodings.ascii",
                "encodings.latin_1",
                "_strptime",
            }:
                raise Refused("package_required_python_input_missing")
            continue
        origin = files.literal(Path(spec.origin)) if spec.origin else None
        if not origin or not origin.startswith("/usr/lib/python3.14/"):
            raise Refused("package_non_system_python_import")
        if not origin.endswith((".py", ".so")):
            raise Refused("package_sourceless_python_refused")
        canonical, data = files.read(origin)
        selected.add(canonical)
        if origin.endswith(".py"):
            package = (
                name if spec.submodule_search_locations else name.rpartition(".")[0]
            )
            pending.update(source_imports(data, package))
            cached = str(
                Path(origin).parent
                / "__pycache__"
                / (Path(origin).stem + "." + context["cacheTag"] + ".pyc")
            )
            if os.path.lexists(files.actual(cached)):
                canonical, cached_data = files.read(cached)
                checked_bytecode(data, cached_data)
                selected.add(canonical)
            else:
                files.inventory(cached)
    return selected


class HostPackage:
    def __init__(self, files, manifest, provenance):
        self.files, self.manifest, self.provenance = files, manifest, provenance

    def check(self):
        self.files.check()

    def close(self):
        if self.files.owner is not None and self.files.owner.unsettled():
            raise Refused("package_custody_retained")
        self.files.close()


def produce(owner, session, elf, sources, *, root=Path("/"), uid=1001):
    owner.prepared()  # Complete held safe ancestry precedes candidate bytes.
    files = ImageFiles(root=root, uid=uid, owner=owner)
    try:
        for name, data in sources.items():
            canonical, actual = files.read(str(session.ROOT / name))
            if actual != data:
                raise Refused("package_source_drift")
        source_inputs = set(files.files)
        loader, _ = files.read("/lib64/ld-linux-x86-64.so.2")
        code, help_data = owner.tool([loader, "--help"], environment=ENV)
        if code:
            raise Refused("trusted_loader_help_failed")
        directories = loader_directories(help_data.decode("ascii"))
        # Initial Python's fixed image configuration is independently bound too.
        for name in (
            "/etc/nsswitch.conf",
            "/etc/hosts",
            "/etc/resolv.conf",
            "/etc/gai.conf",
            "/etc/mime.types",
            "/etc/ssl/openssl.cnf",
            "/usr/lib/ssl/openssl.cnf",
            "/etc/ld.so.conf",
        ):
            if not os.path.lexists(files.actual(name)):
                files.inventory(name)
                continue
            canonical, configuration = files.read(name)
            if name.endswith("openssl.cnf") and re.search(
                rb"(?m)^\s*(?:\.include|(?:providers|engines)\s*=)", configuration
            ):
                raise Refused("package_unreviewed_openssl_configuration")
            if name == "/etc/nsswitch.conf":
                lines = [
                    line.split("#", 1)[0].strip()
                    for line in configuration.decode().splitlines()
                ]
                hosts = [
                    line.split(":", 1)[1].split()
                    for line in lines
                    if line.startswith("hosts:")
                ]
                if hosts != [["files", "dns"]]:
                    raise Refused("package_unreviewed_nss_hosts")
        config_dir = "/etc/ld.so.conf.d"
        files.inventory(config_dir)
        if files.actual(config_dir).is_dir():
            for child in sorted(files.actual(config_dir).iterdir()):
                if not child.name.endswith(".conf"):
                    raise Refused("package_loader_config_entry")
                files.read(config_dir + "/" + child.name)
        cache_name, cache_data = files.read("/etc/ld.so.cache")
        graph = LoaderGraph(files, elf, cache_entries(cache_data), directories)
        node, data = files.read(CANDIDATE, candidate=True)
        if (
            len(data) != CANDIDATE_BYTES
            or hashlib.sha256(data).hexdigest() != CANDIDATE_SHA
        ):
            raise Refused("package_exact_candidate_bytes")
        del data
        node_objects, edges, interpreters = graph.graph(node, candidate=True)
        context = python_context()
        supervisor = python_closure(files, sources, context) | source_inputs
        python = context["executable"]
        supervisor.add(files.read(python)[0])
        for name in list(supervisor):
            fd = files.files[name][0]
            if os.pread(fd, 4, 0) == b"\x7fELF":
                closure, _, _ = graph.graph(name)
                supervisor.update(closure)
        query, _ = files.read("/usr/bin/dpkg-query")
        packages = [
            "libc6",
            "libc-bin",
            "python3.14-minimal",
            "libpython3.14-minimal",
            "libpython3.14-stdlib",
            "util-linux",
            "coreutils",
            "bash",
        ]
        code, data = owner.tool(
            [
                query,
                "-W",
                "-f=${binary:Package}\t${Version}\t${source:Package}\t${source:Version}\t${Status}\n",
                *packages,
            ],
            environment=ENV,
        )
        if code or len(data.splitlines()) != len(packages):
            raise Refused("image_package_provenance_missing")
        provenance = data.decode("utf-8")
        rows = [row.split("\t") for row in provenance.splitlines()]
        if any(
            len(row) != 5 or row[-1] != "install ok installed" or not all(row[:4])
            for row in rows
        ) or {row[0].split(":")[0] for row in rows} != set(packages):
            raise Refused("image_package_provenance_shape")
        closure, _, _ = graph.graph(query)
        supervisor.update(closure)
        loader_inputs = {cache_name, query}
        # Bind the installed package database as input to the root-owned provenance query.
        database, _ = files.read("/var/lib/dpkg/status")
        loader_inputs.add(database)
        for tool in (
            "/usr/bin/timeout",
            "/usr/bin/prlimit",
            "/usr/bin/bash",
            "/usr/bin/wc",
        ):
            target, _ = files.read(tool)
            supervisor.add(target)
            closure, _, _ = graph.graph(target)
            supervisor.update(closure)
        loader_inputs.update(
            name
            for name in files.files
            if name not in node_objects and name not in supervisor and name != node
        )
        host = session.host_identity()
        host["uid"] = uid
        manifest = {
            "path": node,
            "sha256": CANDIDATE_SHA,
            "bytes": CANDIDATE_BYTES,
            "version": "v22.23.4",
            "originalActivation": False,
            "host": host,
            "bootstrapInputs": {
                "libraries": files.hashes(node_objects - {node}),
                "interpreters": files.hashes(set(interpreters.values())),
                "loaderInputs": files.hashes(loader_inputs),
                "supervisorInputs": files.hashes(supervisor),
            },
            "resolutions": edges,
            "aliases": {**files.aliases, **graph.aliases},
            "interpreterAliases": interpreters,
            "lookup": {
                "directories": files.lookup,
                "absent": sorted(files.absent),
                "environment": ENV,
            },
        }
        if len(json.dumps(manifest).encode()) > 1048576:
            raise Refused("package_manifest_bound")
        files.check()
        return HostPackage(
            files,
            manifest,
            {
                "packageRows": provenance,
                "loaderHelpSha256": hashlib.sha256(help_data).hexdigest(),
                "cacheFormat": "glibc-ld.so.cache1.1; x86-64 only; no selected hwcaps or ambiguous targets",
                "pythonProfile": "system3.14; isolated/no-site; selected source literal-import closure and fixed UTF8/ASCII/latin1 codecs; no original/vendor code",
                "administrativeBinding": owner.binding,
                "rootTrust": "exact authorized GitHub image and administrative tooling, not candidate self-attestation",
            },
        )
    except BaseException as error:
        if owner.unsettled():
            owner.retained_inputs = files
            error.owner = owner
        else:
            files.close()
        raise
