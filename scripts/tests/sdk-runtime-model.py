"""Test-only bootstrap stand-in. No runtime report execution or host admission.

The real SDK/sidecar still runs owned JS on the existing local test interpreter.
Its bootstrap loader/cache closure is explicitly modeled, never qualified here.
"""

import os
from pathlib import Path


def observation(custody, node, expected):
    class SoftwareInputs(custody.Inputs):
        def bind_ancestors(self, path):
            # Deliberately no host-ancestry attestation in this software model.
            pass

    capture = SoftwareInputs()
    try:
        capture.bind(node, expected)
        for name in custody.python_inputs():
            capture.bind(name)
        return custody.RuntimeObservation(
            {
                "node": str(Path(node).absolute()),
                "nodeSha256": expected,
                "version": "owned-software-model; not an observed Node version",
                "libraries": {},
                "kernelMappings": [],
                "unresolvedSystemLibraries": [],
                "startup": list(custody.STARTUP),
                "decision": custody.DECISION,
                "ownerUid": os.getuid(),
                "softwareBootstrapModel": True,
            },
            capture,
        )
    except BaseException:
        capture.close()
        raise
