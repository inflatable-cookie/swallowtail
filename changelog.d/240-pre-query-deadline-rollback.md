Fix SDK session recovery when deadline-task reservation or worker creation
fails before query submission. Roll back the exact unsubmitted recipient under
the connection lock; retain all timeout drain protections. Cover both transient
refusals and genuine subsequent completion with owned regression tests.
