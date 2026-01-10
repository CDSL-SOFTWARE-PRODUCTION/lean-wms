# ARCHITECTURE - Kiến trúc hệ thống

**Mục tiêu:** Giải thích cấu trúc Monorepo, kiến trúc tổng thể, và cách các components tương tác với nhau.

---

## 📐 System Architecture Overview

```diagram
┌─────────────────────────────────────────────────────────────┐
│                    LEAN WMS MONOREPO                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐      ┌──────────────┐      ┌────────────┐ │
│  │  client-web  │      │    mobile    │      │    api     │ │
│  │  (React)     │      │   (Expo)     │      │   (Rust)   │ │
│  └──────┬───────┘      └──────┬───────┘      └─────┬──────┘ │
│         │                     │                      │        │
│         └─────────────────────┼──────────────────────┘        │
│                               │                               │
│                    ┌──────────▼──────────┐                    │
│                    │   packages/core     │                    │
│                    │  (TypeScript)      │                    │
│                    │  - FEFO/FIFO      │                    │
│                    │  - Validation     │                    │
│                    │  - Types          │                    │
│                    └───────────────────┘                    │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 🏗️ Monorepo Structure

### Tại sao Monorepo?

**Ưu điểm:**

1. ✅ **Shared Code:** `packages/core` được dùng chung bởi web và mobile
2. ✅ **Type Safety:** Có thể share types giữa frontend và backend (qua rspc/OpenAPI)
3. ✅ **Atomic Changes:** Sửa API + frontend trong cùng 1 PR
4. ✅ **Single Source of Truth:** Một repo, một version, dễ đồng bộ
5. ✅ **Unified CI/CD:** Test toàn bộ cùng lúc

**Nhược điểm:**

1. ⚠️ **Onboarding:** Dev mới cần hiểu nhiều công nghệ (Rust, TypeScript, React)
2. ⚠️ **Build Time:** Có thể chậm hơn nếu không dùng caching (Turbo giải quyết vấn đề này)

**Kết luận:** Với dự án có shared logic và cần atomic changes, Monorepo là lựa chọn hợp lý.

### Cấu trúc thư mục

```folder-tree
lean-wms/
├── apps/
│   ├── api/                    # Backend Rust
│   │   ├── src/
│   │   │   ├── api/           # API routes (REST endpoints)
│   │   │   ├── services/      # Business logic
│   │   │   ├── database/      # Database models (SeaORM)
│   │   │   └── main.rs        # Entry point
│   │   └── Cargo.toml
│   │
│   ├── client-web/            # Frontend Web + Desktop
│   │   ├── src/
│   │   │   ├── components/    # React components
│   │   │   ├── screens/       # Page components
│   │   │   ├── store/         # Redux slices
│   │   │   ├── services/      # API client
│   │   │   └── App.tsx        # Entry point
│   │   └── package.json
│   │
│   └── mobile/                # Mobile App (Expo)
│       ├── src/
│       │   ├── screens/       # Screen components
│       │   ├── store/         # Redux slices
│       │   ├── services/      # API client
│       │   └── App.tsx        # Entry point
│       └── package.json
│
├── packages/
│   └── core/                  # Shared TypeScript logic
│       ├── src/
│       │   ├── inventory/     # FEFO/FIFO algorithms
│       │   ├── validation/    # Validation rules
│       │   ├── types/         # Shared types
│       │   └── index.ts       # Public API
│       └── package.json
│
├── doc/                       # Documentation
├── package.json               # Root workspace (pnpm)
├── pnpm-workspace.yaml        # pnpm workspace config
├── turbo.json                 # Turbo build pipeline
└── Cargo.toml                 # Rust workspace config
```

---

## 🔄 Data Flow

### 1. Frontend → Backend Flow

```diagram
┌─────────────┐         ┌─────────────┐         ┌─────────────┐
│   Web/Mobile│         │  API Client  │         │  Backend API│
│   Component │────────▶│  (axios)     │────────▶│  (Axum)     │
└─────────────┘         └─────────────┘         └─────────────┘
       │                       │                       │
       │                       │                       ▼
       │                       │                ┌─────────────┐
       │                       │                │  PostgreSQL  │
       │                       │                └─────────────┘
       │                       │
       ▼                       ▼
┌─────────────┐         ┌─────────────┐
│ Redux Store │         │   Response  │
│  (State)    │◀────────│   (JSON)    │
└─────────────┘         └─────────────┘
```

### 2. Shared Logic Flow

```diagram
┌─────────────┐
│  client-web │
│   (React)   │─────────┐
└─────────────┘         │
                        │
┌─────────────┐         │      ┌─────────────┐
│   mobile    │─────────┼─────▶│ packages/   │
│   (Expo)    │         │      │   core      │
└─────────────┘         │      │ (TypeScript)│
                        │      └─────────────┘
                        │             │
                        │             │ FEFO/FIFO
                        │             │ Validation
                        │             │ Types
                        │             │
                        │      ┌──────▼──────┐
                        │      │  Business   │
                        │      │   Logic     │
                        │      └─────────────┘
                        │
                        │
┌─────────────┐         │
│     api     │─────────┘
│   (Rust)    │
└─────────────┘
   (Không dùng
    packages/core
    trực tiếp)
```

**Lưu ý:** Backend Rust không dùng `packages/core` trực tiếp vì khác ngôn ngữ. Thay vào đó:

- Backend implement logic tương tự trong Rust
- Có thể share types qua OpenAPI schema hoặc rspc code generation

---

## 🧩 Component Details

### 1. `apps/api` - Backend Rust

**Tech Stack:**

- **Framework:** Axum (async web framework)
- **Protocol:** REST + RSPC (Type-safe procedures)
- **ORM:** SeaORM
- **Database:** PostgreSQL
- **Auth:** JWT với refresh token

**Responsibilities:**

- RESTful API endpoints
- RSPC Procedures for Frontend
- Business logic validation
- Database operations
- Authentication & Authorization
- WebSocket sync (future)

**Key Modules:**

```
src/
├── api/              # API routes
│   ├── auth.rs       # Authentication endpoints
│   ├── products.rs   # Product CRUD
│   ├── orders.rs     # Order management
│   └── sync.rs       # Sync endpoints
├── services/         # Business logic
│   └── auth.rs       # Auth service
├── database/         # Database layer
│   ├── entities.rs   # SeaORM models
│   └── connection.rs # DB connection
└── main.rs           # Entry point
```

### 2. `apps/client-web` - Frontend Web

**Tech Stack:**

- **Framework:** React 19 + TypeScript
- **Build Tool:** Vite
- **State:** Redux Toolkit
- **Routing:** TanStack Router
- **Styling:** Tailwind CSS
- **Desktop:** Tauri (future)

**Responsibilities:**

- Web UI cho quản lý
- Desktop app (Tauri wrapper)
- State management
- API communication

**Key Modules:**

```
src/
├── components/       # Reusable components
├── screens/          # Page components
├── store/            # Redux slices
│   ├── slices/
│   │   ├── authSlice.ts
│   │   ├── productsSlice.ts
│   │   └── ordersSlice.ts
│   └── index.ts
├── services/
│   └── api.ts        # API client (axios)
└── App.tsx           # Entry point
```

### 3. `apps/mobile` - Mobile App

**Tech Stack:**

- **Framework:** Expo (React Native)
- **Language:** TypeScript
- **State:** Redux Toolkit
- **Navigation:** React Navigation
- **Camera:** react-native-vision-camera

**Responsibilities:**

- Mobile UI cho công nhân
- Barcode scanning
- Offline-first operations
- Sync với backend

**Key Modules:**

```
src/
├── screens/          # Screen components
├── store/            # Redux slices
├── services/
│   └── api.ts        # API client
└── App.tsx           # Entry point
```

### 4. `packages/core` - Shared Logic

**Tech Stack:**

- **Language:** TypeScript
- **Validation:** Zod
- **Style:** Functional Programming

**Responsibilities:**

- Business logic (FEFO/FIFO)
- Validation rules
- Shared types
- Utility functions

**Key Modules:**

```
src/
├── inventory/
│   └── index.ts      # FEFO/FIFO algorithms
├── validation/
│   └── index.ts      # Validation schemas
├── types/
│   └── index.ts      # Shared TypeScript types
└── index.ts          # Public API
```

**Usage Example:**

```typescript
// Trong client-web hoặc mobile
import { calculateFEFO, validateLocation } from '@lean-wms/core';

const items = calculateFEFO(inventoryItems);
const isValid = validateLocation(locationCode);
```

---

## 🔧 Build & Development Tools

### Turbo (Build System)

**File:** `turbo.json`

**Purpose:** Parallel builds, caching, dependency management

**Pipeline:**

```json
{
  "pipeline": {
    "dev": { "cache": false, "persistent": true },
    "build": { "dependsOn": ["^build"], "outputs": ["dist/**"] },
    "test": { "dependsOn": ["build"] }
  }
}
```

**Benefits:**

- ✅ Parallel execution
- ✅ Smart caching (chỉ build lại khi code thay đổi)
- ✅ Dependency graph (tự động build dependencies trước)

### pnpm Workspace

**File:** `pnpm-workspace.yaml`

**Purpose:** Manage multiple packages trong một repo

**Config:**

```yaml
packages:
  - 'apps/*'
  - 'packages/*'
```

**Benefits:**

- ✅ Hoisting (shared dependencies ở root)
- ✅ Workspace protocol (`workspace:*` trong package.json)
- ✅ Faster installs

### Cargo Workspace

**File:** `Cargo.toml` (root)

**Purpose:** Manage Rust crates

**Config:**

```toml
[workspace]
members = ["apps/api"]
resolver = "2"
```

---

## 🔐 Authentication Flow

```
┌─────────┐         ┌─────────┐         ┌─────────┐
│ Client  │         │  API    │         │   DB    │
└────┬────┘         └────┬────┘         └────┬────┘
     │                   │                    │
     │ POST /auth/login  │                    │
     │──────────────────▶│                    │
     │                   │  Verify credentials│
     │                   │───────────────────▶│
     │                   │◀───────────────────│
     │                   │                    │
     │  JWT + Refresh    │                    │
     │◀──────────────────│                    │
     │                   │                    │
     │ Store tokens      │                    │
     │ (localStorage/    │                    │
     │  SecureStore)     │                    │
     │                   │                    │
     │ API Request       │                    │
     │ + JWT Header      │                    │
     │──────────────────▶│                    │
     │                   │  Verify JWT        │
     │                   │───────────────────▶│
     │                   │◀───────────────────│
     │  Response         │                    │
     │◀──────────────────│                    │
```

---

## 📊 Database Schema (High-level)

```
┌─────────────┐
│   Users     │
│─────────────│
│ id          │
│ email       │
│ password    │
└──────┬──────┘
       │
       │ 1:N
       │
┌──────▼──────┐
│  Products   │
│─────────────│
│ id          │
│ sku_code    │
│ name        │
└──────┬──────┘
       │
       │ 1:N
       │
┌──────▼──────┐
│ Inventory   │
│─────────────│
│ id          │
│ product_id  │
│ location_id │
│ quantity    │
└─────────────┘
```

**Chi tiết:** Xem [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](BLUEPRINT_ENGINEER_BACKEND.md)

---

## 🚀 Deployment Architecture (Future)

```
┌─────────────┐
│   Mobile    │
│   (Expo)    │
└──────┬──────┘
       │ HTTPS
       │
┌──────▼──────────────────┐
│   Load Balancer        │
└──────┬──────────────────┘
       │
   ┌───┴───┐
   │       │
┌──▼──┐ ┌──▼──┐
│ API │ │ API │
│ (1) │ │ (2) │
└──┬──┘ └──┬──┘
   │       │
   └───┬───┘
       │
┌──────▼──────┐
│ PostgreSQL  │
│  (Primary)  │
└─────────────┘
```

---

## 📚 Related Documentation

- **Onboarding:** [`doc/CONTRIBUTING.md`](CONTRIBUTING.md)
- **Technical Specs:** [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](BLUEPRINT_ENGINEER_BACKEND.md)
- **UI/UX Specs:** [`doc/BLUEPRINT_DESIGNER_FRONTEND.md`](BLUEPRINT_DESIGNER_FRONTEND.md)
- **Overview:** [`doc/BLUEPRINT.md`](BLUEPRINT.md)

---

**Last Updated:** 09-01-2026
