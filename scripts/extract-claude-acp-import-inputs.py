#!/usr/bin/env python3
"""Read pinned npm archives into JSON for the static ACP import analysis.

No archive member is extracted or executed. The output directory must exist.
"""
import argparse
import base64
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / 'crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0'
HOPS = ('0.101.0', '0.101.1', '0.102.0', '0.103.0', '0.104.0', '0.104.1', '0.105.0', '0.106.0')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('archives', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    registry = json.loads((FIXTURES / 'registry.json').read_text())['versions']
    dependencies = json.loads((FIXTURES / 'dependencies.json').read_text())['artifacts']
    extra = json.loads((FIXTURES / 'import-edge-artifacts.json').read_text())
    expected = {v: (registry[v]['tarball_sha256'], registry[v]['integrity']) for v in HOPS}
    for name, row in dependencies.items():
        if name.endswith('@0.3.316'):
            continue
        key = name.replace('/', '_').replace('@', '_')
        expected[key] = (row['tarball_sha256'], row['integrity'])
    for key, row in extra.items():
        expected[key] = (row['sha256'], row['integrity'])
    for key, (sha256, integrity) in sorted(expected.items()):
        archive = args.archives / (key + '.tgz')
        raw = archive.read_bytes()
        if hashlib.sha256(raw).hexdigest() != sha256 or 'sha512-' + base64.b64encode(hashlib.sha512(raw).digest()).decode() != integrity:
            raise ValueError('archive identity mismatch: ' + key)
        files = {}
        with tarfile.open(archive) as tar:
            for member in tar:
                if not member.isfile() or not member.name.endswith(('.js', '.mjs', '.cjs')):
                    continue
                if not member.name.startswith('package/') or member.name in files or member.size > 8 * 1024 * 1024:
                    raise ValueError('unexpected archive member: ' + member.name)
                name = member.name.removeprefix('package/')
                if name in files or '..' in Path(name).parts:
                    raise ValueError('duplicate or escaping member: ' + member.name)
                files[name] = tar.extractfile(member).read().decode('utf-8')
        (args.output / (key + '.json')).write_text(json.dumps(files, sort_keys=True))


if __name__ == '__main__':
    main()
