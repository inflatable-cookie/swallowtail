#!/usr/bin/env python3
"""ACP-private software actor adapter; never invokes a kernel or original mode."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

SHARED = Path(__file__).resolve().parent / 'reviewed'
MAX = 1024 * 1024


def sha(file):
    return hashlib.sha256(Path(file).read_bytes()).hexdigest()


def shared():
    record = json.loads((SHARED / 'snapshot.json').read_bytes())
    if record['sourceHead'] != 'f9025d2197333c53aed2266627dfa39d05bff7e3':
        raise ValueError('shared_source_head')
    if sha(SHARED / 'source.sha256') != '8a819724898208411b81977abbadf05084545a2f6c179ecb08d9775df99b00a9' or sha(SHARED / 'plan.json') != '3956dfa3fefeceffdf5501e2f7e81dbb8bd50c4c83baff08b25b3ddd89da1fe4':
        raise ValueError('reviewed_record_or_plan_changed')
    expected = dict(line.split('  ', 1)[::-1] for line in (SHARED / 'source.sha256').read_text().splitlines())
    if set(record['files']) != {'proposal.py', 'entry.py', 'loader.mjs', 'bootstrap-native.mjs', 'plan.json', 'source.sha256'}:
        raise ValueError('shared_file_set')
    for name, row in record['files'].items():
        wanted = '8a819724898208411b81977abbadf05084545a2f6c179ecb08d9775df99b00a9' if name == 'source.sha256' else expected[row['source']]
        if sha(SHARED / name) != wanted:
            raise ValueError('shared_source_changed')
    spec = importlib.util.spec_from_file_location('reviewed_software', SHARED / 'proposal.py')
    module = importlib.util.module_from_spec(spec)
    exec(compile((SHARED / 'proposal.py').read_bytes(), str(SHARED / 'proposal.py'), 'exec'), module.__dict__)
    # Only these ordinary OS observation/validation functions are used. No
    # rehearsal, ActorRegistry stand-in, context, tracer or kernel branch runs.
    return module.process_identity, module.validate_entry


def run(config_file):
    config = json.loads(Path(config_file).read_bytes())
    if config['schema'] != 'swallowtail.acp227.shared-software.v1' or config['originalActivation'] is not False:
        raise ValueError('software_only_scope')
    observe, validate = shared()
    owner_pid = config['preparedOwnerPid']
    if owner_pid != os.getppid():
        raise ValueError('prepared_owner_parent')
    owner = observe(owner_pid)
    if 'command' in owner:
        valid = owner['command'] == config['preparedOwnerCommand']
    else:
        valid = owner['commandText'] == ' '.join(config['preparedOwnerCommand'])
    if not valid:
        raise ValueError('prepared_owner_launch')
    root = Path(config['root'])
    lock = threading.RLock()
    released = threading.Event()
    state = {'armed': False, 'revoked': False, 'hold': False, 'round': 0, 'actors': {}, 'events': []}

    def verify_sources():
        if state['revoked']:
            raise ValueError('host_admission_revoked')
        for row in config['sources']:
            file = Path(row['path'])
            if file.resolve(strict=True) != file or sha(file) != row['sha256']:
                raise ValueError('host_source_changed')
        current = observe(owner_pid)
        if any(current[k] != owner[k] for k in ('pid', 'parent', 'group', 'start')):
            raise ValueError('prepared_owner_changed')

    def emit(kind, **extra):
        row = {'seq': len(state['events']), 'phase': 'software-admission', 'kind': kind,
               'round': state['round'], 'kernelEnforced': False, **extra}
        state['events'].append(row)
        with (root / 'supervisor-events.jsonl').open('a') as stream:
            stream.write(json.dumps(row) + '\n')
        return row

    class Endpoint(BaseHTTPRequestHandler):
        def log_message(self, *args):
            pass

        def do_POST(self):
            self.connection.settimeout(1)
            status, answer = 403, {}
            try:
                size = int(self.headers.get('Content-Length', '0'))
                if self.path != '/native-entry' or not 0 < size <= 16384:
                    raise ValueError('entry_request')
                value = json.loads(self.rfile.read(size))
                with lock:
                    if not state['armed'] or state['actors'] or value.get('token') != config['token'] or type(value.get('pid')) is not int:
                        raise ValueError('unarmed_duplicate_or_token')
                    verify_sources()
                    native = observe(value['pid'])
                    driver = observe(native['parent'])
                    group = observe(driver['group'])
                    # Actual LocalProcessHost children are siblings. Never
                    # rewrite observed parents to fit the reviewed stand-in.
                    for role, observed, parent, command in [
                        ('group-owner', group, {**owner, 'group': group['pid']}, ['/bin/sleep', '86400']),
                        ('driver', driver, {**owner, 'group': group['pid']}, config['driverCommand']),
                        ('native-entry', native, driver, config['entryCommand']),
                    ]:
                        try:
                            validate(observed, parent, command)
                        except Exception as error:
                            emit('identity_refused', role=role, observed=observed, expectedCommand=command)
                            raise ValueError(role + ':' + str(error)) from error
                    if value.get('entrySourceSha256') != config['entrySourceSha256']:
                        raise ValueError('entry_source_binding')
                    state['actors'] = {'prepared-owner': owner, 'group-owner': group, 'driver': driver, 'native-entry': native}
                    emit('native_registered', actors=state['actors'])
                    hold = state['hold']
                if hold and not released.wait(.7):
                    raise ValueError('entry_release_deadline')
                with lock:
                    verify_sources()
                    for role in ('group-owner', 'driver', 'native-entry'):
                        before = state['actors'][role]
                        actual = observe(before['pid'])
                        if any(actual[k] != before[k] for k in ('pid', 'parent', 'group', 'start')):
                            raise ValueError('actor_changed_before_release')
                    emit('native_released', identity=state['actors']['native-entry'])
                    status = 200
            except Exception as error:
                with lock:
                    emit('entry_refused', reason=str(error))
            self.send_response(status)
            self.end_headers()
            self.wfile.write(json.dumps(answer).encode())

    server = ThreadingHTTPServer(('127.0.0.1', 0), Endpoint)
    server.daemon_threads = False
    entry = {'originalActivation': False, 'port': server.server_port, 'token': config['token'],
             'entry': {'command': config['nativeCommand'], 'nativePath': config['nativePath'],
                       'env': config['nativeEnv'], 'pythonSha256': config['pythonSha256'],
                       'nodeSha256': config['nodeSha256'], 'nativeSha256': config['nativeSha256'],
                       'sourceSha256': config['entrySourceSha256']}}
    (root / 'native-entry.json').write_text(json.dumps(entry))
    # This newly created host input is pinned before any arm/release.
    config['sources'].append({'path': str(root / 'native-entry.json'), 'sha256': sha(root / 'native-entry.json')})
    thread = threading.Thread(target=server.serve_forever, kwargs={'poll_interval': .01})
    thread.start()
    print(json.dumps({'kind': 'ready', 'port': server.server_port, 'pid': os.getpid()}), flush=True)
    try:
        for line in sys.stdin:
            if len(line) > 16384:
                raise ValueError('control_frame_limit')
            command = json.loads(line)
            with lock:
                op = command['op']
                if op == 'arm':
                    if state['armed'] or state['round'] >= 3:
                        raise ValueError('software_scope_budget')
                    state.update(armed=True, revoked=False, hold=bool(command.get('hold')), actors={}, round=state['round']+1)
                    released.clear()
                    verify_sources()
                    emit('scope_armed', context=command['context'])
                elif op == 'revoke':
                    state['revoked'] = True
                    released.set()
                    emit('admission_revoked')
                elif op == 'release':
                    released.set()
                elif op == 'settle':
                    # Observer absence is kept separate from actual software
                    # parent waits in the Rust/SDK receipts. It grants no wait.
                    absent = {}
                    for role, actor in state['actors'].items():
                        if role == 'prepared-owner':
                            continue
                        try:
                            observe(actor['pid'])
                            absent[role] = False
                        except (ProcessLookupError, FileNotFoundError, __import__('subprocess').CalledProcessError):
                            absent[role] = True
                    emit('scope_settled', observedAbsent=absent, softwareWaits=command.get('waits', []), fullKernelTreeSettlement=False)
                    state['armed'] = False
                elif op == 'stop':
                    state['revoked'] = True
                    released.set()
                else:
                    raise ValueError('control_operation')
                print(json.dumps({'reply': op, 'events': state['events']}), flush=True)
                if op == 'stop':
                    break
    finally:
        released.set()
        server.shutdown()
        thread.join(timeout=2)
        server.server_close()
        if thread.is_alive():
            raise ValueError('software_server_join_unconfirmed')


if __name__ == '__main__':
    run(sys.argv[1])
