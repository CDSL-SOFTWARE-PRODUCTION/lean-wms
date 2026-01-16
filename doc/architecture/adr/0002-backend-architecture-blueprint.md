# 2. Backend Architecture Blueprint

Date: 2026-01-15

## Status

Accepted (Migrated from BLUEPRINT_BACKEND.md)

## Context

We need a centralized, stable reference for backend business logic, data flow, and technical specifications to ensure consistency across the mobile app, desktop app, and server. This document formalizes the architectural decisions captured during the initial design phase.

## Decision

We adopt a **Functional Rust Core** with **Offline-First Data Flow** and **Event-Driven Synchronization**.

### Core Principles

1. **Functional Rust Core**: Share business logic (FEFO/FIFO, Validation, Calculations) across all platforms (Mobile via FFI/JSI, Desktop via Tauri, Server via Axum).
2. **Offline-First**: All physical actions (scans, moves) are recorded locally first with 0ms latency.
3. **Optimistic UI**: The UI responds immediately to user actions, assuming sync success.
4. **Conflict Resolution**:
   - **Last Write Wins (LWW)** for location and metadata.
   - **CRDT (Conflict-free Replicated Data Types)** for inventory quantities.
   - **First Come First Served (FCFS)** for final stock depletion in outbound flows.

### Business Logic Highlights

- **Hybrid SKU Generation**: Support both manual input and auto-generated SKU codes.
- **Flexible Location Granularity**: Support both high-precision bin tracking and general area tracking.
- **Record Splitting**: Inventory items are split during outbound to maintain perfect audit trails.
- **Blind Counting**: Workers count items without seeing current stock to prevent bias.

## Consequences

- **High Precision**: Rust core ensures identical logic execution on edge and server.
- **Resilience**: The system remains fully operational during internet outages.
- **Complexity**: Conflict resolution requires careful implementation of timestamps and delta tracking.
- **Maintenance**: Changes to core logic require recompiling native modules for mobile/desktop.
