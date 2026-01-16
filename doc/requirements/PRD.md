# LEAN WMS - Product Requirements Document (PRD)

**Version:** 2.0 (Hyper-Lean)
**Tech Stack:** Supabase (BaaS) + Expo (Mobile) + React (Web Dashboard)
**Status:** Ready for Development

---

## 1. PRODUCT VISION

**Lean WMS** brings standard factory processes (Samsung/Amazon) to small workshops via a "Brainless" Mobile App and a Real-time Web Dashboard.

- **Philosophy:** "No-Code/Low-Code" Speed + "High-Code" Logic.
- **Key Traits:** Offline-first, Validation (Poka-Yoke), Instant Setup.

---

## 2. USER STORIES & MODULES

### 2.1. Mobile App (Worker Focus - Android/iOS)

**Goal:** Zero-friction operations. Fast, big buttons, audio feedback.

#### A. INBOUND (Nhập kho)

*Requirement: Flexible Mapping & Quick Entry.*

1. **Scan Logic:** Scan Barcode/QR.
2. **Hybrid SKU Creation:**
    - *If New:* Auto-generate SKU Code OR allow manual mapping.
    - *If Existing:* Show details.
3. **Put-away:**
    - Suggest default location (Flexible Location).
    - Confirm quantity.
4. **Feedback:** Green screen (Success) / Haptic vibration.

#### B. OUTBOUND (Xuất kho)

*Requirement: Guided Workflow & Validation.*

1. **Pick List:** Show list of items to pick (sorted by location).
2. **Navigation:** "Go to Shelf A1 > Bin 02".
3. **Validation:**
    - Must scan correct Location first (Optional).
    - Must scan correct Product.
    - **FEFO Logic:** Warn if picking newer batch when older exists.
    - **Manager Override:** PIN code to bypass validation if urgent.

#### C. INVENTORY (Kiểm kê/Tồn kho)

*Requirement: Accuracy.*

1. **Blind Count:** Worker scans location -> Counts items -> Enters number (System hides actual stock).
2. **Move (Chuyển vị trí):** Scan Source -> Scan Item -> Scan Destination.

### 2.2. Web Dashboard (Manager Focus - React/Vite)

**Goal:** Visibility & Control.

1. **Inventory Overview:** Real-time stock levels, location map.
2. **Product Management:** Import CSV, Print Barcodes.
3. **Order Management:** Import Orders (CSV/Manual), assign to Workers.
4. **Approvals:** Review & Approve Inventory Adjustment (from Blind Counts).

---

## 2.3. "GOLDEN RULES" BUSINESS LOGIC

*Critical logic extracted from original blueprints to ensure system integrity.*

### 1. Record Splitting (Tách bản ghi)

*Scenario: Picking 4 items from a Bin containing 10 items.*

- **Action:** NEVER simply update quantity from 10 to 6.
- **Logic:**
    1. Find original record (Qty: 10).
    2. Update original -> Qty: 6.
    3. Create **NEW** record -> Qty: 4, Status: `OUTBOUND`/`SHIPPED`.
- **Why:** Maintains full traceability of where the 4 items went.

### 2. FEFO Algorithm (First Expired, First Out)

*Priority Sort Order:*

1. `expiry_date` ASC (Earliest expiry first).
2. `production_date`/`created_at` ASC (Oldest stock first).
3. `location_id` (Path optimization).

### 3. Blind Counting (Kiểm kê mù)

- App **HIDES** current system quantity.
- Worker **MUST** count and enter physical quantity.
- **Mismatch:** If (Count != System) -> Create `Adjustment_Transaction` (Do NOT overwrite). Manager must approve.

### 4. Barcode Mapping (Aliases)

- **Many-to-One:** One SKU (`SKU-001`) can be mapped to multiple barcodes:
  - Supplier A: `893...`
  - Supplier B: `GTIN...`
  - Internal QR: `QR...`
- **Scan:** Scanning *any* of these returns `SKU-001`.

---

## 3. TECHNICAL REQUIREMENTS (The Hyper-Lean Stack)

| Component | Technology | Reasoning |
| :--- | :--- | :--- |
| **Backend/DB** | **Supabase** | PostgreSQL, Auth, Realtime, Auto-generated APIs. Zero infra management. |
| **Mobile App** | **Expo (React Native)** | Cross-platform, OTA Updates, `expo-barcode-scanner`. |
| **Web Admin** | **React + Vite** | Fast development, standard dashboard components (Shadcn/MUI). |
| **Offline** | **SQLite + Action Queue** | Store actions locally, sync to Supabase when online. |
| **Auth** | **Supabase Auth** | Email/Password + Magic Link. |

---

## 4. NON-FUNCTIONAL REQUIREMENTS

1. **Performance:** Scan processing < 300ms.
2. **Offline:** App must allow Inbound/Outbound without internet. Sync automatically.
3. **Usability:** "Brainless" UI - Big text, Color coding (Green/Red), Sound/Haptic feedback.

## 5. DATA FLOW (Simplified)

1. **Sync:** Mobile pushes `Actions` -> Supabase Edge Function processes -> Updates DB.
2. **Realtime:** Web Dashboard subscribes to DB changes for live updates.
