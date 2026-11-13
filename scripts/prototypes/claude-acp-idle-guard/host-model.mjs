// Fake-only host admission model. Production must validate before starting Node.
import { constants, closeSync, fstatSync, openSync, readFileSync, realpathSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { resolve, join } from 'node:path';
import { pathToFileURL } from 'node:url';

const expected = Object.freeze({
  'preload.mjs': '69e92c9441d600427c6ece40f70a0f57d036c06a604e87461297f0fbc0eb9a72',
  'loader.mjs': '512a2b607f94b29ff215ed2eb6028ab09bc6a06ee30e14b6d6c9f798d5f0c71a',
  'entry.mjs': 'd39aac344d9efacbbe3496508da015d63a4e6146f171063ff031215e96c66490',
  'idle.mjs': '3d26aac0f45f61026c81fae01d191086384db796192cc196af092187472b99b2',
});
const dataUrl = source => `data:text/javascript;base64,${Buffer.from(source).toString('base64')}`;

function snapshot(path, digest) {
  if (realpathSync(path) !== path) throw new Error('noncanonical path');
  const fd = openSync(path, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    if (!fstatSync(fd).isFile()) throw new Error('not a regular file');
    const bytes = readFileSync(fd);
    if (createHash('sha256').update(bytes).digest('hex') !== digest) throw new Error('digest mismatch');
    return bytes.toString('utf8');
  } finally { closeSync(fd); }
}

export function prepareFakeLaunch(root, environment = {}, startupArgs = []) {
  if (startupArgs.length !== 0 || ['NODE_OPTIONS', 'NODE_PATH', 'NODE_REPL_EXTERNAL_MODULE'].some(key => environment[key])) {
    throw new Error('unsafe startup control');
  }
  const base = resolve(root);
  if (realpathSync(base) !== base) throw new Error('noncanonical root');
  const sources = Object.fromEntries(Object.entries(expected).map(([name, digest]) => [name, snapshot(join(base, name), digest)]));
  const config = {
    entryUrl: pathToFileURL(join(base, 'entry.mjs')).href,
    idleUrl: pathToFileURL(join(base, 'idle.mjs')).href,
    files: [ { url: pathToFileURL(join(base, 'entry.mjs')).href, format: 'module', source: sources['entry.mjs'] }, { url: pathToFileURL(join(base, 'idle.mjs')).href, format: 'module', source: sources['idle.mjs'] } ],
    edges: [ { parent: null, specifier: pathToFileURL(join(base, 'entry.mjs')).href, target: pathToFileURL(join(base, 'entry.mjs')).href }, { parent: pathToFileURL(join(base, 'entry.mjs')).href, specifier: './idle.mjs', target: pathToFileURL(join(base, 'idle.mjs')).href } ],
    loaderUrl: dataUrl(sources['loader.mjs']),
  };
  const payload = Buffer.from(JSON.stringify(config)).toString('base64url');
  return Object.freeze({ args: Object.freeze(['--import', `${dataUrl(sources['preload.mjs'])}#${payload}`, join(base, 'entry.mjs')]), environment: Object.freeze({}) });
}
