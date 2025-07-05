Please also reference the following documents as needed:

| Document | Description | File Patterns |
|----------|-------------|---------------|
| @.claude/memories/backend.md | Backend development rules and API guidelines | src/api/**/*.ts, src/services/**/*.ts, src/models/**/*.ts |
| @.claude/memories/frontend.md | Frontend development rules and best practices | src/components/**/*.tsx, src/pages/**/*.tsx, **/*.css, **/*.scss |


# Project Overview

## General Guidelines

- Use TypeScript for all new code
- Follow consistent naming conventions
- Write self-documenting code with clear variable and function names
- Prefer composition over inheritance
- Use meaningful comments for complex business logic

## Code Style

- Use 2 spaces for indentation
- Use semicolons
- Use double quotes for strings
- Use trailing commas in multi-line objects and arrays

## Architecture Principles

- Organize code by feature, not by file type
- Keep related files close together
- Use dependency injection for better testability
- Implement proper error handling
- Follow single responsibility principle

