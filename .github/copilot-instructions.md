# 🚨 Most Important Instruction for Copilot 🚨

**ALWAYS consult the file `tasks.md` to understand what to do next. After making any change or completing a task, you MUST update the task list in `tasks.md` to reflect your progress. This takes precedence over all other instructions.** If more information is needed, refer to `blueprint.md` and `project.md` for additional context. These files contain the most up-to-date information about the project and should be used as primary references for architecture, requirements, and implementation details.

You should also refer to the `notes/index.md` file for additional context and implementation notes. This file contains a detailed history of the project, including completed chunks and future development plans. It is important to keep this file updated with any new information or changes made during the development process.

# GitHub Copilot Instructions

## Purpose

This document serves as a guide for GitHub Copilot to understand our development conventions, coding standards, and architectural principles. By following these instructions, Copilot can generate code that seamlessly integrates with our existing codebase and follows our team's best practices.

## General Principles

### Code Quality

- Prioritize readability over cleverness. Write code that is clear and self-explanatory.
- Follow the Single Responsibility Principle: each function, class, or module should have one job.
- Aim for immutability where possible to reduce side effects.
- Use pure functions when appropriate to enhance testability and predictability.
- Avoid deep nesting of logic, preferring early returns and guard clauses.
- Keep functions small and focused (typically < 20 lines).

### Project Structure

- Place related code close together.
- Organize by feature rather than by technical role when appropriate.
- Keep the dependency graph acyclic and as flat as possible.
- Separate concerns between UI, business logic, and data access.
- Use an appropriate folder structure that promotes discoverability.

### Naming Conventions

- Use descriptive names that reveal intent, avoiding abbreviations unless universally understood.
- Prefer longer, clearer names over shorter, ambiguous ones.
- Functions should be named with verbs or verb phrases that describe their action.
- Boolean variables and functions should be phrased as questions: `isValid`, `hasPermission`.
- Use consistent casing according to the project's conventions.
- For collections, use plural nouns: `users`, `orderItems`, etc.

### Documentation

- Write self-documenting code, but include comments for complex logic or non-obvious solutions.
- Document the "why" more than the "what" or "how".
- Include JSDoc/TSDoc style comments for functions with complex signatures.
- Maintain up-to-date README files for major components or modules.
- Document assumptions and constraints rather than implementation details.

## Specific Practices

### Component Design

- Keep components small, focused, and reusable.
- Separate presentational components from logic/container components.
- Make props explicit rather than relying on deeply nested objects.
- Implement clear component interfaces with appropriate typing.
- Avoid direct DOM manipulation when abstraction layers are available.

### State Management

- Keep state as local as possible; elevate only when necessary.
- Prefer immutable state updates.
- Group related state together in meaningful objects.
- Use appropriate state management patterns based on complexity and scope.
- Implement proper state initialization and cleanup.

### Asynchronous Programming

- Use modern async patterns (async/await) rather than nested callbacks.
- Handle error cases explicitly in async functions.
- Implement appropriate loading and error states for async operations.
- Consider implementing cancellation patterns for long-running operations.
- Pay attention to race conditions in concurrent operations.

### Testing

- Write tests that verify behavior, not implementation.
- Follow the AAA pattern: Arrange, Act, Assert.
- Use descriptive test names that explain the scenario and expected outcome.
- Prefer integration tests over unit tests for complex interactions.
- Mock external dependencies, but avoid excessive mocking.
- Test edge cases and error states explicitly.

### Error Handling

- Use explicit error handling rather than allowing errors to propagate unchecked.
- Create meaningful error messages that aid debugging.
- Categorize errors appropriately (user errors vs. system errors).
- Log errors with appropriate context and severity.
- Return useful error responses from functions and APIs.
- Implement consistent error handling patterns across the codebase.

### Performance Considerations

- Avoid premature optimization, but be aware of common performance pitfalls.
- Consider time and space complexity for algorithms dealing with large datasets.
- Implement appropriate caching strategies for expensive operations.
- Be mindful of unnecessary re-renders, re-computations, or resource-intensive operations.
- Consider lazy loading, virtualization, or pagination for large data sets.

### Security Best Practices

- Never store sensitive information (API keys, tokens) in client-side code.
- Always sanitize user input and validate data before processing.
- Implement proper authentication and authorization checks.
- Be aware of common security vulnerabilities (XSS, CSRF, injection attacks).
- Use secure defaults and fail closed rather than fail open.

## Collaboration and Version Control

- Write meaningful commit messages that explain the purpose of changes.
- Keep pull requests small and focused on a single concern.
- Comment on complex or non-obvious decisions in pull request descriptions.
- Use feature flags for significant changes to enable gradual rollouts.
- Document breaking changes and migration paths clearly.

# Reference Files Usage

Always consult the following files as primary references for architecture, requirements, and implementation details:

- **blueprint.md**: Provides a step-by-step plan and high-level deliverables for the HTTPay MVP, including iterative chunks and detailed actionable subtasks for each phase of the project.
- **project.md**: Contains the minimal viable specification, including architecture diagrams, contract-level specs, message formats, storage layouts, developer constraints, and acceptance criteria. This is the authoritative source for how the system should behave and be structured.
- **tasks.md**: Lists a comprehensive, actionable task list for implementing the HTTPay MVP, broken down into specific subtasks with clear deliverables. Use this as a checklist to track progress and ensure all requirements are met.

## How to Use These Files

- Before generating or modifying code, review these files to understand the current requirements, architecture, and implementation plan.
- Use blueprint.md to guide the overall development sequence and to break down work into logical steps.
- Use project.md to ensure all code, messages, and storage structures strictly follow the defined specifications and constraints.
- Use tasks.md to verify that all implementation steps are completed and to avoid missing any deliverables.
- When in doubt about requirements or design, defer to the content of these files.

## Instructions for Copilot

When generating code:

1. **Context Awareness**: Consider the surrounding code and existing patterns before generating new code.

2. **Consistency**: Match the style, naming conventions, and architectural patterns of the existing codebase.

3. **Completeness**: Generate fully implemented solutions rather than stubs when possible, including error handling and edge cases.

4. **Commentary**: Include brief explanations for complex logic or non-obvious design decisions.

5. **Options**: When multiple valid approaches exist, explain the tradeoffs and why you selected a particular solution.

6. **Testing**: Consider how the generated code will be tested and include test suggestions or implementations where appropriate.

7. **Modularity**: Generate code that is modular, reusable, and follows separation of concerns.

8. **Progressive Enhancement**: Start with a simple, working solution that can be iteratively improved.

9. **Accessibility**: Be mindful of accessibility concerns when generating UI components or interactions.

10. **Future-Proofing**: Generate code that will be maintainable and adaptable to future requirements.

Remember that the ultimate goal is to produce code that is not just functional, but sustainable, readable, and aligned with our team's values and practices.

## Most important

- ALWAYS consult the files `tasks.md`, `blueprint.md`, `project.md` and `notes/index.md` to understand the context and what to do next. These files contain the most up-to-date information about the project and should be used as primary references for architecture, requirements, and implementation details.
- ALWAYS update the task list in `tasks.md` and the notes in `notes/index.md` to reflect your progress after making any change or completing a task.
- **ALWAYS look inside the `cosmwasm-docs` folder to get context about CosmWasm development.**