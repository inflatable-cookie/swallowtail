// Captured-source loader. Admission and interpreter custody are host responsibilities.
import { registerHooks, isBuiltin } from 'node:module';
let files;
let edges;
let idleUrl;
const moduleFacades = new Map();
const childFacades = new Map();
let requireParents;
let native;
let platformPrograms;

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
  initialize(config);
  registerHooks({ resolve, load });
}

export function resolve(specifier, context) {
  // Only our generated facade may import the actual module builtin. Vendor
  // callers receive createRequire bound to their captured parent, without
  // loader registration, extensions, caches or arbitrary resolver options.
  if (moduleFacades.has(context.parentURL) && specifier === 'node:module') return { url: specifier, shortCircuit: true };
  if (childFacades.has(context.parentURL) && ['node:child_process', 'node:fs', 'node:crypto'].includes(specifier)) return { url: specifier, shortCircuit: true };
  const target = edges.get(JSON.stringify([context.parentURL ?? null, specifier]));
  if (!target) throw new Error('unapproved module edge');
  if (target === 'node:module') {
    const url = `swallowtail-module:${encodeURIComponent(context.parentURL)}`;
    moduleFacades.set(url, context.parentURL);
    return { url, shortCircuit: true };
  }
  if (target === 'node:child_process') {
    const url = `swallowtail-child:${encodeURIComponent(context.parentURL)}`;
    childFacades.set(url, context.parentURL);
    return { url, shortCircuit: true };
  }
  return { url: target, shortCircuit: true };
}

export function load(url, context, nextLoad) {
  if (isBuiltin(url)) return nextLoad(url, context);
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
  return { format: file.format, source, shortCircuit: true };
}

function childFacade(binding, processStarts) {
  return { format: 'module', shortCircuit: true, source: `
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
export function spawn(command, args = [], options = {}) { return original.spawn(check(command,args,options),args,options); }
export function spawnSync(command, args = [], options = {}) { return original.spawnSync(check(command,args,options),args,options); }
export function execFile(command, args = [], options = {}, callback) { return original.execFile(check(command,args,typeof options === 'object' ? options : {}),args,options,callback); }
export function execFileSync(command, args = [], options = {}) { return original.execFileSync(check(command,args,options),args,options); }
const custom = Symbol.for('nodejs.util.promisify.custom');
if (original.execFile[custom]) Object.defineProperty(execFile,custom,{value:(command,args=[],options={})=>original.execFile[custom](check(command,args,options),args,options)});
export default Object.freeze({spawn,spawnSync,execFile,execFileSync});
` };
}
