// This executable does not admit or execute any original provider artifact.
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

if (process.argv.length !== 3 || process.argv[2] !== '--fake') {
  process.stderr.write('original probe refused: verified runtime custody and separate authority are unavailable\n');
  process.exitCode = 78;
} else {
  const testFile = fileURLToPath(new URL('./guard.test.mjs', import.meta.url));
  const result = spawnSync(process.execPath, ['--test', '--test-timeout=15000', '--test-force-exit', testFile], {
    env: { TMPDIR: process.env.TMPDIR }, encoding: 'utf8', timeout: 30000, maxBuffer: 1024 * 1024,
  });
  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);
  if (result.error || result.status === null) {
    process.stderr.write('fake probe failed or exceeded its bound\n');
    process.exitCode = 1;
  } else process.exitCode = result.status;
}
