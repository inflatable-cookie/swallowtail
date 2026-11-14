// Fake-only synchronous capture: not runtime custody or an original-artifact launcher.
import { registerHooks } from 'node:module';
const config = JSON.parse(Buffer.from(new URL(import.meta.url).hash.slice(1), 'base64url').toString('utf8'));
const files = new Map();
for (const file of config.files) {
  const url = new URL(file.url);
  if (url.protocol !== 'file:' || url.href !== file.url || url.search || url.hash || url.pathname.includes('%') || files.has(file.url) || !['module', 'commonjs'].includes(file.format)) throw new Error('invalid capture identity');
  files.set(file.url, file);
}
const edges = new Map();
for (const edge of config.edges) {
  if (edge.parent !== null && !files.has(edge.parent)) throw new Error('invalid caller');
  if (!files.has(edge.target) && edge.target !== 'node:module') throw new Error('invalid target');
  const key = JSON.stringify([edge.parent, edge.specifier]);
  if (edges.has(key)) throw new Error('duplicate edge');
  edges.set(key, edge.target);
}
registerHooks({
  resolve(specifier, context) {
    const target = edges.get(JSON.stringify([context.parentURL ?? null, specifier]));
    if (!target) throw new Error('unapproved module edge');
    return { url: target, shortCircuit: true };
  },
  load(url, context, nextLoad) {
    if (url === 'node:module') return nextLoad(url, context);
    const file = files.get(url);
    if (!file) throw new Error('unapproved module identity');
    return { format: file.format, source: file.source, shortCircuit: true };
  },
});
