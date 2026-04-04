Rebase the current branch on main and resolve any conflicts. Follow these steps:

1. Run `git fetch origin main` to ensure main is up to date
2. Run `git log --oneline main..HEAD` to see what commits are on this branch
3. Run `git log --oneline HEAD..origin/main` to see what's new on main
4. If already up to date, say so and stop
5. Run `git rebase origin/main`
6. If there are conflicts:
   - For each conflicted file, read the file to understand the conflict markers
   - Resolve conflicts by editing the files, preferring to integrate both sides when possible
   - Run `git add` on resolved files
   - Run `git rebase --continue`
   - Repeat until the rebase is complete
7. Run `git log --oneline -10` to confirm the result

Important:
- Never use `--force` flags without asking first
- If a conflict is ambiguous or risky, ask before resolving
- Always run `gofmt` on any Go files that were conflict-resolved before continuing the rebase
