# 5. Security & Row-Level Security (RLS) Policies

Date: 2026-01-22

## Status

Accepted

## Context

Lean WMS uses Supabase (PostgreSQL) directly from the client. Without a traditional middleware API validating every request, **Database Security is Application Security**. We MUST rely on PostgreSQL's Row-Level Security (RLS) to prevent unauthorized access and data leaks.

## Decision

We grant access based on JWT Claims (`authenticated`, `role`) provided by Supabase Auth.

### 1. User Roles

Defined in `public.users` table and mirrored in JWT custom claims (if optimization allows) or queried directly.

| Role | Description |
| :--- | :--- |
| **ADMIN** | Full access to all data and system configurations. |
| **MANAGER** | Can view all reports, approve adjustments, manage Master Data (Products/Locations). |
| **WORKER** | Can only View Inventory, Create Transactions (Scan), and View their own history. **Cannot** delete or modify Master Data. |

### 2. RLS Policies by Table

**Default Rule:** `ENABLE RLS` on ALL tables. Default `DENY ALL`.

#### A. `public.users`

- **SELECT**: `auth.uid() = id` (User sees self) OR `auth.jwt() ->> 'role' IN ('admin', 'manager')`.
- **UPDATE**: `auth.uid() = id` (User updates self) OR Admin.

#### B. `public.products` & `public.locations` (Master Data)

- **SELECT**: `auth.role() = 'authenticated'` (Anyone logged in can see products).
- **INSERT/UPDATE/DELETE**: `auth.jwt() ->> 'role' IN ('admin', 'manager')` (Only managers change catalog).

#### C. `public.inventory`

- **SELECT**: `auth.role() = 'authenticated'` (Workers need to see stock).
- **INSERT/UPDATE**: **RESTRICTED**.
  - *Note:* Direct manipulation of inventory quantities is DANGEROUS.
  - **Preferred Approach:** Use **Postgres Functions (RPC)** for movements (e.g., `rpc/move_inventory`).
  - **Fallback RLS:** If direct update allowed, only if `auth.jwt() ->> 'role' IN ('admin', 'manager')`. Workers should use `inventory_transactions`.

#### D. `public.inventory_transactions` (The Action Log)

- **INSERT**: `auth.uid() = performed_by`. (Workers can only log actions they perform).
- **SELECT**: `auth.uid() = performed_by` OR `auth.jwt() ->> 'role' IN ('admin', 'manager')`.
- **UPDATE/DELETE**: **NEVER**. Audit logs are immutable.

### 3. Application-Level Security

- **PIN Code (App Side)**: For critical actions (e.g., removing a large batch), the App asks for a Manager PIN. This is a UI-level deterrent, confirmed by backend logs.
- **Edge Functions**: Complex logic (e.g., "Approve Stock Adjustment") runs in Edge Functions with `service_role` key, bypassing RLS but implementing strict business logic validation.

## Consequences

- **Strictness**: Developers must ensure the `role` is correctly propagated.
- **RPC Usage**: We shift some logic to Database Functions to safely bypass complex RLS rules for inventory movements.
- **Testing**: We must write tests that simulate different users to verify RLS triggers correctly (e.g., "Worker tries to delete product -> Fail").
