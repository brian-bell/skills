# Skills Repo

A git repo of Claude Code skills and agent teams, installed via symlinks to `~/.claude/`.

## Repo Layout

- `<name>-team/` directories contain agent teams (a lead agent + specialist agents)
- `<name>-team/SKILL.md` is the user-invocable skill that launches the team
- Other `.md` files in team directories are agent definitions
- `product-manager/` follows the same pattern (SKILL.md + supporting agents/templates) but uses a flat name instead of `-team/` suffix. Not yet wired into `install.sh`.
- `install.sh` symlinks everything to the right locations under `~/.claude/`

## Install Targets

| Repo path | Installed to |
|---|---|
| `<name>-team/SKILL.md` | `~/.claude/skills/<name>/SKILL.md` |
| `<name>-team/<agent>.md` | `~/.claude/agents/<name>-team/<agent>.md` |

Symlinks, not copies — edits to repo files take effect immediately after `/reload-plugins`.

## File Anatomy

### Agent files

```yaml
---
name: security-reviewer          # referenced by subagent_type in Agent tool
description: One-line summary    # shown in agent listings
tools: Read, Glob, Grep, ...    # tools the agent can use
model: sonnet                    # model to use
effort: high                     # effort level
---

System prompt goes here.
```

### Skill files (SKILL.md)

```yaml
---
name: go-review                  # slash command name
description: Usage summary       # shown in skill listings
user_invocable: true             # makes it available as /name
---

Instructions for how to invoke the agent team.
```

## Adding a New Skill/Team

1. Create a `<name>-team/` directory
2. Add `SKILL.md` with `user_invocable: true` frontmatter
3. Add agent `.md` files (lead + specialists)
4. Add symlink entries to `install.sh`
5. Run `./install.sh && /reload-plugins`
