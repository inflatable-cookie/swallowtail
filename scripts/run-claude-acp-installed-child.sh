#!/usr/bin/env bash
# Route-specific fake proof against the installed Darwin runtime. No original code.
set -euo pipefail
cd "$(dirname "$0")/.."
installed_fixture=$(mktemp -d "$TMPDIR/acp-installed-child.XXXXXXXX")
trap 'rm -rf "$installed_fixture"' EXIT
export SWALLOWTAIL_ACP_INSTALLED_RUNTIME_RECORD="$installed_fixture/runtime.json"
python3 - <<'PY'
import importlib.util, json, os, pathlib, platform, shutil, sys
sys.dont_write_bytecode = True
if (platform.system(), platform.machine()) != ('Darwin','arm64'):
    raise SystemExit('installed runtime closure inspector is Darwin arm64 only')
spec=importlib.util.spec_from_file_location('component', 'scripts/claude-acp-component-probe.py')
owner=importlib.util.module_from_spec(spec); spec.loader.exec_module(owner)
node=pathlib.Path(shutil.which('node')).resolve(strict=True)
files,_=owner.runtime_files(node)
record={'node':str(node),'sha256':owner.digest(node),'dependencies':[{'path':str(p),'sha256':owner.digest(p)} for p in files if p!=node],'trustedUsers':sorted(set([0,os.geteuid()])),'trustedGroups':[80]}
pathlib.Path(os.environ['SWALLOWTAIL_ACP_INSTALLED_RUNTIME_RECORD']).write_text(json.dumps(record))
print(json.dumps({'installedRuntime':record},sort_keys=True))
PY
bash scripts/oxide-rust.sh nextest run --locked --profile ci-process -p swallowtail-adapter-claude-agent --test acp_guard_capture
