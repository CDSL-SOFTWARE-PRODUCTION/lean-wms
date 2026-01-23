# 2. Backend Architecture: Backend-as-a-Service (Supabase)

Date: 2026-01-17

## Status

Accepted (Supersedes "Functional Rust Core")

## Context

To achieve maximum speed to market and find Product-Market Fit (PMF), the original plan of a Functional Rust Core was deemed too slow for a small team. We need an infrastructure that provides Authentication, Database, and Real-time syncing out-of-the-box.

## Decision

We adopt **Supabase (BaaS)** as the primary backend engine. This replaces the custom Rust/Axum server with a hosted PostgreSQL model.

### Core Principles

1. **BaaS First (Supabase):** Use Supabase Auth, PostgreSQL, and Realtime Engine. APIs are auto-generated from the schema.
2. **Optimistic UI + Local-First Sync:** Mobile app uses **@legendapp/state** to manage local state and automatically sync changes to Supabase when online.
3. **Real-time Visibility:** Web Dashboard uses Supabase Realtime to show live inventory updates without manual refresh.
4. **Database-Centric Logic:** Business rules (e.g., Poka-Yoke validation) are enforced via PostgreSQL Constraints and Row-Level Security (RLS) where possible. Complex logic uses Supabase Edge Functions.

### Conflict Resolution Strategy (MVP)

- **Last Write Wins (LWW):** Applied to most metadata and location updates.
- **Sequence-based Validation:** For inventory quantities, the server (Postgres) remains the source of truth. If a transaction fails due to stock depletion, the client must re-sync.

## Consequences

- **Development Speed:** 10x faster setup compared to building custom Rust APIs.
- **Reduced DevOps:** Zero infrastructure to manage (managed Supabase).
- **Lock-in:** High dependency on Supabase features.
- **Schema Strictness:** Moving from application-level (Rust) types to DB-level (Postgres) constraints.
