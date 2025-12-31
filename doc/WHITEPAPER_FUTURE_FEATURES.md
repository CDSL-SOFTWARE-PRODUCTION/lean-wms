# LEAN WMS - WHITEPAPER: ROADMAP & FUTURE FEATURES
## Tài liệu định hướng phát triển tính năng

**Mục tiêu:** Định nghĩa các tính năng sẽ được thêm vào trong các phase tiếp theo, đảm bảo khả năng mở rộng và scale của hệ thống.

**Lưu ý:** Tài liệu này sẽ được cập nhật thường xuyên khi có yêu cầu mới hoặc thay đổi ưu tiên.

**Audience:** Product Managers, Frontend Engineers, Backend Engineers, Designers

---

## QUICK REFERENCE (Tóm tắt nhanh)

### Enterprise Processes Mapping

**Đã có trong Phase 1:**
- ✅ Receiving/Inbound → Inbound flow
- ✅ Directed Put-away → Put-away với validation
- ✅ Order Picking → Guided Outbound workflow
- ✅ Cycle Counting → Blind Count
- ✅ Container/LPN Tracking → Container management
- ✅ Batch/Lot Tracking → Batch_no, expiry_date
- ✅ FEFO/FIFO → FEFO/FIFO algorithms

**Sẽ thêm trong Phase 2:**
- 🆕 Quality Control Workflow → QC Inspection với Quarantine (Section 2.7)
- 🆕 Replenishment → Chuyển hàng reserve → forward pick (Section 2.6)

**Sẽ thêm trong Phase 3 (có điều kiện):**
- 🔮 Wave Management → Phân nhóm orders tối ưu (Section 3.3) - Khi >50 doanh nghiệp

### Tài liệu liên quan

- **Frontend Engineers:** Xem `doc/BLUEPRINT_DESIGNER_FRONTEND.md` cho UI/UX specs
- **Backend Engineers:** Xem `doc/BLUEPRINT_ENGINEER_BACKEND.md` cho technical specs và business logic
- **Product Team:** Xem `PRD.md` cho product requirements

---

## PHASE 1: MVP (Minimum Viable Product) - ✅ ĐANG PHÁT TRIỂN

### Mục tiêu
Tạo app mobile cho công nhân kho thực hiện các thao tác cơ bản: Nhập kho, Xuất kho, Kiểm kê.

### Tính năng đã có
- ✅ Quét mã QR/Barcode
- ✅ Nhập kho (Inbound)
- ✅ Xuất kho (Outbound)
- ✅ Kiểm kê (Counting)
- ✅ Tạo mã mới & Gán mã vào hàng (Mapping)
- ✅ Offline-first architecture
- ✅ Sync dữ liệu
- ✅ Poka-Yoke (Chống sai lỗi)

### Giới hạn Phase 1
- ❌ Chưa có Dashboard quản lý
- ❌ Chưa có CRUD sản phẩm trên mobile
- ❌ Chưa có báo cáo
- ❌ Chưa có quản lý người dùng

---

## PHASE 2: MANAGEMENT DASHBOARD - 📋 KẾ HOẠCH

### Mục tiêu
Xây dựng hệ thống quản lý web-based cho Chủ xưởng/Quản lý để quản lý kho hàng, không phụ thuộc vào app mobile. Bổ sung các quy trình enterprise còn thiếu để hoàn thiện hệ thống WMS.

### Enterprise Process Mapping (Ánh xạ quy trình Enterprise)

**Mục đích:** Làm rõ mapping giữa quy trình WMS của nhà máy lớn (Samsung, Amazon) với tính năng của Lean WMS để Frontend/Backend Engineers hiểu rõ yêu cầu.

#### Các quy trình Enterprise đã implement trong Phase 1:

| Enterprise Process | Lean WMS Feature | Status | Notes |
|-------------------|------------------|--------|-------|
| **Receiving/Inbound** | Inbound flow với barcode mapping | ✅ Phase 1 | Đã có đầy đủ: quét mã, mapping linh hoạt, STAGING |
| **Directed Put-away** | Put-away với validation | ✅ Phase 1 | Đã có: Fixed/Mixed bin validation, capacity checking |
| **Order Picking** | Guided Outbound workflow | ✅ Phase 1 | Đã có: Guided workflow, FEFO/FIFO, Poka-Yoke |
| **Cycle Counting** | Blind Count | ✅ Phase 1 | Đã có: Blind count, so sánh với sổ sách |
| **Container/LPN Tracking** | Container management với nesting | ✅ Phase 1 | Đã có: LPN tracking, nested containers |
| **Batch/Lot Tracking** | Batch_no, expiry_date tracking | ✅ Phase 1 | Đã có trong Inventory_Items |
| **FEFO/FIFO** | FEFO/FIFO algorithms | ✅ Phase 1 | Đã implement trong Outbound |

#### Các quy trình Enterprise sẽ bổ sung trong Phase 2:

| Enterprise Process | Lean WMS Feature | Priority | Implementation Notes |
|-------------------|------------------|----------|---------------------|
| **Quality Control Workflow** | QC Inspection workflow | High | Xem section 2.7 - Dedicated QC process với quarantine |
| **Replenishment** | Replenishment automation | High | Xem section 2.6 - Chuyển hàng từ reserve → forward pick |
| **Inventory Adjustments** | Adjust workflow với approval | Medium | Xem section 2.2 - Đã có cơ bản, nâng cấp approval workflow |

#### Các quy trình Enterprise sẽ bổ sung trong Phase 3:

| Enterprise Process | Lean WMS Feature | Điều kiện | Notes |
|-------------------|------------------|----------|-------|
| **Wave Management** | Order grouping & optimization | Khi có >50 doanh nghiệp | Xem section 3.3 - Phân nhóm orders để tối ưu picking |
| **Slotting Optimization** | Smart Put-away suggestions | Phase 3 | AI/ML suggest optimal location |
| **Labor Management** | Performance metrics & tracking | Phase 3 | Đã có cơ bản, nâng cấp thành analytics |

**Lưu ý cho Engineers:**
- Tất cả các tính năng Phase 2 nên tuân theo patterns đã có trong Phase 1
- Sử dụng cùng database schema và API structure
- Maintain offline-first capability nếu feature được implement trên mobile app
- Frontend/Backend cần reference `doc/BLUEPRINT_ENGINEER_BACKEND.md` để hiểu business logic chi tiết

### Tính năng dự kiến

#### 2.1. Quản lý Sản phẩm (SKU Management)
- **CRUD SKU:**
  - Tạo SKU mới
  - Sửa thông tin SKU (tên, đơn vị, giá)
  - Xóa SKU (soft delete)
  - Xem chi tiết SKU
- **Quản lý Mapping:**
  - Xem danh sách mapping mã vạch
  - Tạo mapping mới (gán mã vào SKU)
  - Vô hiệu hóa mapping cũ
  - Import mapping từ Excel
- **Lịch sử thay đổi:**
  - Audit log mọi thay đổi SKU
  - Xem ai thay đổi, lúc nào

#### 2.2. Quản lý Tồn kho (Inventory Management)
- **Xem tổng quan:**
  - Tổng số SKU
  - Tổng giá trị tồn kho
  - Số lượng hàng sắp hết (Low stock alert)
  - Số lượng hàng hết hạn (Expired items)
- **Xem chi tiết:**
  - Tồn kho theo SKU
  - Tồn kho theo vị trí (Location)
  - Tồn kho theo lô (Batch)
  - Lịch sử xuất nhập
- **Điều chỉnh số lượng (Adjust):**
  - Chỉ dành cho Manager
  - Ghi nhận lý do điều chỉnh
  - Yêu cầu phê duyệt (nếu số lượng lớn)

#### 2.3. Quản lý Đơn hàng (Order Management)
- **Tạo đơn hàng:**
  - Sales Order (Đơn xuất bán)
  - Production Order (Đơn sản xuất)
  - Transfer Order (Đơn chuyển kho)
- **Theo dõi tiến độ:**
  - Xem trạng thái đơn hàng (Pending, In Progress, Completed)
  - Xem số lượng đã lấy / số lượng yêu cầu
  - Xem công nhân nào đang xử lý
- **Duyệt đơn hàng:**
  - Phê duyệt đơn hàng quan trọng
  - Hủy đơn hàng
  - Chỉnh sửa đơn hàng

#### 2.4. Báo cáo (Reporting)
- **Báo cáo tồn kho:**
  - Tồn kho hiện tại
  - Biến động tồn kho (theo ngày/tuần/tháng)
  - Hàng sắp hết
  - Hàng hết hạn
- **Báo cáo xuất nhập:**
  - Xuất nhập theo SKU
  - Xuất nhập theo thời gian
  - Top sản phẩm xuất/nhập nhiều nhất
- **Báo cáo lỗi:**
  - Số lần quét sai
  - Số lần điều chỉnh
  - Tỷ lệ lỗi theo công nhân
- **Export:**
  - Export Excel
  - Export PDF
  - Lên lịch gửi báo cáo tự động (Email)

#### 2.5. Quản lý Người dùng (User Management)
- **Quản lý tài khoản:**
  - Tạo tài khoản công nhân
  - Phân quyền (Worker, Manager, Admin)
  - Vô hiệu hóa tài khoản
- **Theo dõi hoạt động:**
  - Xem lịch sử thao tác của từng công nhân
  - Thống kê năng suất
  - Báo cáo lỗi theo người

#### 2.6. Replenishment (Bổ sung hàng) - 🆕 ENTERPRISE FEATURE

**Mục đích:** Tự động chuyển hàng từ khu vực lưu trữ dài hạn (reserve storage) sang khu vực lấy hàng (forward pick locations) khi forward pick sắp hết hàng.

**Enterprise Process Mapping:**
- Tương ứng với quy trình **Replenishment** trong WMS enterprise (Samsung, Amazon)
- Đảm bảo forward pick locations luôn có đủ hàng để công nhân lấy nhanh
- Giảm thời gian di chuyển trong quá trình picking

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

- **Tự động hóa (Optional - có thể Phase 3):**
  - Auto-create Replenishment Order khi forward pick < min_level
  - Auto-assign worker dựa trên vị trí và workload

**Database Schema mở rộng:**
- Thêm bảng `replenishment_orders` (tương tự `orders`)
- Thêm cột `location_type` (RESERVE, FORWARD_PICK) vào `locations`
- Thêm cột `min_level`, `max_level` vào `locations` (optional)

**API Endpoints cần thêm:**
- `GET /api/v1/replenishment-orders` - Lấy danh sách lệnh bổ sung
- `POST /api/v1/replenishment-orders` - Tạo lệnh bổ sung mới
- `POST /api/v1/replenishment-orders/{id}/execute` - Thực hiện lệnh (mobile)

**Notes cho Engineers:**
- Sử dụng cùng Outbound workflow pattern để đảm bảo consistency
- Validation tương tự Put-away (Fixed/Mixed bin, capacity)
- Sync mechanism giống các orders khác

#### 2.7. Quality Control Workflow (Quy trình Kiểm soát Chất lượng) - 🆕 ENTERPRISE FEATURE

**Mục đích:** Quy trình kiểm tra chất lượng hàng hóa chuyên nghiệp với workflow rõ ràng, đảm bảo hàng lỗi được xử lý đúng cách.

**Enterprise Process Mapping:**
- Tương ứng với quy trình **Quality Control/Inspection** trong WMS enterprise
- Phase 1 chỉ có trạng thái Đạt/Lỗi cơ bản trong Inbound
- Phase 2 nâng cấp thành workflow đầy đủ: Inspection → Quarantine → Disposition

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
  - Manager có thể:
    - Xem chi tiết lỗi
    - Quyết định Disposition:
      * **SCRAP:** Hủy hàng → Trừ kho, ghi nhận tổn thất
      * **RETURN:** Trả về nhà cung cấp
      * **REWORK:** Sửa chữa → Chuyển về quy trình sản xuất
      * **RELEASE:** Sau khi sửa, kiểm tra lại và release về AVAILABLE

- **Báo cáo QC:**
  - Tỷ lệ hàng đạt/không đạt theo SKU, nhà cung cấp
  - Top lỗi thường gặp
  - Chi phí tổn thất (nếu có giá trị)

**Database Schema mở rộng:**
- Thêm bảng `qc_orders` (tương tự `orders`)
- Thêm bảng `qc_results` để lưu kết quả kiểm tra
- Thêm enum `QUARANTINE` vào `status` của `inventory_items`
- Thêm cột `qc_status`, `qc_result_id` vào `inventory_items`

**API Endpoints cần thêm:**
- `GET /api/v1/qc-orders` - Lấy danh sách lệnh QC
- `POST /api/v1/qc-orders` - Tạo lệnh QC mới
- `POST /api/v1/qc-orders/{id}/inspect` - Ghi nhận kết quả QC (mobile)
- `POST /api/v1/quarantine/{id}/disposition` - Xử lý hàng quarantine

**Notes cho Engineers:**
- QC workflow nên có approval mechanism cho disposition decisions
- Hàng QUARANTINE phải bị block trong Outbound flow
- Cần audit log đầy đủ cho QC decisions (compliance)

### Công nghệ dự kiến
- **Frontend:** React + TypeScript (hoặc Vue.js)
- **Backend:** API đã có sẵn (từ Phase 1)
- **Database:** Tận dụng schema đã thiết kế
- **Deployment:** Web app, có thể responsive cho tablet

### Timeline ước tính
- **Thiết kế UI/UX:** 2-3 tuần
- **Phát triển Frontend:** 6-8 tuần
- **Tích hợp Backend:** 2-3 tuần
- **Testing & Bug fix:** 2-3 tuần
- **Tổng:** ~12-17 tuần (3-4 tháng)

---

## PHASE 3: ADVANCED FEATURES - 🔮 TƯƠNG LAI XA

### Mục tiêu
Nâng cấp hệ thống với các tính năng nâng cao, tối ưu hóa quy trình và tự động hóa.

### Tính năng dự kiến

#### 3.1. Tự động hóa (Automation)
- **Auto Reorder Point:**
  - Tự động cảnh báo khi tồn kho xuống dưới ngưỡng
  - Tự động tạo đơn nhập hàng
- **Smart Put-away:**
  - Gợi ý vị trí cất hàng tối ưu (dựa trên lịch sử)
  - Tự động sắp xếp hàng theo FEFO/FIFO
- **Predictive Analytics:**
  - Dự đoán nhu cầu hàng hóa
  - Dự đoán thời điểm cần nhập hàng

#### 3.2. Tích hợp (Integration)
- **ERP Integration:**
  - Kết nối với hệ thống ERP có sẵn
  - Đồng bộ dữ liệu 2 chiều
- **E-commerce Integration:**
  - Tích hợp với Shopify, WooCommerce
  - Tự động cập nhật tồn kho khi có đơn hàng
- **Accounting Integration:**
  - Kết nối với phần mềm kế toán
  - Tự động hạch toán xuất nhập

#### 3.3. Nâng cao (Advanced Features)
- **Multi-warehouse:**
  - Quản lý nhiều kho
  - Chuyển hàng giữa các kho
- **Serial Number Tracking:**
  - Theo dõi từng sản phẩm bằng serial number
  - Hữu ích cho hàng giá trị cao
- **Wave Management (Quản lý Đợt) - 🆕 ENTERPRISE FEATURE:**
  
  **Mục đích:** Phân nhóm orders thành các "waves" (đợt) để tối ưu hóa quá trình picking, giảm thời gian di chuyển và tăng hiệu quả.
  
  **Enterprise Process Mapping:**
  - Tương ứng với quy trình **Wave Management** trong WMS enterprise (Amazon, Samsung)
  - Chỉ cần thiết khi scale lớn (nhiều orders, nhiều workers đồng thời)
  
  **Điều kiện triển khai:**
  - Khi hệ thống đã có **>50 doanh nghiệp** sử dụng
  - Hoặc khi doanh nghiệp có **>100 orders/ngày** và **>10 workers** đồng thời
  
  **Tính năng:**
  - **Tạo Wave:**
    - Manager hoặc hệ thống tự động nhóm orders thành wave
    - Criteria phân nhóm:
      * Theo khu vực (zone-based wave)
      * Theo thời gian xuất hàng (shipment time)
      * Theo ưu tiên (priority-based)
      * Theo loại hàng (product category)
  
  - **Wave Optimization:**
    - Tự động sắp xếp thứ tự picking để giảm quãng đường di chuyển
    - Phân bổ orders vào workers dựa trên workload
    - Tối ưu route picking (pick path optimization)
  
  - **Wave Execution:**
    - Worker nhận wave assignment từ app
    - App hiển thị tất cả orders trong wave cùng lúc
    - Guided workflow với route tối ưu
    - Real-time tracking tiến độ wave
  
  - **Wave Completion:**
    - Tự động đóng wave khi tất cả orders hoàn thành
    - Báo cáo hiệu quả wave (thời gian, năng suất)
  
  **Lưu ý:**
  - Feature này KHÔNG cần thiết cho kho nhỏ (<10 workers, <50 orders/ngày)
  - Nên đánh giá nhu cầu thực tế từ users trước khi implement
  - Có thể bắt đầu với manual wave creation, sau đó mới auto-optimize
  
- **Mobile App cho Manager:**
  - App quản lý trên mobile
  - Xem báo cáo, duyệt đơn hàng trên điện thoại

#### 3.4. AI & Machine Learning
- **Image Recognition:**
  - Nhận diện sản phẩm bằng hình ảnh (không cần mã vạch)
  - Phát hiện hàng lỗi bằng camera
- **Optimization:**
  - Tối ưu hóa vị trí cất hàng
  - Tối ưu hóa lộ trình nhặt hàng (Picking route)

---

## KIẾN TRÚC MỞ RỘNG (Scalability)

### Database
- ✅ Schema đã được thiết kế với khả năng scale
- ✅ Indexes đã được tối ưu
- ✅ Hỗ trợ phân vùng (Partitioning) nếu cần
- ✅ Có thể migrate sang PostgreSQL/MySQL nếu cần

### API
- ✅ RESTful API đã được định nghĩa rõ ràng
- ✅ Hỗ trợ versioning (`/api/v1/`)
- ✅ Có thể thêm GraphQL nếu cần
- ✅ Có thể thêm WebSocket cho real-time updates

### Mobile App
- ✅ Offline-first architecture
- ✅ Có thể scale lên hàng nghìn thiết bị
- ✅ Hỗ trợ nhiều ngôn ngữ (i18n)
- ✅ Có thể thêm tablet support

### Backend
- ✅ Microservices-ready (có thể tách thành services riêng)
- ✅ Có thể thêm message queue (RabbitMQ, Kafka) nếu cần
- ✅ Có thể thêm caching layer (Redis) nếu cần

---

## ƯU TIÊN PHÁT TRIỂN

### High Priority (Ưu tiên cao)
1. **Phase 2 - Management Dashboard Core** (Sau khi MVP hoàn thành)
   - Quản lý SKU (Section 2.1)
   - Quản lý Tồn kho (Section 2.2)
   - Quản lý Đơn hàng (Section 2.3)
   - Báo cáo cơ bản (Section 2.4)

2. **Phase 2 - Enterprise Workflows** (Sau khi Dashboard Core hoàn thành)
   - Quality Control Workflow (Section 2.7) - Quan trọng cho compliance
   - Replenishment (Section 2.6) - Tối ưu hiệu quả kho

### Medium Priority (Ưu tiên trung bình)
3. **Phase 2 - Advanced Features**
   - Quản lý Người dùng (Section 2.5)
   - Advanced Reporting với Export Excel/PDF (Section 2.4)
   - Lên lịch gửi báo cáo tự động

4. **Phase 3 - Multi-warehouse**
   - Quản lý nhiều kho
   - Chuyển hàng giữa kho

### Low Priority (Ưu tiên thấp) / Conditional (Có điều kiện)
5. **Phase 3 - Wave Management** (Section 3.3)
   - Chỉ implement khi có >50 doanh nghiệp hoặc >100 orders/ngày
   - Đánh giá nhu cầu thực tế từ users trước

6. **Phase 3 - AI Features**
   - Image Recognition
   - Predictive Analytics
   - Smart Put-away optimization

7. **Phase 3 - ERP Integration**
   - Tích hợp với hệ thống bên ngoài

---

## LƯU Ý QUAN TRỌNG

✅ **NÊN:**
- Phát triển từng phase một cách có hệ thống
- Test kỹ trước khi chuyển phase
- Lấy feedback từ người dùng thật
- Cập nhật tài liệu này khi có thay đổi

❌ **KHÔNG NÊN:**
- Nhảy cóc phase (bỏ qua Phase 2, làm Phase 3)
- Thêm tính năng không có trong roadmap
- Bỏ qua testing và documentation

---

## CẬP NHẬT

**Version:** 2.0  
**Ngày tạo:** 2024  
**Ngày cập nhật cuối:** 2025-01-XX  
**Người cập nhật:** Team

**Lịch sử cập nhật:**
- v2.0 (2025-01): 
  - ✅ Thêm Enterprise Process Mapping section để làm rõ mapping với WMS enterprise
  - ✅ Thêm Replenishment workflow vào Phase 2 (Section 2.6)
  - ✅ Thêm Quality Control Workflow vào Phase 2 (Section 2.7)
  - ✅ Thêm Wave Management vào Phase 3 với điều kiện (>50 doanh nghiệp)
  - ✅ Cập nhật ưu tiên phát triển
  - ✅ Thêm Quick Reference section
- v1.0 (2024): Tạo tài liệu ban đầu

---

**Tài liệu này sẽ được cập nhật thường xuyên khi có yêu cầu mới hoặc thay đổi ưu tiên.**

