import { IdleCompaction } from './idle.mjs';
const events = ['entrypoint'];
const clock = { now: 0, tasks: [], schedule(fn) { this.tasks.push({ at: this.now + 2147483647, fn }); }, advance(ms) { this.now += ms; for (const task of this.tasks) if (task.at <= this.now && !task.done) { task.done = true; task.fn(); } } };
const permission = { user: 'denyInsteadOfPrompting', managed: true };
const sessions = ['new', 'load', 'resume'].map(kind => ({ kind, idle: new IdleCompaction(clock), closed: false }));
for (const session of sessions) { session.idle.onIdle(); session.idle.onIdle(); }
clock.advance(1000);
const firedBeforeDeadline = sessions.map(session => session.idle.fired);
clock.advance(30 * 24 * 60 * 60 * 1000);
for (const session of sessions) session.idle.fire();
const operations = sessions.map(session => new Promise(resolve => { session.finish = () => queueMicrotask(() => { session.closed = true; resolve(); }); }));
sessions[0].cancelled = true;
for (const session of sessions) session.finish();
await Promise.all(operations);
events.push('joined');
let prototypeMutationRefused = false;
let ownMutationRefused = false;
try { Object.defineProperty(IdleCompaction.prototype, 'onIdle', { value() {} }); } catch { prototypeMutationRefused = true; }
try { Object.defineProperty(sessions[0].idle, 'onIdle', { value() {} }); } catch { ownMutationRefused = true; }

process.stdout.write(JSON.stringify({ events, permission, sessions, firedBeforeDeadline, prototypeMutationRefused, ownMutationRefused, scheduled: clock.tasks.length }));
