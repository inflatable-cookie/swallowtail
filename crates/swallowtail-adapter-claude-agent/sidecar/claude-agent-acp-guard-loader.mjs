// Captured-source loader. Admission and interpreter custody are host responsibilities.
import * as nodeModule from 'node:module';
const { registerHooks, isBuiltin } = nodeModule;
let files;
let edges;
let idleUrl;
let mcpRefusalUrl;
let remoteControlUrl;
let deniedChildParents = new Set();
const moduleFacades = new Map();
const childFacades = new Map();
const deniedChildFacades = new Set();
let requireParents;
let native;
let platformPrograms;
const childWaits = new Map();

// The official ACP shutdown calls process.exit after synchronous SDK close.
// Keep the real ChildProcess close/wait obligation before letting ACP exit.
export function trackChild(child) {
  const waited = new Promise(resolve => child.once('close', resolve));
  childWaits.set(child, waited);
  waited.then(() => childWaits.delete(child));
  return child;
}

function installJoinedExit() {
  const exit = process.exit.bind(process);
  let exiting = false;
  Object.defineProperty(process, 'exit', { configurable: false, writable: false, value(code) {
    if (exiting) return;
    exiting = true;
    const stop = setTimeout(() => {
      for (const child of childWaits.keys()) child.kill('SIGKILL');
    }, 3000);
    const failed = setTimeout(() => exit(79), 5000);
    (async () => {
      while (childWaits.size) await Promise.all([...childWaits.values()]);
      clearTimeout(stop); clearTimeout(failed); exit(code);
    })();
  }});
}

export function initialize(config) {
  if (!config || !Array.isArray(config.files) || !Array.isArray(config.edges)) throw new Error('invalid capture');
  files = new Map();
  for (const file of config.files) {
    if (typeof file.url !== 'string' || typeof file.source !== 'string' || !['module', 'commonjs', 'json'].includes(file.format)) throw new Error('invalid module');
    const url = new URL(file.url);
    if (url.protocol !== 'file:' || url.href !== file.url || url.search || url.hash || url.pathname.includes('%') || files.has(file.url)) throw new Error('invalid module identity');
    files.set(file.url, file);
  }
  idleUrl = config.idleUrl;
  if (!files.has(idleUrl)) throw new Error('missing idle module');
  mcpRefusalUrl = config.mcpRefusalUrl ?? null;
  if (mcpRefusalUrl !== null && (typeof mcpRefusalUrl !== 'string' || !files.has(mcpRefusalUrl))) throw new Error('missing mcp refusal module');
  remoteControlUrl = config.remoteControlUrl ?? null;
  if (remoteControlUrl !== null && (typeof remoteControlUrl !== 'string' || !files.has(remoteControlUrl))) throw new Error('missing remote control module');
  deniedChildParents = new Set();
  for (const parent of config.deniedChildParents ?? []) {
    if (typeof parent !== 'string' || !files.has(parent) || deniedChildParents.has(parent)) throw new Error('invalid denied child parent');
    deniedChildParents.add(parent);
  }
  if (config.platform && config.platform !== `${process.platform}-${process.arch}`) throw new Error('runtime platform mismatch');
  native = config.native;
  platformPrograms = new Map((config.platformPrograms ?? []).map(program => [program.parent, program.binding]));
  requireParents = new Map();
  for (const [caller, parent] of config.requireParents ?? []) {
    if (!files.has(caller) || !files.has(parent)) throw new Error('invalid require parent');
    const parents = requireParents.get(caller) ?? new Set();
    if (parents.has(parent)) throw new Error('duplicate require parent');
    parents.add(parent); requireParents.set(caller, parents);
  }
  edges = new Map();
  for (const edge of config.edges) {
    if (typeof edge.specifier !== 'string' || (edge.parent !== null && !files.has(edge.parent)) || (!files.has(edge.target) && !isBuiltin(edge.target) && edge.target !== native?.url)) throw new Error('invalid edge');
    if (edge.target === 'node:child_process' && !native) throw new Error('missing native custody');
    const key = JSON.stringify([edge.parent, edge.specifier]);
    if (edges.has(key)) throw new Error('duplicate edge');
    edges.set(key, edge.target);
  }
}

export function install(config) {
  if (Number(process.versions.node.split('.')[0]) < 22 || typeof registerHooks !== 'function' || typeof isBuiltin !== 'function') throw new Error('required Node hook APIs unavailable');
  initialize(config);
  if (native) installJoinedExit();
  registerHooks({ resolve, load });
}

export function resolve(specifier, context) {
  // Only our generated facade may import the actual module builtin. Vendor
  // callers receive createRequire bound to their captured parent, without
  // loader registration, extensions, caches or arbitrary resolver options.
  if (moduleFacades.has(context.parentURL) && specifier === 'node:module') return { url: specifier, shortCircuit: true };
  if (childFacades.has(context.parentURL) && specifier === import.meta.url) return { url: specifier, shortCircuit: true };
  if (childFacades.has(context.parentURL) && ['node:child_process', 'node:fs', 'node:crypto'].includes(specifier)) return { url: specifier, shortCircuit: true };
  const target = edges.get(JSON.stringify([context.parentURL ?? null, specifier]));
  if (!target) throw new Error('unapproved module edge');
  if (target === 'node:module') {
    const url = `swallowtail-module:${encodeURIComponent(context.parentURL)}`;
    moduleFacades.set(url, context.parentURL);
    return { url, shortCircuit: true };
  }
  if (target === 'node:child_process') {
    if (deniedChildParents.has(context.parentURL)) {
      const url = `swallowtail-child-denied:${encodeURIComponent(context.parentURL)}`;
      deniedChildFacades.add(url);
      return { url, shortCircuit: true };
    }
    const url = `swallowtail-child:${encodeURIComponent(context.parentURL)}`;
    childFacades.set(url, context.parentURL);
    return { url, shortCircuit: true };
  }
  return { url: target, shortCircuit: true };
}

export function load(url, context, nextLoad) {
  if (isBuiltin(url) || url === import.meta.url) return nextLoad(url, context);
  if (deniedChildFacades.has(url)) return deniedChildFacade();
  if (childFacades.has(url)) return childFacade(native, platformPrograms.get(childFacades.get(url)));
  if (moduleFacades.has(url)) {
    const parent = JSON.stringify(moduleFacades.get(url));
    const allowedParents = [moduleFacades.get(url), ...requireParents.get(moduleFacades.get(url)) ?? []];
    const allowed = JSON.stringify(allowedParents.slice(1));
    const resolutions = JSON.stringify([...edges].filter(([key]) => allowedParents.includes(JSON.parse(key)[0])));
    return { format: 'module', shortCircuit: true, source: `
import { createRequire as original } from 'node:module';
const parent = ${parent};
const allowed = [parent, ...${allowed}];
const resolutions = new Map(${resolutions});
export function createRequire(filename) {
  const value = filename instanceof URL ? filename.href : filename;
  const selected = allowed.find(url => value === url || value === new URL(url).pathname);
  if (!selected) throw new Error('require parent mismatch');
  const raw = original(selected);
  const target = specifier => {
    if (typeof specifier !== 'string') throw new Error('unapproved module edge');
    const found = resolutions.get(JSON.stringify([selected, specifier]));
    if (!found) throw new Error('unapproved module edge');
    return found;
  };
  const require = specifier => { target(specifier); return raw(specifier); };
  // Node22 does not route createRequire().resolve through synchronous hooks.
  // Resolve only the admitted captured/native/builtin edge, without disk IO.
  Object.defineProperty(require, 'resolve', { value: (...args) => {
    if (args.length !== 1) throw new Error('resolver options refused');
    const found = target(args[0]);
    return found.startsWith('node:') ? found : new URL(found).pathname;
  } });
  return Object.freeze(require);
}
export default Object.freeze({ createRequire });
` };
  }
  if (!files.has(url)) throw new Error('unapproved module identity');
  const file = files.get(url);
  let source = file.source;
  if (url === idleUrl) source += `
Object.defineProperty(IdleCompaction.prototype, 'onIdle', { value() {}, writable: false, configurable: false });
Object.defineProperty(IdleCompaction.prototype, 'fire', { value() {}, writable: false, configurable: false });
const SwallowtailIdleOriginal = IdleCompaction;
let SwallowtailIdleGuard;
SwallowtailIdleGuard = new Proxy(SwallowtailIdleOriginal, {
  construct(target, args, newTarget) {
    if (newTarget !== SwallowtailIdleGuard) throw new Error('idle subclass refused');
    return Object.preventExtensions(Reflect.construct(target, args));
  }
});
Object.defineProperty(IdleCompaction.prototype, 'constructor', { value: SwallowtailIdleGuard, writable: false, configurable: false });
Object.freeze(IdleCompaction.prototype);
IdleCompaction = SwallowtailIdleGuard;
Object.freeze(IdleCompaction);
`;
  if (url === mcpRefusalUrl) source += `
if (typeof startAroundRefusedMcpServers !== 'function' || typeof refusedMcpServers !== 'function') throw new Error('missing mcp refusal guard target');
startAroundRefusedMcpServers = async function startAroundRefusedMcpServers(_hasMcpServers, start) {
  return await start(false);
};
refusedMcpServers = function refusedMcpServers() {
  throw new Error('silent MCP server drop refused');
};
Object.freeze(startAroundRefusedMcpServers);
Object.freeze(refusedMcpServers);
`;
  if (url === remoteControlUrl) source += `
if (typeof RemoteControlAtStartup !== 'function') throw new Error('missing remote control guard target');
Object.defineProperty(RemoteControlAtStartup.prototype, 'connect', { value() {}, writable: false, configurable: false });
const SwallowtailRemoteControlOriginal = RemoteControlAtStartup;
let SwallowtailRemoteControlGuard;
SwallowtailRemoteControlGuard = new Proxy(SwallowtailRemoteControlOriginal, {
  construct(target, _args, newTarget) {
    if (newTarget !== SwallowtailRemoteControlGuard) throw new Error('remote control subclass refused');
    return Object.preventExtensions(Reflect.construct(target, [false]));
  }
});
Object.defineProperty(RemoteControlAtStartup.prototype, 'constructor', { value: SwallowtailRemoteControlGuard, writable: false, configurable: false });
Object.freeze(RemoteControlAtStartup.prototype);
RemoteControlAtStartup = SwallowtailRemoteControlGuard;
Object.freeze(RemoteControlAtStartup);
if (typeof startsWithRemoteControl === 'function') {
  startsWithRemoteControl = function startsWithRemoteControl() { return false; };
  Object.freeze(startsWithRemoteControl);
}
if (typeof startRemoteControl === 'function') {
  startRemoteControl = async function startRemoteControl() { throw new Error('remote control at startup refused'); };
  Object.freeze(startRemoteControl);
}
`;
  return { format: file.format, source, shortCircuit: true };
}

function deniedChildFacade() {
  return { format: 'module', shortCircuit: true, source: `
function refused() { throw new Error('child process invocation refused'); }
export function spawn() { refused(); }
export function spawnSync() { refused(); }
export function execFile() { refused(); }
export function execFileSync() { refused(); }
const custom = Symbol.for('nodejs.util.promisify.custom');
Object.defineProperty(execFile, custom, { value: refused, writable: false, configurable: false });
Object.freeze(spawn); Object.freeze(spawnSync); Object.freeze(execFile); Object.freeze(execFileSync);
export default Object.freeze({ spawn, spawnSync, execFile, execFileSync });
` };
}

function childFacade(binding, processStarts) {
  return { format: 'module', shortCircuit: true, source: `
import { trackChild } from ${JSON.stringify(import.meta.url)};
import * as original from 'node:child_process';
import { readFileSync, realpathSync, lstatSync } from 'node:fs';
import { createHash } from 'node:crypto';
const native = ${JSON.stringify(binding)};
const ps = ${JSON.stringify(processStarts ?? null)};
function check(command, args, options) {
  if (!Array.isArray(args) || args.some(arg => typeof arg !== 'string') || options?.shell) throw new Error('unapproved native invocation');
  let binding = native;
  if (command === 'ps' && ps && args.length === 4 && args[0] === '-o' && args[1] === 'pid=,lstart=' && args[2] === '-p' && /^[0-9]+(,[0-9]+)*$/.test(args[3]) && args[3].length <= 4096 && options?.timeout === 1000) { command = ps.path; binding = ps; }
  if (command !== binding.path) throw new Error('unapproved native invocation');
  if (Object.keys(options?.env ?? process.env).some(key => /^(NODE_|LD_|DYLD_|BUN_|OPENSSL_)/i.test(key))) throw new Error('unsafe native startup environment');
  for (const parent of binding.ancestors) {
    const info = lstatSync(parent.path, {bigint:true});
    if (!info.isDirectory() || info.dev.toString() !== parent.dev || info.ino.toString() !== parent.ino || info.uid.toString() !== String(parent.uid) || info.mode.toString() !== String(parent.mode)) throw new Error('native ancestor custody changed');
  }
  const info = lstatSync(command, {bigint:true});
  if (realpathSync(command) !== command || !info.isFile() || info.uid.toString() !== String(binding.uid) || info.size.toString() !== String(binding.bytes) || info.dev.toString() !== binding.dev || info.ino.toString() !== binding.ino || (info.mode & 0o022n) !== 0n || (info.mode & 0o111n) === 0n) throw new Error('native custody changed');
  const bytes = readFileSync(command);
  if (bytes.length !== binding.bytes || createHash('sha256').update(bytes).digest('hex') !== binding.sha256) throw new Error('native source changed');
  return command;
}
export function spawn(command, args = [], options = {}) { return trackChild(original.spawn(check(command,args,options),args,options)); }
export function spawnSync(command, args = [], options = {}) { return original.spawnSync(check(command,args,options),args,options); }
export function execFile(command, args = [], options = {}, callback) { return trackChild(original.execFile(check(command,args,typeof options === 'object' ? options : {}),args,options,callback)); }
export function execFileSync(command, args = [], options = {}) { return original.execFileSync(check(command,args,options),args,options); }
const custom = Symbol.for('nodejs.util.promisify.custom');
if (original.execFile[custom]) Object.defineProperty(execFile,custom,{value:(command,args=[],options={})=>{const result=original.execFile[custom](check(command,args,options),args,options);trackChild(result.child);return result;}});
export default Object.freeze({spawn,spawnSync,execFile,execFileSync});
` };
}

// Private selected-SDK absence probe. Never forwards to an ambient builtin API.
export function sdkStartupBuiltinLookup(...args) {
  if (args.length === 1 && args[0] === "async_hooks") return undefined;
  throw new Error("direct builtin access refused");
}
