# OCR_Bill2Sheet Constitution

## Core Principles

### I. UI-Driven Development (Shadcn-First)
The frontend **must strictly use Shadcn UI components**.  
No custom UI components are allowed unless explicitly extended via Shadcn configuration.  
Design must follow the official Shadcn UI templates to maintain consistency and visual coherence.

### II. Component Validation with `mcp`
Before implementing any frontend feature, all UI components must be validated using `mcp` (Manual Component Preview) to ensure:
- The UI structure is achievable using Shadcn components.
- Visual and behavioral expectations align with Shadcn defaults.
- Component customization remains minimal and compliant.

This step is **mandatory** before writing any JSX/TSX code.

### III. TDD is Explicitly Prohibited
Test-Driven Development (TDD) is strictly excluded from the development workflow.
- No test scaffolding, test planning, or test writing is permitted before or during feature development.
- All development efforts must prioritize speed, prototype delivery, and UI-first iteration.
- Any introduction of automated testing will be evaluated only post-MVP and must be proposed via a formal amendment to this constitution.

No developer may require or enforce TDD or testing workflows at any stage of pre-MVP development.

### IV. Backend Simplicity with Axum + SQLx
The backend is built using:
- **Axum** as the HTTP framework
- **SQLx** for async database access
- **PostgreSQL** as the primary data store

All data access must use SQLx’s query macros (`query!`, `query_as!`, etc.) with compile-time SQL validation.  
Connection pooling is required. Migrations are managed externally or via `sqlx migrate`.

### V. Strict Technology Stack Enforcement
- **Frontend**: Next.js 14+, TypeScript, Shadcn UI, `lucide-react` icons.
- **Backend**: Rust, Axum, SQLx, PostgreSQL.
- **Interfacing**: JSON over HTTP.
- **Design**: Mobile-first layout. No external CSS frameworks allowed beyond Tailwind (used by Shadcn).

## Development Constraints

- **Frontend components must be verified via `mcp` before implementation.**
- All styling and layout must come from Shadcn UI — no ad-hoc CSS or utility overrides allowed.
- Only `lucide-react` is permitted for icons.
- No TDD or test scaffolding required until stabilization phase.
- Backend must use connection pooling, environment-configured database URLs, and modular route separation.

## Workflow & Review Process

- **Frontend PRs must include screenshots or videos of component previews verified via `mcp`.**
- **Backend PRs must include explanation of route purpose and any SQL used (inline or via `.sql` files).**
- Reviews focus on adherence to:
  - Shadcn UI consistency
  - Clean separation of concerns
  - Stack compliance (no unofficial libraries)
- No test coverage requirement enforced at this stage.
- MVP iteration speed is prioritized over polish.

## Governance

- This constitution supersedes informal practices or opinions.
- Any changes to stack, tools, or principles must be proposed in writing with:
  - Technical justification
  - Migration or rollback plan (if needed)
  - Approval from project lead
- Runtime practices and component usage must follow guidance from [`shadcn/ui`](https://ui.shadcn.dev) and [`lucide.dev`](https://lucide.dev).

**Version**: 1.0.0 | **Ratified**: 2025-09-19 | **Last Amended**: 2025-09-19
