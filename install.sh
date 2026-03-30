#!/bin/bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
CLAUDE_DIR="$HOME/.claude"

# go-review skill
mkdir -p "$CLAUDE_DIR/skills/go-review"
ln -sf "$REPO_DIR/go-review-team/SKILL.md" "$CLAUDE_DIR/skills/go-review/SKILL.md"

# go-review-team agents
mkdir -p "$CLAUDE_DIR/agents/go-review-team"
for agent in review-lead security-reviewer style-reviewer error-reviewer structure-reviewer; do
  ln -sf "$REPO_DIR/go-review-team/$agent.md" "$CLAUDE_DIR/agents/go-review-team/$agent.md"
done

# feature-review skill
mkdir -p "$CLAUDE_DIR/skills/feature-review"
ln -sf "$REPO_DIR/feature-review-team/SKILL.md" "$CLAUDE_DIR/skills/feature-review/SKILL.md"

# feature-review-team agents
mkdir -p "$CLAUDE_DIR/agents/feature-review-team"
for agent in acceptance-lead product-reviewer safety-reviewer quality-reviewer maintainability-reviewer documentation-reviewer; do
  ln -sf "$REPO_DIR/feature-review-team/$agent.md" "$CLAUDE_DIR/agents/feature-review-team/$agent.md"
done

echo "Installed:"
echo "  ~/.claude/skills/go-review/SKILL.md -> go-review-team/SKILL.md"
echo "  ~/.claude/agents/go-review-team/ -> go-review-team/*.md (5 agents)"
echo "  ~/.claude/skills/feature-review/SKILL.md -> feature-review-team/SKILL.md"
echo "  ~/.claude/agents/feature-review-team/ -> feature-review-team/*.md (6 agents)"
