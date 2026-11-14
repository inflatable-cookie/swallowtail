// Owned ACP peer reached by the real prepared driver; no preloaded server replies.
import { IdleCompaction } from './idle.mjs';
import * as sdk from './sdk.mjs';
import { appendFileSync, readFileSync, writeFileSync } from 'node:fs';
const trace=(phase,extra={})=>appendFileSync(process.env.SWALLOWTAIL_FAKE_ROOT+'/trace.jsonl',JSON.stringify({phase,pid:process.pid,...extra})+'\n');
if(process.argv.includes('--version'))process.stdout.write('0.106.0\n');
else {
  let model='default',pendingPermission;
  const sessionFile=process.env.SWALLOWTAIL_FAKE_ROOT+'/session.json';
  const id='owned-full-route-session';
  const write=value=>process.stdout.write(JSON.stringify({jsonrpc:'2.0',...value})+'\n');
  const reply=(request,result)=>write({id:request,result});
  const update=value=>write({method:'session/update',params:{sessionId:id,update:value}});
  const options=()=>[{type:'select',id:'model',name:'Model',category:'model',currentValue:model,options:[{value:'default',name:'Default'},{value:'claude-sonnet-4-6',name:'Sonnet'}]}];
  const activity=status=>update({sessionUpdate:status==='in_progress'?'tool_call':'tool_call_update',toolCallId:'owned-read',title:'Read owned fixture',kind:'read',status});
  const permission=()=>new Promise(resolve=>{pendingPermission=resolve;trace('acp-permission');write({id:900,method:'session/request_permission',params:{sessionId:id,toolCall:{toolCallId:'owned-read',title:'Read owned fixture',kind:'read',status:'pending'},options:[{optionId:'allow-once',name:'Allow once',kind:'allow_once'},{optionId:'reject-once',name:'Reject once',kind:'reject_once'}]}});});
  function dispatch(m) {
    if(m.id===900&&!m.method){trace('acp-permission-response');pendingPermission(m.result?.outcome?.optionId==='allow-once');return;}
    trace('acp-method',{method:m.method});
    if(m.method==='initialize')reply(m.id,{protocolVersion:1,agentCapabilities:{_meta:{claudeCode:{promptQueueing:true}},promptCapabilities:{image:true,embeddedContext:true},loadSession:true,sessionCapabilities:{close:{},delete:{},resume:{}},providers:{}},agentInfo:{name:'@agentclientprotocol/claude-agent-acp',version:'0.106.0'},authMethods:[],_meta:{steering:{supported:true}}});
    else if(['session/new','session/load','session/resume'].includes(m.method)) {
      const idle=new IdleCompaction();idle.onIdle();idle.fire();if(idle.armed||idle.fired)throw Error('guard veto missing');
      if(m.method==='session/new')writeFileSync(sessionFile,JSON.stringify({model}));
      else model=JSON.parse(readFileSync(sessionFile,'utf8')).model;
      sdk.open(m.method,permission,()=>activity('in_progress'));
      reply(m.id,{sessionId:id,modes:{currentModeId:'default',availableModes:[{id:'default',name:'Default'}]},configOptions:options()});
    } else if(m.method==='session/set_config_option'){model=m.params.value;writeFileSync(sessionFile,JSON.stringify({model}));reply(m.id,{configOptions:options()});}
    else if(m.method==='session/prompt')sdk.prompt(m.params.prompt.some(part=>part.text==='hold')).then(result=>{
      if(result.kind==='cancelled'){reply(m.id,{stopReason:'cancelled',usage:{inputTokens:0,outputTokens:0,cachedReadTokens:0,cachedWriteTokens:0,totalTokens:0}});return;}
      activity('in_progress');activity(result.denied?'failed':'completed');update({sessionUpdate:'agent_message_chunk',content:{type:'text',text:result.text}});reply(m.id,{stopReason:'end_turn',usage:{inputTokens:1,outputTokens:1,cachedReadTokens:0,cachedWriteTokens:0,totalTokens:2}});
    });
    else if(m.method==='session/cancel')sdk.cancel();
    else if(m.method==='session/close')sdk.close().then(()=>reply(m.id,{}));
    else throw Error('unexpected owned ACP method: '+m.method);
  }
  let buffer='';process.stdin.setEncoding('utf8');process.stdin.on('data',chunk=>{buffer+=chunk;if(buffer.length>32768)throw Error('owned ACP input limit');for(;;){const end=buffer.indexOf('\n');if(end<0)break;const m=JSON.parse(buffer.slice(0,end));buffer=buffer.slice(end+1);dispatch(m);}});
  process.stdin.on('end',()=>{trace('acp-eof');sdk.close().catch(error=>{process.stderr.write(error.message);process.exitCode=1;});});
}
