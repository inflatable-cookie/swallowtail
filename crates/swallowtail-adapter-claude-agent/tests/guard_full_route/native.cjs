// First-party native peer. Never loads Claude code or contacts a provider.
const fs = require('node:fs');
const http = require('node:http');
const root = process.argv.at(-2);
const port = Number(process.argv.at(-1));
const trace = (phase, extra={}) => fs.appendFileSync(root+'/trace.jsonl', JSON.stringify({phase,pid:process.pid,parent:process.ppid,...extra})+'\n');
const send = value => process.stdout.write(JSON.stringify(value)+'\n');
let active;
let buffer='';
trace('native-start');
function finish(denied) {
  if (!denied) { fs.readFileSync(root+'/tool.txt'); trace('tool-read'); }
  const request = http.request({host:'127.0.0.1',port,path:denied?'/turn/denied':'/turn/allowed',method:'POST',headers:{'content-type':'application/json'}},response=>{
    let bytes='';response.on('data',part=>{bytes+=part;if(bytes.length>4096)request.destroy(Error('owned endpoint output limit'));});
    response.on('end',()=>{trace('endpoint-result',{denied});send({kind:'done',denied,text:JSON.parse(bytes).text});active=undefined;});
  });
  request.setTimeout(1000,()=>request.destroy(Error('owned endpoint timeout')));
  request.on('error',error=>{send({kind:'error',message:error.message});process.exitCode=1;});
  request.end(JSON.stringify({denied}));
}
function message(value) {
  if(value.kind==='prompt') {
    trace('native-prompt');active=value;
    if(value.hold) {send({kind:'holding'});return;}
    const settings=JSON.parse(fs.readFileSync(root+'/settings.json','utf8'));
    const denied=settings.managed?.permissions?.denyInsteadOfPrompting ?? settings.user?.permissions?.denyInsteadOfPrompting ?? false;
    if(denied) {trace('native-denied');finish(true);} else {trace('native-ask');send({kind:'ask'});}
  } else if(value.kind==='permission') {
    trace('native-permission',{allowed:value.allowed});finish(!value.allowed);
  } else if(value.kind==='cancel') {
    trace('native-cancel');active=undefined;send({kind:'cancelled'});
  } else if(value.kind==='close') {
    trace('native-close');process.stdout.write(JSON.stringify({kind:'closed'})+'\n',()=>process.exit(0));
  } else throw Error('unexpected owned native input');
}
process.stdin.setEncoding('utf8');
process.stdin.on('data',chunk=>{buffer+=chunk;if(buffer.length>16384)throw Error('owned native input limit');for(;;){const end=buffer.indexOf('\n');if(end<0)break;const line=buffer.slice(0,end);buffer=buffer.slice(end+1);message(JSON.parse(line));}});
process.stdin.on('end',()=>{trace('native-eof');process.exit(0);});
