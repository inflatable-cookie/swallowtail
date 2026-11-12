// Fake-only host admission model. Production must validate before starting Node.
import { constants, closeSync, fstatSync, openSync, readFileSync, realpathSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { resolve, join } from 'node:path';
import { pathToFileURL } from 'node:url';

const expected = Object.freeze({
  'preload.mjs': '69e92c9441d600427c6ece40f70a0f57d036c06a604e87461297f0fbc0eb9a72',
  'loader.mjs': 'de6365aa9e9dff14717d4a26d4c833a836bba8a63b649cd730256cdc7f67b30a',
  'entry.mjs': '7fb4e6c07bb456352e24d0d301506f0d492caf18e07e9762aa1de8d42a558210',
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
    entrySource: sources['entry.mjs'], idleSource: sources['idle.mjs'],
    loaderUrl: dataUrl(sources['loader.mjs']),
  };
  const payload = Buffer.from(JSON.stringify(config)).toString('base64url');
  return Object.freeze({ args: Object.freeze(['--import', `${dataUrl(sources['preload.mjs'])}#${payload}`, join(base, 'entry.mjs')]), environment: Object.freeze({}) });
}
