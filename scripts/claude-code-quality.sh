#!/usr/bin/env bash
#
#
# Claude Code Hook: Code Quality Check
# Runs cargo check and rustfmt fix after file modifications

echo "🔍 Running code quality checks..."

# Run cargo check
echo "  → Type checking..."
cargo check
if [ $? -ne 0 ]; then
    echo "❌ errors found"
    echo '{"decision": "block", "reason": "Errors detected. Please fix compile errors before proceeding."}'
    exit 0
fi

# Run pre-commit hooks for formatting and linting
echo "  → Checking formatting and linting"
just pre-commit-all
if [ $? -ne 0 ]; then
    echo "Formatting and linting failed"
    echo '{"decision": "block", "reason": "Formatting or linting failed. Please fix linting errors manually."}'
    exit 0
fi

echo "✅ Code quality checks passed"
echo '{"decision": "approve"}'
exit 0
