import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, copyFileSync, writeFileSync, symlinkSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';
import { prepareFakeLaunch } from './host-model.mjs';
import * as loader from '../../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const names = ['preload.mjs', 'loader.mjs', 'entry.mjs', 'idle.mjs'];
function fixture(missing) {
  const dir = mkdtempSync(join(process.env.TMPDIR, 'acp-guard-'));
  for (const name of names) if (name !== missing) copyFileSync(name === 'loader.mjs' ? join(here, '../../../crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs') : join(here, name === 'entry.mjs' || name === 'idle.mjs' ? 'fakes' : '', name), join(dir, name));
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
  assert.equal(baseline.prototypeMutationRefused, false);
  assert.equal(baseline.ownMutationRefused, false);
  assert.deepEqual(baseline.firedBeforeDeadline, [0, 0, 0]);
  assert.deepEqual(baseline.sessions.map(s => s.idle.fired), [3, 3, 3]);
  const guarded = prepareFakeLaunch(dir);
  const result = run(guarded.args, guarded.environment);
  assert.deepEqual(result.events, ['entrypoint', 'joined']);
  assert.deepEqual(result.permission, { user: 'denyInsteadOfPrompting', managed: true });
  assert.deepEqual(result.sessions.map(s => s.kind), ['new', 'load', 'resume']);
  assert.equal(result.scheduled, 0);
  assert.equal(result.prototypeMutationRefused, true);
  assert.equal(result.ownMutationRefused, true);
  assert.deepEqual(result.sessions.map(s => s.idle.fired), [0, 0, 0]);
  assert.ok(result.sessions.every(s => s.closed));
  assert.equal(result.sessions[0].cancelled, true);
});

test('rejects startup controls, missing, changed, duplicate and aliased modules before spawn', () => {
  const dir = fixture();
  for (const key of ['NODE_OPTIONS', 'NODE_PATH', 'NODE_REPL_EXTERNAL_MODULE']) assert.throws(() => prepareFakeLaunch(dir, { [key]: '--require bad' }), /unsafe/);
  for (const args of [['--require', 'bad'], ['--import', 'bad'], ['--loader', 'bad'], ['--import', 'one', '--import', 'one']]) assert.throws(() => prepareFakeLaunch(dir, {}, args), /unsafe/);
  for (const name of names) assert.throws(() => prepareFakeLaunch(fixture(name)));
  for (const name of names) { const changed = fixture(); writeFileSync(join(changed, name), 'changed'); assert.throws(() => prepareFakeLaunch(changed), /digest mismatch/); }
  const changed = fixture();
  writeFileSync(join(changed, 'entry.mjs'), "import './idle.mjs'; import './idle.mjs';");
  assert.throws(() => prepareFakeLaunch(changed), /digest mismatch/);
  const target = fixture();
  const alias = join(target, 'alias');
  symlinkSync(target, alias);
  assert.throws(() => prepareFakeLaunch(alias), /noncanonical/);
});

test('loader rejects query and caller aliases', () => {
  loader.initialize({ idleUrl: 'file:///fake/idle.mjs', files: [{url:'file:///fake/entry.mjs',source:''},{url:'file:///fake/idle.mjs',source:''}], edges:[{parent:'file:///fake/entry.mjs',specifier:'./idle.mjs',target:'file:///fake/idle.mjs'}] });
  assert.equal(loader.resolve('./idle.mjs', { parentURL: 'file:///fake/entry.mjs' }).url, 'file:///fake/idle.mjs');
  for (const [specifier, parentURL] of [['./idle.mjs?x', 'file:///fake/entry.mjs'], ['file:///fake/idle.mjs', 'file:///fake/entry.mjs'], ['./idle.mjs', 'file:///fake/entry.mjs?x'], ['./idle.mjs', 'file:///fake/other.mjs']]) assert.throws(() => loader.resolve(specifier, { parentURL }), /unapproved/);
  assert.throws(() => loader.load('file:///fake/idle.mjs?x'), /unapproved/);
  assert.throws(() => loader.resolve('node:worker_threads', {parentURL: 'file:///fake/entry.mjs'}), /unapproved/);
});

test('launch uses the verified byte snapshots after all paths are replaced', () => {
  const dir = fixture();
  const launch = prepareFakeLaunch(dir);
  for (const name of names) writeFileSync(join(dir, name), "throw new Error('replacement executed')");
  const result = run(launch.args, launch.environment);
  assert.equal(result.scheduled, 0);
  assert.equal(result.prototypeMutationRefused, true);
  assert.equal(result.ownMutationRefused, true);
  assert.deepEqual(result.events, ['entrypoint', 'joined']);
});


test('capture rejects duplicate identities and edges and noncanonical URLs', () => {
  const idle = {url:'file:///fake/idle.mjs',source:'export class IdleCompaction {}'};
  const edge = {parent:null,specifier:'file:///fake/idle.mjs',target:idle.url};
  assert.throws(() => loader.initialize({idleUrl:idle.url,files:[idle,idle],edges:[edge]}), /identity/);
  assert.throws(() => loader.initialize({idleUrl:idle.url,files:[idle],edges:[edge,edge]}), /duplicate/);
  assert.throws(() => loader.initialize({idleUrl:'file:///fake/idle.mjs?x',files:[{...idle,url:'file:///fake/idle.mjs?x'}],edges:[]}), /identity/);
  assert.throws(() => loader.initialize({idleUrl:'file:///fake/%69dle.mjs',files:[{...idle,url:'file:///fake/%69dle.mjs'}],edges:[]}), /identity/);
  assert.throws(() => loader.initialize({idleUrl:idle.url,files:[idle],edges:[{...edge,parent:'file:///fake/other.mjs'}]}), /edge/);
});


test('original probe mode refuses without running a provider', () => {
  const probe = join(here, 'probe.mjs');
  const child = spawnSync(process.execPath, [probe, '--original'], {env:{}, encoding:'utf8', timeout:5000});
  assert.equal(child.status, 78);
  assert.match(child.stderr, /original probe refused/);
  assert.equal(child.stdout, '');
});

test('synchronous hooks serve captured CommonJS through aliased createRequire and deny escapes', () => {
  const dir = mkdtempSync(join(process.env.TMPDIR, 'acp-sync-'));
  const entryPath = join(dir, 'entry.mjs'), bridgePath = join(dir, 'bridge.cjs'), leafPath = join(dir, 'leaf.cjs');
  const entry = `import { createRequire } from 'node:module'; const require = createRequire(import.meta.url); const alias = require; const name = require.resolve('./bridge.cjs'); const result = alias('./bridge.cjs'); const denied = []; for (const specifier of ['./bridge.cjs?x', './escape.cjs', './alias.cjs', 'node:worker_threads', 'node:fs']) { try { alias(specifier); denied.push(false); } catch { denied.push(true); } try { alias.resolve(specifier); denied.push(false); } catch { denied.push(true); } } process.stdout.write(JSON.stringify({name, result, denied, runtime: process.version}));`;
  const bridge = `const alias = require; let escapeDenied = false; try { alias('./leaf.cjs?x'); } catch { escapeDenied = true; } module.exports = { leaf: alias('./leaf.cjs'), escapeDenied };`;
  const leaf = `module.exports = 'captured';`;
  for (const [file, text] of [[entryPath, entry], [bridgePath, bridge], [leafPath, leaf]]) writeFileSync(file, text);
  symlinkSync(leafPath, join(dir, 'alias.cjs'));
  const url = file => pathToFileURL(file).href;
  const entryUrl = url(entryPath), bridgeUrl = url(bridgePath), leafUrl = url(leafPath);
  const config = { files: [{url:entryUrl,format:'module',source:entry},{url:bridgeUrl,format:'commonjs',source:bridge},{url:leafUrl,format:'commonjs',source:leaf}], edges: [
    {parent:null,specifier:entryUrl,target:entryUrl}, {parent:entryUrl,specifier:'node:module',target:'node:module'},
    {parent:entryUrl,specifier:'./bridge.cjs',target:bridgeUrl}, {parent:bridgeUrl,specifier:'./leaf.cjs',target:leafUrl},
  ] };
  const preload = fileURLToPath(new URL('./sync-preload.mjs', import.meta.url));
  const preloadUrl = `data:text/javascript;base64,${Buffer.from(readFileSync(preload)).toString('base64')}#${Buffer.from(JSON.stringify(config)).toString('base64url')}`;
  for (const file of [entryPath, bridgePath, leafPath]) writeFileSync(file, "throw new Error('changed file executed')");
  const result = run(['--import', preloadUrl, entryPath]);
  assert.equal(result.name, bridgePath);
  assert.equal(result.runtime, process.version);
  assert.deepEqual(result.result, {leaf:'captured', escapeDenied:true});
  assert.deepEqual(result.denied, Array(10).fill(true));

  const duplicate = {...config, edges:[...config.edges, config.edges[0]]};
  const duplicatePreload = `data:text/javascript;base64,${Buffer.from(readFileSync(preload)).toString('base64')}#${Buffer.from(JSON.stringify(duplicate)).toString('base64url')}`;
  const refused = spawnSync(process.execPath, ['--import', duplicatePreload, entryPath], {env:{}, encoding:'utf8', timeout:10000});
  assert.notEqual(refused.status, 0);
  assert.match(refused.stderr, /duplicate edge/);
  assert.equal(refused.stdout, '');
});
