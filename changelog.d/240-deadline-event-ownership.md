Keep a timed-out SDK turn as the connection event recipient until its exact
native abort terminal is drained and the owned interrupt write has settled.
Refuse new queries while either is pending; serialize expiry, event dispatch
and replacement under the existing connection lock. Preserve one public
TimedOut outcome and bounded close/wait when a terminal never arrives.

Cover held, missing, duplicate and racing terminals plus normal reuse with
controlled owned peers. Refresh only affected original prepared lifecycle cases
on disposable Linux/Mac runners; retain accepted component evidence, frozen
identities, security fixes and the trimmed source scope.

Reject a terminal before the replacement turn has actually started, so a stale
record cannot supply its public outcome. Duplicate starts and terminals refuse.
