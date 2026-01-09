# @lean-wms/core

Shared TypeScript business logic for Lean WMS.

## Architecture

This package contains functional programming-style business logic that can be easily migrated to Rust in the future.

- **Pure functions:** Clear input/output, no side effects
- **Type-safe:** Full TypeScript strict mode
- **Functional style:** Designed for future Rust migration

## Modules

- `inventory/` - FEFO/FIFO algorithms
- `validation/` - Poka-Yoke rules, Zod schemas
- `location/` - Location hierarchy logic
- `types/` - Shared TypeScript types

