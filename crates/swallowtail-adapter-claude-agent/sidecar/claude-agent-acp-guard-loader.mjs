// Captured-source loader. Admission and interpreter custody are host responsibilities.
import { registerHooks } from 'node:module';
let files;
let edges;
let idleUrl;

export function initialize(config) {
  if (!config || !Array.isArray(config.files) || !Array.isArray(config.edges)) throw new Error('invalid capture');
  files = new Map();
  for (const file of config.files) {
    if (typeof file.url !== 'string' || typeof file.source !== 'string' || !['module', 'commonjs'].includes(file.format)) throw new Error('invalid module');
    const url = new URL(file.url);
    if (url.protocol !== 'file:' || url.href !== file.url || url.search || url.hash || url.pathname.includes('%') || files.has(file.url)) throw new Error('invalid module identity');
    files.set(file.url, file);
  }
  idleUrl = config.idleUrl;
  if (!files.has(idleUrl)) throw new Error('missing idle module');
  edges = new Map();
  for (const edge of config.edges) {
    if (typeof edge.specifier !== 'string' || (edge.parent !== null && !files.has(edge.parent)) || (!files.has(edge.target) && !(edge.target === 'node:module' && edge.specifier === edge.target))) throw new Error('invalid edge');
    const key = JSON.stringify([edge.parent, edge.specifier]);
    if (edges.has(key)) throw new Error('duplicate edge');
    edges.set(key, edge.target);
  }
  registerHooks({ resolve, load });
}

export function resolve(specifier, context) {
  const target = edges.get(JSON.stringify([context.parentURL ?? null, specifier]));
  if (!target) throw new Error('unapproved module edge');
  return { url: target, shortCircuit: true };
}

export function load(url, context, nextLoad) {
  if (url === 'node:module') return nextLoad(url, context);
  if (!files.has(url)) throw new Error('unapproved module identity');
  const file = files.get(url);
  let source = file.source;
  if (url === idleUrl) source += `
Object.defineProperty(IdleCompaction.prototype, 'onIdle', { value() {}, writable: false, configurable: false });
Object.defineProperty(IdleCompaction.prototype, 'fire', { value() {}, writable: false, configurable: false });
Object.freeze(IdleCompaction.prototype);
const SwallowtailIdleOriginal = IdleCompaction;
let SwallowtailIdleGuard;
SwallowtailIdleGuard = new Proxy(SwallowtailIdleOriginal, {
  construct(target, args, newTarget) {
    if (newTarget !== SwallowtailIdleGuard) throw new Error('idle subclass refused');
    return Object.preventExtensions(Reflect.construct(target, args));
  }
});
IdleCompaction = SwallowtailIdleGuard;
Object.freeze(IdleCompaction);
`;
  return { format: file.format, source, shortCircuit: true };
}
