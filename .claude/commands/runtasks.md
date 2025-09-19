---
description: Systematically execute the feature’s tasks.md via the runtask subagent, saving progress after each task.
---

Given the context provided as an argument, do this:

1. Locate the tasks file
   - If a path is provided in $ARGUMENTS, use it.
   - Otherwise, derive from the current Git branch name: `.specify/specs/[feature-name]/tasks.md`.
   - Resolve to an absolute path. Fail with a clear message if the file does not exist.

2. Determine execution plan
   - Parse `tasks.md` to identify ordered task IDs (e.g., T001, T002, …), statuses, and dependencies.
   - Select the next incomplete task in dependency order.

3. Execute tasks sequentially
   - For each incomplete task in order:
     - Invoke the `runtask` subagent (`.claude/agents/runtask`) with:
       - task-file: absolute path to `tasks.md`
       - task-id: exact task number (e.g., T003)
       - arguments: pass-through `$ARGUMENTS`
     - Run non-interactively. Stop immediately if:
       - The subagent reports a blocker or explicit STOP condition
       - Instructions are unclear or unsafe
     - On success:
       - Mark the task as completed in `tasks.md` (use a checkmark)
       - Add brief notes if relevant
       - Save `tasks.md` to disk immediately before continuing

4. Safety and constraints
   - Use absolute paths for all file operations.
   - Never perform destructive actions or anything that could cause data loss without explicit human approval.
   - If any prerequisite is missing, stop and report.

5. Error handling and recovery
   - If a task fails, record the failure reason in `tasks.md` under that task, save the file, and stop.
   - Surface the exact failing task-id and error summary.

6. Reporting
   - When stopping (complete, blocked, or error), report:
     - Tasks file path
     - Completed task IDs
     - Next task to run
     - Stopping reason

Context for execution: $ARGUMENTS

Use absolute paths with the repository root for all file operations to avoid path issues.