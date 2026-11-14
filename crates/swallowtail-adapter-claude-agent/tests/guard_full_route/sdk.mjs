// Replaced constants are captured with this owned SDK hook source by the test host.
import { spawn } from 'node:child_process';
import { appendFileSync } from 'node:fs';
const nativePath = __NATIVE_PATH__;
const nativeProgram = __NATIVE_PROGRAM__;
const root = process.env.SWALLOWTAIL_FAKE_ROOT;
const trace = (phase,extra={}) => appendFileSync(root+'/trace.jsonl',JSON.stringify({phase,pid:process.pid,...extra})+'\n');
let child, exited, pending;
export function open(kind, permission, activity) {
  trace('sdk-open',{kind});
  child=spawn(nativePath,['--no-addons','--openssl-config=/dev/null','-e',nativeProgram,root,process.env.SWALLOWTAIL_FAKE_ENDPOINT_PORT],{env:{},stdio:['pipe','pipe','pipe']});
  trace('sdk-native-start',{child:child.pid});
  exited=new Promise((resolve,reject)=>{child.once('error',reject);child.once('exit',(code,signal)=>{trace('sdk-native-wait',{child:child.pid,code,signal});code===0&&signal===null?resolve():reject(Error('owned native exit'));});});
  let bytes='';
  child.stdout.setEncoding('utf8');child.stdout.on('data',chunk=>{bytes+=chunk;if(bytes.length>16384)throw Error('owned SDK output limit');for(;;){const end=bytes.indexOf('\n');if(end<0)break;const value=JSON.parse(bytes.slice(0,end));bytes=bytes.slice(end+1);
    if(value.kind==='ask') {trace('sdk-can-use-tool');permission().then(allowed=>child.stdin.write(JSON.stringify({kind:'permission',allowed})+'\n'));}
    else if(value.kind==='holding')activity();
    else if(value.kind==='done'||value.kind==='cancelled') {const settle=pending;pending=undefined;settle(value);}
    else if(value.kind==='error')throw Error(value.message);
    else if(value.kind!=='closed')throw Error('unexpected owned native output');
  }});
}
export function prompt(hold) {return new Promise(resolve=>{pending=resolve;child.stdin.write(JSON.stringify({kind:'prompt',hold})+'\n');});}
export function cancel() {trace('sdk-cancel');child.stdin.write(JSON.stringify({kind:'cancel'})+'\n');}
export async function close() {if(child){child.stdin.write(JSON.stringify({kind:'close'})+'\n');await exited;trace('sdk-joined');child=undefined;}}
