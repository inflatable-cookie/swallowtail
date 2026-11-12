let config;
export function initialize(value) { config = value; }
export function resolve(specifier, context) {
  if (specifier === config.entryUrl && context.parentURL === undefined) {
    return { url: config.entryUrl, shortCircuit: true };
  }
  if (specifier === './idle.mjs' && context.parentURL === config.entryUrl) {
    return { url: config.idleUrl, shortCircuit: true };
  }
  throw new Error(`unapproved module edge: ${specifier} from ${context.parentURL}`);
}
export function load(url) {
  if (url === config.entryUrl) return { format: 'module', source: config.entrySource, shortCircuit: true };
  if (url === config.idleUrl) {
    const source = config.idleSource + '\nIdleCompaction.prototype.onIdle = function () {};\nIdleCompaction.prototype.fire = function () {};\n';
    return { format: 'module', source, shortCircuit: true };
  }
  throw new Error('unapproved module identity');
}
