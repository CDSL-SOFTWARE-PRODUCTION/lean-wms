# CONTRIBUTING - Hướng dẫn cho Developer mới

**Mục tiêu:** Giúp developer mới tham gia dự án có thể setup và bắt đầu code trong **5 phút**.

---

## 🚀 Quick Start (5 phút)

### 1. Prerequisites

Đảm bảo bạn đã cài đặt:

- **Node.js** 18+ và **pnpm** 10.6.4+
- **Rust** 1.70+ (cho backend API)
- **PostgreSQL** 14+ (hoặc dùng Docker)
- **Git**

### 2. Clone & Install

```bash
# Clone repository
git clone <repository-url>
cd lean-wms

# Install dependencies (tất cả apps)
pnpm install

# Build shared packages (dùng Turborepo)
pnpm build:core
```

### 3. Setup Database

```bash
# Chạy PostgreSQL qua Docker
docker-compose up -d

# Hoặc setup PostgreSQL thủ công (xem apps/api/env.example)
```

### 4. Chạy Development

```bash
# Chạy từng phần riêng:
pnpm dev:web      # Chỉ frontend web (port 5173)
pnpm dev:mobile   # Chỉ mobile app (Expo)
pnpm dev:api      # Chỉ backend API (port 3000)
```

### 5. Verify Setup

- ✅ Web: Mở <http://localhost:5173>
- ✅ API: Mở <http://localhost:3000/health> (nếu có endpoint)
- ✅ Mobile: Scan QR code từ Expo CLI

---

## 📁 Cấu trúc Monorepo

Dự án sử dụng **Monorepo** với Turbo + pnpm workspace để quản lý nhiều apps cùng lúc.

```folder-tree
lean-wms/
├── apps/
│   ├── api/              # Backend Rust (Axum)
│   ├── client-web/       # Frontend React (Web + Desktop Tauri)
│   └── mobile/           # Mobile Expo (React Native)
├── packages/
│   └── core/             # Shared TypeScript logic (FEFO/FIFO, validation)
├── doc/                   # Documentation
├── package.json          # Root workspace config
├── pnpm-workspace.yaml   # pnpm workspace definition
├── turbo.json            # Turbo build pipeline
└── Cargo.toml            # Rust workspace config
```

### Tại sao Monorepo?

1. **Shared Code:** `packages/core` được dùng chung bởi web và mobile
2. **Type Safety:** Có thể share types giữa frontend và backend
3. **Atomic Changes:** Sửa API + frontend trong cùng 1 PR
4. **Single Source of Truth:** Một repo, một version, dễ đồng bộ

---

## 🛠️ Development Workflow

### Làm việc với Frontend Web

Ở root directory, chạy lệnh:

```bash
pnpm dev:web
```

**Tech Stack:**

- React 19 + TypeScript
- Vite (build tool)
- Redux Toolkit (state management)
- TanStack Router (routing)
- Tailwind CSS (styling)

**Key Files:**

- `src/App.tsx` - Entry point
- `src/store/` - Redux slices
- `src/services/api.ts` - API client
- `src/components/` - Reusable components

### Làm việc với Mobile App

Ở root directory, chạy lệnh:

```bash
pnpm dev:mobile
```

**Tech Stack:**

- Expo (React Native)
- TypeScript
- Redux Toolkit
- React Navigation

**Key Files:**

- `src/screens/` - Screen components
- `src/store/` - Redux slices
- `src/services/api.ts` - API client

### Làm việc với Backend API

Ở root directory, chạy lệnh:

```bash
pnpm dev:api
```

**Tech Stack:**

- Rust
- Axum (web framework)
- RSPC (Type-safe procedures)
- SeaORM (ORM)
- PostgreSQL

**Key Files:**

- `src/main.rs` - Entry point
- `src/api/` - API routes
- `src/services/` - Business logic
- `src/database/` - Database models

### Làm việc với Shared Core

Ở root directory, chạy lệnh:

```bash
pnpm build:core    # Build core package (dùng Turborepo)
# Hoặc: pnpm --filter @lean-wms/core build
```

**Tech Stack:**

- TypeScript
- Zod (validation)
- Functional Programming style

**Key Files:**

- `src/inventory/` - Inventory logic (FEFO/FIFO)
- `src/validation/` - Validation rules
- `src/types/` - Shared types

---

## 🔧 Common Tasks

### Thêm dependency mới

```bash
# Thêm vào frontend web
cd apps/client-web
pnpm add <package-name>

# Thêm vào mobile
cd apps/mobile
pnpm add <package-name>

# Thêm vào shared core
cd packages/core
pnpm add <package-name>

# Thêm vào backend (Rust)
cd apps/api
cargo add <crate-name>
```

### Chạy Tests

```bash
# Test tất cả (dùng Turborepo)
pnpm test

# Test từng app (dùng Turborepo)
pnpm test:web      # Test client-web
pnpm test:mobile   # Test mobile
pnpm test:api      # Test Rust API

# Hoặc dùng filter (tương đương)
pnpm --filter client-web test
pnpm --filter mobile test
pnpm --filter api test
```

### Build & Deploy (Manual CLI)

Bạn có thể deploy thủ công bằng CLI nếu cần test deployment:

```bash
# 1. Cài đặt CLIs
pnpm add -g vercel @railway/cli eas-cli

# 2. Deploy Frontend (Vercel)
# Lưu ý: Vercel sẽ tự động detect cấu hình từ vercel.json hoặc UI setting.
# Nếu deploy manual từ CLI:
cd apps/client-web
vercel login    # Đăng nhập lần đầu
vercel pull     # Link project
vercel build    # Build locally
vercel deploy --prebuilt --prod # Deploy artifacts đã build

# 3. Deploy Backend (Railway)
# Railway thường dùng GitHub Trigger, nhưng có thể dùng CLI:
railway login
railway up --service api-dev  # Push code lên service dev

# 4. Deploy Mobile (Expo)
# Mobile build local thường dùng:
cd apps/mobile
npx expo start --no-dev --minify # Chạy production mode local

# Để publish OTA update (cập nhật code JS không qua Store):
eas login
eas update --branch preview --message "Quick fix"
```

## ⚡ TurboRepo Guide

Dự án này sử dụng TurboRepo để tăng tốc độ build/test.

### 1. Caching

Turbo sẽ cache kết quả của các tasks (`build`, `lint`, `test`). Nếu bạn chạy lại lệnh mà không thay đổi code, nó sẽ trả về kết quả ngay lập tức (hit cache).

- **Outputs:** Xem `.turbo` folder.
- **Force clean:** `pnpm clean` nếu gặp lỗi cache lạ.

### 2. Filtering (Chạy có chọn lọc)

Thay vì chạy hết tất cả apps, bạn có thể chạy riêng lẻ:

```bash
# Chạy dev cho Web
pnpm dev:web

# Chạy build cho Core package
pnpm build:core

# Chạy type-check cho Mobile
pnpm turbo type-check --filter=mobile

# Chạy tất cả TRỪ mobile
pnpm turbo build --filter=!mobile
```

### 3. Dependency Graph

Apps phụ thuộc vào packages. Turbo hiểu rõ thứ tự này:

- `apps/client-web` -> `packages/core`
- Khi chạy `pnpm build`, Turbo sẽ build `core` trước, sau đó mới build `client-web`.

### 4. Configuration Isolation

**Lưu ý quan trọng cho Mobile Developer:**

- `apps/mobile/tsconfig.json` được cấu hình **độc lập** (không extends root).
- Điều này để tránh conflict với DOM libs của Web.
- Khi sửa config cho mobile, hãy sửa trực tiếp trong folder `apps/mobile`.

---

### Code Quality Commands

```bash
# Lint tất cả
pnpm lint
pnpm lint:fix      # Auto-fix linting issues

# Format code (tất cả workspaces)
pnpm format        # Format và save
pnpm format:check  # Check formatting (dùng trong CI)

# Type checking (TypeScript)
pnpm type-check    # Type check tất cả TypeScript packages

# Rust-specific
pnpm rust:fmt      # Check Rust formatting
pnpm rust:fmt:fix  # Fix Rust formatting
pnpm rust:clippy   # Run Rust linter (Clippy)
pnpm rust:test     # Test Rust code

# Clean build artifacts
pnpm clean         # Clean tất cả (node_modules, .turbo, target, dist)
```

---

## 📝 Code Style & Standards

### TypeScript/JavaScript

- **Linter:** ESLint (config trong từng app)
- **Formatter:** Prettier
- **Type Safety:** Strict TypeScript, không dùng `any`

### Rust

- **Formatter:** `cargo fmt`
- **Linter:** `cargo clippy`
- **Style:** Follow Rust official style guide

### Git Commit Messages

Sử dụng conventional commits:

```bash
feat: add product search feature
fix: resolve inventory sync issue
docs: update API documentation
refactor: simplify authentication logic
```

---

## 🐛 Debugging

### Frontend Web

```bash
# Chạy qua Turborepo từ root (khuyên dùng)
pnpm dev:web
# Hoặc: pnpm --filter client-web dev

# Hoặc chạy trực tiếp
cd apps/client-web
pnpm dev
# Mở DevTools trong browser (F12)
```

### Mobile

```bash
# Chạy qua Turborepo từ root (khuyên dùng)
pnpm dev:mobile
# Hoặc: pnpm --filter mobile dev

# Hoặc chạy trực tiếp
cd apps/mobile
pnpm start
# Sử dụng React Native Debugger hoặc Expo DevTools
```

### Backend

```bash
# Chạy qua Turborepo (khuyên dùng)
RUST_LOG=debug pnpm dev:api

# Hoặc chạy trực tiếp với Cargo
cd apps/api
RUST_LOG=debug cargo run
# Logs sẽ hiển thị trong console
```

---

## 📚 Tài liệu tham khảo

- **Architecture:** Xem [`doc/ARCHITECTURE.md`](ARCHITECTURE.md)
- **Technical Specs:** Xem [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](BLUEPRINT_ENGINEER_BACKEND.md)
- **UI/UX Specs:** Xem [`doc/BLUEPRINT_DESIGNER_FRONTEND.md`](BLUEPRINT_DESIGNER_FRONTEND.md)
- **Product Requirements:** Xem [`PRD.md`](../PRD.md)

---

## ❓ FAQ

### Q: Tại sao dùng Monorepo thay vì tách riêng?

**A:** Vì có shared code (`packages/core`) và cần atomic changes (API + Frontend cùng lúc). Xem chi tiết trong [`doc/ARCHITECTURE.md`](ARCHITECTURE.md).

### Q: Làm sao để chỉ chạy một app?

**A:** Sử dụng Turborepo filter (có 2 cách tương đương):

```bash
# Cách 1: Dùng shortcut commands (khuyên dùng)
pnpm dev:web       # Chỉ frontend web
pnpm dev:mobile    # Chỉ mobile app
pnpm dev:api       # Chỉ backend API

# Cách 2: Dùng filter trực tiếp (tương đương)
pnpm --filter client-web dev
pnpm --filter mobile dev
pnpm --filter api dev
```

### Q: Làm sao để thêm shared code mới?

**A:** Thêm vào `packages/core/src/`, sau đó import trong các app:

```typescript
import { someFunction } from '@lean-wms/core';
```

### Q: Backend Rust có thể dùng shared TypeScript không?

**A:** Không trực tiếp. Backend dùng Rust, nhưng có thể share types qua code generation (rspc) hoặc OpenAPI schema.

---

## 🤝 Getting Help

- **Technical Questions:** Đọc [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](BLUEPRINT_ENGINEER_BACKEND.md)
- **Architecture Questions:** Đọc [`doc/ARCHITECTURE.md`](ARCHITECTURE.md)
- **Product Questions:** Đọc [`PRD.md`](../PRD.md)

---

**Last Updated:** 09-01-2026
