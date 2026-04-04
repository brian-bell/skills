#!/bin/bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")" && pwd)"
CLAUDE_DIR="$HOME/.claude"
AGENTS_DIR="$HOME/.agents"

# Claude-facing skills and commands still use symlinks so repo edits remain live
# after a Claude reload. Codex-managed skills under ~/.agents/skills are copied
# instead because symlinked SKILL.md/openai.yaml files are not discovered
# reliably by Codex skill loading.

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

# product-manager skill
mkdir -p "$CLAUDE_DIR/skills/product-manager"
ln -sf "$REPO_DIR/product-manager/SKILL.md" "$CLAUDE_DIR/skills/product-manager/SKILL.md"
ln -sf "$REPO_DIR/product-manager/research-agent.md" "$CLAUDE_DIR/skills/product-manager/research-agent.md"
ln -sf "$REPO_DIR/product-manager/product-brief-template.md" "$CLAUDE_DIR/skills/product-manager/product-brief-template.md"

# commands
mkdir -p "$CLAUDE_DIR/commands"
for cmd in commit docs rebase ship; do
  ln -sf "$REPO_DIR/commands/$cmd.md" "$CLAUDE_DIR/commands/$cmd.md"
done

# Repo-managed Codex skills are copied into ~/.agents/skills so Codex sees
# regular files during discovery. Recreate each managed directory on install to
# avoid leaving stale files behind after skill updates.
mkdir -p "$AGENTS_DIR/skills"
for skill_dir in "$REPO_DIR"/codex-skills/*; do
  [ -d "$skill_dir" ] || continue

  skill_name="$(basename "$skill_dir")"
  target_dir="$AGENTS_DIR/skills/$skill_name"
  rm -rf "$target_dir"
  mkdir -p "$target_dir"
  cp -R "$skill_dir"/. "$target_dir"/
done

echo "Installed:"
echo "  ~/.claude/skills/go-review/SKILL.md -> go-review-team/SKILL.md"
echo "  ~/.claude/agents/go-review-team/ -> go-review-team/*.md (5 agents)"
echo "  ~/.claude/skills/feature-review/SKILL.md -> feature-review-team/SKILL.md"
echo "  ~/.claude/agents/feature-review-team/ -> feature-review-team/*.md (6 agents)"
echo "  ~/.claude/skills/product-manager/ -> product-manager/*.md (skill + 2 templates)"
echo "  ~/.claude/commands/ -> commands/*.md (4 commands)"
echo "  ~/.agents/skills/ <= copied from codex-skills/* (2 skills)"
