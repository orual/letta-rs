#!/usr/bin/env bash

# Claude Code Hook: Branch Protection
# Prevents direct commits/pushes to main branch

# Get the git command from Claude tool input
COMMAND=$(echo "$CLAUDE_TOOL_INPUT" | jq -r '.command')

# Check for dangerous git operations on main branch
if echo "$COMMAND" | grep -q -E 'git\s+(commit|push.*\smain|push.*origin\s+main)'; then
    # Block the tool call with structured JSON output
    cat <<EOF
{
  "decision": "block",
  "reason": "🚫 Direct commits to main prohibited. Please use feature branches:\n  git checkout -b feat/your-feature-name\n  # make changes\n  git add . && git commit -m 'feat: description'\n  git push -u origin feat/your-feature-name\n  gh pr create"
}
EOF
    exit 0
fi

# Approve the tool call
echo '{"decision": "approve"}'
exit 0
