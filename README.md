# LEAN WMS - Lean Warehouse Management System

Hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng, mang quy trình chuẩn của nhà máy lớn nén vào thiết bị di động.

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

---

## ✨ Features (Phase 1 MVP)

- ✅ **Nhập kho (Inbound)**: Mapping mã vạch linh hoạt (nhiều mã -> 1 SKU).
- ✅ **Xuất kho (Outbound)**: Guided workflow với Poka-Yoke, FEFO/FIFO.
- ✅ **Kiểm kê (Counting)**: Blind count đảm bảo tính chính xác.
- ✅ **Cất hàng (Put-away)**: Validation vị trí và sức chứa.
- ✅ **Offline-first**: Hoạt động khi mất mạng, tự động đồng bộ.
- ✅ **Phản hồi tức thì**: Màu sắc, âm thanh, và haptic feedback.

---

## 🛠️ Technical Stack

| Thành phần | Công nghệ | Mục tiêu |
| :--- | :--- | :--- |
| **Mobile App** | Expo (React Native) | Camera/Scanner performance & Cross-platform. |
| **Backend API** | Rust (Axum + SeaORM) | Hiệu năng cao, Type-safety, xử lý logic kho. |
| **Frontend Web** | React 19 + Vite | Dashboard quản lý mượt mà. |
| **Desktop App** | Tauri (Rust wrapper) | App quản lý nhẹ, bảo mật cao. |
| **Core Logic** | TypeScript (Shared) | Thuật toán FEFO/FIFO và Validation dùng chung. |

---

## 📚 Documentation

Tài liệu chi tiết được tổ chức trong thư mục [`doc/`](doc/):

### 📐 Architecture

- **[Architecture Overview](doc/architecture/ARCHITECTURE.md)**: Sơ đồ hệ thống, Monorepo, Data Flow.
- **[Backend Blueprint](doc/architecture/BLUEPRINT_BACKEND.md)**: Technical spec & DB Schema.
- **[Frontend Blueprint](doc/architecture/BLUEPRINT_FRONTEND.md)**: UI/UX & Design guidelines.

### 🛠️ Developer Guides

- **[CONTRIBUTING.md](doc/guides/CONTRIBUTING.md)**: **Bắt đầu tại đây!** Setup & Workflow.
- **[DEVOPS_GUIDE.md](doc/guides/DEVOPS_GUIDE.md)**: Deploy & Management.
- **[RELEASE_PROCESS.md](doc/guides/RELEASE_PROCESS.md)**: Quy trình release.

### 📋 Product & Planning

- **[PRD.md](doc/requirements/PRD.md)**: Yêu cầu sản phẩm chi tiết.
- **[ROADMAP.md](doc/planning/ROADMAP.md)**: Tầm nhìn dài hạn (Phase 2, 3, và AI).

---

## 🤝 Community & Support

- Open a [Discussion](https://github.com/hungchann/lean-wms/discussions)
- Submit a [Pull Request](https://github.com/hungchann/lean-wms/pulls)

**Last Updated:** 14-01-2026
