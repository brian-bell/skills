# skills

A collection of Claude Code skills, agent-installed skills, and agent teams for code review, feature acceptance review, and product analysis.

## Skills

### `/go-review` — Go Code Review

Spawns a team of 4 specialized reviewers (structure, errors, style, security) that analyze all Go source files and produce a consolidated, prioritized report.

```
/go-review                        # review entire project
/go-review ./cmd/server           # review a subdirectory
/go-review . security             # run only the security reviewer
/go-review ./pkg error,style      # combine path and focus
```

**Focus options:** `structure`, `error`, `style`, `security`

### `/feature-review` — Feature Acceptance Review

Spawns a team of 5 specialized reviewers (product, safety, quality, maintainability, documentation) that evaluate a feature or PR and produce an acceptance verdict.

```
/feature-review #42               # review a pull request
/feature-review scanner           # review a feature by name
/feature-review #15 safety,quality  # review specific aspects only
```

**Focus options:** `product`, `safety`, `quality`, `maintainability`, `documentation`

### `/product-manager` — Product Analysis

Analyzes a codebase, dispatches 4 research agents to investigate competitors, market trends, user pain points, and distribution channels, then delivers a structured product brief with prioritized feature recommendations.

## Agent-Installed Skills

The repo also includes two skills installed to `~/.agents/skills/`:

- `commit` — Create clean local-only git commits from the current worktree without pushing or opening a PR.
- `ship` — Follow the `commit` workflow, then push the branch and create a PR only if one does not already exist.

### Commands

Slash commands for common git workflows:

- `/commit` — Commit the current changeset (splits discrete changes into separate commits)
- `/docs` — Update CLAUDE.md and README.md to reflect the current codebase
- `/rebase` — Rebase the current branch on main and resolve conflicts
- `/ship` — Commit, push, and open a PR for the current branch

## Installation

Clone the repo and run the install script:

```bash
git clone <repo-url> ~/dev/skills
cd ~/dev/skills
./install.sh
```

The installer uses two install modes:

- Claude-facing skills, agents, and commands are symlinked into `~/.claude/`.
- Repo-managed Codex skills under `codex-skills/` are copied into `~/.agents/skills/`.

This split is intentional: Codex skill discovery does not reliably pick up symlinked skill files under `~/.agents/skills/`.

After changing Claude-facing skills or commands, run `/reload-plugins` in Claude Code.
After changing `codex-skills/`, rerun `./install.sh` to refresh the copied files.

## Directory Structure

```
skills/
├── go-review-team/
│   ├── SKILL.md                   # /go-review skill
│   ├── review-lead.md             # orchestrator agent
│   ├── security-reviewer.md
│   ├── style-reviewer.md
│   ├── error-reviewer.md
│   └── structure-reviewer.md
├── feature-review-team/
│   ├── SKILL.md                   # /feature-review skill
│   ├── acceptance-lead.md         # orchestrator agent
│   ├── product-reviewer.md
│   ├── safety-reviewer.md
│   ├── quality-reviewer.md
│   ├── maintainability-reviewer.md
│   └── documentation-reviewer.md
├── product-manager/
│   ├── SKILL.md                   # /product-manager skill
│   ├── research-agent.md
│   └── product-brief-template.md
├── commands/
│   ├── commit.md                  # /commit command
│   ├── docs.md                    # /docs command
│   ├── rebase.md                  # /rebase command
│   └── ship.md                    # /ship command
├── codex-skills/
│   ├── commit/
│   └── ship/
└── install.sh
```

## Updating

- Changes under `go-review-team/`, `feature-review-team/`, `product-manager/`, and `commands/` are live through symlinks. Run `/reload-plugins` inside Claude Code to pick them up.
- Changes under `codex-skills/` are copied into `~/.agents/skills/`. Rerun `./install.sh` after editing them.
