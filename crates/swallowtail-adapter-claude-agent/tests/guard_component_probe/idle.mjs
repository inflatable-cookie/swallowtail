// Repository-owned structural peer; never a vendor behavior specimen.
export class IdleCompaction {
  constructor(host,options){this.host=host;this.setTimer=options.setTimeout;this.disposed=false;}
  noteRequest(){}
  onIdle(){if(!this.disposed)this.setTimer(()=>void this.fire(),2147483647);}
  async fire(){if(!this.disposed)await this.host.compact();}
  dispose(){this.disposed=true;}
}
