# LEAN WMS - BLUEPRINT (Tài liệu Tổng quan)

**Mục tiêu:** Tài liệu tổng quan và tham khảo nhanh cho toàn bộ hệ thống.

> **📌 LƯU Ý:** Tài liệu chi tiết đã được chia thành các file chuyên biệt.
> File này chỉ giữ lại tổng quan và các tham chiếu để tra cứu nhanh.

---

## Liên kết Nhanh (Quick Links)

### Tài liệu Cốt lõi (Core Documents)

- **`PRD.md`** - Tài liệu Yêu cầu Sản phẩm (Tầm nhìn, mục tiêu, user stories, lộ trình)
- **`doc/BLUEPRINT_DESIGNER_FRONTEND.md`** - Đặc tả UI/UX & Hướng dẫn thiết kế
- **`doc/BLUEPRINT_ENGINEER_BACKEND.md`** - Đặc tả Kỹ thuật (Database, API, Bảo mật, Business Logic)
- **`doc/WHITEPAPER_FUTURE_FEATURES.md`** - Lộ trình Phase 2, Phase 3

### Tài liệu cho Developer

- **`doc/guides/CONTRIBUTING.md`** - **🚀 Bắt đầu tại đây!** Hướng dẫn onboarding cho developer mới (setup, workflow, FAQ)
- **`doc/architecture/ARCHITECTURE.md`** - Kiến trúc hệ thống, cấu trúc monorepo, luồng dữ liệu, chi tiết thành phần

### Đối với các Vai trò khác nhau

- **Developer Mới:** Đọc `doc/guides/CONTRIBUTING.md` → `doc/architecture/ARCHITECTURE.md` → `doc/architecture/BLUEPRINT_ENGINEER_BACKEND.md`
- **Product Manager:** Đọc `PRD.md`
- **Designer:** Đọc `doc/architecture/BLUEPRINT_DESIGNER_FRONTEND.md`
- **Engineer:** Đọc `doc/architecture/BLUEPRINT_ENGINEER_BACKEND.md`
- **Stakeholders/Investors:** Đọc `PRD.md`

### Tài liệu Bổ trợ

- **`doc/illustration/plantuml-diagram-ui.txt`** - Biểu đồ PlantUML cho các luồng UI
- **`doc/illustration/plantuml-diagram-backend.txt`** - Biểu đồ PlantUML cho Kiến trúc Backend & ERD
- **`doc/illustration/architecture-diagram.txt`** - Biểu đồ kiến trúc hệ thống (monorepo, luồng dữ liệu, triển khai)

---

## Tổng quan Hệ thống

**Lean WMS** là hệ thống Quản lý Kho & Sản xuất Tinh gọn cho xưởng nhỏ và kho hàng, đem quy trình chuẩn của nhà máy lớn (Samsung, Amazon) nén vào chiếc điện thoại di động.

### Tính năng Cốt lõi (Phase 1 MVP)

1. ✅ **Nhập kho (Inbound)** - Với mapping mã vạch linh hoạt (nhiều mã → 1 SKU)
2. ✅ **Xuất kho (Outbound)** - Quy trình hướng dẫn (Guided workflow) với Poka-Yoke, FEFO/FIFO
3. ✅ **Kiểm kê (Counting)** - Kiểm kê mù (Blind count) để đảm bảo tính chính xác
4. ✅ **Cất hàng (Put-away)** - Validation vị trí (Fixed Bin/Mixed Bin, Sức chứa)
5. ✅ **Offline-first** - Hoạt động khi mất mạng, đồng bộ tự động
6. ✅ **Phản hồi tức thì** - Màu sắc, âm thanh, phản hồi xúc giác (haptic)

### Nguyên tắc Chính

- **Vận hành không cần suy nghĩ (Brainless operation):** Giao diện cực đơn giản, công nhân không cần nhớ nhiều
- **Poka-Yoke:** Chống sai lỗi, validation real-time
- **Offline-first:** Hoạt động bình thường khi mất mạng, tự động sync khi có mạng lại
- **Guided workflow:** Tuyệt đối không để công nhân tự hỏi "Giờ làm gì tiếp?"

---

## Tổng quan Stack Kỹ thuật

| Thành phần | Lựa chọn | Mục đích |
|---|---|---|
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
  - Phase 2: Hỗ trợ 2D Area Imager qua Bluetooth (Professional Tier)
- State Management: Redux Toolkit / Zustand

**Backend:**

- API: RESTful API (JSON) + WebSockets
- Database: PostgreSQL/MySQL
- Authentication: JWT với refresh token, device binding
- Sync: WebSockets/NATS cho real-time sync với conflict resolution

**Logic Core (TypeScript):**

- Shared business logic giữa Web và Mobile (TypeScript)
- Functional Programming style
- Validate rules, tính toán tồn kho, thuật toán FEFO/FIFO
- **Note:** Backend Rust implement logic tương tự (không dùng packages/core trực tiếp)

**Mục tiêu Hiệu năng:**

- Quét mã: < 500ms (Camera Phone) / < 100ms (2D Imager - Phase 2)
- Phản hồi UI: < 100ms
- Ghi Local DB: < 50ms
- Dung lượng Offline: 10,000+ actions

---

## Logic Nghiệp vụ Cốt lõi (Tóm tắt)

### Quản lý Thực thể (Entity Management)

- **Products (SKU):** Đối tượng gốc, có nhiều barcode mappings
- **Locations:** Phân cấp Kho (Warehouse) → Kệ (Rack) → Tầng (Shelf) → Ô (Bin)
- **Containers (LPN):** Vật chứa với logic lồng nhau (nesting)
- **Inventory Items:** Tồn kho thực tế (cái gì, ở đâu, bao nhiêu)

### Quy trình Kho

1. **Inbound:** Quét mã → Mapping (nếu cần) → Nhập thông tin → Quét vị trí → STAGING
2. **Put-away:** Quét hàng STAGING → Quét vị trí đích → Validation → AVAILABLE
3. **Outbound:** Chọn đơn → Guided workflow → FEFO/FIFO → Quét vị trí/hàng → SHIPPED
4. **Counting:** Blind count → So sánh với sổ sách → Đánh dấu lệch để Quản lý duyệt

### Thuật toán Chính

- **FEFO (First Expired, First Out):** Ưu tiên hàng sắp hết hạn (sort by expiry_date ASC)
- **FIFO (First In, First Out):** Ưu tiên hàng nhập trước (sort by created_at ASC)
- **Fixed Bin Validation:** Bin chỉ chứa 1 SKU cụ thể
- **Capacity Checking:** Kiểm tra sức chứa tối đa (max_capacity) khi Cất hàng (Put-away)

---

## Trạng thái Giai đoạn (Phase Status)

**Phase 1 (MVP) - ĐANG PHÁT TRIỂN:**

- App mobile cho công nhân (Worker)
- Các hoạt động kho cốt lõi
- Kiến trúc Offline-first

**Phase 2 (Tương lai):**

- Desktop App quản lý (Tauri - cho chủ xưởng)
- Dashboard quản lý với báo cáo nâng cao
- **Hỗ trợ 2D Area Imager** (Professional Tier - Kho lớn 100+ công nhân)
- Tích hợp Cân điện tử & Máy in tem

**Phase 3 (Tương lai):**

- Đa kho (Multi-warehouse)
- Tính năng AI/ML
- Tích hợp ERP

Xem chi tiết lộ trình trong `doc/planning/ROADMAP.md` (trước đây là `WHITEPAPER_FUTURE_FEATURES.md`)

---

## Chiến lược Tài liệu Tương lai (Automated C4 Model)

Để đảm bảo tài liệu luôn đồng bộ với code (Living Documentation), dự án sẽ áp dụng chiến lược **"Architecture as Code"** để tự động sinh ra C4 Diagrams:

1. **Frontend (Expo/TS):** Sử dụng `madge` hoặc `dependency-cruiser` trong CI/CD pipeline để quét dependencies và sinh ra Component Diagrams (Level 3). - Command: `madge --extensions ts,tsx --image doc/arch/frontend.svg src/`
2. **Backend (Rust):** Sử dụng `cargo-modules` để visualize cấu trúc module và dependencies của Rust crates. - Command: `cargo modules graph > doc/arch/backend.dot`
3. **System Level (C4 Context/Container):** Sử dụng **Structurizr DSL** để định nghĩa High-level architecture. Kết hợp script quét code để auto-inject danh sách Components vào DSL, đảm bảo sơ đồ Level 1-2 luôn khớp với thực tế.

---

## Cấu trúc Tài liệu

```tree
├── PRD.md                              # Yêu cầu Sản phẩm (Product Manager)
├── README.md                           # Hướng dẫn bắt đầu nhanh
└── doc/
    ├── architecture/
    │   ├── BLUEPRINT_OVERVIEW.md       # File này (Tổng quan & Tham chiếu nhanh)
    │   ├── ARCHITECTURE.md             # Kiến trúc hệ thống & cấu trúc monorepo
    │   ├── BLUEPRINT_FRONTEND.md       # Đặc tả UI/UX (Designer)
    │   └── BLUEPRINT_BACKEND.md        # Đặc tả Kỹ thuật (Engineer)
    ├── guides/
    │   └── CONTRIBUTING.md             # 🚀 Hướng dẫn onboarding (Developer mới)
    ├── planning/
    │   └── ROADMAP.md                  # Lộ trình (Tương lai)
    └── illustration/
        ├── plantuml-diagram-ui.txt      # Biểu đồ PlantUML
        ├── plantuml-diagram-backend.txt # Biểu đồ kiến trúc Backend
        └── architecture-diagram.txt     # Biểu đồ kiến trúc Hệ thống
```

---

**📝 Ghi chú:** Để biết chi tiết về bất kỳ phần nào, vui lòng tham khảo các file chuyên biệt tương ứng được liệt kê ở trên.
