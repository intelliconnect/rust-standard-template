#!/bin/bash

BRANCH_NAME=$(git rev-parse --abbrev-ref HEAD)
PATTERN="^(feature|bugfix|hotfix|release|add|refactor)/[a-z0-9]+([-_.][a-z0-9]+)*$"

if [[ ! $BRANCH_NAME =~ $PATTERN ]]; then
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "❌ BRANCH NAME REJECTED"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
  echo "Your current branch: \"$BRANCH_NAME\""
  echo ""
  echo "Branch names must follow this pattern:"
  echo "  <type>/<description>"
  echo ""
  echo "Allowed branch types:"
  echo "  • feature/   - New features (feature/add-login)"
  echo "  • bugfix/    - Bug fixes (bugfix/fix-crash)"
  echo "  • hotfix/    - Critical fixes (hotfix/security-patch)"
  echo "  • release/   - Release branches (release/v1.2.0)"
  echo "  • add/       - Small additions (add/update-readme)"
  echo "  • refactor/  - Internal changes (refactor/cleanup-auth)"
  echo ""
  echo "✅ Valid examples:"
  echo "  git checkout -b feature/add-expense-tracking"
  echo "  git checkout -b bugfix/fix-calculation-error"
  echo "  git checkout -b hotfix/fix-security-issue"
  echo "  git checkout -b add/update-readme"
  echo "  git checkout -b refactor/cleanup-auth"
  echo ""
  echo "To fix: Rename your branch"
  echo "  git branch -m $BRANCH_NAME feature/your-feature-name"
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo ""
  exit 1
fi

echo "✅ Branch name is valid: \"$BRANCH_NAME\""
exit 0