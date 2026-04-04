# Skills Repo

A git repo of Claude Code skills, agent-installed skills, and agent teams. Claude-facing assets are installed via symlinks to `~/.claude/`; repo-managed Codex skills are copied into `~/.agents/skills/`.

## Repo Layout

- `<name>-team/` directories contain agent teams (a lead agent + specialist agents)
- `<name>-team/SKILL.md` is the user-invocable skill that launches the team
- Other `.md` files in team directories are agent definitions
- `product-manager/` follows the same pattern (SKILL.md + supporting templates) but uses a flat name instead of `-team/` suffix
- `commands/` contains slash commands (e.g. `/commit`, `/ship`)
- `codex-skills/` contains repo-managed agent-installed skills
- `install.sh` symlinks Claude-facing assets into `~/.claude/` and copies repo-managed Codex skills into `~/.agents/skills/`

## Install Targets

| Repo path | Installed to |
|---|---|
| `<name>-team/SKILL.md` | `~/.claude/skills/<name>/SKILL.md` |
| `<name>-team/<agent>.md` | `~/.claude/agents/<name>-team/<agent>.md` |
| `commands/<cmd>.md` | `~/.claude/commands/<cmd>.md` |
| `codex-skills/<name>/*` | `~/.agents/skills/<name>/*` |

Claude-facing installs use symlinks, so edits take effect after `/reload-plugins`.
Repo-managed Codex skills are copied, so rerun `./install.sh` after editing `codex-skills/`.

Current repo-managed agent-installed skills:

- `commit`
- `ship`

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
4. Add install wiring to `install.sh` if the new skill needs it
5. Run `./install.sh && /reload-plugins`
