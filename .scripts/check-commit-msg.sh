#!/bin/bash

# Get commit message from file
COMMIT_MSG_FILE=$1
COMMIT_MSG=$(head -n1 "$COMMIT_MSG_FILE")

# Conventional Commit types
MSG_PATTERN="^(feat|fix|chore|docs|style|refactor|test|perf|ci): .+"

if [[ ! $COMMIT_MSG =~ $MSG_PATTERN ]]; then
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "❌ COMMIT MESSAGE REJECTED"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
  echo "Your message: \"$COMMIT_MSG\""
  echo ""
  echo "Commit messages must follow Conventional Commits format:"
  echo "  <type>: <description>"
  echo ""
  echo "Allowed message types:"
  echo "  • feat     - New feature (feat: add login page)"
  echo "  • fix      - Bug fix (fix: resolve crash on startup)"
  echo "  • chore    - Maintenance (chore: update dependencies)"
  echo "  • docs     - Documentation (docs: update README)"
  echo "  • style    - Formatting (style: fix indentation)"
  echo "  • refactor - Code refactor (refactor: improve performance)"
  echo "  • test     - Tests (test: add unit tests)"
  echo "  • perf     - Performance (perf: optimize query)"
  echo "  • ci       - CI/CD (ci: update workflow)"
  echo ""
  echo "✅ Valid examples:"
  echo "  git commit -m \"feat: add expense tracking feature\""
  echo "  git commit -m \"fix: resolve notes deletion bug\""
  echo "  git commit -m \"chore: update dependencies\""
  echo ""
  echo "To fix: Use correct format"
  echo "  git commit --amend -m \"<type>: <message>\""
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
  exit 1
fi

echo "✅ Commit message is valid: \"$COMMIT_MSG\""
exit 0