# LEAN WMS - Lean Warehouse Management System

Hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng.

---

## 🚀 Quick Start

### For Developers

1. **Product Requirements:** Đọc [`PRD.md`](PRD.md) để hiểu product vision và requirements
2. **UI/UX Specs:** Đọc [`doc/BLUEPRINT_DESIGNER_FRONTEND.md`](doc/BLUEPRINT_DESIGNER_FRONTEND.md) cho design guidelines
3. **Technical Specs:** Đọc [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](doc/BLUEPRINT_ENGINEER_BACKEND.md) cho database, API, business logic
4. **Overview:** Đọc [`doc/BLUEPRINT.md`](doc/BLUEPRINT.md) để có cái nhìn tổng quan

### For Product Managers / Stakeholders

- **Start here:** [`PRD.md`](PRD.md) - Product Requirements Document
- **Roadmap:** [`doc/WHITEPAPER_FUTURE_FEATURES.md`](doc/WHITEPAPER_FUTURE_FEATURES.md)

---

## 🛠️ Development Setup

## 📚 Documentation Structure

### Core Documents

| Document | Audience | Purpose |
|----------|----------|---------|
| [`PRD.md`](PRD.md) | Product Manager, Stakeholders | Product vision, goals, user stories, timeline |
| [`doc/BLUEPRINT.md`](doc/BLUEPRINT.md) | All | Overview & quick reference |
| [`doc/BLUEPRINT_DESIGNER_FRONTEND.md`](doc/BLUEPRINT_DESIGNER_FRONTEND.md) | Designer | UI/UX specifications, wireframes, design guidelines |
| [`doc/BLUEPRINT_ENGINEER_BACKEND.md`](doc/BLUEPRINT_ENGINEER_BACKEND.md) | Engineer | Technical specs: Database, API, Security, Business Logic |
| [`doc/WHITEPAPER_FUTURE_FEATURES.md`](doc/WHITEPAPER_FUTURE_FEATURES.md) | All | Roadmap Phase 2, Phase 3 |

### Supporting Materials

- [`doc/illustration/plantuml-diagram-ui.txt`](doc/illustration/plantuml-diagram-ui.txt) - PlantUML diagrams

---

## 🎯 What is Lean WMS?

**Lean WMS** đem quy trình chuẩn của nhà máy lớn (Samsung, Amazon) nén vào chiếc điện thoại di động cho xưởng nhỏ.

### Key Features

- ✅ **Quét QR/Barcode** bằng camera điện thoại (không cần thiết bị đặc biệt)
- ✅ **Offline-first:** Hoạt động khi mất mạng, tự động đồng bộ khi có mạng
- ✅ **Brainless operation:** Giao diện cực đơn giản, phản hồi bằng màu sắc/âm thanh
- ✅ **Poka-Yoke:** Chống sai lỗi, validation real-time

### Core Workflows

1. **Nhập kho (Inbound)** - Hybrid (Nhập tay / Auto-gen SKU)
2. **Xuất kho (Outbound)** - Guided workflow với FEFO/FIFO & Manager Override
3. **Kiểm kê (Counting)** - Blind count với Audit Log
4. **Cất hàng (Put-away)** - Validation vị trí linh hoạt (Visual Capacity)

---

## 🛠️ Technical Stack

- **Mobile:** Expo (Android 8.0+, iOS 12.0+) - Local First
- **Backend:** Rust Server (Axum/Actix) + WebSockets
- **Auth:** JWT với refresh token
- **Architecture:** Offline-first với local database (WatermelonDB)

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

Khi cần thêm hoặc sửa documentation:

- **Product changes:** Update `PRD.md`
- **UI/UX changes:** Update `doc/BLUEPRINT_DESIGNER_FRONTEND.md`
- **Technical changes:** Update `doc/BLUEPRINT_ENGINEER_BACKEND.md`
- **Roadmap changes:** Update `doc/WHITEPAPER_FUTURE_FEATURES.md`
- **Overview changes:** Update `doc/BLUEPRINT.md` (keep it concise!)

---

## 📞 Contact & Resources

- Xem [`PRD.md`](PRD.md) cho product goals và success metrics
- Xem [`doc/BLUEPRINT.md`](doc/BLUEPRINT.md) cho quick reference

---

**Last Updated:** 04-01-2026

