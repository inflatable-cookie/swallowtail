export class IdleCompaction {
  constructor(clock) { this.clock = clock; this.scheduled = 0; this.fired = 0; }
  onIdle() { this.scheduled += 1; this.clock.schedule(() => this.fire()); }
  fire() { this.fired += 1; }
}
