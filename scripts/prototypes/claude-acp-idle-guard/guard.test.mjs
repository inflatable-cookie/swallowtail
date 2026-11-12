import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, copyFileSync, writeFileSync, symlinkSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { prepareFakeLaunch } from './host-model.mjs';
import * as loader from './loader.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const names = ['preload.mjs', 'loader.mjs', 'entry.mjs', 'idle.mjs'];
function fixture(missing) {
  const dir = mkdtempSync(join(process.env.TMPDIR, 'acp-guard-'));
  for (const name of names) if (name !== missing) copyFileSync(join(here, name === 'entry.mjs' || name === 'idle.mjs' ? 'fakes' : '', name), join(dir, name));
  return dir;
}
function run(args, env = {}) {
  const child = spawnSync(process.execPath, args, { env, encoding: 'utf8', timeout: 10000 });
  assert.equal(child.status, 0, child.stderr || String(child.error));
  return JSON.parse(child.stdout);
}

test('guard precedes entry effects while sessions survive repeated and direct idle', () => {
  const dir = fixture();
  const baseline = run([join(dir, 'entry.mjs')]);
  assert.equal(baseline.scheduled, 6);
  assert.deepEqual(baseline.firedBeforeDeadline, [0, 0, 0]);
  assert.deepEqual(baseline.sessions.map(s => s.idle.fired), [3, 3, 3]);
  const guarded = prepareFakeLaunch(dir);
  const result = run(guarded.args, guarded.environment);
  assert.deepEqual(result.events, ['entrypoint', 'joined']);
  assert.deepEqual(result.permission, { user: 'denyInsteadOfPrompting', managed: true });
  assert.deepEqual(result.sessions.map(s => s.kind), ['new', 'load', 'resume']);
  assert.equal(result.scheduled, 0);
  assert.deepEqual(result.sessions.map(s => s.idle.fired), [0, 0, 0]);
  assert.ok(result.sessions.every(s => s.closed));
  assert.equal(result.sessions[0].cancelled, true);
});

test('rejects startup controls, missing, changed, duplicate and aliased modules before spawn', () => {
  const dir = fixture();
  for (const key of ['NODE_OPTIONS', 'NODE_PATH', 'NODE_REPL_EXTERNAL_MODULE']) assert.throws(() => prepareFakeLaunch(dir, { [key]: '--require bad' }), /unsafe/);
  for (const args of [['--require', 'bad'], ['--import', 'bad'], ['--loader', 'bad'], ['--import', 'one', '--import', 'one']]) assert.throws(() => prepareFakeLaunch(dir, {}, args), /unsafe/);
  for (const name of names) assert.throws(() => prepareFakeLaunch(fixture(name)));
  const changed = fixture();
  writeFileSync(join(changed, 'entry.mjs'), "import './idle.mjs'; import './idle.mjs';");
  assert.throws(() => prepareFakeLaunch(changed), /digest mismatch/);
  const target = fixture();
  const alias = join(target, 'alias');
  symlinkSync(target, alias);
  assert.throws(() => prepareFakeLaunch(alias), /noncanonical/);
});

test('loader rejects query and caller aliases', () => {
  loader.initialize({ entryUrl: 'file:///fake/entry.mjs', idleUrl: 'file:///fake/idle.mjs' });
  assert.equal(loader.resolve('./idle.mjs', { parentURL: 'file:///fake/entry.mjs' }).url, 'file:///fake/idle.mjs');
  for (const [specifier, parentURL] of [['./idle.mjs?x', 'file:///fake/entry.mjs'], ['file:///fake/idle.mjs', 'file:///fake/entry.mjs'], ['./idle.mjs', 'file:///fake/entry.mjs?x'], ['./idle.mjs', 'file:///fake/other.mjs']]) assert.throws(() => loader.resolve(specifier, { parentURL }), /unapproved/);
  assert.throws(() => loader.load('file:///fake/idle.mjs?x'), /unapproved/);
});

test('launch uses the verified byte snapshots after all paths are replaced', () => {
  const dir = fixture();
  const launch = prepareFakeLaunch(dir);
  for (const name of names) writeFileSync(join(dir, name), "throw new Error('replacement executed')");
  const result = run(launch.args, launch.environment);
  assert.equal(result.scheduled, 0);
  assert.deepEqual(result.events, ['entrypoint', 'joined']);
});
