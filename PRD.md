# LEAN WMS - Product Requirements Document (PRD)

**Version:** 1.0  
**Last Updated:** 04-01-2026  
**Status:** Phase 1 - MVP  
**Owner:** Product Team

---

## 1. EXECUTIVE SUMMARY

### 1.1. Product Vision

**Lean WMS** là hệ thống Quản lý Kho & Sản xuất Tinh gọn, đem quy trình chuẩn của nhà máy lớn (Samsung, Amazon) nén vào chiếc điện thoại di động cho hộ gia đình và xưởng nhỏ.

### 1.2. Problem Statement

Các kho hàng nhỏ và xưởng sản xuất đang gặp các vấn đề:

- **Không biết hàng ở đâu:** Vị trí sản phẩm không rõ, mất thời gian tìm kiếm
- **Lỗi nhập/xuất:** Công nhân nhập liệu bằng tay dễ sai sót, thiếu kiểm soát
- **Không theo dõi được:** Không biết ai lấy hàng, lúc nào, còn bao nhiêu
- **Thiết bị đắt tiền:** Giải pháp hiện tại yêu cầu phần cứng chuyên dụng tốn kém

### 1.3. Solution Overview

Ứng dụng mobile (Android/iOS) cho phép:

- **Quét QR/Barcode** bằng camera điện thoại (không cần thiết bị đặc biệt)
- **Offline-first:** Hoạt động khi mất mạng, tự động đồng bộ khi có mạng
- **Brainless operation:** Giao diện đơn giản, phản hồi bằng màu sắc/âm thanh
- **Chính xác:** Poka-Yoke (chống sai lỗi), validation real-time

### 1.4. Target Users

**Primary Users:**

- **Công nhân kho (Worker):** Quét mã, nhập/xuất hàng, kiểm kê
- **Chủ xưởng/Quản lý (Manager/Owner):** Tạo đơn hàng, xem báo cáo, phê duyệt

**Target Market:**

- Xưởng sản xuất nhỏ (5-50 công nhân)
- Kho hàng nhỏ và vừa
- Hộ gia đình làm thủ công

---

## 2. PRODUCT GOALS & SUCCESS METRICS

### 2.1. Product Goals

**Phase 1 (MVP) - App cho công nhân:**

1. Nhập kho (Inbound) với mapping mã vạch linh hoạt
2. Xuất kho (Outbound) với guided workflow
3. Kiểm kê (Counting) với blind count
4. Cất hàng (Put-away) với validation vị trí
5. Offline-first với sync tự động
6. Phản hồi tức thì (màu sắc, âm thanh, haptic)

**Phase 2 (Future):**

- Desktop App quản lý (Tauri - cho chủ xưởng)
- Dashboard quản lý với báo cáo nâng cao
- Multi-warehouse
- **2D Area Imager Support:** Tích hợp Bluetooth 2D Imager cho kho lớn (100+ công nhân) - Tốc độ quét <100ms, đọc barcode hỏng tốt hơn

### 2.2. Success Metrics

**Performance Metrics:**

- Tốc độ quét: < 500ms (Camera Phone) / < 100ms (2D Imager - Phase 2)
- Phản hồi UI: < 100ms
- Offline capacity: Lưu được 10,000+ actions

**User Metrics:**

- Thời gian training: < 5 phút cho công nhân mới
- Tỷ lệ lỗi quét: < 1%
- Tỷ lệ sync thành công: > 99%

**Business Metrics:**

- Giảm thời gian tìm hàng: 50%
- Giảm lỗi nhập/xuất: 80%
- Tăng độ chính xác tồn kho: 95%+

---

## 3. USER STORIES & FEATURES

### 3.1. User Stories - Worker

- **US-001: Đăng nhập**

- **As a** công nhân
- **I want to** đăng nhập bằng tài khoản
- **So that** tôi có thể sử dụng app và hệ thống ghi nhận hành động của tôi

- **US-002: Nhập kho**

- **As a** công nhân
- **I want to** quét mã hàng và nhập số lượng
- **So that** hàng được nhập vào kho một cách nhanh chóng và chính xác

- **US-003: Gán mã hàng lạ (Hybrid Creation)**

- **As a** công nhân
- **I want to** gán mã vạch mới vào SKU có sẵn hoặc tạo SKU mới (nhập tay hoặc tự sinh mã)
- **So that** tôi không cần in tem mới cho hàng đã có mã vạch sẵn và xử lý nhanh hàng chưa có mã

- **US-004: Xuất kho theo đơn hàng**

- **As a** công nhân
- **I want to** xem danh sách đơn hàng cần xử lý và được hướng dẫn đến vị trí cụ thể
- **So that** tôi lấy đúng hàng, đúng số lượng, không cần hỏi thủ kho

- **US-005: Kiểm kê**

- **As a** công nhân
- **I want to** quét vị trí và nhập số lượng đếm được
- **So that** hệ thống so sánh với sổ sách và phát hiện lệch

- **US-006: Sản xuất**

- **As a** công nhân
- **I want to** tạo 1 sản phẩm mới với danh sách nguyên liệu và số lượng cần sản xuất (hỗ trợ trừ kho sau - Backflush)
- **So that** tôi có thể sản xuất sản phẩm một cách nhanh chóng và linh hoạt theo thực tế xưởng

### 3.2. User Stories - Manager

- **US-101: Tạo đơn hàng**

- **As a** chủ xưởng
- **I want to** tạo đơn hàng xuất/nhập với danh sách sản phẩm
- **So that** công nhân có thể thực hiện theo đơn

- **US-102: Xem báo cáo tồn kho**

- **As a** chủ xưởng
- **I want to** xem tồn kho theo sản phẩm và vị trí
- **So that** tôi biết còn bao nhiêu hàng, ở đâu

- **US-103: Duyệt điều chỉnh kiểm kê**

- **As a** chủ xưởng
- **I want to** xem và duyệt các điều chỉnh sau khi kiểm kê
- **So that** tôi kiểm soát được các thay đổi về số lượng tồn kho

---

## 4. CORE FEATURES

### 4.1. Inbound (Nhập kho)

**Description:** Cho phép công nhân nhập hàng vào kho với mapping mã vạch linh hoạt.

**User Flow:**

1. Bấm nút "NHẬP" trên Dashboard
2. Quét mã hàng (QR/Barcode)
3. Nếu mã chưa có mapping → Gán vào SKU có sẵn hoặc tạo SKU mới
   - **Hybrid Creation:** Cho phép nhập SKU thủ công HOẶC bấm nút "Tự sinh mã" (Auto-generate)
4. Nhập số lượng, hạn sử dụng (nếu có)
5. Quét mã vị trí Nhận hàng (Receiving/Staging Location)
   - **Flexible Location:** Nếu kho nhỏ không quản lý vị trí chi tiết, hệ thống tự gán vị trí mặc định
6. Xác nhận → Hàng chuyển sang trạng thái STAGING

**Acceptance Criteria:**

- ✅ Quét được cả QR và Barcode
- ✅ Mapping mã vạch linh hoạt (nhiều mã → 1 SKU)
- ✅ **Hybrid SKU Generation (Manual Input + Auto-gen)**
- ✅ Validation vị trí kệ (hỗ trợ bỏ qua cho xưởng nhỏ)
- ✅ Phản hồi tức thì (xanh/đỏ)
- ✅ Phản hồi âm thanh: "Tít" (thành công), "Bíp bíp" (lỗi)
- ✅ Phản hồi rung (haptic): Rung nhẹ (thành công), rung mạnh (lỗi)

### 4.2. Outbound (Xuất kho)

**Description:** Guided workflow để công nhân xuất hàng theo đơn hàng với Poka-Yoke.

**User Flow:**

1. Bấm nút "XUẤT" → Chọn đơn hàng
2. App hiển thị hướng dẫn: "Đến kệ A1, ô số 3" (hoặc chỉ hiện tên hàng nếu không quản lý vị trí)
3. Bấm "Bắt đầu" → Quét mã vị trí (Optional nếu cấu hình tắt)
4. Validation vị trí → Quét mã hàng
5. Validation hàng/số lượng → Cập nhật tiến độ (5/10)
   - **Manager Override:** Cho phép Quản lý nhập mã PIN/xác nhận để bỏ qua validation trong trường hợp khẩn cấp (có log lại)
6. Hoàn thành khi đủ số lượng

**Acceptance Criteria:**

- ✅ Guided workflow (không để công nhân tự hỏi "làm gì tiếp?")
- ✅ Validation real-time (sai hàng/vị trí → màn hình đỏ ngay)
- ✅ **Manager Override:** Cơ chế "bẻ khóa" an toàn cho quản lý
- ✅ Hiển thị tiến độ rõ ràng
- ✅ FEFO/FIFO: Ưu tiên lấy hàng cũ trước
- ✅ Phản hồi âm thanh: "Tít" (thành công), "Bíp bíp" (lỗi)
- ✅ Phản hồi rung (haptic): Rung nhẹ (thành công), rung mạnh (lỗi)

### 4.3. Counting (Kiểm kê)

**Description:** Blind count để đảm bảo tính chính xác.

**User Flow:**

1. Bấm nút "KIỂM KHO"
2. Chọn khu vực cần kiểm kê
3. Quét mã vị trí Bin
4. Quét/Đếm hàng thực tế trong Bin
5. Nhập số lượng đếm được
6. Hệ thống so sánh với sổ sách → Hiển thị lệch (nếu có)
   - Ghi nhận lịch sử điều chỉnh (Audit Log) chứ không ghi đè số lượng

**Acceptance Criteria:**

- ✅ Blind count (KHÔNG hiển thị số lượng tồn kho hiện tại)
- ✅ Bắt buộc quét/đếm thực tế
- ✅ **Double Entry/Audit Log:** Ghi nhận giao dịch điều chỉnh (Adjustment)
- ✅ So sánh và flag lệch để Manager duyệt
- ✅ Phản hồi âm thanh: "Tít" (thành công), "Bíp bíp" (lỗi)
- ✅ Phản hồi rung (haptic): Rung nhẹ (thành công), rung mạnh (lỗi)

### 4.4. Put-away (Cất hàng)

**Description:** Chuyển hàng từ STAGING sang AVAILABLE với validation vị trí.

**User Flow:**

1. Quét mã hàng/Container ở trạng thái STAGING
2. Quét mã vị trí đích
3. Validation (Bin có đủ chỗ? Phù hợp loại hàng?)
   - **Visual Fullness Flag:** Cho phép công nhân đánh dấu "Bin đầy" bằng nút bấm nếu không đo được thể tích
4. Xác nhận → Chuyển trạng thái STAGING → AVAILABLE

**Acceptance Criteria:**

- ✅ Chỉ quét được hàng ở trạng thái STAGING
- ✅ Validation Fixed Bin (nếu là Fixed Bin)
- ✅ **Visual Capacity:** Nút "Bin đầy" để chặn cất thêm hàng
- ✅ Ghi nhận người cất, thời gian, vị trí
- ✅ Phản hồi âm thanh: "Tít" (thành công), "Bíp bíp" (lỗi)
- ✅ Phản hồi rung (haptic): Rung nhẹ (thành công), rung mạnh (lỗi)

### 4.5. Offline-First & Sync

**Description:** App hoạt động offline, tự động sync khi có mạng.

**Behavior:**

- Dữ liệu được ghi NGAY vào Local DB (độ trễ = 0ms)
- UI cập nhật ngay (Optimistic UI)
- Action Queue được sync background mỗi 30 giây
- Conflict resolution: Chiến lược phân biệt theo loại dữ liệu

**Conflict Resolution Strategy:**

- **Last Write Wins (LWW):** Áp dụng cho dữ liệu vị trí (location), metadata, và thông tin mô tả
  - Ví dụ: Khi 2 công nhân cùng cập nhật vị trí hàng → Giá trị cuối cùng ghi đè
  - Timestamp từ server là source of truth
- **CRDT (Conflict-free Replicated Data Types):** Áp dụng cho số lượng tồn kho (inventory quantities)
  - Khi 2 công nhân cùng quét 1 mã → Cộng dồn số lượng thay vì ghi đè
  - Tránh mất dữ liệu khi có xung đột đồng thời
  - Ví dụ: Worker A quét +10, Worker B quét +5 cùng lúc → Kết quả: +15 (không mất dữ liệu)

**Acceptance Criteria:**

- ✅ Hoạt động bình thường khi mất mạng
- ✅ Sync tự động khi có mạng lại
- ✅ Xử lý conflict đúng cách (LWW cho location, CRDT cho quantity)
- ✅ Không mất dữ liệu số lượng khi có xung đột đồng thời
- ✅ Badge hiển thị trạng thái offline/online

---

## 5. TECHNICAL REQUIREMENTS

### 5.1. Technology Stack

| Thành phần           | Lựa chọn          | Tại sao?                                                                 |
| -------------------- | ----------------- | ------------------------------------------------------------------------ |
| **Mobile App**       | Expo              | Tận dụng thư viện Camera/Scanner tốt nhất cho WMS.                       |
| **State Management** | Redux Toolkit     | Quản lý trạng thái ứng dụng một cách hiệu quả và nhất quán.              |
| **Logic Core**       | Rust              | Viết các hàm Functional xử lý tồn kho, validation để dùng chung mọi nơi. |
| **Desktop App**      | Tauri (Rust)      | App quản lý cho chủ xưởng mượt, nhẹ, bảo mật cao.                        |
| **Backend Server**   | Rust (Axum/Actix) | API trung tâm, tái sử dụng Logic Core, xử lý Multi-tenant.               |
| **Sync Protocol**    | WebSockets/NATS   | Đảm bảo tính real-time khi có mạng lại.                                  |

**Chi tiết Implementation:**

**Mobile App (Expo):**

- Framework: Expo với TypeScript
- State Management: Redux Toolkit / Zustand
- Navigation: React Navigation
- Camera/Scanner:
  - Phase 1: react-native-vision-camera + react-native-vision-camera-code-scanner (Camera Phone)
  - Phase 2: Bluetooth HID Scanner support (2D Area Imager) - Auto-detect và fallback
- Network: Axios / Fetch với retry logic
- Storage: AsyncStorage / SecureStore
- **Internationalization (i18n):** Default 'vi-VN', fallback 'en-US'

**State Management (Redux Toolkit):**

- Reactive database với lazy loading
- Tối ưu cho offline-first architecture
- Hỗ trợ sync conflict resolution
- Performance cao với 10,000+ records

**Logic Core (Rust):**

- Shared business logic giữa Mobile, Desktop và Server
- Compile thành native modules (FFI) cho React Native
- Type-safe và performance cao
- Validation rules, inventory calculations, FEFO/FIFO algorithms

**Desktop App (Tauri):**

- Frontend: React/Vue (web technologies)
- Backend: Rust core (shared với mobile)
- Bundle size nhỏ, bảo mật cao
- Native performance

**Sync Protocol:**

- **WebSockets:** Real-time bidirectional communication
- **NATS:** Message queue cho sync batching và reliability
- Fallback: REST API cho initial sync và compatibility

### 5.2. Platform & Compatibility

**Mobile App:**

- Android 8.0+ (Oreo)
- iOS 12.0+
- Camera: 8MP+, autofocus (Phase 1 - Camera Phone)
- Bluetooth: 4.0+ (Phase 2 - 2D Imager support)
- RAM: 2GB+
- Storage: 500MB+ free

**Desktop App (Phase 2):**

- Windows 10+
- macOS 10.15+
- Linux (Ubuntu 20.04+)

**Backend:**

- RESTful API + WebSockets
- Database: PostgreSQL/MySQL
- Authentication: JWT với refresh token
- Message Queue: NATS
- **Server Runtime:** Rust (High performance, Memory safety)

### 5.3. Performance Requirements

**Phase 1 (Camera Phone):**

- Quét mã: < 500ms
- Phản hồi UI: < 100ms
- Ghi Local DB: < 50ms
- Sync batch: Mỗi 30 giây
- Offline capacity: 10,000+ actions

**Phase 2 (2D Area Imager - Professional Tier):**

- **Thiết bị:** Phải mua 2D Area Imager riêng ($200-400/thiết bị) - Kết nối với điện thoại qua Bluetooth
- **Điện thoại:** Vẫn là thiết bị chính chạy app, 2D Imager chỉ là thiết bị quét ngoại vi
- Quét mã: < 100ms (5x nhanh hơn camera phone)
- Đọc barcode hỏng: Tốt hơn camera phone
- Hoạt động trong ánh sáng yếu: Tốt hơn camera phone
- Auto-detect và fallback: App tự động detect 2D Imager nếu có → Dùng nó, nếu không → Dùng camera phone

### 5.4. Security Requirements

- JWT authentication với refresh token
- Device binding
- Secure storage (Keychain/Keystore)
- HTTPS only
- Input validation & sanitization

---

## 6. NON-FUNCTIONAL REQUIREMENTS

### 6.1. Usability

- **Training time:** < 5 phút cho công nhân mới
- **Error rate:** < 1%
- **Accessibility:** High contrast, lớn font, hỗ trợ screen reader
- **Internationalization (i18n):**
  - Ngôn ngữ mặc định: Tiếng Việt (vi-VN)
  - Ngôn ngữ phụ: Tiếng Anh (en-US)
  - Hỗ trợ chuyển đổi ngôn ngữ trong cài đặt
- **Multi-modal feedback:**
  - Âm thanh: "Tít" (thành công, tần số cao, ngắn 200ms), "Bíp bíp" (lỗi, tần số thấp, dài 800ms)
  - Haptic: Rung nhẹ 100ms (thành công), rung mạnh 500ms (lỗi)
  - Visual: Màn hình xanh (#00FF00) 500ms (thành công), đỏ (#FF0000) 1000ms (lỗi)
- **Noise-resistant design:** Âm thanh và haptic feedback quan trọng hơn màu sắc trong môi trường kho ồn

### 6.2. Reliability

- **Uptime:** 99%+
- **Data integrity:** ACID transactions
- **Offline resilience:** Hoạt động 2 tuần offline

### 6.3. Scalability

- Hỗ trợ 100+ users đồng thời
- 10,000+ products
- 1,000+ locations
- **Multi-tenancy:** Hệ thống Server hỗ trợ nhiều tenant (xưởng) khác nhau, dữ liệu biệt lập

---

## 7. OUT OF SCOPE (Phase 1)

**Không bao gồm trong MVP:**

- ❌ Desktop App quản lý (Tauri) (Phase 2)
- ❌ Báo cáo nâng cao (Phase 2)
- ❌ Multi-warehouse (Phase 3)
- ❌ AI/ML features (Phase 3)
- ❌ Integration với ERP (Phase 3)
- ❌ Cân điện tử integration (Phase 2)
- ❌ Máy in tem integration (Phase 2)
- ❌ 2D Area Imager support (Phase 2 - Professional Tier)
- ❌ Isometric UI 3D (Nice to have, not MVP)

---

## 8. DEPENDENCIES & CONSTRAINTS

### 8.1. Dependencies

**Phase 1 (MVP):**

- Camera permission (required)
- Internet connection (optional, for sync)
- Server backend API (required for sync)
- **External Orders:** Middle Layer (Order Aggregator) để map đơn từ Shopee/TikTok (nếu có)

**Phase 2 (Professional Tier):**

- **Điện thoại:** Vẫn là thiết bị chính chạy app (không thay đổi)
- **Bluetooth permission:** Cần để kết nối với 2D Imager (optional)
- **2D Area Imager:** Thiết bị ngoại vi riêng (phải mua, $200-400/thiết bị)
  - Đề xuất: Zebra DS2208 ($200-300), Honeywell CT60 ($250-350), Datalogic QuickScan ($200-280)
  - Kết nối với điện thoại qua Bluetooth
  - Hỗ trợ Bluetooth HID (keyboard mode) - Không cần driver đặc biệt
  - Đọc QR Code + 2D barcode (Data Matrix, PDF417, Aztec, etc.)
  - **Lưu ý:** Nếu không có 2D Imager, app vẫn hoạt động bình thường với camera phone

### 8.2. Constraints

- **Hardware:** Phải có camera (không hỗ trợ tablet không có camera)
- **Network:** Hoạt động offline, nhưng cần mạng để sync
- **Platform:** Không hỗ trợ web (chỉ mobile native)
- **Flexible Location Granularity:** Hỗ trợ cấu hình mức độ quản lý vị trí (Kho lớn: Chi tiết Bin / Kho nhỏ: Vị trí chung)

---

## 9. RISKS & MITIGATION

### 9.1. Technical Risks

**Risk:** Camera không đọc được mã trong điều kiện ánh sáng yếu  
**Mitigation:** Tích hợp flash toggle, zoom control, auto-focus

**Risk:** Sync conflict khi 2 người cùng xuất hàng cuối  
**Mitigation:** First Come First Served, hiển thị lỗi rõ ràng

**Risk:** Local DB corruption  
**Mitigation:** Auto-detect corruption, force re-login và pull lại dữ liệu

**Risk:** Poka-Yoke quá cứng nhắc, chặn đứng hoạt động khi có ngoại lệ  
**Mitigation:** **Manager Override:** Cho phép quản lý "bẻ khóa" (nhập mã PIN) để tiếp tục, hệ thống ghi log warning.

### 9.2. User Adoption Risks

**Risk:** Công nhân không muốn dùng (phức tạp)  
**Mitigation:** UI cực đơn giản, training < 5 phút, phản hồi tức thì

**Risk:** Chủ xưởng không muốn setup (mất thời gian)  
**Mitigation:** Setup đơn giản, hướng dẫn rõ ràng, hỗ trợ onboarding

---

## 10. TIMELINE & MILESTONES

### Phase 1: MVP (3-4 months)

- **Month 1-2: Foundation**

- Database schema
- Backend API (Auth, Sync)
- Mobile app core (Auth, Scanner)

- **Month 3: Core Features**

- Inbound flow
- Outbound flow
- Put-away flow
- Counting flow

- **Month 4: Polish & Test**

- Offline-first implementation
- Sync mechanism
- Testing & bug fixes
- Beta testing với users thật

---

## 11. APPENDIX

### 11.1. Related Documents

- `doc/BLUEPRINT.md` - Master document (technical overview)
- `doc/BLUEPRINT_DESIGNER.md` - UI/UX specifications
- `doc/BLUEPRINT_ENGINEER.md` - Technical specifications
- `doc/WHITEPAPER_FUTURE_FEATURES.md` - Phase 2+ roadmap

### 11.2. Glossary

- **SKU:** Stock Keeping Unit (Mã sản phẩm)
- **LPN:** License Plate Number (Mã Container)
- **FEFO:** First Expired, First Out
- **FIFO:** First In, First Out
- **Poka-Yoke:** Chống sai lỗi
- **STAGING:** Trạng thái chờ cất
- **AVAILABLE:** Trạng thái sẵn sàng xuất
- **2D Area Imager:** Thiết bị quét 2D chuyên dụng, đọc QR Code và 2D barcode, kết nối qua Bluetooth HID

---

**Document Status:** ✅ Ready for Development (after completing missing backend specifications)
