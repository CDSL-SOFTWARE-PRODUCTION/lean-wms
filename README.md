# LEAN WMS - Lean Warehouse Management System

Hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng.

---

## 🚀 Quick Start

Dựng hệ thống trong 5 phút.

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop)
- [Node.js](https://nodejs.org/) (v20+)
- [pnpm](https://pnpm.io/installation)

### Installation

```bash
# 1. Clone repository
git clone https://github.com/hungchann/lean-wms.git
cd lean-wms

# 2. Install dependencies
pnpm install

# 3. Start Database (PostgreSQL via Docker)
docker compose up -d

# 4. Run Development Server (All apps)
pnpm dev
```

Truy cập:

- Frontend: `http://localhost:5173`
- API: `http://localhost:3000`

---

## ✨ Features

| Feature | Lean WMS | Traditional WMS | Excel/Paper |
|---------|----------|-----------------|-------------|
| **Cost** | Low/Free (Self-host) | High ($$$) | Low |
| **Mobile** | Native (Expo) | Usually Web-based | None |
| **Offline**| ✅ First-class | ❌ Rare | ✅ (Manual) |
| **Speed** | ⚡ High (Rust) | 🐢 Slow (Legacy) | ⚡ Fast (Local) |
| **Logic** | FEFO/FIFO Auto | Complex Config | Manual |

### Pros & Cons

**Pros:**

- **Performance**: Backend Rust + Frontend React = Tốc độ cao.
- **Offline-First**: Mobile app hoạt động kể cả khi mất mạng.
- **Monorepo**: Code được chia sẻ chặt chẽ giữa Frontend/Mobile/Backend.

**Cons:**

- **Learning Curve**: Cần kiến thức về Rust, React, và TypeScript để maintain.
- **Setup**: Cần Docker để chạy đầy đủ stack.

---

## 📚 Documentation Index

Chúng tôi tổ chức tài liệu theo các thư mục chuyên biệt:

### 🏗️ Architecture & Decisions

- **[Architecture Overview](doc/architecture/ARCHITECTURE.md)**: Sơ đồ hệ thống, luồng dữ liệu, cấu trúc Monorepo.
- **[Backend Blueprint](doc/architecture/BLUEPRINT_BACKEND.md)**: Chi tiết thiết kế kỹ thuật Backend.
- **[Frontend Blueprint](doc/architecture/BLUEPRINT_FRONTEND.md)**: Chi tiết thiết kế UI/UX.
- **[Decisions (ADR)](doc/architecture/adr/)**: Các quyết định kỹ thuật quan trọng.

### 📋 Requirements & Planning

- **[Project Plan](doc/planning/PROJECT_PLAN.md)**: Timeline, rủi ro, và mục tiêu dự án.
- **[Roadmap](doc/planning/ROADMAP.md)**: Các tính năng tương lai (Phase 2, 3).
- **[Product Requirements (PRD)](doc/requirements/PRD.md)**: User stories và yêu cầu sản phẩm.

### 🛠️ Guides & Manuals

- **[Quick Start / Contributing](doc/guides/CONTRIBUTING.md)**: Hướng dẫn cho Developer mới.
- **[DevOps Guide](doc/guides/DEVOPS_GUIDE.md)**: Hướng dẫn deploy và quản lý server.
- **[Testing Strategy](doc/guides/TESTING.md)**: Chiến lược kiểm thử.
- **[API Docs](doc/api/README.md)**: Tài liệu API.

---

## 🤝 Community & Support

- Open a [Discussion](https://github.com/hungchann/lean-wms/discussions)
- Submit a [Pull Request](https://github.com/hungchann/lean-wms/pulls)

**Last Updated:** 13-01-2026
