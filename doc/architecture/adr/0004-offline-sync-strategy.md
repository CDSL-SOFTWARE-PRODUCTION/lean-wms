# 4. Offline Sync Strategy & Action Queue

Date: 2026-01-22

## Status

Accepted

## Context

Warehouses often have dead zones (Wi-Fi blind spots). The Mobile App must function seamlessly without internet access for core operations: Inbound, Outbound, and Inventory Counting.
We need a robust mechanism to store actions locally and sync them when connectivity is restored, ensuring data integrity.

## Decision

We implement an **Optimistic UI with Background Sync** using a local `Action Queue` pattern.

### 1. The Action Queue (Local Store)

All state-modifying user actions are **never** sent directly to the API. Instead, they are wrapped in an `Action` object and persisted to the local SQLite database.

**Data Structure (TypeScript Interface):**

```typescript
type ActionType = 
  | 'INBOUND_RECEIVE'
  | 'INVENTORY_MOVE'
  | 'OUTBOUND_PICK'
  | 'STOCK_COUNT';

interface OfflineAction {
  id: string;             // UUID v4
  timestamp: number;      // Unix timestamp (Client time)
  type: ActionType;
  payload: Record<string, any>; // The data needed for the action
  status: 'PENDING' | 'SYNCING' | 'COMPLETED' | 'FAILED';
  retry_count: number;
  error_log?: string;     // For debugging failed syncs
}
```

**Workflow:**

1. **User acts** (e.g., Scans item).
2. **Local validation** (Check local cache of products/locations).
3. **Persist**: Save `OfflineAction` to SQLite with `status: 'PENDING'`.
4. **Optimistic Update**: Immediately update global Redux state to reflect the change (UI updates instantly).
5. **Trigger Sync**: If online, attempt to process the queue immediately.

### 2. Sync Mechanism (The Replay Loop)

A background worker (or `NetInfo` listener) monitors connectivity. When Online:

1. **Fetch Pending Actions**: Select `* FROM action_queue WHERE status = 'PENDING' ORDER BY timestamp ASC`.
2. **Sequential Processing**: Process items one by one (FIFO) to maintain causal consistency.
3. **API Call**: Send payload to Supabase (Edge Function or Table Insert).
4. **Ack**:
    - **Success (2xx)**: Mark `status: 'COMPLETED'`, optionally delete after 24h.
    - **Transient Error (Network)**: Keep `status: 'PENDING'`, retry later.
    - **Permanent Error (4xx/5xx Business Logic)**: Mark `status: 'FAILED'`, rollback local state, alert user.

### 3. Conflict Resolution Strategy

Since we use Optimistic UI, the server state might diverge.

**Strategy: Server Authority with "Fail & Fix"**

| Scenario | Resolution |
| :--- | :--- |
| **Simple Data** (e.g., Scan logs) | **Last Write Wins (LWW)** (Server timestamp dominates). |
| **Inventory Quantity** (Race Condition) | **Server Constraints**. If Worker A picks item X, and Worker B tries to pick the same X later (offline):<br>1. Worker B's action fails on sync (DB constraint: negative stock not allowed).<br>2. Worker B receives error alert.<br>3. App triggers "Force Sync" to pull fresh stock levels.<br>4. Worker B sees item is gone. |
| **Master Data** (Product Names) | **Read-only on Mobile**. Workers cannot edit master data, avoiding those conflicts. |

## Consequences

- **Complexity in UI**: We must handle "Syncing..." indicators and "Sync Failed" error states gracefully.
- **Rollback Logic**: If an optimistic action fails 10 minutes later, we must have a way to revert the UI state (e.g., reload fresh data from Server).
- **Edge Cases**: Timestamp drift between devices (handled by relying on Server Time for ordering where possible, but preserving Client Time for audit).
