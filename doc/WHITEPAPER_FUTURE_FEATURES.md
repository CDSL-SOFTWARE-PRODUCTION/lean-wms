# LEAN ERP/WMS - WHITEPAPER: EVOLUTION ROADMAP

## Từ Kho hàng tinh gọn đến Hệ quản trị doanh nghiệp thông minh

**Version:** 3.0  
**Last Updated:** 09-01-2026  
**Status:** Strategic Vision Document  
**Audience:** Product Managers, Business Stakeholders, Engineering Leads

---

## 1. TẦM NHÌN CHIẾN LƯỢC (Strategic Vision)

### 1.1. Product Evolution Path

**Lean WMS** không chỉ là một hệ thống quản lý kho. Đây là lộ trình tiến hóa từ **WMS (Warehouse Management System)** thành **ERP (Enterprise Resource Planning)** hoàn chỉnh, được xây dựng trên nền tảng **Functional Architecture** và **AI-driven Intelligence**.

### 1.2. Business Model Strategy

- **Giai đoạn 1: Thu thập dữ liệu (Free Tier)**

- Cung cấp công cụ WMS miễn phí để số hóa mọi biến động vật lý trong kho
- Thu thập dữ liệu thực tế từ hàng nghìn doanh nghiệp nhỏ
- Xây dựng cộng đồng người dùng và tích lũy domain knowledge

- **Giai đoạn 2: Monetization (Paid Tiers)**

- **Professional Tier:** Thu phí các module quản trị nâng cao và khả năng đồng bộ đa thiết bị
- **Enterprise Tier:** Thu phí module Kế toán (Financial ERP) và Trợ lý ảo AI (RAG Agent)
- Tạo giá trị từ dữ liệu đã thu thập: AI insights, predictive analytics, automation

### 1.3. Evolution Framework

Lộ trình phát triển được tổ chức theo 3 trục chính:

| Trục                       | Mô tả                                         | Phase     | Business Value              |
| -------------------------- | --------------------------------------------- | --------- | --------------------------- |
| **Operational (Vận hành)** | Số hóa quy trình vật lý trong kho và sản xuất | Phase 1-2 | Tăng hiệu quả, giảm lỗi     |
| **Financial (Tài chính)**  | Chuyển hóa dữ liệu kho thành dữ liệu tiền tệ  | Phase 3   | Quản trị tài chính, báo cáo |
| **Intelligence (Trí tuệ)** | AI-driven insights và tự động hóa             | Phase 4   | Dự đoán, tối ưu, tự động    |

---

## 2. QUICK REFERENCE (Tóm tắt nhanh)

### 2.1. Roadmap Overview

| Phase       | Tên                  | Trọng tâm                   | Pricing | Timeline           |
| ----------- | -------------------- | --------------------------- | ------- | ------------------ |
| **Phase 1** | Mobile-First WMS     | Operational Core            | 🆓 Free | ✅ Đang phát triển |
| **Phase 2** | Operational ERP      | Mở rộng quy trình nghiệp vụ | 🆓 Free | 📋 Kế hoạch        |
| **Phase 3** | Financial ERP & Sync | Quản trị tài chính          | 💰 Paid | 🔮 Tương lai       |
| **Phase 4** | AI-Driven ERP        | Trí tuệ nhân tạo            | 💰 Paid | 🔮 Tương lai       |

### 2.2. Enterprise Process Mapping

**Đã có trong Phase 1:**

- ✅ Receiving/Inbound → Inbound flow
- ✅ Directed Put-away → Put-away với validation
- ✅ Order Picking → Guided Outbound workflow
- ✅ Cycle Counting → Blind Count
- ✅ Container/LPN Tracking → Container management
- ✅ Batch/Lot Tracking → Batch_no, expiry_date
- ✅ FEFO/FIFO → FEFO/FIFO algorithms

**Sẽ thêm trong Phase 2:**

- 🆕 Quality Control Workflow → QC Inspection với Quarantine
- 🆕 Replenishment → Chuyển hàng reserve → forward pick
- 🆕 Production Management → BOM, Work Orders, Production Tracking
- 🆕 Procurement & Sales → Purchase Orders, Sales Orders

**Sẽ thêm trong Phase 3:**

- 💰 Financial Accounting → General Ledger, AP/AR, Costing
- 💰 Cross-device Sync → Real-time sync giữa các chi nhánh

**Sẽ thêm trong Phase 4:**

- 💰 AI RAG Agents → Semantic search, Auto-auditor, Predictive ordering

### 2.3. Tài liệu liên quan

- **Product Team:** Xem `PRD.md` cho product requirements chi tiết
- **Frontend Engineers:** Xem `doc/BLUEPRINT_DESIGNER_FRONTEND.md` cho UI/UX specs
- **Backend Engineers:** Xem `doc/BLUEPRINT_ENGINEER_BACKEND.md` cho technical specs và business logic

---

## 3. PHASE 1: MOBILE-FIRST WMS (Cốt lõi vận hành) - ✅ ĐANG THỰC HIỆN

### 3.1. Mục tiêu

Số hóa mọi biến động vật lý trong kho bằng thiết bị di động. Đây là nền tảng để thu thập dữ liệu thực tế từ hàng nghìn doanh nghiệp nhỏ.

### 3.2. Tính năng đã có

- ✅ Quét mã QR/Barcode bằng camera điện thoại
- ✅ Nhập kho (Inbound) với mapping mã vạch linh hoạt
- ✅ Xuất kho (Outbound) với guided workflow và Poka-Yoke
- ✅ Kiểm kê (Counting) với blind count
- ✅ Cất hàng (Put-away) với validation vị trí
- ✅ Client-side Architecture (Redux Toolkit + Rust Functional Core)
- ✅ Sync dữ liệu tự động khi có mạng
- ✅ Phản hồi đa phương thức (âm thanh, haptic, visual)

### 3.3. Giới hạn Phase 1

- ❌ Chưa có Dashboard quản lý web-based
- ❌ Chưa có quản lý sản xuất (MES)
- ❌ Chưa có quản lý mua hàng/bán hàng
- ❌ Chưa có module kế toán
- ❌ Chưa có AI features

### 3.4. Business Value

**Cho người dùng:**

- Giảm 50% thời gian tìm hàng
- Giảm 80% lỗi nhập/xuất
- Tăng độ chính xác tồn kho lên 95%+

**Cho sản phẩm:**

- Thu thập dữ liệu thực tế từ hàng nghìn kho hàng
- Xây dựng cộng đồng người dùng miễn phí
- Tạo nền tảng dữ liệu cho AI/ML trong tương lai

---

## 4. PHASE 2: OPERATIONAL ERP (Mở rộng quy trình nghiệp vụ) - 📋 KẾ HOẠCH

### 4.1. Mục tiêu

Chuyển từ "Quản lý hàng" sang "Quản lý dòng việc". Đây là bước đệm quan trọng để nâng cấp lên ERP, mở rộng quy trình nghiệp vụ ra ngoài phạm vi kho hàng.

### 4.2. Module Sản xuất (MES Lite) - Nâng cấp từ US-006

**Mục đích:** Quản lý quy trình sản xuất từ nguyên liệu đến thành phẩm, tích hợp chặt chẽ với WMS.

**Tính năng:**

- **BOM (Bill of Materials):**
  - Định mức nguyên vật liệu cho 1 sản phẩm
  - Quản lý nhiều phiên bản BOM (theo thời gian, theo đơn hàng)
  - Tính toán tự động số lượng nguyên liệu cần thiết

- **Work Orders (Lệnh sản xuất):**
  - Tạo lệnh sản xuất dựa trên tồn kho nguyên liệu hiện tại
  - Kiểm tra tự động: Đủ nguyên liệu để sản xuất không?
  - Tự động trừ nguyên liệu khi bắt đầu sản xuất
  - Tự động cộng thành phẩm khi hoàn thành

- **Production Tracking:**
  - Ghi nhận sản lượng theo ca/công nhân
  - Theo dõi tiến độ sản xuất real-time
  - Báo cáo hiệu quả sản xuất (tỷ lệ hoàn thành, thời gian)

**User Flow:**

1. Manager tạo Work Order từ Dashboard: "Sản xuất 100 sản phẩm A"
2. Hệ thống kiểm tra: "Cần 50kg nguyên liệu X, 30kg nguyên liệu Y"
3. Nếu đủ nguyên liệu → Tự động tạo Outbound Order để lấy nguyên liệu
4. Worker thực hiện Outbound để lấy nguyên liệu (theo workflow Phase 1)
5. Worker bắt đầu sản xuất → Quét mã Work Order → Hệ thống trừ nguyên liệu
6. Worker hoàn thành → Quét mã thành phẩm → Hệ thống cộng thành phẩm vào kho

**Business Value:**

- Tự động hóa quy trình sản xuất, giảm lỗi thủ công
- Theo dõi chính xác chi phí nguyên liệu
- Tối ưu hóa sử dụng nguyên liệu, giảm lãng phí

### 4.3. Module Mua hàng & Bán hàng (Procurement & Sales)

**Mục đích:** Quản lý quan hệ với nhà cung cấp và khách hàng, tích hợp trực tiếp với Inbound/Outbound.

**Tính năng:**

- **Purchase Orders (PO - Đơn mua hàng):**
  - Tạo đơn mua hàng từ nhà cung cấp
  - Quản lý danh sách nhà cung cấp (Supplier Management)
  - Tự động tạo Inbound Order khi nhận hàng từ nhà cung cấp
  - Theo dõi trạng thái đơn mua (Pending, Received, Paid)

- **Sales Orders (SO - Đơn bán hàng):**
  - Tạo đơn bán hàng cho khách hàng
  - Quản lý danh sách khách hàng (Customer Management)
  - Tự động tạo Outbound Order khi cần xuất hàng
  - Theo dõi trạng thái đơn bán (Pending, Picked, Shipped, Delivered)

- **Price Management:**
  - Quản lý bảng giá linh hoạt cho từng nhóm khách hàng
  - Giá theo số lượng (volume discount)
  - Giá theo thời gian (promotion pricing)
  - Tự động tính giá khi tạo Sales Order

**User Flow - Purchase Order:**

1. Manager tạo PO: "Mua 100kg nguyên liệu X từ nhà cung cấp ABC"
2. Khi nhận hàng → Manager xác nhận "Đã nhận hàng"
3. Hệ thống tự động tạo Inbound Order
4. Worker thực hiện Inbound theo workflow Phase 1

**User Flow - Sales Order:**

1. Manager tạo SO: "Bán 50 sản phẩm A cho khách hàng XYZ"
2. Hệ thống tự động tạo Outbound Order
3. Worker thực hiện Outbound theo workflow Phase 1
4. Khi xuất hàng xong → Manager cập nhật "Đã giao hàng"

**Business Value:**

- Tự động hóa quy trình mua/bán, giảm thời gian xử lý
- Theo dõi công nợ nhà cung cấp/khách hàng (tiền đề cho Phase 3)
- Tích hợp liền mạch với WMS, không cần nhập liệu thủ công

### 4.4. Quality Control Workflow (Quy trình Kiểm soát Chất lượng)

**Mục đích:** Quy trình kiểm tra chất lượng hàng hóa chuyên nghiệp với workflow rõ ràng, đảm bảo hàng lỗi được xử lý đúng cách.

**Tính năng:**

- **Tạo lệnh QC:**
  - Manager tạo QC Order từ Dashboard
  - Chọn hàng cần kiểm tra (theo batch, location, hoặc SKU cụ thể)
  - Gán cho QC Inspector (worker có quyền QC)

- **QC Inspection trên Mobile App:**
  - QC Inspector chọn lệnh QC từ danh sách
  - Guided workflow:
    1. Quét mã hàng/batch cần kiểm tra
    2. Nhập kết quả kiểm tra:
       - **PASS:** Hàng đạt chất lượng → Chuyển về AVAILABLE
       - **FAIL:** Hàng lỗi → Chuyển sang QUARANTINE
       - **PARTIAL:** Một phần đạt, một phần lỗi → Tách thành 2 lots
    3. Nếu FAIL/PARTIAL: Nhập số lượng lỗi, mô tả lỗi, hình ảnh (optional)
    4. Xác nhận kết quả

- **Quarantine Management:**
  - Hàng ở trạng thái QUARANTINE không thể xuất kho
  - Dashboard hiển thị danh sách hàng trong quarantine
  - Manager có thể quyết định Disposition:
    - **SCRAP:** Hủy hàng → Trừ kho, ghi nhận tổn thất
    - **RETURN:** Trả về nhà cung cấp
    - **REWORK:** Sửa chữa → Chuyển về quy trình sản xuất
    - **RELEASE:** Sau khi sửa, kiểm tra lại và release về AVAILABLE

- **Báo cáo QC:**
  - Tỷ lệ hàng đạt/không đạt theo SKU, nhà cung cấp
  - Top lỗi thường gặp
  - Chi phí tổn thất (nếu có giá trị)

**Business Value:**

- Đảm bảo chất lượng hàng hóa, tuân thủ quy định
- Giảm thiểu tổn thất do hàng lỗi
- Cung cấp dữ liệu để đánh giá nhà cung cấp

### 4.5. Replenishment (Bổ sung hàng)

**Mục đích:** Tự động chuyển hàng từ khu vực lưu trữ dài hạn (reserve storage) sang khu vực lấy hàng (forward pick locations) khi forward pick sắp hết hàng.

**Tính năng:**

- **Cảnh báo Replenishment:**
  - Tự động phát hiện khi forward pick location sắp hết hàng (dưới ngưỡng min_level)
  - Dashboard hiển thị danh sách locations cần bổ sung
  - Manager có thể set min_level và max_level cho mỗi location

- **Tạo lệnh Replenishment:**
  - Manager tạo Replenishment Order từ Dashboard
  - Hệ thống tự động suggest: Từ location nào (reserve) → Đến location nào (forward pick)
  - Manager có thể chỉnh sửa hoặc chấp nhận suggestion

- **Thực hiện trên Mobile App (Worker):**
  - Worker chọn lệnh Replenishment từ danh sách đơn hàng
  - Guided workflow tương tự Outbound:
    1. App hướng dẫn: "Đến kệ B5 (reserve) để lấy hàng"
    2. Quét mã vị trí nguồn (reserve location)
    3. Quét mã hàng/số lượng
    4. App hướng dẫn: "Cất vào kệ A2, ô số 1 (forward pick)"
    5. Quét mã vị trí đích (forward pick location)
    6. Validation và xác nhận
  - Hệ thống tự động trừ kho reserve, cộng kho forward pick

**Business Value:**

- Đảm bảo forward pick locations luôn có đủ hàng
- Giảm thời gian di chuyển trong quá trình picking
- Tăng hiệu quả hoạt động kho

### 4.6. Management Dashboard (Web-based)

**Mục đích:** Cung cấp công cụ quản lý cho Chủ xưởng/Quản lý, không phụ thuộc vào app mobile.

**Tính năng:**

- **Quản lý Sản phẩm (SKU Management):**
  - CRUD SKU (Tạo, Sửa, Xóa, Xem)
  - Quản lý Mapping mã vạch
  - Import/Export từ Excel
  - Audit log mọi thay đổi

- **Quản lý Tồn kho (Inventory Management):**
  - Xem tổng quan: Tổng SKU, Tổng giá trị, Hàng sắp hết, Hàng hết hạn
  - Xem chi tiết: Theo SKU, Location, Batch
  - Điều chỉnh số lượng (Adjust) với approval workflow

- **Quản lý Đơn hàng (Order Management):**
  - Tạo và theo dõi đơn hàng (Sales, Purchase, Production, Transfer)
  - Duyệt đơn hàng quan trọng
  - Xem tiến độ real-time

- **Báo cáo (Reporting):**
  - Báo cáo tồn kho, xuất nhập, lỗi
  - Export Excel/PDF
  - Lên lịch gửi báo cáo tự động (Email)

- **Quản lý Người dùng (User Management):**
  - Tạo tài khoản, phân quyền
  - Theo dõi hoạt động và năng suất

**Business Value:**

- Quản lý tập trung, không cần phụ thuộc mobile
- Báo cáo và phân tích để ra quyết định
- Quản lý người dùng và phân quyền

### 4.6. 2D Area Imager Support (Professional Tier - Phase 2)

**Mục đích:** Mở rộng sang Segment 2 (Kho lớn 100+ công nhân) với tốc độ quét nhanh hơn và độ chính xác cao hơn, trong khi vẫn giữ nguyên core architecture.

**Tính năng:**

- **Bluetooth 2D Imager Integration:**
  - **Thiết bị:** 2D Area Imager là thiết bị ngoại vi riêng, phải mua ($200-400/thiết bị)
  - **Điện thoại:** Vẫn là thiết bị chính chạy app, 2D Imager chỉ là thiết bị quét ngoại vi
  - **Kết nối:** Tích hợp qua Bluetooth HID (keyboard mode) - Không cần SDK phức tạp
  - **Auto-detect:** App tự động phát hiện và dùng 2D Imager nếu có kết nối
  - **Fallback mechanism:** Tự động chuyển về camera nếu mất kết nối hoặc không có 2D Imager
  - **Tương thích với:** Zebra DS2208, Honeywell CT60, Datalogic QuickScan
  - **Không bắt buộc:** Nếu không có 2D Imager, app vẫn hoạt động với camera phone (Free Tier)

- **Performance Improvements:**
  - Tốc độ quét: < 100ms (5x nhanh hơn camera phone)
  - Đọc barcode hỏng: Tốt hơn camera phone (có LED illumination)
  - Hoạt động trong ánh sáng yếu: Tốt hơn camera phone
  - Đọc nhiều loại barcode: QR Code, Data Matrix, PDF417, Aztec, EAN-13, Code 128, UPC-A

- **Architecture Benefits:**
  - **Giữ nguyên core:** Vẫn dùng barcode/QR, không thay đổi data model
  - **Same business logic:** Dùng chung Rust core (validation, FEFO/FIFO)
  - **Zero learning curve:** Công nhân không cần thay đổi workflow
  - **Seamless fallback:** Nếu mất kết nối → Tự động dùng camera

**User Flow:**

1. **Kho lớn mua 2D Area Imager** ($200-400/thiết bị) - Thiết bị ngoại vi riêng
2. **Kết nối Bluetooth** với điện thoại (điện thoại vẫn là thiết bị chính chạy app)
3. **App tự động detect** và dùng 2D Imager nếu có kết nối
4. **Công nhân quét như bình thường** (không thay đổi workflow)
5. **Nếu mất kết nối hoặc không có 2D Imager** → App tự động fallback về camera phone

**Lưu ý:**

- 2D Area Imager là thiết bị ngoại vi riêng, phải mua
- Điện thoại vẫn là thiết bị chính chạy app
- Nếu không có 2D Imager, app vẫn hoạt động bình thường với camera phone (Free Tier)

**Business Value:**

- Mở rộng sang Segment 2 (Kho lớn) mà không phá vỡ core architecture
- Tốc độ quét nhanh hơn 5x → Tăng năng suất cho kho lớn
- Đọc barcode hỏng tốt hơn → Giảm lỗi quét
- Professional Tier pricing → Monetization từ kho lớn

**Technical Implementation:**

- Rust Core: Scanner abstraction trait
- React Native: Auto-detect và fallback mechanism
- Bluetooth HID: Listen keyboard events (không cần SDK)
- Same data flow: Camera Scanner → Business Logic → Redux Store

### 4.7. Pricing Strategy

**Phase 2 có 2 tiers:**

- **Free Tier:** Tất cả tính năng Operational ERP (Production, Procurement, Sales, QC, Replenishment, Dashboard) - Tiếp tục thu thập dữ liệu
- **Professional Tier (Paid):** 2D Area Imager support - Thu phí từ kho lớn cần tốc độ quét cao

---

## 5. PHASE 3: FINANCIAL ERP & SYNC (Quản trị & Tài chính) - 💰 THU PHÍ (PAID)

### 5.1. Mục tiêu

Chuyển hóa dữ liệu kho thành dữ liệu tiền tệ. Đây là lúc WMS chính thức trở thành ERP, cung cấp khả năng quản trị tài chính hoàn chỉnh.

### 5.2. Module Kế toán (Functional Accounting)

**Mục đích:** Tự động hạch toán từ các sự kiện Kho/Sản xuất (Event-driven), không cần nhập liệu thủ công.

**Tính năng:**

- **General Ledger (Sổ cái tổng hợp):**
  - Tự động hạch toán từ các sự kiện:
    - **Inbound:** Tăng Tồn kho, Tăng Công nợ nhà cung cấp (nếu chưa thanh toán)
    - **Outbound:** Giảm Tồn kho, Tăng Công nợ khách hàng (nếu chưa thanh toán)
    - **Production:** Chuyển giá trị từ Nguyên liệu sang Thành phẩm
    - **Adjust:** Điều chỉnh giá trị tồn kho
  - Event-driven architecture: Mỗi sự kiện kho tự động tạo bút toán kế toán
  - Functional Rust Core đảm bảo tính toán chính xác tuyệt đối

- **Accounts Payable (Công nợ phải trả):**
  - Quản lý công nợ nhà cung cấp
  - Tự động tạo công nợ khi nhận hàng (từ Purchase Order)
  - Theo dõi thanh toán và số dư công nợ
  - Báo cáo công nợ theo nhà cung cấp

- **Accounts Receivable (Công nợ phải thu):**
  - Quản lý công nợ khách hàng
  - Tự động tạo công nợ khi xuất hàng (từ Sales Order)
  - Theo dõi thanh toán và số dư công nợ
  - Báo cáo công nợ theo khách hàng

- **Costing (Tính giá thành):**
  - **Standard Cost:** Giá thành chuẩn (định trước)
  - **Moving Average:** Giá thành bình quân (tự động tính từ lịch sử nhập)
  - Tính giá vốn hàng bán (COGS) tự động khi xuất hàng
  - Báo cáo lãi/lỗ theo sản phẩm, đơn hàng

**User Flow:**

1. Worker xuất hàng theo Outbound Order (Phase 1 workflow)
2. Hệ thống tự động:
   - Trừ tồn kho (WMS)
   - Tính giá vốn (COGS) dựa trên giá thành hiện tại
   - Tạo bút toán: Nợ COGS, Có Tồn kho
   - Tạo công nợ khách hàng (nếu chưa thanh toán)
3. Manager xem báo cáo tài chính trên Dashboard

**Business Value:**

- Tự động hóa kế toán, không cần nhập liệu thủ công
- Tính toán chính xác giá vốn và lãi/lỗ
- Quản lý công nợ hiệu quả
- Báo cáo tài chính real-time

**Technical Note cho Engineers:**
Khi thiết kế bảng `Inventory_Transactions` trong Phase 1, hãy thêm trường `value_at_time` (giá trị hàng lúc đó). Sau này module Kế toán chỉ cần quét bảng này là tính được COGS mà không cần sửa cấu trúc dữ liệu cũ.

### 5.3. Sync & Distributed System (0-Downtime)

**Mục đích:** Đồng bộ dữ liệu real-time giữa các chi nhánh qua Cloud, đảm bảo hệ thống hoạt động liên tục.

**Tính năng:**

- **Cross-device Sync:**
  - Đồng bộ dữ liệu real-time giữa nhiều thiết bị
  - Đồng bộ giữa các chi nhánh qua Cloud
  - Conflict resolution thông minh (LWW cho location, CRDT cho quantity)

- **Local-first Sync Engine:**
  - Server sập app vẫn chạy bình thường (offline-first)
  - Có mạng tự động bù dữ liệu (sync queue)
  - Đảm bảo không mất dữ liệu khi có sự cố mạng

- **Multi-warehouse Support:**
  - Quản lý nhiều kho trong cùng một hệ thống
  - Chuyển hàng giữa các kho (Inter-warehouse Transfer)
  - Báo cáo tổng hợp từ nhiều kho

**Business Value:**

- Quản lý tập trung nhiều chi nhánh
- Đảm bảo tính liên tục của hệ thống
- Đồng bộ dữ liệu real-time, không bị trễ

### 5.4. Pricing Strategy

**Phase 3 là Professional/Enterprise Tier (Paid):**

- Thu phí theo subscription (monthly/annual)
- Pricing dựa trên số lượng users, số lượng warehouses
- Cung cấp support và training cho khách hàng trả phí

---

## 6. PHASE 4: AI-DRIVEN ERP (Trí tuệ nhân tạo) - 💰 THU PHÍ (PAID)

### 6.1. Mục tiêu

Tích hợp AI RAG Agent để biến ERP thành "Cursor cho doanh nghiệp" - một trợ lý ảo thông minh có thể trả lời câu hỏi, phân tích dữ liệu, và đưa ra đề xuất.

### 6.2. AI RAG Agents (Integrated Agent)

**Mục đích:** Sử dụng Retrieval-Augmented Generation (RAG) để tạo ra AI Agent hiểu rõ dữ liệu và quy trình của doanh nghiệp.

**Tính năng:**

- **Semantic Search (Tìm kiếm ngữ nghĩa):**
  - Tìm kiếm thông tin bằng ngôn ngữ tự nhiên
  - Ví dụ: "Còn bao nhiêu hàng tồn kho đủ sản xuất 100 đơn hàng A?"
  - Ví dụ: "Nhà cung cấp nào giao hàng chậm nhất trong tháng này?"
  - AI hiểu ngữ cảnh và trả lời chính xác

- **Auto-Auditor (Tự động kiểm toán):**
  - AI tự động rà soát các bút toán lệch
  - Phát hiện các hành vi xuất nhập kho bất thường
  - Cảnh báo khi có dấu hiệu gian lận hoặc sai sót
  - Gợi ý điều chỉnh để đảm bảo tính chính xác

- **Predictive Ordering (Dự báo đặt hàng):**
  - AI dự báo nhu cầu nhập hàng dựa trên lịch sử
  - Phân tích xu hướng tiêu thụ theo mùa, theo tuần
  - Gợi ý số lượng đặt hàng tối ưu để tránh thiếu/thừa hàng
  - Tự động tạo Purchase Order khi cần

- **Business Intelligence (Phân tích kinh doanh):**
  - AI phân tích dữ liệu và đưa ra insights
  - Ví dụ: "Sản phẩm A có xu hướng bán chạy vào cuối tuần"
  - Ví dụ: "Chi phí nguyên liệu tăng 10% so với tháng trước"
  - Gợi ý hành động để tối ưu hóa hoạt động

**User Flow:**

1. Manager mở AI Chat trên Dashboard
2. Nhập câu hỏi: "Tôi cần sản xuất 200 sản phẩm A, còn đủ nguyên liệu không?"
3. AI truy vấn database, phân tích BOM và tồn kho
4. Trả lời: "Thiếu 50kg nguyên liệu X. Bạn có muốn tạo Purchase Order không?"
5. Manager xác nhận → AI tự động tạo PO

**Business Value:**

- Tiết kiệm thời gian tìm kiếm và phân tích dữ liệu
- Phát hiện vấn đề sớm (thiếu hàng, lỗi kế toán)
- Đưa ra quyết định dựa trên dữ liệu (data-driven decisions)
- Tự động hóa các quy trình lặp lại

### 6.3. Advanced Automation

**Tính năng:**

- **Smart Put-away Optimization:**
  - AI gợi ý vị trí cất hàng tối ưu dựa trên lịch sử
  - Phân tích tần suất xuất hàng để đặt hàng thường dùng gần cửa kho
  - Giảm thời gian di chuyển trong quá trình picking

- **Predictive Analytics:**
  - Dự đoán nhu cầu hàng hóa theo mùa, theo tuần
  - Dự đoán thời điểm cần nhập hàng
  - Dự đoán rủi ro thiếu hàng hoặc tồn kho quá mức

- **Auto-Replenishment:**
  - Tự động tạo Replenishment Order khi forward pick < min_level
  - Tự động assign worker dựa trên vị trí và workload
  - Tối ưu hóa quy trình replenishment

**Business Value:**

- Tự động hóa hoàn toàn các quy trình lặp lại
- Tối ưu hóa hiệu quả kho
- Giảm thiểu rủi ro thiếu hàng hoặc tồn kho quá mức

### 6.4. Pricing Strategy

**Phase 4 là Enterprise Tier (Paid):**

- Thu phí cao hơn Phase 3 do giá trị AI mang lại
- Pricing dựa trên số lượng queries, số lượng warehouses
- Cung cấp custom AI training cho khách hàng lớn

---

## 7. BUSINESS MODEL (Bản đồ chi phí)

### 7.1. Free Tier (WMS Core)

**Đối tượng:** Hộ kinh doanh cá thể, xưởng nhỏ

**Tính năng:**

- ✅ App Mobile quét mã (Phase 1)
- ✅ Quản lý kho nội bộ 1 máy đơn lẻ
- ✅ Inbound, Outbound, Counting, Put-away
- ✅ Production Management (Phase 2)
- ✅ Procurement & Sales (Phase 2)
- ✅ QC & Replenishment (Phase 2)
- ✅ Management Dashboard (Phase 2)

**Hạn chế:**

- ❌ Không có Cross-device Sync
- ❌ Không có Multi-warehouse
- ❌ Không có Financial Accounting
- ❌ Không có AI Features

**Mục đích:** Thu thập dữ liệu, xây dựng cộng đồng

### 7.2. Professional Tier (Paid Scanner & Sync) - Phase 2-3

**Đối tượng:** Doanh nghiệp vừa và nhỏ, kho lớn (100+ công nhân), nhiều chi nhánh

**Tính năng Phase 2:**

- ✅ Tất cả tính năng Free Tier
- ✅ **2D Area Imager Support** (Bluetooth integration)
  - Tốc độ quét < 100ms (5x nhanh hơn camera phone)
  - Đọc barcode hỏng tốt hơn
  - Hoạt động trong ánh sáng yếu
  - Auto-detect và fallback mechanism

**Tính năng Phase 3:**

- ✅ Cross-device Sync (đồng bộ nhiều máy)
- ✅ Multi-warehouse Support
- ✅ Real-time Dashboard (Web/Desktop - Tauri)
- ✅ Advanced Reporting với Export

**Pricing:**

- **Phase 2:** Subscription: $X/tháng hoặc $Y/năm (cho 2D Imager support)
- **Phase 3:** Subscription: $X/tháng hoặc $Y/năm (cho sync và multi-warehouse)
- Dựa trên số lượng users, số lượng warehouses

**Mục đích:**

- Phase 2: Monetization từ kho lớn cần tốc độ quét cao
- Phase 3: Monetization từ sync và multi-warehouse

### 7.3. Enterprise Tier (Paid AI & Finance) - Phase 3-4

**Đối tượng:** Doanh nghiệp lớn, cần quản trị tài chính và AI

**Tính năng:**

- ✅ Tất cả tính năng Professional Tier
- ✅ Financial Accounting (General Ledger, AP/AR, Costing)
- ✅ AI RAG Agents (Semantic Search, Auto-Auditor, Predictive Ordering)
- ✅ Advanced Automation (Smart Put-away, Predictive Analytics)
- ✅ Custom AI Training
- ✅ Priority Support

**Pricing:**

- Subscription: $Z/tháng hoặc $W/năm (cao hơn Professional)
- Dựa trên số lượng users, warehouses, AI queries

**Mục đích:** Monetization từ AI và Financial features

---

## 8. SYSTEM DESIGN FOR ERP (Thiết kế hệ thống để scale)

### 8.1. Architecture Principles

Để lộ trình này không bị Technical Debt, chúng ta áp dụng các nguyên tắc sau:

| Thành phần     | Chiến lược               | Lợi ích cho ERP                                                                                                                      |
| -------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| **Logic**      | **Functional Rust Core** | Tính toán tiền tệ, thuế, tồn kho chính xác tuyệt đối, không có side effects. Dùng chung giữa Mobile và Desktop.                      |
| **Giao tiếp**  | **Event-Driven**         | Khi Kho xuất hàng, một "Event" được bắn ra. Module Kế toán chỉ việc nghe và tăng công nợ khách hàng. Loose coupling giữa các module. |
| **Dữ liệu**    | **Modular Schema**       | Mỗi module (Kho, Kế toán, AI) sở hữu bảng riêng, giao tiếp qua ID. Dễ dàng tách ra microservices khi cần.                            |
| **Deployment** | **Modular Monolith**     | Một bộ code duy nhất nhưng bật/tắt tính năng theo License (Free/Paid). Dễ maintain, dễ scale.                                        |

### 8.2. Data Flow Architecture

**Event-Driven Flow:**

1. **Operational Event** (Phase 1-2): Worker xuất hàng → Event "OutboundCompleted"
2. **Financial Event** (Phase 3): Event listener trong Accounting module → Tự động tạo bút toán
3. **AI Event** (Phase 4): Event listener trong AI module → Cập nhật training data

**Functional Core:**

- Tất cả business logic (tính toán tồn kho, giá thành, thuế) nằm trong Rust Core
- Mobile và Desktop đều gọi cùng một Rust Core
- Đảm bảo tính nhất quán và chính xác

**Modular Schema:**

- `inventory_*` tables: WMS module
- `accounting_*` tables: Financial module
- `ai_*` tables: AI module
- Giao tiếp qua foreign keys, không có tight coupling

### 8.3. Scalability Considerations

**Database:**

- ✅ Schema đã được thiết kế với khả năng scale
- ✅ Indexes đã được tối ưu
- ✅ Hỗ trợ phân vùng (Partitioning) nếu cần
- ✅ Có thể migrate sang PostgreSQL/MySQL nếu cần

**API:**

- ✅ RESTful API đã được định nghĩa rõ ràng
- ✅ Hỗ trợ versioning (`/api/v1/`)
- ✅ Có thể thêm GraphQL nếu cần
- ✅ WebSocket cho real-time updates

**Mobile App:**

- ✅ Offline-first architecture
- ✅ Có thể scale lên hàng nghìn thiết bị
- ✅ Hỗ trợ nhiều ngôn ngữ (i18n)

**Backend:**

- ✅ Microservices-ready (có thể tách thành services riêng)
- ✅ Có thể thêm message queue (RabbitMQ, Kafka) nếu cần
- ✅ Có thể thêm caching layer (Redis) nếu cần

---

## 9. ƯU TIÊN PHÁT TRIỂN (Development Priorities)

### 9.1. High Priority (Ưu tiên cao)

1. **Phase 1 - MVP Completion** (Đang thực hiện)
   - Hoàn thiện các tính năng WMS core
   - Testing và bug fixes
   - Beta testing với users thật

2. **Phase 2 - Operational ERP Core** (Sau khi MVP hoàn thành)
   - Production Management (BOM, Work Orders)
   - Procurement & Sales (PO, SO)
   - Management Dashboard

3. **Phase 2 - Enterprise Workflows**
   - Quality Control Workflow
   - Replenishment

### 9.2. Medium Priority (Ưu tiên trung bình)

1. **Phase 2 - Advanced Features**
   - Advanced Reporting với Export
   - User Management nâng cao
   - Multi-language support

2. **Phase 3 - Financial ERP** (Khi có đủ users)
   - General Ledger
   - Accounts Payable/Receivable
   - Costing

3. **Phase 3 - Sync & Multi-warehouse**
   - Cross-device Sync
   - Multi-warehouse Support

### 9.3. Low Priority / Conditional (Ưu tiên thấp / Có điều kiện)

1. **Phase 4 - AI Features** (Khi có đủ dữ liệu)
   - AI RAG Agents
   - Predictive Analytics
   - Advanced Automation

2. **Phase 3 - Wave Management** (Khi có >50 doanh nghiệp)
   - Order grouping & optimization
   - Chỉ implement khi thực sự cần thiết

---

## 10. LƯU Ý QUAN TRỌNG

### 10.1. NÊN (Do's)

✅ **Phát triển từng phase một cách có hệ thống**

- Không nhảy cóc phase
- Đảm bảo mỗi phase hoàn thiện trước khi chuyển sang phase tiếp theo

✅ **Test kỹ trước khi chuyển phase**

- Unit tests, integration tests
- Beta testing với users thật
- Performance testing

✅ **Lấy feedback từ người dùng thật**

- User interviews
- Analytics data
- Support tickets

✅ **Cập nhật tài liệu này khi có thay đổi**

- Version control cho documentation
- Changelog rõ ràng

✅ **Thiết kế với tư duy ERP từ đầu**

- Thêm `value_at_time` vào `Inventory_Transactions` ngay từ Phase 1
- Event-driven architecture để dễ mở rộng
- Modular schema để dễ tách module

### 10.2. KHÔNG NÊN (Don'ts)

❌ **Nhảy cóc phase**

- Không bỏ qua Phase 2, làm Phase 3
- Mỗi phase là nền tảng cho phase tiếp theo

❌ **Thêm tính năng không có trong roadmap**

- Tránh feature creep
- Nếu có yêu cầu mới, cập nhật roadmap trước

❌ **Bỏ qua testing và documentation**

- Technical debt sẽ tích lũy
- Khó maintain và scale

❌ **Thiết kế quá phức tạp cho tương lai**

- YAGNI principle
- Chỉ implement những gì cần thiết cho phase hiện tại
- Nhưng thiết kế với khả năng mở rộng

---

## 11. INDUSTRY VERTICAL EXTENSIONS (Mở rộng đa ngành)

### 11.1. Core Philosophy: Universal Tracking Engine

Chúng tôi xây dựng Lean WMS không chỉ cho kho hàng. Core Technology (Rust + Redux Toolkit) được thiết kế xoay quanh tư duy trừu tượng: **"Quản lý trạng thái và vị trí của thực thể theo thời gian thực"**.

Điều này cho phép tái sử dụng 80% code base để tấn công các thị trường ngách khác (Verticals) chỉ bằng cách thay đổi "Lớp da" (UI/Terminology):

| Ngành      | Entity (Thực thể)  | Location (Vị trí) | Action (Hành động)    |
| ---------- | ------------------ | ----------------- | --------------------- |
| **WMS**    | Hàng hóa (SKU)     | Kệ (Bin)          | Nhập / Xuất / Tồn     |
| **HIS**    | Bệnh nhân / Thuốc  | Giường / Phòng    | Nhập viện / Xuất viện |
| **Asset**  | Máy móc / Thiết bị | Công trường / Xe  | Bàn giao / Bảo trì    |
| **Retail** | Sản phẩm           | Cửa hàng / Online | Bán / Đổi trả         |

### 11.2. Lean HIS (Hospital Information System Lite)

**Bài toán:** Bệnh viện nhỏ/phòng khám tư nhân thường mất kiểm soát vật tư tiêu hao (thuốc, găng tay) và khó theo dõi luồng di chuyển của bệnh nhân/hồ sơ bệnh án.

**Giải pháp:**

- **Patient Tracking:** Dán QR code lên vòng tay bệnh nhân. Quét để biết bệnh nhân đang ở phòng nào, trạng thái chờ (Waiting) hay đang khám (Processing).
- **Inventory:** Quản lý thuốc/vật tư y tế theo hạn sử dụng (FEFO cực kỳ quan trọng trong y tế).
- **Asset Tracking:** Quản lý xe đẩy thuốc, máy thở đang nằm ở tầng nào.

### 11.3. Lean Asset (Construction & Rental)

**Bài toán:** Các công ty xây dựng/cho thuê sự kiện thường mất mát công cụ dụng cụ (khoan, máy cắt) khi di chuyển giữa các công trường.

**Giải pháp:**

- **Check-in/Check-out:** Quét mã QR trên máy khoan để bàn giao cho công nhân A.
- **Maintenance Schedule:** Cảnh báo khi thiết bị đến hạn bảo trì (tương tự Expiry Date trong WMS).
- **Site Transfer:** Quản lý việc chuyển máy móc từ Công trường A sang Công trường B.

---

## 12. CẬP NHẬT (Changelog)

**Version:** 3.1  
**Ngày tạo:** 28-12-2025
**Ngày cập nhật cuối:** 09-01-2026
**Người cập nhật:** theboysavior

**Lịch sử cập nhật:**

- **v3.1 (09-01-2026):** Update status and dates
  - ✅ Verified Tech Stack (React 19, RSPC)
  - ✅ Confirm Monorepo Structure

- **v3.1 (04-01-2026):** Thêm Industry Vertical Extensions
  - ✅ Thêm Lean HIS (Hospital Information System Lite)
  - ✅ Thêm Lean Asset (Construction & Rental)
  - ✅ Thêm Lean Retail (Point of Sale)
  - ✅ Thêm Lean Manufacturing (MES Lite)
  - ✅ Thêm Lean Supply Chain (Procurement & Sales)

- **v1.0 (2025):** Tạo tài liệu ban đầu

---

**Tài liệu này sẽ được cập nhật thường xuyên khi có yêu cầu mới hoặc thay đổi ưu tiên.**
