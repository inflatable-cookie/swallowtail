// Static syntax inspection only: never evaluates an artifact's source.
// Input files are JSON maps of package-relative JS paths to captured text.
const crypto = require('node:crypto');
const fs = require('node:fs');
const path = require('node:path');
const builtIn = process.binding('natives')['internal/deps/acorn/acorn/dist/acorn'];
if (!builtIn) throw new Error('Node embedded Acorn is unavailable');
const parserModule = { exports: {} };
new Function('exports', 'module', builtIn)(parserModule.exports, parserModule);
const parser = parserModule.exports;
const result = { parserNode: process.version, parserSha256: crypto.createHash('sha256').update(builtIn).digest('hex'), packages: {} };
for (const name of fs.readdirSync(process.argv[2]).filter(x => x.endsWith('.json')).sort()) {
  const input = JSON.parse(fs.readFileSync(path.join(process.argv[2], name), 'utf8'));
  const files = {};
  for (const filename of Object.keys(input).sort()) {
    const text = input[filename];
    const ast = parser.parse(text, { ecmaVersion: 'latest', sourceType: filename.endsWith('.cjs') ? 'script' : 'module', allowHashBang: true, allowReturnOutsideFunction: true });
    const watchedAliases = new Map();
    for (const declaration of ast.body) if (declaration.type === 'ImportDeclaration') {
      for (const specifier of declaration.specifiers) if (specifier.type === 'ImportSpecifier' && ['createRequire', 'Worker'].includes(specifier.imported.name)) watchedAliases.set(specifier.local.name, specifier.imported.name);
    }
    const edges = [], cautions = [], stack = [ast];
    while (stack.length) {
      const n = stack.pop();
      if (!n || typeof n !== 'object') continue;
      let kind = null, arg = null;
      if (['ImportDeclaration', 'ExportNamedDeclaration', 'ExportAllDeclaration'].includes(n.type) && n.source) { kind = 'static'; arg = n.source; }
      if (n.type === 'ImportExpression') { kind = 'dynamic'; arg = n.source; }
      if (n.type === 'CallExpression' && n.callee.type === 'Identifier' && n.callee.name === 'require') { kind = 'require'; arg = n.arguments[0]; }
      if (n.type === 'CallExpression' && n.callee.type === 'MemberExpression' && n.callee.object.type === 'MetaProperty' && n.callee.object.meta.name === 'import' && (n.callee.computed ? n.callee.property.value : n.callee.property.name) === 'resolve') { kind = 'import-meta-resolve'; arg = n.arguments[0]; }
      if (n.type === 'CallExpression' && n.callee.type === 'Identifier' && watchedAliases.get(n.callee.name) === 'createRequire') cautions.push({ kind: 'call:createRequire-alias', offset: n.start });
      if (n.type === 'NewExpression' && n.callee.type === 'Identifier' && (n.callee.name === 'Worker' || watchedAliases.get(n.callee.name) === 'Worker')) { kind = 'worker'; arg = n.arguments[0]; }
      if (kind) {
        if (arg && arg.type === 'Literal' && typeof arg.value === 'string') edges.push({ kind, specifier: arg.value, offset: n.start });
        else cautions.push({ kind: kind + '-nonliteral', offset: n.start });
      }
      if (n.type === 'Identifier' && ['Worker', 'createRequire', 'require', 'eval', 'Function'].includes(n.name)) cautions.push({ kind: 'identifier:' + n.name, offset: n.start });
      for (const [key, value] of Object.entries(n)) if (key !== 'loc' && value && typeof value === 'object') Array.isArray(value) ? stack.push(...value) : stack.push(value);
    }
    const order = (a, b) => a.offset - b.offset || a.kind.localeCompare(b.kind);
    edges.sort(order); cautions.sort(order);
    const uniqueCautions = cautions.filter((value, index) => index === 0 || value.offset !== cautions[index - 1].offset || value.kind !== cautions[index - 1].kind);
    files[filename] = { sha256: crypto.createHash('sha256').update(text).digest('hex'), edges, cautions: uniqueCautions };
  }
  result.packages[name.slice(0, -5)] = files;
}
process.stdout.write(JSON.stringify(result, null, 2) + '\n');
