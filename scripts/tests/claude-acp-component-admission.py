#!/usr/bin/env python3
"""Owned fake component proof with positive and independent admission refusals."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

parser = argparse.ArgumentParser()
parser.add_argument('--owner', type=Path, required=True)
parser.add_argument('--owner-sha256', required=True)
parser.add_argument('--node', type=Path, required=True)
args = parser.parse_args()
root = Path(tempfile.mkdtemp(prefix='acp-component-admission-',dir=os.environ['TMPDIR']))
base = [sys.executable,'scripts/claude-acp-component-probe.py','--fake','--owner',str(args.owner),'--owner-sha256',args.owner_sha256,'--node',str(args.node)]

def run(index, extra, failure=None):
    attempt = root/str(index)
    attempt.mkdir()
    try:
        result = subprocess.run(base+extra,env={**os.environ,'TMPDIR':str(attempt),'TMP':str(attempt),'TEMP':str(attempt)},capture_output=True,text=True,timeout=180)
        if failure:
            assert result.returncode != 0 and failure in result.stderr, result.stderr[-1500:]
            assert not result.stdout, 'refused admission must produce no component result'
            return None
        assert result.returncode == 0, result.stderr[-1500:]
        return json.loads(result.stdout)
    finally:
        shutil.rmtree(attempt)

try:
    witness = run(0,[])
    record=root/'host-runtime.json'
    record.write_text(json.dumps(witness['runtime']))
    observed=witness['results'][0]
    assert observed['rustRuntimeSha256']==witness['results'][1]['rustRuntimeSha256']
    assert observed['launchPolicySha256']==witness['results'][1]['launchPolicySha256']
    bindings=['--host-runtime-record',str(record),'--runtime-closure-sha256',witness['runtimeClosureSha256'],'--node-sha256',witness['runtime']['sha256']]
    approved=run(1,bindings+['--rust-runtime-closure-sha256',observed['rustRuntimeSha256'],'--launch-policy-sha256',observed['launchPolicySha256']])
    assert approved['results'][0]['rustRuntimeSha256']==observed['rustRuntimeSha256']
    run(2,bindings+['--rust-runtime-closure-sha256','00'*32,'--launch-policy-sha256',observed['launchPolicySha256']],'operator-approved runtime/policy identity mismatch')
    run(3,bindings+['--rust-runtime-closure-sha256',observed['rustRuntimeSha256'],'--launch-policy-sha256','00'*32],'operator-approved runtime/policy identity mismatch')
    changed=json.loads(record.read_text()); changed['trustedGroups']=[]
    record.write_text(json.dumps(changed))
    changed_hash=hashlib.sha256(json.dumps(changed,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    run(4,['--host-runtime-record',str(record),'--runtime-closure-sha256',changed_hash,'--rust-runtime-closure-sha256',observed['rustRuntimeSha256'],'--launch-policy-sha256',observed['launchPolicySha256']],'runtime custody refused')
    print(json.dumps(approved,sort_keys=True))
    print('fake admission: approved Rust custody/policy matched; wrong custody, policy and principals refused before Node entry')
finally:
    shutil.rmtree(root)
