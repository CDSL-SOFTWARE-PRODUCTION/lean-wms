# LEAN WMS - Lean Warehouse Management System

Hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng.

---

## 🚀 Quick Start

### For Developers

**🚀 New Developer?** Bắt đầu với:

1. **Onboarding:** Đọc [`doc/CONTRIBUTING.md`](doc/CONTRIBUTING.md) - Setup và development workflow
2. **Architecture:** Đọc [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) - Hiểu cấu trúc monorepo và data flow
3. **Overview:** Đọc [`doc/BLUEPRINT.md`](doc/BLUEPRINT.md) - Tổng quan hệ thống

**📚 Deep Dive:**

- **Product Requirements:** [`PRD.md`](PRD.md) - Product vision và requirements
- **UI/UX Specs:** [`doc/BLUEPRINT_DESIGNER_FRONTEND.md`](doc/BLUEPRINT_DESIGNER_FRONTEND.md) - Design guidelines
- **Technical Specs:** [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](doc/BLUEPRINT_ENGINEER_BACKEND.md) - Database, API, business logic

### For Product Managers / Stakeholders

- **Start here:** [`PRD.md`](PRD.md) - Product Requirements Document
- **Roadmap:** [`doc/WHITEPAPER_FUTURE_FEATURES.md`](doc/WHITEPAPER_FUTURE_FEATURES.md)

---

## 🛠️ Development Setup

### Quick Setup (5 phút)

```bash
# 1. Install dependencies
pnpm install

# 2. Setup database (Docker)
docker-compose up -d

# 3. Run all apps
pnpm dev
```

**Chi tiết:** Xem [`doc/CONTRIBUTING.md`](doc/CONTRIBUTING.md) cho hướng dẫn đầy đủ.

### Monorepo Structure

Dự án sử dụng **Monorepo** với:

- **Frontend:** React (Web) + Expo (Mobile)
- **Backend:** Rust (Axum)
- **Shared:** TypeScript core package

**Architecture:** Xem [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) để hiểu cấu trúc và data flow.

---

## 📚 Documentation Structure

### Core Documents

| Document | Audience | Purpose |
| ----------- | -------- | --------- |
| [`PRD.md`](PRD.md) | Product Manager, Stakeholders | Product vision, goals, user stories, timeline |
| [`doc/BLUEPRINT.md`](doc/BLUEPRINT.md) | All | Overview & quick reference |
| [`doc/BLUEPRINT_DESIGNER_FRONTEND.md`](doc/BLUEPRINT_DESIGNER_FRONTEND.md) | Designer | UI/UX specifications, wireframes, design guidelines |
| [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](doc/BLUEPRINT_ENGINEER_BACKEND.md) | Engineer | Technical specs: Database, API, Security, Business Logic |
| [`doc/WHITEPAPER_FUTURE_FEATURES.md`](doc/WHITEPAPER_FUTURE_FEATURES.md) | All | Roadmap Phase 2, Phase 3 |

### Developer Documentation

| Document | Audience | Purpose |
| ----------- | -------- | --------- |
| [`doc/CONTRIBUTING.md`](doc/CONTRIBUTING.md) | **New Developers** | Onboarding guide, setup instructions, development workflow |
| [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) | Engineers | System architecture, monorepo structure, data flow, component details |

### Supporting Materials

- [`doc/illustration/plantuml-diagram-ui.txt`](doc/illustration/plantuml-diagram-ui.txt) - PlantUML diagrams
- [`doc/illustration/plantuml-diagram-backend.txt`](doc/illustration/plantuml-diagram-backend.txt) - Backend architecture diagrams
- [`doc/illustration/architecture-diagram.txt`](doc/illustration/architecture-diagram.txt) - System architecture diagrams

---

## 🎯 What is Lean WMS?

- **Lean WMS** là hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng, đem quy trình chuẩn của nhà máy lớn (Samsung, Amazon) nén vào chiếc điện thoại di động cho hộ gia đình và xưởng nhỏ.

---

## 🛠️ Technical Stack

- **Mobile:** Expo (Android 8.0+, iOS 12.0+) - Local First
- **Backend:** Rust Server (Axum/Actix) + WebSockets
- **Auth:** JWT với refresh token
- **Architecture:** Client-server (Redux Toolkit for state management)

**Key Principles:**

- **Adaptable:** Cấu hình linh hoạt cho xưởng nhỏ (tắt validation vị trí) hoặc kho lớn (quy trình chặt chẽ).
- **Local-First:** Hoạt động tốt ngay cả khi server sập, sync sau.

**Performance Targets:**

- Quét mã: < 500ms
- Phản hồi UI: < 100ms
- Offline capacity: 10,000+ actions

---

## 📋 Phase Status

**Phase 1 (MVP) - In Development:**

- App mobile cho công nhân (Worker)
- Core warehouse operations
- Offline-first architecture

**Phase 2 (Future):**

- Desktop App quản lý (Tauri - cho chủ xưởng)
- Báo cáo nâng cao

**Phase 3 (Future):**

- Multi-warehouse
- AI/ML features
- ERP integration

Xem chi tiết trong [`doc/WHITEPAPER_FUTURE_FEATURES.md`](doc/WHITEPAPER_FUTURE_FEATURES.md)

---

## 🤝 Contributing

**New to the project?** Start with [`doc/CONTRIBUTING.md`](doc/CONTRIBUTING.md) - Hướng dẫn onboarding cho developer mới.

### Documentation Updates

Khi cần thêm hoặc sửa documentation:

- **Product changes:** Update `PRD.md`
- **UI/UX changes:** Update `doc/BLUEPRINT_DESIGNER_FRONTEND.md`
- **Technical changes:** Update `doc/BLUEPRINT_ENGINEER_BACKEND.md`
- **Architecture changes:** Update `doc/ARCHITECTURE.md`
- **Roadmap changes:** Update `doc/WHITEPAPER_FUTURE_FEATURES.md`
- **Overview changes:** Update `doc/BLUEPRINT.md` (keep it concise!)

---

## 📞 Contact & Resources

- Xem [`PRD.md`](PRD.md) cho product goals và success metrics
- Xem [`doc/BLUEPRINT.md`](doc/BLUEPRINT.md) cho quick reference

---

**Last Updated:** 04-01-2026
