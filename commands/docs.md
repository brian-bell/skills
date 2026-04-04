Update CLAUDE.md and README.md to reflect the current state of the codebase, and scan docs/ for outdated content. Follow these steps:

1. **Gather current state**: Read the codebase to understand the project's actual behavior:
   - Read all Go source files (cmd/, model/, ui/, scanner/, and any other packages)
   - Read the Makefile, go.mod, CI config, and any other config files
   - Run `git log --oneline -20` to see recent changes
   - Read the legacy/ directory if it exists

2. **Update CLAUDE.md**: Read the existing CLAUDE.md (create it if missing). Update it to accurately describe:
   - What the project is and how it's structured
   - How to build, test, and run (`make build`, `make test`, `go run ./cmd/wt`, etc.)
   - Key packages and their responsibilities
   - Any conventions or patterns used in the code
   - Remove anything that no longer matches the code

3. **Update README.md**: Read the existing README.md. Compare it against the actual code and update:
   - Features and commands — ensure they match what the code actually supports
   - Installation instructions — ensure they match the current build system
   - Usage examples — ensure they work with the current CLI interface
   - Requirements — ensure they list actual dependencies
   - Remove references to things that no longer exist; add references to things that are missing

4. **Scan docs/ folder**: If a docs/ directory exists, read every file in it and:
   - Flag content that contradicts the current source code
   - Update outdated instructions, API references, or architecture descriptions
   - Remove docs for features that no longer exist
   - If no docs/ folder exists, skip this step silently

5. **Show a summary**: After making changes, output a brief list of what was updated and why

Important:
- Only change documentation files — never modify source code
- Preserve the existing tone and structure of each file where possible
- When in doubt about intended behavior, read the source code — it is the source of truth
- Run `gofmt` checks or `make test` if needed to verify your understanding of the code
