# LEAN WMS - Product Requirements Document (PRD)

**Version:** 1.0  
**Last Updated:** 2025-01-XX  
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

- Dashboard quản lý (web/mobile)
- Báo cáo nâng cao
- Multi-warehouse

### 2.2. Success Metrics

**Performance Metrics:**

- Tốc độ quét: < 500ms từ khi quét đến khi nhận diện
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

**US-001: Đăng nhập**
- **As a** công nhân  
- **I want to** đăng nhập bằng tài khoản
- **So that** tôi có thể sử dụng app và hệ thống ghi nhận hành động của tôi

**US-002: Nhập kho**
- **As a** công nhân  
- **I want to** quét mã hàng và nhập số lượng  
- **So that** hàng được nhập vào kho một cách nhanh chóng và chính xác

**US-003: Gán mã hàng lạ**
- **As a** công nhân  
- **I want to** gán mã vạch mới vào SKU có sẵn hoặc tạo SKU mới  
- **So that** tôi không cần in tem mới cho hàng đã có mã vạch sẵn

**US-004: Xuất kho theo đơn hàng**
- **As a** công nhân  
- **I want to** xem danh sách đơn hàng cần xử lý và được hướng dẫn đến vị trí cụ thể  
- **So that** tôi lấy đúng hàng, đúng số lượng, không cần hỏi thủ kho

**US-005: Kiểm kê**
- **As a** công nhân  
- **I want to** quét vị trí và nhập số lượng đếm được  
- **So that** hệ thống so sánh với sổ sách và phát hiện lệch

US-006: Sản xuất
- As a công nhân
- I want to tạo 1 sản phẩm mới với danh sách nguyên liệu và số lượng cần sản xuất
- So that tôi có thể sản xuất sản phẩm một cách nhanh chóng và chính xác

### 3.2. User Stories - Manager

**US-101: Tạo đơn hàng**
- **As a** chủ xưởng  
- **I want to** tạo đơn hàng xuất/nhập với danh sách sản phẩm  
- **So that** công nhân có thể thực hiện theo đơn

**US-102: Xem báo cáo tồn kho**
- **As a** chủ xưởng  
- **I want to** xem tồn kho theo sản phẩm và vị trí  
- **So that** tôi biết còn bao nhiêu hàng, ở đâu

**US-103: Duyệt điều chỉnh kiểm kê**
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
4. Nhập số lượng, hạn sử dụng (nếu có)
5. Quét mã vị trí kệ
6. Xác nhận → Hàng chuyển sang trạng thái STAGING

**Acceptance Criteria:**
- ✅ Quét được cả QR và Barcode
- ✅ Mapping mã vạch linh hoạt (nhiều mã → 1 SKU)
- ✅ Validation vị trí kệ
- ✅ Phản hồi tức thì (xanh/đỏ)

### 4.2. Outbound (Xuất kho)

**Description:** Guided workflow để công nhân xuất hàng theo đơn hàng với Poka-Yoke.

**User Flow:**
1. Bấm nút "XUẤT" → Chọn đơn hàng
2. App hiển thị hướng dẫn: "Đến kệ A1, ô số 3"
3. Bấm "Bắt đầu" → Quét mã vị trí
4. Validation vị trí → Quét mã hàng
5. Validation hàng/số lượng → Cập nhật tiến độ (5/10)
6. Hoàn thành khi đủ số lượng

**Acceptance Criteria:**
- ✅ Guided workflow (không để công nhân tự hỏi "làm gì tiếp?")
- ✅ Validation real-time (sai hàng/vị trí → màn hình đỏ ngay)
- ✅ Hiển thị tiến độ rõ ràng
- ✅ FEFO/FIFO: Ưu tiên lấy hàng cũ trước

### 4.3. Counting (Kiểm kê)

**Description:** Blind count để đảm bảo tính chính xác.

**User Flow:**
1. Bấm nút "KIỂM KHO"
2. Chọn khu vực cần kiểm kê
3. Quét mã vị trí Bin
4. Quét/Đếm hàng thực tế trong Bin
5. Nhập số lượng đếm được
6. Hệ thống so sánh với sổ sách → Hiển thị lệch (nếu có)

**Acceptance Criteria:**
- ✅ Blind count (KHÔNG hiển thị số lượng tồn kho hiện tại)
- ✅ Bắt buộc quét/đếm thực tế
- ✅ So sánh và flag lệch để Manager duyệt

### 4.4. Put-away (Cất hàng)

**Description:** Chuyển hàng từ STAGING sang AVAILABLE với validation vị trí.

**User Flow:**
1. Quét mã hàng/Container ở trạng thái STAGING
2. Quét mã vị trí đích
3. Validation (Bin có đủ chỗ? Phù hợp loại hàng?)
4. Xác nhận → Chuyển trạng thái STAGING → AVAILABLE

**Acceptance Criteria:**
- ✅ Chỉ quét được hàng ở trạng thái STAGING
- ✅ Validation Fixed Bin (nếu là Fixed Bin)
- ✅ Ghi nhận người cất, thời gian, vị trí

### 4.5. Offline-First & Sync

**Description:** App hoạt động offline, tự động sync khi có mạng.

**Behavior:**
- Dữ liệu được ghi NGAY vào Local DB (độ trễ = 0ms)
- UI cập nhật ngay (Optimistic UI)
- Action Queue được sync background mỗi 30 giây
- Conflict resolution: First Come First Served

**Acceptance Criteria:**
- ✅ Hoạt động bình thường khi mất mạng
- ✅ Sync tự động khi có mạng lại
- ✅ Xử lý conflict đúng cách
- ✅ Badge hiển thị trạng thái offline/online

---

## 5. TECHNICAL REQUIREMENTS

### 5.1. Platform & Compatibility

**Mobile App:**
- Android 8.0+ (Oreo)
- iOS 12.0+
- Camera: 8MP+, autofocus
- RAM: 2GB+
- Storage: 500MB+ free

**Backend:**
- RESTful API
- Database: PostgreSQL/MySQL
- Authentication: JWT

### 5.2. Performance Requirements

- Quét mã: < 500ms
- Phản hồi UI: < 100ms
- Ghi Local DB: < 50ms
- Sync batch: Mỗi 30 giây
- Offline capacity: 10,000+ actions

### 5.3. Security Requirements

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

### 6.2. Reliability

- **Uptime:** 99%+
- **Data integrity:** ACID transactions
- **Offline resilience:** Hoạt động 2 tuần offline

### 6.3. Scalability

- Hỗ trợ 100+ users đồng thời
- 10,000+ products
- 1,000+ locations

---

## 7. OUT OF SCOPE (Phase 1)

**Không bao gồm trong MVP:**

- ❌ Dashboard quản lý web (Phase 2)
- ❌ Báo cáo nâng cao (Phase 2)
- ❌ Multi-warehouse (Phase 3)
- ❌ AI/ML features (Phase 3)
- ❌ Integration với ERP (Phase 3)
- ❌ Cân điện tử integration (Phase 2)
- ❌ Máy in tem integration (Phase 2)

---

## 8. DEPENDENCIES & CONSTRAINTS

### 8.1. Dependencies

- Camera permission (required)
- Internet connection (optional, for sync)
- Server backend API (required for sync)

### 8.2. Constraints

- **Hardware:** Phải có camera (không hỗ trợ tablet không có camera)
- **Network:** Hoạt động offline, nhưng cần mạng để sync
- **Platform:** Không hỗ trợ web (chỉ mobile native)

---

## 9. RISKS & MITIGATION

### 9.1. Technical Risks

**Risk:** Camera không đọc được mã trong điều kiện ánh sáng yếu  
**Mitigation:** Tích hợp flash toggle, zoom control, auto-focus

**Risk:** Sync conflict khi 2 người cùng xuất hàng cuối  
**Mitigation:** First Come First Served, hiển thị lỗi rõ ràng

**Risk:** Local DB corruption  
**Mitigation:** Auto-detect corruption, force re-login và pull lại dữ liệu

### 9.2. User Adoption Risks

**Risk:** Công nhân không muốn dùng (phức tạp)  
**Mitigation:** UI cực đơn giản, training < 5 phút, phản hồi tức thì

**Risk:** Chủ xưởng không muốn setup (mất thời gian)  
**Mitigation:** Setup đơn giản, hướng dẫn rõ ràng, hỗ trợ onboarding

---

## 10. TIMELINE & MILESTONES

### Phase 1: MVP (3-4 months)

**Month 1-2: Foundation**
- Database schema
- Backend API (Auth, Sync)
- Mobile app core (Auth, Scanner)

**Month 3: Core Features**
- Inbound flow
- Outbound flow
- Put-away flow
- Counting flow

**Month 4: Polish & Test**
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

---

**Document Status:** ✅ Ready for Development (after completing missing backend specifications)

