# 4. Offline Sync Strategy & Action Queue

Date: 2026-01-22

## Status

Accepted

## Context

Warehouses often have dead zones (Wi-Fi blind spots). The Mobile App must function seamlessly without internet access for core operations: Inbound, Outbound, and Inventory Counting.
We need a robust mechanism to store actions locally and sync them when connectivity is restored, ensuring data integrity.

## Decision

We implement an **Optimistic UI with Background Sync** using **@legendapp/state** (v3).

### 1. Local-First State (The Observable)

State is managed using Legend State's observables, which serve as the single source of truth for the UI.

- **Persistence**: The global state observable is automatically persisted to local storage (SQLite or AsyncStorage) using `persistObservable`. This ensures that even if the app is killed, the last known state is restored immediately upon launch.
- **Optimistic UI**: All user actions (e.g., scanning an item) mutate the local observable immediately. The UI updates instantly without waiting for a server response.
- **Mutation Tracking**: Legend State tracks all local changes (mutations) that haven't been synced to the server yet.

### 2. Synchronization (Supabase Plugin)

We utilize the `@legendapp/state/sync-plugins/supabase` (or a custom sync configuration) to manage data synchronization.

**Workflow:**

1. **Change Detection**: When a user performs an action, the observable is updated. Legend State marks this change as "pending sync".
2. **Background Sync**:
    - If **Online**: The sync system pushes the pending changes to Supabase (via `upsert`/`insert`) and pulls the latest data.
    - If **Offline**: The changes remain in the local persistence layer. Ideally, the `sync` system retries automatically when connectivity is restored.
3. **Real-time Updates**: The app subscribes to Supabase Realtime (via the plugin) to receive updates from other devices/users, keeping the local observable fresh.

### 3. Conflict Resolution Strategy

Since we use Optimistic UI, the server state might diverge from the client state.

**Strategy: Server Authority with "Fail & Fix"**

| Scenario | Resolution |
| :--- | :--- |
| **Simple Data** (e.g., Scan logs) | **Last Write Wins (LWW)** (Server timestamp usually dominates, or merge strategies provided by the sync plugin). |
| **Inventory Quantity** (Race Condition) | **Server Constraints**. If Worker A picks item X, and Worker B tries to pick the same X later (offline):<br>1. Worker B's action fails on sync (DB constraint: negative stock not allowed).<br>2. Worker B receives error alert (via sync error callback).<br>3. App triggers "Force Sync" to pull fresh stock levels.<br>4. Worker B sees item is gone. |
| **Master Data** (Product Names) | **Read-only on Mobile**. Workers cannot edit master data, avoiding those conflicts. |

## Consequences

- **Complexity in UI**: We must handle "Syncing..." indicators and "Sync Failed" error states explicitly (reading from Legend State's sync status).
- **Dependency**: Heavy reliance on `@legendapp/state`'s internal sync logic.
- **Edge Cases**: Timestamp drift and "glitchy" updates if real-time messages arrive out of order (mitigated by using Supabase's authoritative state).
