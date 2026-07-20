---
description: Create a git branch for the current feature
agent: build
---

Execute the following to create a feature branch:

```bash
.specify/scripts/bash/create-feature-branch.sh $ARGUMENTS
```

Parse the JSON output and report BRANCH_NAME to the user.
If the script fails, report the error.
