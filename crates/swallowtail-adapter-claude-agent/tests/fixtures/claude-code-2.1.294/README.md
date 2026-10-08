# Claude Code 2.1.294 headless currentness stop

This supplemental fixture extends Research 374 with the exact npm wrapper, Darwin arm64 package, and Linux x64 package for 2.1.294. `dist-inventory.json` retains every complete file inventory from 2.1.281 through 2.1.293 and adds the complete recursive 2.1.294 tree and adjacent deltas. Tarball SHA-512 integrity matches current npm registry metadata. No downloaded executable was run.

At observation, npm `latest` and GitHub latest non-prerelease agree on 2.1.294; npm `stable` remains a delayed channel at 2.1.286. npm `next` points to 2.1.295, which is not part of the stable target.

The 2.1.294 release notes identify changes to instruction-form prompt and agent hook safety and Stop/SubagentStop behavior. These hooks remain active through the selected `user,project,local` settings. The changes require adaptation beyond the approved 2.1.287 stream and 2.1.290 post-PreToolUse recheck work. Research 415 records this stop; the production claim remains through 2.1.281.
