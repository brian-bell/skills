# skills

A collection of Claude Code skills and agent teams for code review, feature acceptance review, and product analysis.

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

## Installation

Clone the repo and run the install script:

```bash
git clone <repo-url> ~/dev/skills
cd ~/dev/skills
./install.sh
```

The script creates symlinks from the repo into `~/.claude/`, so edits to files in this repo take effect immediately — just run `/reload-plugins` in your Claude Code session.

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
└── install.sh
```

## Updating

Since `install.sh` creates symlinks, any edits you make in this repo are live immediately. Run `/reload-plugins` inside Claude Code to pick up changes to skill and agent definitions.
