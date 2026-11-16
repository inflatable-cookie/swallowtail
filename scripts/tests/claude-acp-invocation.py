"""Owned metadata only: no host observation, OS launch or vendor evaluation."""

import copy
import importlib.util
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location(
    "invocation", ROOT / "scripts/claude-acp-invocation.py"
)
inv = importlib.util.module_from_spec(spec)
exec(compile(Path(spec.origin).read_bytes(), spec.origin, "exec"), inv.__dict__)


def packet(kind="sdk-transport", context="new", hop="0.106.0"):
    manifest = inv.build()
    key = hop + "/" + kind
    profile = manifest["profiles"][key]
    native = (
        profile["images"]["darwin-arm64"]
        if profile["images"]
        else {"sha256": "ab" * 32, "bytes": 17}
    )
    command = ["/owned/python", "-I", "-B", "-S", "/owned/entry.py"]
    launch = {
        "caller": profile["caller"],
        "api": profile["caller"]["api"],
        "executable": "/owned/native" if kind != "registry-ps" else "/owned/ps",
        "image": {
            "platform": "darwin-arm64",
            "sha256": native["sha256"],
            "bytes": native["bytes"],
        },
        "argv": inv.BASE_ARGV.copy()
        if kind == "sdk-transport"
        else ["auth", "status", "--json"]
        if kind == "auth-status"
        else ["-o", "pid=,lstart=", "-p", "30,40"],
        "env": {
            "PATH": "/owned/bin",
            "HOME": "/owned/home",
            **({"LC_ALL": "C", "TZ": "UTC"} if kind == "registry-ps" else {}),
        },
        "cwd": "/owned/work",
        "stdio": ["pipe", "pipe", "pipe"],
        "detached": False,
        "shell": False,
        "timeoutMs": profile["timeoutMs"],
        "windowsHide": kind == "sdk-transport",
        "signalRef": "owned-abort-handle" if kind == "sdk-transport" else None,
        "descriptors": ["owned-stdin", "owned-stdout", "owned-stderr"],
    }
    actors = {
        name: {
            "pid": pid,
            "parent": parent,
            "group": group,
            "start": "owned-start-" + str(pid),
            "command": argv,
        }
        for name, pid, parent, group, argv in [
            ("prepared-owner", 10, 1, 10, ["/owned/rust-owner"]),
            ("group-owner", 20, 10, 20, ["/owned/sleep"]),
            ("checked-acp", 30, 10, 20, ["/owned/node", "/owned/captured-entry"]),
            ("entry", 40, 30, 20, command),
        ]
    }
    ref = {
        "id": "owned-one-shot",
        "context": context,
        "fixture": "managed-deny",
        "fixtureBinding": "cd" * 32,
        "profile": key,
        "resolution": {
            **copy.deepcopy(manifest["resolution"]),
            "resolver": profile["resolver"],
        },
        "launch": copy.deepcopy(launch),
        "entryCommand": command,
        "entrySourceSha256": manifest["entrySource"]["sha256"],
        "entryConfigSha256": "ef" * 32,
        "actors": actors,
    }
    return manifest, {
        "schema": "swallowtail.acp239.owned-invocations.v1",
        "scope": "owned-metadata-counterexamples",
        "hop": hop,
        "sourceManifestSha256": inv.digest(manifest),
        "originalActivation": False,
        "references": [ref],
    }


def observed(ref, entry=False):
    keys = (
        (
            "context",
            "fixture",
            "fixtureBinding",
            "launch",
            "actors",
            "entryCommand",
            "entrySourceSha256",
            "entryConfigSha256",
        )
        if entry
        else ("context", "fixture", "fixtureBinding", "launch")
    )
    return copy.deepcopy({k: ref[k] for k in keys})


class Invocation(unittest.TestCase):
    def test_all_hops_kinds_contexts_are_source_bound_and_still_closed(self):
        manifest = inv.build()
        self.assertEqual(len(manifest["profiles"]), 24)
        self.assertIsNone(manifest["actualPlan"])
        self.assertIsNone(manifest["originalCommand"])
        self.assertEqual(manifest["admission"], "closed")
        for key in manifest["profiles"]:
            hop, kind = key.split("/", 1)
            for context in inv.CONTEXTS:
                m, p = packet(kind, context, hop)
                model = inv.OwnedPreEntry(m, p)
                ref = p["references"][0]
                self.assertFalse(
                    model.reserve(ref["id"], observed(ref))["originalAllowed"]
                )
                result = model.pre_entry(ref["id"], observed(ref, True))
                self.assertFalse(result["hostObserved"])
                self.assertFalse(result["kernelEnforced"])

    def test_each_launch_field_drift_is_refused_and_spent(self):
        m, p = packet()
        for field, wrong in [
            ("executable", "/owned/other"),
            ("argv", ["--cli"]),
            ("env", {}),
            ("cwd", "/owned/other"),
            ("stdio", ["inherit"]),
            ("detached", True),
            ("shell", True),
            ("timeoutMs", 1),
            ("signalRef", "other"),
            ("descriptors", ["x", "y", "z"]),
            ("caller", {}),
            ("api", "execFile"),
            ("image", {}),
        ]:
            model = inv.OwnedPreEntry(m, p)
            value = observed(p["references"][0])
            value["launch"][field] = wrong
            with self.subTest(field=field), self.assertRaises(inv.Refused):
                model.reserve("owned-one-shot", value)
            self.assertIn("owned-one-shot", model.used)
            self.assertTrue(model.revoked)

    def test_source_and_original_scope_forgery_refuse(self):
        m, p = packet()
        for change in [
            lambda v: v.update(originalActivation=True),
            lambda v: v.update(scope="original"),
            lambda v: v.update(sourceManifestSha256="00" * 32),
            lambda v: v["references"][0].update(profile="0.106.0/auth-logout"),
            lambda v: v["references"][0].update(context="login"),
        ]:
            value = copy.deepcopy(p)
            change(value)
            with self.assertRaises(inv.Refused):
                inv.OwnedPreEntry(m, value)
        changed = copy.deepcopy(m)
        changed["originalCommand"] = ["native"]
        with self.assertRaises(inv.Refused):
            inv.OwnedPreEntry(changed, p)

    def test_unsupported_resolution_settings_and_extra_args_refuse(self):
        m, p = packet()
        for change in [
            lambda r: r["resolution"].update(clientExtraArgs={"arbitrary": "x"}),
            lambda r: r["resolution"].update(settingsOverride={"permissions": {}}),
            lambda r: r["resolution"].update(executableArgs=["--eval"]),
            lambda r: r["resolution"].update(customSpawn=True),
            lambda r: r["resolution"].update(resolver="environment-override"),
            lambda r: r["resolution"].update(unresolved=["argv"]),
            lambda r: r["launch"].update(argv=inv.BASE_ARGV + ["--unknown"]),
            lambda r: r["launch"]["env"].update(NODE_OPTIONS="unsafe"),
        ]:
            value = copy.deepcopy(p)
            change(value["references"][0])
            with self.assertRaises(inv.Refused):
                inv.OwnedPreEntry(m, value)

    def test_auth_settings_and_ps_controls_do_not_flatten_into_sdk(self):
        for kind, args in [
            ("auth-status", ["--settings", "{}", "auth", "status", "--json"]),
            ("auth-status", ["auth", "logout"]),
            ("registry-ps", ["-o", "pid=,lstart=", "-p", "1;exec"]),
            ("registry-ps", ["-o", "pid=,lstart=", "-p", ""]),
            ("registry-ps", inv.BASE_ARGV),
        ]:
            m, p = packet(kind)
            p["references"][0]["launch"]["argv"] = args
            with self.subTest(kind=kind, args=args), self.assertRaises(inv.Refused):
                inv.OwnedPreEntry(m, p)

    def test_sibling_topology_cannot_be_replaced_by_standin_or_text_argv(self):
        m, p = packet()
        for change in [
            lambda a: a["checked-acp"].update(parent=20),
            lambda a: a["entry"].update(parent=20),
            lambda a: a["entry"].update(group=40),
            lambda a: a["entry"].update(pid=30),
            lambda a: a["entry"].update(commandText="joined text"),
        ]:
            value = copy.deepcopy(p)
            change(value["references"][0]["actors"])
            with self.assertRaises(inv.Refused):
                inv.OwnedPreEntry(m, value)

    def test_preentry_rechecks_every_actor_and_launch_after_reservation(self):
        m, p = packet()
        ref = p["references"][0]
        for change in [
            lambda o: o["actors"]["entry"].update(start="reused-pid"),
            lambda o: o["actors"]["checked-acp"].update(parent=20),
            lambda o: o["launch"]["env"].update(EXTRA="late"),
            lambda o: o.update(entrySourceSha256="00" * 32),
            lambda o: o.update(entryCommand=["/owned/other"]),
            lambda o: o.update(context="resume"),
            lambda o: o.update(fixture="absent"),
        ]:
            model = inv.OwnedPreEntry(m, p)
            model.reserve(ref["id"], observed(ref))
            value = observed(ref, True)
            change(value)
            with self.assertRaises(inv.Refused):
                model.pre_entry(ref["id"], value)
            self.assertTrue(model.revoked)
            self.assertIn(ref["id"], model.entered)

    def test_duplicate_launch_entry_revoke_and_unreserved_entry_refuse(self):
        m, p = packet()
        ref = p["references"][0]
        model = inv.OwnedPreEntry(m, p)
        with self.assertRaises(inv.Refused):
            model.pre_entry(ref["id"], observed(ref, True))
        model.reserve(ref["id"], observed(ref))
        with self.assertRaises(inv.Refused):
            model.reserve(ref["id"], observed(ref))
        model.pre_entry(ref["id"], observed(ref, True))
        with self.assertRaises(inv.Refused):
            model.pre_entry(ref["id"], observed(ref, True))
        model = inv.OwnedPreEntry(m, p)
        model.reserve(ref["id"], observed(ref))
        model.revoke()
        with self.assertRaises(inv.Refused):
            model.pre_entry(ref["id"], observed(ref, True))
        p["references"].append(copy.deepcopy(ref))
        with self.assertRaises(inv.Refused):
            inv.OwnedPreEntry(m, p)

    def test_zero_launch_prompt_has_no_implicit_transport_allowance(self):
        m, p = packet(context="prompt")
        ref = p["references"][0]
        p["references"] = []
        model = inv.OwnedPreEntry(m, p)
        with self.assertRaises(inv.Refused):
            model.reserve(ref["id"], observed(ref))

    def test_mixed_hops_unknown_bindings_and_boolean_coercion_refuse(self):
        m, p = packet()
        for change in [
            lambda r: r.update(profile="0.105.0/sdk-transport"),
            lambda r: r.update(fixtureBinding=None),
            lambda r: r.update(entryConfigSha256=None),
            lambda r: r["launch"].update(cwd=None),
            lambda r: r["launch"].update(detached=0),
            lambda r: r["launch"].update(unlistedOption=True),
        ]:
            value = copy.deepcopy(p)
            change(value["references"][0])
            with self.assertRaises(inv.Refused):
                inv.OwnedPreEntry(m, value)

    def test_late_fixture_configuration_and_phase_drift_refuse(self):
        m, p = packet()
        ref = p["references"][0]
        for change in [
            lambda o: o.update(fixtureBinding="00" * 32),
            lambda o: o.update(entryConfigSha256="00" * 32),
            lambda o: o.update(context="prompt"),
        ]:
            model = inv.OwnedPreEntry(m, p)
            model.reserve(ref["id"], observed(ref))
            value = observed(ref, True)
            change(value)
            with self.assertRaises(inv.Refused):
                model.pre_entry(ref["id"], value)


if __name__ == "__main__":
    if "--compile-only" in sys.argv:
        for name in (
            "scripts/claude-acp-invocation.py",
            "scripts/tests/claude-acp-invocation.py",
        ):
            compile((ROOT / name).read_bytes(), name, "exec")
        print("Private invocation Python compile passed")
    else:
        unittest.main(verbosity=2)
