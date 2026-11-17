// Source inspection only. Acorn parses strings; no artifact is imported/evaluated.
// The tool's own embedded-parser bootstrap is separate from vendor capabilities.
const fs = require('node:fs');
const crypto = require('node:crypto');
const builtIn = process.binding('natives')['internal/deps/acorn/acorn/dist/acorn'];
if (!builtIn) throw new Error('embedded Acorn unavailable');
const parserModule = { exports: {} };
new Function('exports', 'module', builtIn)(parserModule.exports, parserModule);
const parser = parserModule.exports;
const hash = value => crypto.createHash('sha256').update(value).digest('hex');
const input = JSON.parse(fs.readFileSync(0, 'utf8'));
const result = {};
for (const [key, source] of Object.entries(input)) {
  const ast = parser.parse(source, { ecmaVersion: 'latest', sourceType: 'module', allowHashBang: true });
  const nodes = [];
  function visit(node, ancestors = []) {
    if (!node || typeof node.type !== 'string') return;
    nodes.push({ node, ancestors });
    for (const value of Object.values(node)) {
      if (Array.isArray(value)) for (const child of value) visit(child, [...ancestors, node]);
      else if (value && typeof value === 'object') visit(value, [...ancestors, node]);
    }
  }
  visit(ast);
  function span(node) {
    const text = source.slice(node.start, node.end);
    if (Buffer.byteLength(text) > 32768) throw new Error('selected expression bound: ' + key);
    return { startByte: Buffer.byteLength(source.slice(0, node.start)), endByte: Buffer.byteLength(source.slice(0, node.end)), sha256: hash(text), text };
  }
  function owner(ancestors) {
    const node = [...ancestors].reverse().find(n => /Function/.test(n.type) && (n.id || ancestors[ancestors.indexOf(n)-1]?.type === 'MethodDefinition')) ?? [...ancestors].reverse().find(n => /Function/.test(n.type));
    const method = node && ancestors[ancestors.indexOf(node)-1];
    return node ? { startByte: Buffer.byteLength(source.slice(0, node.start)), name: node.id?.name ?? method?.key?.name ?? '<arrow>' } : { name: '<module>' };
  }
  const imports = [], aliases = new Map(), promisifiers = new Set(), creators = new Set(), requires = new Map();
  for (const n of ast.body) if (n.type === 'ImportDeclaration') {
    for (const spec of n.specifiers) {
      if (['child_process', 'node:child_process'].includes(n.source.value)) {
        if (spec.type !== 'ImportSpecifier') throw new Error('unclassified child-process namespace');
        aliases.set(spec.local.name, { imported: spec.imported.name, via: 'import' });
      }
      if (['util', 'node:util'].includes(n.source.value) && spec.imported?.name === 'promisify') promisifiers.add(spec.local.name);
      if (['module', 'node:module'].includes(n.source.value) && spec.imported?.name === 'createRequire') creators.add(spec.local.name);
    }
    if (['child_process', 'node:child_process', 'module', 'node:module', 'process', 'node:process', 'worker_threads', 'node:worker_threads', 'vm', 'node:vm'].includes(n.source.value)) imports.push(span(n));
  }
  for (const {node:n} of nodes) if (n.type === 'VariableDeclarator' && n.id.type === 'Identifier' && n.init?.type === 'CallExpression' && promisifiers.has(n.init.callee.name) && aliases.has(n.init.arguments[0]?.name)) {
    aliases.set(n.id.name, { ...aliases.get(n.init.arguments[0].name), via: span(n) });
  }
  // The index's dynamic import destructures spawn inside its --cli branch.
  for (const {node:n} of nodes) if (n.type === 'VariableDeclarator' && n.id.type === 'ObjectPattern' && n.init?.type === 'AwaitExpression' && n.init.argument.type === 'ImportExpression' && n.init.argument.source.value === 'node:child_process') {
    for (const prop of n.id.properties) aliases.set(prop.value.name, { imported: prop.key.name, via: span(n) });
  }
  for (const {node:n} of nodes) if (n.type === 'VariableDeclarator' && n.id.type === 'ObjectPattern' && n.init?.type === 'AwaitExpression' && n.init.argument.type === 'ImportExpression' && n.init.argument.source.value === 'node:module') {
    for (const prop of n.id.properties) if (prop.key.name === 'createRequire') creators.add(prop.value.name);
  }
  for (const {node:n} of nodes) if (n.type === 'VariableDeclarator' && n.id.type === 'Identifier' && n.init?.type === 'CallExpression' && creators.has(n.init.callee.name)) requires.set(n.id.name,span(n));
  const calls = [], dynamic = [], raw = [], references = [], importsDynamic = [];
  for (const {node:n,ancestors} of nodes) {
    if (n.type === 'CallExpression' && n.callee.type === 'Identifier' && aliases.has(n.callee.name)) calls.push({ alias: n.callee.name, ...aliases.get(n.callee.name), owner: owner(ancestors), expression: span(n), arguments: n.arguments.map(span) });
    if (n.type === 'Identifier' && aliases.has(n.name)) references.push({ alias: n.name, parent: ancestors.at(-1)?.type, startByte: Buffer.byteLength(source.slice(0,n.start)) });
    if (n.type === 'Identifier' && ['Function','eval','Worker','require','createRequire'].includes(n.name)) {
      const p = ancestors.at(-1);
      let classification = 'reference-needs-context';
      if (n.name === 'Function') {
        if (p.type === 'MemberExpression' && p.object === n && p.property.name === 'prototype') classification = 'prototype-access-not-code-generation';
        else if (['CallExpression','NewExpression'].includes(p.type) && p.callee === n) classification = 'string-code-generation';
        else if (p.type === 'VariableDeclarator' && p.init === n) classification = 'constructor-alias';
      }
      const aliasCalls = [];
      if (classification === 'constructor-alias' && p.id.type === 'Identifier') for (const item of nodes) {
        const c = item.node;
        if (['CallExpression','NewExpression'].includes(c.type) && c.callee.type === 'Identifier' && c.callee.name === p.id.name && owner(item.ancestors).startByte === owner(ancestors).startByte) aliasCalls.push(span(c));
      }
      dynamic.push({ name:n.name, classification, owner:owner(ancestors), expression:span(p), aliasCalls, context:source.slice(Math.max(0,n.start-180),Math.min(source.length,n.end+350)) });
    }
    if (n.type === 'MemberExpression' && ['binding','_linkedBinding','execve','dlopen','mainModule','getBuiltinModule'].includes(n.computed ? n.property.value : n.property.name)) raw.push({ owner:owner(ancestors), expression:span(n), context:source.slice(Math.max(0,n.start-350),Math.min(source.length,n.end+350)) });
    if (n.type === 'ImportExpression') importsDynamic.push({ owner:owner(ancestors), expression:span(n), literal:n.source.type === 'Literal' });
    if (n.type === 'CallExpression' && (creators.has(n.callee.name) || requires.has(n.callee.name) || n.callee.type === 'MemberExpression' && requires.has(n.callee.object.name))) dynamic.push({ name:n.callee.name ?? n.callee.object.name, classification:creators.has(n.callee.name) ? 'caller-bound-createRequire' : 'captured-require-or-resolve', owner:owner(ancestors), expression:span(n), binding:requires.get(n.callee.name ?? n.callee.object.name) ?? null });
  }
  const effectOwners = new Set(calls.map(c => c.owner.name).filter(n => !n.startsWith('<')));
  const callers = [], definitions = [];
  for (const {node:n,ancestors} of nodes) {
    if (n.type === 'CallExpression' && n.callee.type === 'Identifier' && effectOwners.has(n.callee.name)) callers.push({ callee:n.callee.name, owner:owner(ancestors), expression:span(n) });
    if (n.type === 'CallExpression' && n.callee.type === 'MemberExpression' && effectOwners.has(n.callee.property.name)) callers.push({ callee:source.slice(n.callee.start,n.callee.end), owner:owner(ancestors), expression:span(n) });
    if (n.type === 'FunctionDeclaration' && effectOwners.has(n.id?.name)) definitions.push(span(n));
    if (n.type === 'MethodDefinition' && effectOwners.has(n.key?.name)) definitions.push(span(n));
  }
  if (imports.length || calls.length || dynamic.length || raw.length || importsDynamic.length) result[key] = { sha256:hash(source), imports, aliases:Object.fromEntries(aliases), calls, references, callers, definitions, dynamic, raw, importsDynamic };
}
process.stdout.write(JSON.stringify({ parserSha256:hash(builtIn), files:result })+'\n');
