// Private SDK234 owned-graph loader, derived from ACP loader c0d591438.
// Retains captured resolve/load and createRequire; removes ACP idle mutation.
// This is selected graph preparation, not a general JavaScript sandbox.
import * as nodeModule from 'node:module';
const { registerHooks, isBuiltin } = nodeModule;
let files;
let edges;
const moduleFacades = new Map();
const childFacades = new Map();
let requireParents;
let native;

export function initialize(config) {
  if (!config || !Array.isArray(config.files) || !Array.isArray(config.edges)) throw new Error('invalid capture');
  files = new Map();
  for (const file of config.files) {
    if (typeof file.url !== 'string' || typeof file.source !== 'string' || !['module', 'commonjs', 'json'].includes(file.format)) throw new Error('invalid module');
    const url = new URL(file.url);
    if (url.protocol !== 'file:' || url.href !== file.url || url.search || url.hash || url.pathname.includes('%') || files.has(file.url)) throw new Error('invalid module identity');
    files.set(file.url, file);
  }
  if (config.platform && config.platform !== `${process.platform}-${process.arch}`) throw new Error('runtime platform mismatch');
  native = config.native;
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
  if (childFacades.has(url)) return childFacade(native, childFacades.get(url));
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
  return { format: file.format, source: file.source, shortCircuit: true };
}

function childFacade(binding, caller) {
  const allowed = binding.callers.includes(caller);
  return { format: 'module', shortCircuit: true, source: `
import * as original from 'node:child_process';
import { readFileSync, realpathSync, lstatSync } from 'node:fs';
import { createHash } from 'node:crypto';
const binding = ${JSON.stringify(binding)};
let used = false;
function refuse() { throw new Error('unapproved child edge'); }
function check(command, args, options) {
  if (!${JSON.stringify(allowed)} || used || command !== binding.path ||
      JSON.stringify(args) !== JSON.stringify(binding.argv) || options?.cwd !== binding.cwd ||
      options?.shell || options?.detached ||
      JSON.stringify(Object.entries(options?.env ?? {}).sort()) !== JSON.stringify(Object.entries(binding.env).sort()) ||
      JSON.stringify(options?.stdio) !== JSON.stringify(['pipe','pipe','pipe'])) refuse();
  for (const parent of binding.ancestors) {
    const info = lstatSync(parent.path, {bigint:true});
    if (!info.isDirectory() || info.dev.toString() !== parent.dev || info.ino.toString() !== parent.ino ||
        info.uid.toString() !== parent.uid || info.mode.toString() !== parent.mode) refuse();
  }
  const info = lstatSync(command, {bigint:true});
  if (realpathSync(command) !== command || !info.isFile() || info.uid.toString() !== binding.uid ||
      info.size.toString() !== binding.bytes || info.dev.toString() !== binding.dev || info.ino.toString() !== binding.ino ||
      (info.mode & 0o022n) !== 0n || (info.mode & 0o111n) === 0n ||
      createHash('sha256').update(readFileSync(command)).digest('hex') !== binding.sha256) refuse();
  used = true;
}
export function spawn(command, args, options) { check(command,args,options); return original.spawn(command,args,options); }
export const spawnSync = refuse;
export const execFile = refuse;
export const execFileSync = refuse;
export const exec = refuse;
export const execSync = refuse;
export const fork = refuse;
export default Object.freeze({spawn,spawnSync,execFile,execFileSync,exec,execSync,fork});
` };
}
