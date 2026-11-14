#!/usr/bin/env bash
# Explicit ordinary source/fake proof. No shared privileged/original branch.
set -euo pipefail
cd "$(dirname "$0")/.."
if [[ "$(uname -s):$(uname -m)" != Darwin:arm64 ]]; then
  echo 'not applicable: this captured installed-runtime rehearsal is Darwin arm64 only'
  exit 0
fi
shared_fixture=$(mktemp -d "$TMPDIR/acp-shared-runtime.XXXXXXXX")
trap 'rm -rf "$shared_fixture"' EXIT
export SWALLOWTAIL_ACP_INSTALLED_RUNTIME_RECORD="$shared_fixture/runtime.json"
export SWALLOWTAIL_ACP_SHARED_RUNTIME_RECORD="$shared_fixture/runtime.json"
python3 -I -B - <<'PY'
import grp,hashlib,importlib.util,json,os,pathlib,shutil,sys
sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('owned_inspector','scripts/claude-acp-component-probe.py')
inspector=importlib.util.module_from_spec(spec);spec.loader.exec_module(inspector)
node=pathlib.Path(shutil.which('node')).resolve(strict=True)
launcher=pathlib.Path(sys.executable).resolve(strict=True)
# Xcode's bin/python re-execs the framework app binary; bind that actual
# interpreter directly, rather than treating its launcher as the entered inode.
python=(launcher.parent.parent/'Resources/Python.app/Contents/MacOS/Python').resolve(strict=True)
if not python.is_file(): raise RuntimeError('installed framework interpreter missing')
files,_=inspector.runtime_files(node)
record={'node':str(node),'sha256':inspector.digest(node),'dependencies':[{'path':str(p),'sha256':inspector.digest(p)} for p in files if p!=node],'trustedUsers':sorted(set([0,os.geteuid()])),'trustedGroups':[grp.getgrnam('admin').gr_gid],'python':{'path':str(python),'sha256':inspector.digest(python),'launcherPath':str(launcher),'launcherSha256':inspector.digest(launcher)}}
pathlib.Path(os.environ['SWALLOWTAIL_ACP_SHARED_RUNTIME_RECORD']).write_text(json.dumps(record))
print(json.dumps({'ownedSoftwareRuntime':record},sort_keys=True))
PY
bash scripts/oxide-rust.sh nextest run --locked --profile ci-process -p swallowtail-adapter-claude-agent --test acp_guard_capture --run-ignored only -E 'test(/shared_software::/)' --success-output final
