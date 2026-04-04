Commit the current changeset. If there are multiple discrete logical changesets, create a separate commit for each. Follow these steps:

1. Run `git status`, `git diff HEAD`, and `git log --oneline -5` to understand what changed and match commit style
2. If on `main`, create and switch to a descriptive new branch
3. Run `git fetch origin` and check if the current branch tracks a remote. If it does and the remote is ahead, rebase onto the remote tracking branch before committing
4. Analyze the changes and group them into discrete logical changesets (by feature, purpose, or module). Unrelated changes should be separate commits.
5. For each changeset, stage only the relevant files and create a commit with a concise message
6. Run `git log --oneline -5` to confirm the result

Important:
- Do NOT push to any remote branch
- Do NOT create a pull request
- Do NOT include a Co-Authored-By trailer in commit messages
- Always run `gofmt` on changed Go files before committing
