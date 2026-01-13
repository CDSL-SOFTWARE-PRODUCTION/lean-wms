# Test Strategy

## 🧪 Testing Levels

### 1. Unit Tests

- **Scope**: Individual functions (FEFO logic, Validation).
- **Tool**: `cargo test` (Rust), `jest` (TS).
- **Coverage Goal**: >80% for Core logic.

### 2. Integration Tests

- **Scope**: API endpoints + DB interactions.
- **Tool**: `cargo test` with test DB.

### 3. E2E Tests

- **Scope**: Full user flows (Login -> Create Order).
- **Tool**: Playwright / Cypress.

## 🤖 CI Pipeline

- Run linting (`cargo clippy`, `eslint`).
- Run unit tests.
- Build checking.
