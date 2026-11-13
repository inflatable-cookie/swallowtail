// Original-class component witness. Every host operation below is owned fake work.
import { IdleCompaction } from './idle.mjs';

let now = 0;
let armed = 0;
let fired = 0;
const timers = new Set();
const pending = new Set();
const settings = Object.freeze({idleCompaction:true,autoCompactEnabled:true,permissions:Object.freeze({denyInsteadOfPrompting:true}),env:Object.freeze({DISABLE_COMPACT:'false',DISABLE_AUTO_COMPACT:'false'})});
const settingsBefore = JSON.stringify(settings);
Date.now = () => now;
const originalClear = globalThis.clearTimeout;
globalThis.clearTimeout = timer => timers.has(timer) ? timers.delete(timer) : originalClear(timer);
function setTimer(fn, ms) {
  armed++;
  const timer = {at:now+ms,fn,unref(){}};
  timers.add(timer);
  return timer;
}
function finish() {
  for (const job of pending) job.resolve();
  return Promise.all([...pending].map(job=>job.promise));
}
const host = {
  settings:()=>settings, contextTokens:()=>250000, isBusy:()=>false, log(){},
  compact() {
    fired++;
    const job = {};
    job.promise = new Promise(resolve=>{job.resolve=resolve;});
    pending.add(job);
    return job.promise.finally(()=>pending.delete(job));
  },
};
const sessions = ['new-model','load-model','resume-model'].map(kind=>({kind,idle:new IdleCompaction(host,{env:{CLAUDE_AGENT_ACP_IDLE_COMPACT_DELAY_MS:'2147483647',DISABLE_COMPACT:'1'},setTimeout:setTimer})}));
for (let cycle=0;cycle<2;cycle++) {
  for (const session of sessions) {
    session.idle.noteRequest({cache_creation:{ephemeral_1h_input_tokens:250000}},String(cycle));
    session.idle.onIdle(); session.idle.onIdle();
  }
  now += 2147483648;
  for (const timer of [...timers]) if(timer.at<=now) {timers.delete(timer);timer.fn();}
  await finish();
  now += 30*24*60*60*1000;
}
for (const session of sessions) {
  session.idle.noteRequest({cache_creation:{ephemeral_1h_input_tokens:250000}},'direct');
  const direct=session.idle.fire();
  await finish(); await direct;
  session.idle.dispose();
}
let prototypeLocked=false, ownLocked=false;
try { Object.defineProperty(IdleCompaction.prototype,'onIdle',{value(){}}); } catch {prototypeLocked=true;}
try { Object.defineProperty(sessions[0].idle,'fire',{value(){}}); } catch {ownLocked=true;}
process.stdout.write(JSON.stringify({entryRan:true,armed,fired,joined:pending.size===0,settingsUnchanged:settingsBefore===JSON.stringify(settings),prototypeLocked,ownLocked,modeledContexts:sessions.map(s=>s.kind)}));
