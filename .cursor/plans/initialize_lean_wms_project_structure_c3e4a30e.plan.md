---
name: Initialize Lean WMS Project Structure
overview: Khởi tạo cấu trúc Monorepo Product-Focused cho Lean WMS - TypeScript core, Rust backend, unified client-web cho Web & Desktop
todos:
  - id: create-directory-structure
    content: Tạo thư mục apps/ và packages/ bằng mkdir
    status: pending
  - id: init-root-pnpm-workspace
    content: "CLI: pnpm init, setup pnpm-workspace.yaml (apps/* và packages/*)"
    status: pending
    dependencies:
      - create-directory-structure
  - id: init-core-package
    content: "CLI: Tạo packages/core với TypeScript, setup Zod validation, business logic (FEFO/FIFO, inventory) - Functional Programming style"
    status: pending
    dependencies:
      - init-root-pnpm-workspace
  - id: init-client-web
    content: "CLI: pnpm create vite apps/client-web với React TypeScript template, setup Tauri integration, shadcn/ui"
    status: pending
    dependencies:
      - init-root-pnpm-workspace
      - init-core-package
  - id: init-mobile-app
    content: "CLI: pnpm create expo-app apps/mobile với TypeScript template, API calls (không WatermelonDB), import từ @lean-wms/core"
    status: pending
    dependencies:
      - init-root-pnpm-workspace
      - init-core-package
  - id: init-api-backend
    content: "CLI: cargo new api trong apps/, setup Axum + SeaORM + PostgreSQL, rspc cho shared types"
    status: pending
    dependencies:
      - create-directory-structure
  - id: init-root-cargo-workspace
    content: "CLI: cargo init (root), edit Cargo.toml để add api workspace member"
    status: pending
    dependencies:
      - init-api-backend
  - id: configure-turbo-repo
    content: Tạo turbo.json config cho parallel dev, build tasks (client-web, mobile, core)
    status: pending
    dependencies:
      - init-root-pnpm-workspace
      - init-client-web
      - init-mobile-app
      - init-core-package
  - id: configure-workspace-files
    content: Edit Cargo.toml và pnpm-workspace.yaml, tạo .gitignore, .editorconfig
    status: pending
    dependencies:
      - init-root-cargo-workspace
      - configure-turbo-repo
  - id: setup-dev-environment
    content: Tạo docker-compose.yml cho PostgreSQL, tạo .env.example
    status: pending
    dependencies:
      - init-api-backend
  - id: setup-code-quality-config
    content: Config ESLint, Prettier cho tất cả apps (Husky hooks optional cho MVP)
    status: pending
    dependencies:
      - init-client-web
      - init-mobile-app
      - configure-workspace-files
  - id: setup-cicd
    content: Tạo .github/workflows/ci.yml với GitHub Actions (basic checks)
    status: pending
    dependencies:
      - setup-code-quality-config
---

# Plan: Initialize Lean WMS Project Structure (Product-Focused)

## Architecture Decision: Product-Focused Monorepo

**Nguyên tắc Product-Focused:**

- **Ra demo sớm:** Ưu tiên ra bản demo sớm nhất cho khách hàng (công ty dược phẩm)
- **Shared Logic bằng TypeScript:** `packages/core` bằng TS ngay từ đầu (Web, Mobile, Desktop đều đọc được)
- **Hợp nhất Web & Desktop:** Một dự án React duy nhất `client-web`, Tauri chỉ wrap nó
- **Rust đúng vị trí:** Backend API + Tauri commands (KHÔNG dùng Rust FFI cho mobile)
- **Functional Programming:** Viết logic theo FP (input/output rõ ràng) để dễ migrate sang Rust sau
- **Data Integrity First:** Database design chuẩn chỉnh ngay từ đầu (Migration, Constraints, Indexing)

**Phase 1 (MVP) - Cần:**

- `packages/core` - TypeScript business logic (FEFO/FIFO, validation) - Functional style
- `apps/client-web` - React + Vite (Web & Desktop Tauri) - dùng `@lean-wms/core` + shadcn/ui
- `apps/mobile` - Expo (React Native) - dùng `@lean-wms/core`, API calls (KHÔNG WatermelonDB)
- `apps/api` - Rust (Axum) - Backend API, rspc cho shared types

## Project Structure (MVP - Phase 1)

```javascript
lean-wms/
├── README.md                    # ✅ Đã có
├── PRD.md                       # ✅ Đã có
├── doc/                         # ✅ Đã có
│   ├── BLUEPRINT.md
│   ├── BLUEPRINT_ENGINEER_BACKEND.md
│   ├── BLUEPRINT_DESIGNER_FRONTEND.md
│   └── WHITEPAPER_FUTURE_FEATURES.md
│
├── packages/
│   └── core/                    # TypeScript: Business logic, validation, types
│       ├── package.json
│       ├── tsconfig.json
│       ├── src/
│       │   ├── index.ts
│       │   ├── inventory/       # FEFO/FIFO algorithms (Functional style)
│       │   ├── validation/      # Poka-Yoke rules, Zod schemas
│       │   ├── location/       # Location hierarchy logic
│       │   └── types/          # Shared TypeScript types
│       └── README.md
│
├── apps/
│   ├── client-web/              # React + Vite (Web & Desktop Tauri)
│   │   ├── package.json
│   │   ├── vite.config.ts
│   │   ├── tsconfig.json
│   │   ├── index.html
│   │   ├── src-tauri/           # Tauri backend (Rust commands nếu cần)
│   │   │   ├── Cargo.toml
│   │   │   ├── src/
│   │   │   │   └── main.rs       # Tauri commands
│   │   │   └── tauri.conf.json
│   │   ├── src/
│   │   │   ├── screens/         # Dashboard, Reports, Settings
│   │   │   ├── components/      # shadcn/ui components
│   │   │   ├── services/        # API client (rspc)
│   │   │   ├── store/           # Redux/Zustand
│   │   │   └── utils/
│   │   └── README.md
│   │
│   ├── mobile/                  # Expo (React Native) - Phase 1 MVP
│   │   ├── app.json
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── babel.config.js
│   │   ├── src/
│   │   │   ├── screens/         # Inbound, Outbound, Counting, Put-away
│   │   │   ├── components/      # ScannerView, FeedbackOverlay, etc.
│   │   │   ├── services/        # API client (axios/rspc)
│   │   │   ├── store/           # Redux/Zustand
│   │   │   └── utils/
│   │   └── README.md
│   │
│   └── api/                     # Rust (Axum) - Backend API
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   ├── api/             # REST endpoints + rspc router
│       │   │   ├── auth.rs
│       │   │   ├── products.rs
│       │   │   ├── orders.rs
│       │   │   └── sync.rs
│       │   ├── database/        # SeaORM models & migrations
│       │   ├── services/        # Business logic
│       │   ├── middleware/      # Auth, logging, etc.
│       │   └── config.rs
│       └── migrations/           # SeaORM migrations
│
├── Cargo.toml                   # Root workspace cho Rust (api + tauri)
├── package.json                 # Root workspace (pnpm)
├── pnpm-workspace.yaml          # pnpm workspace config
├── turbo.json                   # TurboRepo config
├── .gitignore
├── .env.example
└── docker-compose.yml           # PostgreSQL
```

**Key Decisions:**

- **`packages/core`:** TypeScript business logic (Functional style) - dùng cho Web, Mobile, Desktop
- **`client-web`:** React + Vite, Tauri wrap nó → Web deploy Vercel/Docker, Desktop = .exe/.app
- **Mobile:** Expo, import `@lean-wms/core`, API calls (KHÔNG WatermelonDB cho MVP)
- **API:** Rust + rspc cho shared types (tự động sync Type từ Rust → TS)

## Implementation Steps (CLI-First) - MVP Only

### Phase 1: Root Workspace Setup

#### 1.1. Create Directory Structure

```bash
mkdir -p apps
```



#### 1.2. Initialize Root pnpm Workspace

```bash
# Tạo root package.json
pnpm init

# Tạo pnpm-workspace.yaml
cat > pnpm-workspace.yaml << EOF
packages:
    - 'apps/*'
    - 'packages/*'
EOF

# Install TurboRepo (cần cho multi-apps)
pnpm add -D -w turbo
```



### Phase 2: Shared Core Package (TypeScript)

#### 2.1. Initialize Core Package (TypeScript Business Logic)

```bash
# Tạo core package
cd packages
mkdir core && cd core
pnpm init

# Install dependencies
pnpm add zod                    # Validation schemas
pnpm add date-fns               # Date utilities cho FEFO
pnpm add -D typescript @types/node

# Tạo structure (Functional Programming style)
mkdir -p src/inventory src/validation src/location src/types

# Tạo index.ts để export
cat > src/index.ts << EOF
export * from './inventory';
export * from './validation';
export * from './location';
export * from './types';
EOF
```

**Core Package Structure (Functional Style):**

- `src/inventory/` - FEFO/FIFO algorithms (pure functions, input/output rõ ràng)
- `src/validation/` - Poka-Yoke rules, Zod schemas
- `src/location/` - Location hierarchy logic
- `src/types/` - Shared TypeScript types

**Nguyên tắc viết code:**

- **Functional Programming:** Pure functions, không side effects
- **Input/Output rõ ràng:** Dễ migrate sang Rust sau
- **Type-safe:** Sử dụng TypeScript strict mode

### Phase 3: Client-Web (React + Vite + Tauri)

#### 3.1. Initialize Client-Web (Unified Web & Desktop)

```bash
# Tạo Vite + React app
cd ../../apps
mkdir client-web && cd client-web
pnpm create vite@latest . --template react-ts --yes

# Install dependencies
pnpm install
pnpm add @tanstack/react-router  # hoặc react-router-dom
pnpm add @reduxjs/toolkit react-redux
pnpm add axios
pnpm add @lean-wms/core          # Import business logic từ core

# Install shadcn/ui
pnpm add tailwindcss postcss autoprefixer
pnpm add class-variance-authority clsx tailwind-merge
pnpm add -D @vitejs/plugin-react
pnpm dlx tailwindcss init -p
pnpm dlx shadcn-ui@latest init

# Install Tauri (optional, có thể add sau khi web chạy ổn)
pnpm add -D @tauri-apps/cli
pnpm add @tauri-apps/api

# Setup Tauri
pnpm dlx create-tauri-app --yes
# Chọn: React, TypeScript, Vite
# Tauri sẽ tự detect Vite config và point đến nó

# Install dev dependencies
pnpm add -D eslint @typescript-eslint/eslint-plugin @typescript-eslint/parser
pnpm add -D prettier
```

**Client-Web Structure:**

- **Web:** Deploy lên Vercel/Docker như bình thường
- **Desktop:** Tauri wrap `client-web` → .exe/.app
- **UI:** shadcn/ui components (Button, Input, etc.)
- **Logic:** Import từ `@lean-wms/core`

### Phase 4: Mobile App (Expo)

#### 4.1. Initialize Mobile App (Expo)

```bash
# Tạo Expo app
cd ../mobile
pnpm create expo-app@latest . --template blank-typescript --yes

# Install dependencies
pnpm add react-native-vision-camera
pnpm add @reduxjs/toolkit react-redux
pnpm add @react-navigation/native @react-navigation/stack
pnpm add axios
pnpm add expo-secure-store
pnpm add @lean-wms/core          # Import business logic từ core

# Lưu ý: KHÔNG cài WatermelonDB cho MVP (giảm complexity)
# Chỉ dùng API calls thông thường

# Install dev dependencies
pnpm add -D eslint @typescript-eslint/eslint-plugin @typescript-eslint/parser
pnpm add -D prettier
pnpm add -D jest @testing-library/react-native

# Setup ESLint
pnpm dlx eslint --init --yes
```

**Mobile App Structure:**

- `src/screens/` - Inbound, Outbound, Counting, Put-away
- `src/components/` - ScannerView, FeedbackOverlay, etc.
- `src/services/` - API client (axios, không WatermelonDB)
- `src/store/` - Redux/Zustand
- **Logic:** Import từ `@lean-wms/core` (không inline)

### Phase 5: API Backend (Rust)

#### 5.1. Initialize API Backend (Rust)

```bash
# Tạo Rust backend
cd ../api
cargo new --name lean-wms-api .

# Add dependencies
cargo add axum --features macros
cargo add tokio --features full
cargo add sea-orm --features sqlx-postgres,runtime-tokio-native-tls
cargo add serde --features derive
cargo add jsonwebtoken
cargo add tower tower-http --features cors,compression-gzip
cargo add log env_logger
cargo add dotenv
cargo add utoipa --features axum_extras
cargo add utoipa-swagger-ui --features axum
cargo add thiserror
cargo add rspc --features axum  # Shared types Rust → TypeScript

# Tạo module structure
mkdir -p src/api src/database src/services src/middleware
mkdir -p migrations

# Setup SeaORM migrations
cargo install sea-orm-cli
sea-orm-cli migrate init
```

**API Backend Structure:**

- **Rust đúng vị trí:** Backend API + Tauri commands (không FFI cho mobile)
- **rspc:** Shared types tự động sync từ Rust → TypeScript
- **Database:** PostgreSQL với SeaORM migrations (Data Integrity First)

### Phase 6: Root Workspace Configuration

#### 6.1. Initialize Root Cargo Workspace

```bash
# Tạo root Cargo.toml
cd ../..
cargo init --name lean-wms-root

# Edit Cargo.toml để add workspace
# [workspace]
# members = ["apps/api", "apps/client-web/src-tauri"]
# resolver = "2"
```



#### 6.2. Configure TurboRepo

```bash
# Tạo turbo.json
cat > turbo.json << EOF
{
  "$schema": "https://turbo.build/schema.json",
  "pipeline": {
    "dev": {
      "cache": false,
      "persistent": true
    },
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", ".next/**", "build/**"]
    },
    "test": {
      "dependsOn": ["build"],
      "outputs": []
    },
    "lint": {
      "outputs": []
    }
  }
}
EOF

# Add scripts vào root package.json
# "dev": "turbo dev",
# "build": "turbo build",
# "test": "turbo test",
# "lint": "turbo lint"
```



### Phase 7: Development Environment

#### 5.1. Setup Docker Compose

```bash
# Tạo docker-compose.yml
cat > docker-compose.yml << EOF
version: '3.8'
services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: leanwms
      POSTGRES_PASSWORD: leanwms
      POSTGRES_DB: leanwms
    ports:
                        - "5432:5432"
    volumes:
                        - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
EOF
```



#### 5.2. Create .env.example

```bash
# Tạo .env.example
cat > .env.example << EOF
# Database
DATABASE_URL=postgresql://leanwms:leanwms@localhost:5432/leanwms

# JWT
JWT_SECRET=your-secret-key-here
JWT_REFRESH_SECRET=your-refresh-secret-here

# API
API_PORT=3000
API_URL=http://localhost:3000

# Logging
RUST_LOG=info
EOF
```



### Phase 8: Code Quality Setup

#### 6.1. Setup Basic Code Quality (Optional cho MVP)

```bash
# Tạo .prettierrc (basic config)
cat > .prettierrc << EOF
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5"
}
EOF

# Lưu ý: Husky + lint-staged có thể thêm sau khi có nhiều contributors
# MVP: Chỉ cần ESLint + Prettier trong IDE
```



#### 6.2. Create .editorconfig

```bash
cat > .editorconfig << EOF
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 2

[*.rs]
indent_size = 4

[*.{json,yml,yaml}]
indent_size = 2
EOF
```



#### 6.3. Create .gitignore

```bash
cat > .gitignore << EOF
# Dependencies
node_modules/
.pnp
.pnp.js

# Build outputs
dist/
build/
*.exe
*.app
*.dmg

# Rust
target/
Cargo.lock

# Environment
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Expo
.expo/
.expo-shared/

# Tauri
src-tauri/target/

# Logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

# Testing
coverage/
.nyc_output/

# Turbo
.turbo/
EOF
```



### Phase 9: CI/CD Setup (Basic cho MVP)

#### 7.1. Create GitHub Actions Workflow

```bash
mkdir -p .github/workflows

cat > .github/workflows/ci.yml << 'EOF'
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  lint-and-test:
    runs-on: ubuntu-latest
    steps:
                        - uses: actions/checkout@v4
      
                        - name: Setup pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8
      
                        - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'
      
                        - name: Install dependencies
        run: pnpm install --frozen-lockfile
      
                        - name: Lint
        run: pnpm turbo lint
      
                        - name: Test
        run: pnpm turbo test
      
                        - name: Build
        run: pnpm turbo build

  rust-check:
    runs-on: ubuntu-latest
    steps:
            - uses: actions/checkout@v4
      
            - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      
            - name: Check format
        run: cd apps/api && cargo fmt -- --check
      
            - name: Clippy
        run: cd apps/api && cargo clippy -- -D warnings
      
            - name: Test
        run: cd apps/api && cargo test
EOF
```



## Technical Decisions

### Backend Stack

- **Framework:** Axum (async, lightweight)
- **ORM:** SeaORM (async, ActiveRecord-like)
- **Database:** PostgreSQL
- **Logging:** `log` crate + `env_logger`
- **API Docs:** utoipa (OpenAPI/Swagger)
- **Env:** dotenv

### Frontend Stack

- **Web & Desktop:** React + Vite (unified codebase trong `client-web`)
- **Mobile:** Expo (React Native)
- **State Management:** Redux Toolkit
- **UI Library:** 
                                - Web/Desktop: shadcn/ui (Tailwind CSS)
                                - Mobile: React Native components (có thể dùng Tamagui nếu muốn share UI)
- **Validation:** Zod (trong `packages/core`)

### Business Logic

- **Language:** TypeScript (trong `packages/core`)
- **Style:** Functional Programming (pure functions, input/output rõ ràng)
- **Future-proof:** Dễ migrate sang Rust nếu cần performance cao
- **Shared Types:** rspc (tự động sync Type từ Rust Backend → TypeScript Frontend)

### Database Strategy	

- **Backend:** PostgreSQL với SeaORM migrations (Data Integrity First)
- **Mobile:** API calls thông thường (KHÔNG WatermelonDB cho MVP - giảm complexity)
- **Offline-first:** Có thể thêm WatermelonDB sau khi cần

### Code Quality

- **Rust:** `cargo fmt`, `cargo clippy`
- **TypeScript:** ESLint + Prettier
- **Pre-commit:** Husky + lint-staged
- **Monorepo:** TurboRepo

### Testing

- **Strategy:** Basic unit tests (MVP first)
- **Rust:** Built-in `#[cfg(test)]`
- **TypeScript:** Jest

### CI/CD

- **Platform:** GitHub Actions
- **Checks:** Format, lint, test, build
- **Monorepo:** TurboRepo cache

## Development Workflow (MVP)

```bash
# Start all services (TurboRepo)
pnpm turbo dev

# Individual services
pnpm --filter client-web dev      # Web (Vite dev server :5173)
pnpm --filter mobile dev          # Mobile (Expo :19000)
pnpm --filter api dev             # Backend (Rust API :3000)

# Tauri Desktop (sử dụng client-web)
cd apps/client-web
pnpm tauri dev                     # Tauri sẽ dùng Vite dev server

# Start PostgreSQL
docker-compose up -d              # PostgreSQL :5432
```



## Key Principles (Product-Focused)

1. **Ra demo sớm:** Ưu tiên ra bản demo sớm nhất cho khách hàng (công ty dược phẩm)
2. **Shared Logic bằng TypeScript:** `packages/core` bằng TS ngay từ đầu (Web, Mobile, Desktop đều đọc được)
3. **Hợp nhất Web & Desktop:** Một dự án React duy nhất `client-web`, Tauri chỉ wrap nó
4. **Rust đúng vị trí:** Backend API + Tauri commands (KHÔNG dùng Rust FFI cho mobile)
5. **Functional Programming:** Logic viết theo FP (input/output rõ ràng) để dễ migrate sang Rust sau
6. **Data Integrity First:** Database design chuẩn chỉnh ngay từ đầu (Migration, Constraints, Indexing)

## Migration Path (Future)

### TypeScript Core → Rust Core (nếu cần performance cao)

**Trigger:** Khi thuật toán sắp xếp kho cần hiệu năng cực cao**Steps:**

1. Logic đã viết theo Functional Programming (pure functions)
2. Chỉ cần "dịch" TypeScript → Rust
3. Không cần tư duy lại từ đầu

### Mobile: API Calls → WatermelonDB (nếu cần Offline-first)

**Trigger:** Khi thực sự cần Offline-first (không phải MVP)**Steps:**