# LEAN WMS - BLUEPRINT (Master Document)

**Mục tiêu:** Tài liệu tổng quan và quick reference cho toàn bộ hệ thống.

> **📌 LƯU Ý:** Tài liệu chi tiết đã được chia thành các file chuyên biệt.  
> File này chỉ giữ lại overview và references để tra cứu nhanh.

---

## Quick Links

### Core Documents

- **`PRD.md`** - Product Requirements Document (Product vision, goals, user stories, timeline)
- **`doc/BLUEPRINT_DESIGNER_FRONTEND.md`** - UI/UX specifications & Design guidelines
- **`doc/BLUEPRINT_ENGINEER_BACKEND.md`** - Technical specifications (Database, API, Security, Business Logic)
- **`doc/WHITEPAPER_FUTURE_FEATURES.md`** - Roadmap Phase 2, Phase 3

### Developer Documentation

- **`doc/CONTRIBUTING.md`** - **🚀 Start here!** Onboarding guide cho developer mới (setup, workflow, FAQ)
- **`doc/ARCHITECTURE.md`** - System architecture, monorepo structure, data flow, component details

### For Different Roles

- **New Developer:** Đọc `doc/CONTRIBUTING.md` → `doc/ARCHITECTURE.md` → `doc/BLUEPRINT_ENGINEER_BACKEND.md`
- **Product Manager:** Đọc `PRD.md`
- **Designer:** Đọc `doc/BLUEPRINT_DESIGNER_FRONTEND.md`
- **Engineer:** Đọc `doc/BLUEPRINT_ENGINEER_BACKEND.md`
- **Stakeholders/Investors:** Đọc `PRD.md`

### Supporting Documents

- **`doc/illustration/plantuml-diagram-ui.txt`** - PlantUML diagrams cho UI flows
- **`doc/illustration/plantuml-diagram-backend.txt`** - PlantUML diagrams cho Backend Architecture & ERD
- **`doc/illustration/architecture-diagram.txt`** - System architecture diagrams (monorepo, data flow, deployment)

---

## System Overview

**Lean WMS** là hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng, đem quy trình chuẩn của nhà máy lớn (Samsung, Amazon) nén vào chiếc điện thoại di động.

### Core Features (Phase 1 MVP)

1. ✅ **Nhập kho (Inbound)** - Với mapping mã vạch linh hoạt (nhiều mã → 1 SKU)
2. ✅ **Xuất kho (Outbound)** - Guided workflow với Poka-Yoke, FEFO/FIFO
3. ✅ **Kiểm kê (Counting)** - Blind count để đảm bảo tính chính xác
4. ✅ **Cất hàng (Put-away)** - Validation vị trí (Fixed Bin/Mixed Bin, Capacity)
5. ✅ **Offline-first** - Hoạt động khi mất mạng, sync tự động
6. ✅ **Phản hồi tức thì** - Màu sắc, âm thanh, haptic feedback

### Key Principles

- **Brainless operation:** Giao diện cực đơn giản, công nhân không cần nhớ nhiều
- **Poka-Yoke:** Chống sai lỗi, validation real-time
- **Offline-first:** Hoạt động bình thường khi mất mạng, tự động sync khi có mạng lại
- **Guided workflow:** Tuyệt đối không để công nhân tự hỏi "Giờ làm gì tiếp?"

---

## Technical Stack Overview

| Thành phần | Lựa chọn | Mục đích |
| ----------- | -------- | --------- |
| **Mobile App** | Expo | Tận dụng thư viện Camera/Scanner tốt nhất cho WMS |
| **State Management** | Redux Toolkit | Quản lý trạng thái tập trung |
| **Logic Core** | Rust | Các hàm Functional xử lý tồn kho, validation dùng chung |
| **Desktop App** | Tauri (Rust) | App quản lý cho chủ xưởng mượt, nhẹ, bảo mật cao |
| **Sync Protocol** | WebSockets/NATS | Đảm bảo tính real-time khi có mạng lại |

**Chi tiết:**

**Mobile App (Expo):**

- Platform: Android 8.0+ (Oreo), iOS 12.0+
- Architecture: Client-server with Redux Toolkit
- Scanner:
  - Phase 1: Camera QR/Barcode scanning (react-native-vision-camera)
  - Phase 2: 2D Area Imager support qua Bluetooth (Professional Tier)
- State Management: Redux Toolkit / Zustand

**Backend:**

- API: RESTful API (JSON) + WebSockets
- Database: PostgreSQL/MySQL
- Authentication: JWT với refresh token, device binding
- Sync: WebSockets/NATS cho real-time sync với conflict resolution

**Logic Core (TypeScript):**

- Shared business logic giữa Web và Mobile (TypeScript)
- Functional Programming style
- Validation rules, inventory calculations, FEFO/FIFO algorithms
- **Note:** Backend Rust implement logic tương tự (không dùng packages/core trực tiếp)

**Performance Targets:**

- Quét mã: < 500ms (Camera Phone) / < 100ms (2D Imager - Phase 2)
- Phản hồi UI: < 100ms
- Ghi Local DB: < 50ms
- Offline capacity: 10,000+ actions

---

## Core Business Logic (Tóm tắt)

### Entity Management

- **Products (SKU):** Đối tượng gốc, có nhiều barcode mappings
- **Locations:** Phân cấp Warehouse → Rack → Shelf → Bin
- **Containers (LPN):** Vật chứa với nesting logic
- **Inventory Items:** Tồn kho thực tế (cái gì, ở đâu, bao nhiêu)

### Warehouse Processes

1. **Inbound:** Quét mã → Mapping (nếu cần) → Nhập thông tin → Quét vị trí → STAGING
2. **Put-away:** Quét hàng STAGING → Quét vị trí đích → Validation → AVAILABLE
3. **Outbound:** Chọn đơn → Guided workflow → FEFO/FIFO → Quét vị trí/hàng → SHIPPED
4. **Counting:** Blind count → So sánh với sổ sách → Flag lệch để Manager duyệt

### Key Algorithms

- **FEFO (First Expired, First Out):** Ưu tiên hàng sắp hết hạn (sort by expiry_date ASC)
- **FIFO (First In, First Out):** Ưu tiên hàng nhập trước (sort by created_at ASC)
- **Fixed Bin Validation:** Bin chỉ chứa 1 SKU cụ thể
- **Capacity Checking:** Kiểm tra max_capacity khi Put-away

---

## Phase Status

**Phase 1 (MVP) - ĐANG PHÁT TRIỂN:**

- App mobile cho công nhân (Worker)
- Core warehouse operations
- Offline-first architecture

**Phase 2 (Future):**

- Desktop App quản lý (Tauri - cho chủ xưởng)
- Dashboard quản lý với báo cáo nâng cao
- **2D Area Imager Support** (Professional Tier - Kho lớn 100+ công nhân)
- Cân điện tử & Máy in tem integration

**Phase 3 (Future):**

- Multi-warehouse
- AI/ML features
- ERP integration

Xem chi tiết roadmap trong `doc/WHITEPAPER_FUTURE_FEATURES.md`

---

## Future Documentation Strategy (Automated C4 Model)

Để đảm bảo tài liệu luôn đồng bộ với code (Living Documentation), dự án sẽ áp dụng chiến lược **"Architecture as Code"** để tự động sinh ra C4 Diagrams:

1.**Frontend (Expo/TS):** Sử dụng `madge` hoặc `dependency-cruiser` trong CI/CD pipeline để quét dependencies và sinh ra Component Diagrams (Level 3).
    - Command: `madge --extensions ts,tsx --image doc/arch/frontend.svg src/`
2.  **Backend (Rust):** Sử dụng `cargo-modules` để visualize cấu trúc module và dependencies của Rust crates.
    - Command: `cargo modules graph > doc/arch/backend.dot`
3.  **System Level (C4 Context/Container):** Sử dụng **Structurizr DSL** để định nghĩa High-level architecture. Kết hợp script quét code để auto-inject danh sách Components vào DSL, đảm bảo sơ đồ Level 1-2 luôn khớp với thực tế.

---

## Documentation Structure

```tree
├── PRD.md                              # Product Requirements (Product Manager)
├── README.md                           # Quick start guide
└── doc/
    ├── BLUEPRINT.md                    # This file (Overview & Quick Reference)
    ├── CONTRIBUTING.md                 # 🚀 Onboarding guide (New Developers)
    ├── ARCHITECTURE.md                 # System architecture & monorepo structure
    ├── BLUEPRINT_DESIGNER_FRONTEND.md  # UI/UX specs (Designer)
    ├── BLUEPRINT_ENGINEER_BACKEND.md   # Technical specs (Engineer)
    ├── WHITEPAPER_FUTURE_FEATURES.md   # Roadmap (Future)
    └── illustration/
        ├── plantuml-diagram-ui.txt      # PlantUML diagrams
        ├── plantuml-diagram-backend.txt # Backend architecture diagrams
        └── architecture-diagram.txt     # System architecture diagrams
```

---

**📝 Note:** Để biết chi tiết về bất kỳ phần nào, vui lòng tham khảo các file chuyên biệt tương ứng được liệt kê ở trên.
