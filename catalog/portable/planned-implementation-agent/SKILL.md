---
name: planned-implementation-agent
description: Plan and execute implementation tasks through a reviewed plan, TDD, review-loop quality gates, and a worker subagent. Use when the user asks to implement a task by first writing a detailed implementation plan, wants the plan itself reviewed or iterated, or wants a subagent dispatched to carry out an approved implementation plan.
---

# Planned Implementation Agent

Use this workflow to turn a task into a reviewed implementation plan, then delegate execution to a worker subagent. The main agent owns planning quality, subagent scope, verification, and the final report.

## Defaults

- Plan review loops: minimum `1`
- Implementation review loops: minimum `2`
- Quality gate: `8/10`
- TDD expectation: write or update failing tests before behavior changes whenever the codebase supports tests.
- Handoff style: bounded worker subagent with a concrete plan, acceptance criteria, and verification commands.

Honor user overrides for loop count, quality gate, test strategy, delegation boundaries, or whether the main agent should execute instead of a subagent.

## 1. Establish Scope

1. Read project instructions and the files needed to understand the request.
2. Discover existing test, format, lint, typecheck, and build commands from repo docs, manifests, and CI.
3. Check `git status --short` and protect unrelated user work.
4. Identify:
   - User-visible behavior or API changes.
   - Files and modules likely to change.
   - Existing tests that should guide the work.
   - Risks, migrations, compatibility boundaries, and rollback concerns.

Ask only for missing requirements that cannot be inferred safely from the repo or the user's request.

## 2. Write The Implementation Plan

Create a detailed plan before editing implementation files. The plan may live in the conversation unless the user asks for a file.

The plan must include:

- Goal and non-goals.
- Current system observations with file references.
- Proposed implementation steps in dependency order.
- A TDD section with the first failing tests or test updates to write, expected red state, implementation path, and refactor pass.
- A review-loop section describing at least one implementation critique loop, review criteria, quality gate, and how findings will be addressed.
- Verification commands and expected evidence.
- Subagent handoff package: exact scope, constraints, files to inspect, plan steps to execute, acceptance criteria, and reporting format.
- Risks and explicit stop conditions.

Keep the plan concrete enough that another agent can execute it without re-discovering the entire problem.

## 3. Review-Loop The Plan

Before dispatching implementation, run at least one review-loop on the plan itself.

For each plan review loop:

1. Give the reviewer the user request, relevant repo observations, and the draft plan.
2. Ask for a score, blocking gaps, missing tests, scope risks, sequencing problems, and clearer acceptance criteria.
3. Revise the plan for actionable findings.
4. Continue until the required minimum loops are complete and the plan meets the quality gate, or explain why the remaining findings are out of scope.

Do not dispatch the worker subagent until the plan has completed at least one review loop.

## 4. Dispatch The Worker Subagent

Use the available multi-agent/subagent tooling. If subagent tools are not visible, search for them with `tool_search` using a query such as `subagent` or `multi-agent`.

Give the worker:

- The reviewed implementation plan.
- The TDD requirement: create or update tests first, observe the expected failure, then implement.
- The implementation review-loop requirement and quality gate.
- The exact verification commands to run.
- Guardrails to avoid reverting unrelated work or widening scope.
- A request to report changed files, tests added, review-loop score/findings, verification results, and unresolved risks.

The worker may implement and iterate, but the main agent remains responsible for final validation.

## 5. Inspect And Integrate

After the worker returns:

1. Read the worker's summary, diff, and verification output.
2. Inspect the actual changes directly; do not rely only on the worker report.
3. Run or re-run the most relevant checks in the main context when feasible.
4. Address any remaining review findings, failed checks, or integration issues.
5. If substantial fixes are needed after the worker returns, run another implementation review-loop before finalizing.

## 6. Report

Summarize:

- Plan review-loop score and key revisions.
- Worker subagent outcome.
- TDD evidence: failing tests introduced or updated, implementation, and passing result.
- Implementation review-loop score and any fixes made.
- Verification commands and results.
- Changed files and any residual risks.

## Guardrails

- Never skip the plan review loop unless the user explicitly cancels this skill's workflow.
- Never give a worker vague authority to redesign unrelated parts of the repo.
- Never claim TDD was used without naming the tests and red/green evidence.
- Never hide failed checks, unresolved review findings, or skipped verification.
- Do not let tests or smoke runs touch real user configuration or production services unless explicitly configured.
